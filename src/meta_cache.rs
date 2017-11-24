#![allow(unused_imports)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::Bound;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use futures::{
    Future,
    Poll,
    future,
};
use futures::future::Either;
use parking_lot::Mutex;

use Error;
use HostPort;
use MasterErrorCode;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TableId;
use TabletId;
use TabletServerId;
use backoff::Backoff;
use master::MasterProxy;
use pb::ExpectField;
use pb::master::{
    MasterService,
    GetTableLocationsRequestPb,
    GetTableLocationsResponsePb,
    TabletLocationsPb,
};
use tablet::Tablet;

const MAX_RETURNED_TABLE_LOCATIONS: u32 = 10;

/// Backoff used for retrying after a `TABLET_NOT_RUNNING` error.
fn backoff() -> Backoff {
    Backoff::with_duration_range(100, 60_000_000)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    Tablet(Tablet),
    NonCoveredRange {
        partition_lower_bound: Vec<u8>,
        partition_upper_bound: Vec<u8>,
    }
}

impl Entry {

    fn non_covered_range(partition_lower_bound: Vec<u8>, partition_upper_bound: Vec<u8>) -> Entry {
        Entry::NonCoveredRange {
            partition_lower_bound: partition_lower_bound,
            partition_upper_bound: partition_upper_bound,
        }
    }

    fn is_tablet(&self) -> bool {
        match *self {
            Entry::Tablet(_) => true,
            Entry::NonCoveredRange { .. } => false,
        }
    }

    fn partition_lower_bound(&self) -> &[u8] {
        match *self {
            Entry::Tablet(ref tablet) => tablet.partition().lower_bound_key(),
            Entry::NonCoveredRange { ref partition_lower_bound, .. } => partition_lower_bound,
        }
    }

    fn partition_upper_bound(&self) -> &[u8] {
        match *self {
            Entry::Tablet(ref tablet) => tablet.partition().upper_bound_key(),
            Entry::NonCoveredRange { ref partition_upper_bound, .. } => partition_upper_bound,
        }
    }

    fn contains_partition_key(&self, partition_key: &[u8]) -> bool {
        let upper_bound = self.partition_upper_bound();
        (upper_bound.is_empty() || partition_key < upper_bound)
            && partition_key >= self.partition_lower_bound()
    }

    /// Returns `Ordering::Equal` if the entries intersect, `Ordering::Less` if this entry falls
    /// before the other entry, or `Ordering::Greater` if this entry falls after the other entry.
    fn cmp_entry(&self, other: &Entry) -> Ordering {
        if !self.partition_upper_bound().is_empty() &&
            self.partition_upper_bound() <= other.partition_lower_bound() {
            Ordering::Less
        } else if !other.partition_upper_bound().is_empty() &&
                   other.partition_upper_bound() <= self.partition_lower_bound() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Returns `true` if the entries are `Tablet`s and the tablet IDs match, or if the entries are
    /// non-covered ranges and the lower and upper bounds match.
    fn equiv(&self, other: &Entry) -> bool {
        match (self, other) {
            (&Entry::Tablet(ref a), &Entry::Tablet(ref b)) => {
                // Sanity check that if the tablet IDs match, the ranges also match. If this fails,
                // something is very wrong (possibly in the server).
                debug_assert!(a.id() != b.id() ||
                              (a.partition().lower_bound_key() == b.partition().lower_bound_key() &&
                               a.partition().upper_bound_key() == b.partition().upper_bound_key()));
                a.id() == b.id()
            },
            (&Entry::NonCoveredRange { partition_lower_bound: ref a_lower,
                                       partition_upper_bound: ref a_upper },
             &Entry::NonCoveredRange { partition_lower_bound: ref b_lower,
                                       partition_upper_bound: ref b_upper }) => {
                a_lower == b_lower && a_upper == b_upper
            },
            _ => false,
        }
    }
}

/// TODO:
///     - Retry RPCS that fail with tablet not running.
///     - Limit the number of concurrent lookups, if it's not already being done by the master
///       proxy.
#[derive(Clone)]
pub(crate) struct MetaCache {
    master: MasterProxy,
    inner: Arc<Inner>,
}

struct Inner {
    table: TableId,
    primary_key_schema: Schema,
    partition_schema: PartitionSchema,
    entries: Mutex<BTreeMap<Vec<u8>, Entry>>,
}

impl MetaCache {

    pub fn new(table: TableId,
               primary_key_schema: Schema,
               partition_schema: PartitionSchema,
               master: MasterProxy)
               -> MetaCache {
        MetaCache {
            master: master,
            inner: Arc::new(Inner {
                table: table,
                primary_key_schema: primary_key_schema,
                partition_schema: partition_schema,
                entries: Mutex::new(BTreeMap::new()),
            })
        }
    }

    pub fn entry(&self, partition_key: Vec<u8>) -> impl Future<Item=Entry, Error=Error> {
        self.extract(partition_key, Entry::clone)
    }

    fn cached_entry(&self, partition_key: &[u8]) -> Option<Entry> {
        self.extract_cached(partition_key, &Entry::clone)
    }

    pub fn tablet_id(&self, partition_key: Vec<u8>) -> impl Future<Item=Option<TabletId>, Error=Error> {
        self.extract(partition_key, |entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                Some(tablet.id())
            } else {
                None
            }
        })
    }

    pub fn tablet_leader(&self, partition_key: Vec<u8>) -> impl Future<Item=Option<Box<[HostPort]>>, Error=Error> {
        self.extract(partition_key, |entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                tablet.replicas()
                      .iter()
                      .find(|replica| replica.role() == RaftRole::Leader)
                      .map(|replica| replica.rpc_addrs().to_owned().into_boxed_slice())
            } else {
                None
            }
        })
    }

    fn extract<Extractor, T>(&self,
                             partition_key: Vec<u8>,
                             extractor: Extractor) -> impl Future<Item=T, Error=Error>
    where Extractor: Fn(&Entry) -> T + Send + Sync {

        if let Some(value) = self.extract_cached(&*partition_key, &extractor) {
            return Either::A(future::ok(value));
        };

        let request = Arc::new(GetTableLocationsRequestPb {
            table: self.inner.table.into(),
            partition_key_start: Some(partition_key.clone()),
            partition_key_end: None,
            max_returned_locations: Some(MAX_RETURNED_TABLE_LOCATIONS),
        });
        let call = MasterService::get_table_locations(request,
                                                      Instant::now() + self.master.options().admin_timeout);

        let mut meta_cache = self.clone();
        Either::B(
            meta_cache
                .master
                .send(call)
                .and_then(move |response: GetTableLocationsResponsePb| {
                    meta_cache.add_tablet_locations(&*partition_key,
                                                    response.tablet_locations,
                                                    &extractor)
                }))
    }

    pub fn table(&self) -> TableId {
        self.inner.table
    }

    pub fn primary_key_schema(&self) -> &Schema {
        &self.inner.primary_key_schema
    }

    pub fn partition_schema(&self) -> &PartitionSchema {
        &self.inner.partition_schema
    }

    fn add_tablet_locations<Extractor, T>(&self,
                                          partition_key: &[u8],
                                          tablets: Vec<TabletLocationsPb>,
                                          extractor: Extractor) -> Result<T>
    where Extractor: Fn(&Entry) -> T + Send + Sync {
        let entries = self.tablet_locations_to_entries(partition_key, tablets)?;
        let index = match entries.binary_search_by_key(&partition_key, |entry| entry.partition_lower_bound()) {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        debug_assert!(entries[index].contains_partition_key(partition_key));
        let value = extractor(&entries[index]);

        self.splice_entries(entries);
        Ok(value)
    }

    fn extract_cached<Extractor, T>(&self, partition_key: &[u8], extractor: &Extractor) -> Option<T>
    where Extractor: Fn(&Entry) -> T {
        let entries = self.inner.entries.lock();
        match entries.range::<[u8], _>((Bound::Unbounded, Bound::Included(partition_key))).next_back() {
            Some(ref entry) if entry.1.contains_partition_key(partition_key) => Some(extractor(&entry.1)),
            _ => None,
        }
    }

    fn splice_entries(&self, entries: Vec<Entry>) {
        let mut left = self.inner.entries.lock();

        // NLL hack.
        let mut right = {
            let lower_bound = entries[0].partition_lower_bound();
            let upper_bound = entries[entries.len() - 1].partition_lower_bound();

            let mut right = left.split_off(lower_bound);

            if upper_bound.is_empty() {
                BTreeMap::new()
            } else {
                right.split_off(upper_bound)
            }
        };

        for entry in entries {
            left.insert(entry.partition_lower_bound().to_owned(), entry);
        }

        left.append(&mut right);
    }

    /// Converts the results of a `GetTableLocations` RPC to a set of entries for the meta cache.
    /// The entries are guaranteed to be contiguous in the partition key space. The partition key
    /// must match the partition key of the get table locations request. The request must not
    /// have an end key.
    fn tablet_locations_to_entries(&self,
                                   partition_key: &[u8],
                                   tablets: Vec<TabletLocationsPb>)
                                   -> Result<Vec<Entry>> {
        if tablets.is_empty() {
            // If there are no tablets in the response, then the table is empty. If
            // there were any tablets in the table they would have been returned, since
            // the master guarantees that the if the partition key falls in a
            // non-covered range, the previous tablet will be returned, and we did not
            // set an upper bound partition key on the request.
            let mut entries = Vec::with_capacity(1);
            entries.push(Entry::non_covered_range(Vec::new(), Vec::new()));
            return Ok(entries);
        }

        let tablet_count = tablets.len();
        let mut entries = Vec::with_capacity(tablets.len());
        let mut last_upper_bound: Vec<u8> = tablets[0].partition
                                                      .as_ref()
                                                      .expect_field("TabletLocationsPb", "partition")?
                                                      .partition_key_start()
                                                      .to_owned();

        if partition_key < &*last_upper_bound {
            // If the first tablet is past the requested partition key, then the partition key fell
            // in an initial non-covered range.
            entries.push(Entry::non_covered_range(Vec::new(), last_upper_bound.clone()));
        }

        for tablet in tablets {
            let tablet = Tablet::from_pb(&self.inner.primary_key_schema,
                                         self.inner.partition_schema.clone(),
                                         tablet)?;
            if tablet.partition().lower_bound_key() > &*last_upper_bound {
                entries.push(Entry::non_covered_range(last_upper_bound,
                                                      tablet.partition().lower_bound_key().to_owned()));
            }
            last_upper_bound = tablet.partition().upper_bound_key().to_owned();
            entries.push(Entry::Tablet(tablet));
        }

        if !last_upper_bound.is_empty() && tablet_count < MAX_RETURNED_TABLE_LOCATIONS as usize {
            entries.push(Entry::non_covered_range(last_upper_bound, Vec::new()));
        }

        trace!("{:?}: discovered entries: {:?}", self, entries);
        Ok(entries)
    }

    pub fn clear(&self) {
        self.inner.entries.lock().clear()
    }
}

impl fmt::Debug for MetaCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaCache")
         .field("table", &self.inner.table)
         .finish()
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};

    use env_logger;
    use tokio::reactor::Core;

    use Client;
    use ClientBuilder;
    use RangePartitionBound;
    use TableBuilder;
    use mini_cluster::MiniCluster;
    use schema::tests::simple_schema;

    use super::*;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn single_tablet() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut reactor = Core::new().unwrap();

        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("single_tablet", schema.clone());
        table_builder.set_range_partition_columns(vec!["key".to_owned()]);
        table_builder.set_num_replicas(1);

        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let table = reactor.run(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.meta_cache().clone();

        {
            let entries = cache.inner.entries.lock().clone();
            assert!(entries.is_empty());
        }

        {
            let entry = reactor.run(cache.entry(vec![])).expect("entry");

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.partition_lower_bound());
            assert_eq!(b"", entry.partition_upper_bound());

            let entries = cache.inner.entries.lock().clone();
            assert_eq!(1, entries.len());
            assert!(entry.equiv(entries.values().next().unwrap()));

            assert!(entry.equiv(&cache.cached_entry(b"").unwrap()));
            assert!(entry.equiv(&cache.cached_entry(b"foo").unwrap()));
        }

        cache.clear();
        {
            let entry = reactor.run(cache.entry(b"some-key".as_ref().to_owned())).expect("entry");

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.partition_lower_bound());
            assert_eq!(b"", entry.partition_upper_bound());
        }
    }

    #[test]
    fn multi_tablet() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut reactor = Core::new().unwrap();

        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let table = reactor.run(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.meta_cache().clone();

        let first = reactor.run(cache.entry(vec![0, 0, 0, 0, 1])).expect("entry");

        assert!(first.is_tablet());
        assert_eq!(b"", first.partition_lower_bound());
        assert_eq!(vec![0, 0, 0, 1], first.partition_upper_bound());

        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        assert_eq!(10, entries.len());
        assert!(first.equiv(&entries[0]));

        assert!(first.equiv(&cache.cached_entry(b"").unwrap()));
        assert!(cache.cached_entry(b"foo").is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());

        let last = reactor.run(cache.entry(vec![0, 0, 0, 11])).expect("entry");

        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();

        assert_eq!(11, entries.len());
        assert!(entries[10].equiv(&last));

        assert!(cache.cached_entry(b"foo").unwrap().equiv(&last));
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10, 5]).is_none());

        reactor.run(cache.entry(vec![0, 0, 0, 9])).expect("entry");
        assert_eq!(11, cache.inner.entries.lock().len());

        reactor.run(cache.entry(vec![0, 0, 0, 10])).expect("entry");
        assert_eq!(12, cache.inner.entries.lock().len());
    }

    #[test]
    fn multi_tablet_concurrent() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut reactor = Core::new().unwrap();

        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet_concurrent", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let table = reactor.run(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.meta_cache().clone();

        reactor.run(cache.entry(vec![0, 0, 0, 0]).join(cache.entry(vec![0, 0, 0, 9])))
               .expect("entry");
        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        // Technically this could be 10 if the first request comes back before the second is
        // initiated, but in practice this doesn't really happen.
        assert!(entries.len() == 12);
    }

    #[test]
    fn non_covered_ranges() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut reactor = Core::new().unwrap();

        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("non_covered_ranges", schema.clone());
        table_builder.set_range_partition_columns(vec!["key"]);

        let mut lower_bound1 = schema.new_row();
        lower_bound1.set(0, "a").unwrap();

        let mut upper_bound1 = schema.new_row();
        upper_bound1.set(0, "m").unwrap();
        table_builder.add_range_partition(RangePartitionBound::Inclusive(lower_bound1),
                                          RangePartitionBound::Exclusive(upper_bound1));

        let mut lower_bound2 = schema.new_row();
        lower_bound2.set(0, "p").unwrap();

        let mut upper_bound2 = schema.new_row();
        upper_bound2.set(0, "s").unwrap();
        table_builder.add_range_partition(RangePartitionBound::Inclusive(lower_bound2),
                                          RangePartitionBound::Exclusive(upper_bound2));
        table_builder.set_num_replicas(1);

        let mut split = schema.new_row();
        split.set(0, "c").unwrap();
        table_builder.add_range_partition_split(split);

        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let table = reactor.run(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.meta_cache().clone();

        reactor.run(cache.entry(vec![0])).expect("entry");
        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        assert_eq!(6, entries.len());

        let expected: Vec<(&[u8], &[u8], bool)> = vec![ (b"",  b"a", false),
                                                        (b"a", b"c",  true),
                                                        (b"c", b"m",  true),
                                                        (b"m", b"p", false),
                                                        (b"p", b"s",  true),
                                                        (b"s", b"",  false) ];

        for (entry, &(lower, upper, covered)) in entries.iter().zip(expected.iter()) {
            assert_eq!(lower, entry.partition_lower_bound());
            assert_eq!(upper, entry.partition_upper_bound());
            assert_eq!(covered, entry.is_tablet());
        }

        let cases: Vec<(&[u8], usize)> = vec![(b"", 6), (b"\0", 6), (b"`", 6),
                                              (b"a", 5), (b"a\0", 5), (b"b", 5),
                                              (b"c", 4), (b"d", 4), (b"l", 4),
                                              (b"m", 4), (b"n", 4), (b"o", 4),
                                              (b"p", 2), (b"q", 2), (b"r", 2),
                                              (b"s", 2), (b"z", 2), (b"zzz", 2)];

        for (key, expected_entries) in cases {
            cache.clear();
            reactor.run(cache.entry(key.to_owned())).expect("entry");

            let new_entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();

            assert_eq!(&entries[6 - expected_entries..], &new_entries[..],
                       "key: {:?}, expected entries: {}, entries: {:?}",
                       key, expected_entries, &new_entries[..]);
        }
    }
}
