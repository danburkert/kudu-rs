#![allow(unused_imports)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use futures::Poll;
use parking_lot::Mutex;
use pb::master::{GetTableLocationsRequestPb, TabletLocationsPb};

use Error;
use MasterErrorCode;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TableId;
use TabletId;
use backoff::Backoff;
use master::MasterProxy;
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

    /// Compare this entry against a partition key to determine if the key is covered by the entry
    /// (`Ordering::Equal`), after the entry (`Ordering::Lower`), or before the entry
    /// (`Ordering::Greater`).
    fn cmp_partition_key(&self, partition_key: &[u8]) -> Ordering {
        let upper_bound = self.partition_upper_bound();
        if !upper_bound.is_empty() && partition_key >= upper_bound {
            Ordering::Less
        } else if partition_key < self.partition_lower_bound() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
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

#[derive(Clone)]
pub struct MetaCache {
    master: MasterProxy,
    inner: Arc<Inner>,
}

pub struct Inner {
    table: TableId,
    primary_key_schema: Schema,
    partition_schema: PartitionSchema,
    entries: Mutex<Vec<Entry>>,
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
                entries: Mutex::new(Vec::new()),
            })
        }
    }

    pub fn poll_entry(&self, partition_key: &[u8]) -> Poll<Entry, Error> {
        unimplemented!()
    }

    fn cached_entry(&self, partition_key: &[u8]) -> Option<Entry> {
        match self.extract_cached(partition_key, |entry| entry.clone()) {
            ExtractCachedResult::Value(value) => Some(value),
            ExtractCachedResult::Extractor(_) => None,
        }
    }

    pub fn tablet_id<F>(&self,
                        partition_key: Vec<u8>,
                        deadline: Instant,
                        cb: F)
    where F: FnOnce(Result<Option<TabletId>>) + Send + 'static {
        let extractor = |entry: &Entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                Some(tablet.id())
            } else {
                None
            }
        };
        self.extract(partition_key, deadline, backoff(), extractor, cb);
    }

    /*
    pub fn tablet_leader<F>(&self,
                            partition_key: Vec<u8>,
                            deadline: Instant,
                            cb: F)
    where F: FnOnce(Result<Option<(TabletId, Vec<SocketAddr>)>>) + Send + 'static {
        let mut addrs = Vec::with_capacity(1);
        let extractor = move |entry: &Entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                if tablet.replicas()[0].role() == RaftRole::Leader {
                    addrs.extend_from_slice(&tablet.replicas()[0].resolved_rpc_addrs());
                }
                Some((tablet.id(), addrs))
            } else {
                None
            }
        };
        self.extract(partition_key, deadline, backoff(), extractor, cb);
    }
    */

    fn extract<Extractor, T, F>(&self,
                                partition_key: Vec<u8>,
                                deadline: Instant,
                                backoff: Backoff,
                                extractor: Extractor,
                                cb: F)
    where Extractor: FnOnce(&Entry) -> T + Send + 'static,
          F: FnOnce(Result<T>) + Send + 'static {

        let extractor = match self.extract_cached(&*partition_key, extractor) {
            ExtractCachedResult::Extractor(extractor) => extractor,
            ExtractCachedResult::Value(value) => return cb(Ok(value)),
        };

        let request = GetTableLocationsRequestPb {
            table: self.inner.table.into(),
            partition_key_start: Some(partition_key.clone()),
            max_returned_locations: Some(MAX_RETURNED_TABLE_LOCATIONS),
            ..Default::default()
        };

        let meta_cache = self.clone();
        /*
        self.master.get_table_locations(deadline, request, move |resp| {
            match resp {
                Ok(mut resp) => {
                    meta_cache.add_tablet_locations(partition_key,
                                                    resp.take_tablet_locations().into_vec(),
                                                    extractor,
                                                    cb);
                },
                Err(Error::Master(ref error)) if error.code() == MasterErrorCode::TabletNotRunning => {
                    let duration = Duration::from_millis(backoff.next_backoff_ms());
                    let messenger = meta_cache.master.messenger().clone();
                    messenger.timer(duration, Box::new(move || {
                        meta_cache.extract(partition_key, deadline, backoff, extractor, cb);
                    }));
                }
                Err(error) => cb(Err(error)),
            }
        });
        */
        unimplemented!()
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

    fn add_tablet_locations<F, Extractor, T>(&self,
                                             partition_key: Vec<u8>,
                                             tablets: Vec<TabletLocationsPb>,
                                             extractor: Extractor,
                                             cb: F)
    where Extractor: FnOnce(&Entry) -> T + Send + 'static,
          F: FnOnce(Result<T>) + Send + 'static {
        let meta_cache = self.clone();
        thread::spawn(move || {
            match meta_cache.tablet_locations_to_entries(&partition_key, tablets) {
                Ok(entries) => {
                    meta_cache.splice_entries(entries);
                    match meta_cache.extract_cached(&partition_key, extractor) {
                        ExtractCachedResult::Extractor(_) => unreachable!(),
                        ExtractCachedResult::Value(value) => cb(Ok(value)),
                    }
                },
                Err(error) => cb(Err(error)),
            }
        });
    }

    fn extract_cached<Extractor, T>(&self,
                                    partition_key: &[u8],
                                    extractor: Extractor)
                                    -> ExtractCachedResult<Extractor, T>
    where Extractor: FnOnce(&Entry) -> T {
        let entries = self.inner.entries.lock();
        match entries.binary_search_by(|entry| entry.cmp_partition_key(partition_key)) {
            Ok(index) => ExtractCachedResult::Value(extractor(&entries[index])),
            Err(_) => ExtractCachedResult::Extractor(extractor),
        }
    }

    fn splice_entries(&self, mut new_entries: VecDeque<Entry>) {
        let mut entries = self.inner.entries.lock();
        let splice_point = match entries.binary_search_by(|entry| entry.cmp_entry(&new_entries[0])) {
            Ok(idx) | Err(idx) => idx,
        };

        let mut existing_entries = entries.drain(splice_point..).collect::<VecDeque<_>>();

        let mut existing_entry = existing_entries.pop_front();
        let mut new_entry = new_entries.pop_front();

        loop {
            match (existing_entry.take(), new_entry.take()) {
                (Some(existing), Some(new)) => match existing.cmp_entry(&new) {
                    Ordering::Equal => {
                        // Remove all existing entries that overlap the new entry.
                        // TODO: test this once range add/drop is implemented.
                        while let Some(existing) = existing_entries.pop_front() {
                            if existing.cmp_entry(&new) != Ordering::Equal {
                                existing_entries.push_front(existing);
                                break;
                            }
                        }
                        entries.push(new)
                    },
                    Ordering::Less => {
                        entries.push(existing);
                        new_entry = Some(new);
                    },
                    Ordering::Greater => {
                        entries.push(new);
                        existing_entry = Some(existing);
                    },
                },
                (Some(existing), None) => entries.push(existing),
                (None, Some(new)) => entries.push(new),
                (None, None) => break,
            }
            if existing_entry.is_none() { existing_entry = existing_entries.pop_front(); }
            if new_entry.is_none() { new_entry = new_entries.pop_front(); }
        }
    }

    /// Converts the results of a `GetTableLocations` RPC to a set of entries for the meta cache.
    /// The entries are guaranteed to be contiguous in the partition key space. The partition key
    /// must match the partition key of the get table locations request. The request must not
    /// have an end key.
    fn tablet_locations_to_entries(&self,
                                   partition_key: &[u8],
                                   tablets: Vec<TabletLocationsPb>)
                                   -> Result<VecDeque<Entry>> {
/*
        if tablets.is_empty() {
            // If there are no tablets in the response, then the table is empty. If
            // there were any tablets in the table they would have been returned, since
            // the master guarantees that the if the partition key falls in a
            // non-covered range, the previous tablet will be returned, and we did not
            // set an upper bound partition key on the request.
            let mut entries = VecDeque::with_capacity(1);
            entries.push_back(Entry::non_covered_range(Vec::new(), Vec::new()));
            return Ok(entries);
        }

        let tablet_count = tablets.len();
        let mut entries = VecDeque::with_capacity(tablets.len());
        let mut last_upper_bound = tablets[0].get_partition().get_partition_key_start().to_owned();

        if partition_key < &last_upper_bound {
            // If the first tablet is past the requested partition key, then the partition key fell
            // in an initial non-covered range.
            entries.push_back(Entry::non_covered_range(Vec::new(), last_upper_bound.clone()));
        }

        for tablet in tablets {
            let tablet = try!(Tablet::from_pb(&self.inner.primary_key_schema,
                                              self.inner.partition_schema.clone(),
                                              tablet));
            if tablet.partition().lower_bound_key() > &last_upper_bound {
                entries.push_back(Entry::non_covered_range(last_upper_bound,
                                                           tablet.partition().lower_bound_key().to_owned()));
            }
            last_upper_bound = tablet.partition().upper_bound_key().to_owned();
            entries.push_back(Entry::Tablet(tablet));
        }

        if !last_upper_bound.is_empty() && tablet_count < MAX_RETURNED_TABLE_LOCATIONS as usize {
            entries.push_back(Entry::non_covered_range(last_upper_bound, Vec::new()));
        }

        trace!("discovered table locations: {:?}", entries);
        Ok(entries)
*/
        unimplemented!()
    }

    pub fn clear(&self) {
        self.inner.entries.lock().clear()
    }
}

/// Unfortunate brwck hack.
enum ExtractCachedResult<Extractor, T> {
    Extractor(Extractor),
    Value(T),
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use env_logger;

    use Client;
    use ClientConfig;
    use RangePartitionBound;
    use TableBuilder;
    use mini_cluster::MiniCluster;
    use schema::tests::simple_schema;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn single_tablet() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("single_tablet", schema.clone());
        table_builder.set_range_partition_columns(vec!["key".to_owned()]);
        table_builder.set_num_replicas(1);

        let table_id = client.create_table(table_builder, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();
        let cache = table.meta_cache().clone();

        let (send, recv) = sync_channel(0);

        {
            let entries = cache.inner.entries.lock().clone();
            assert!(entries.is_empty());
        }

        {
            let send = send.clone();
            cache.entry(vec![], deadline(), move |entry| {
                send.send(entry).unwrap();
            });
            let entry = recv.recv().unwrap().unwrap();

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.partition_lower_bound());
            assert_eq!(b"", entry.partition_upper_bound());

            let entries = cache.inner.entries.lock().clone();
            assert!(entry.equiv(&entries[0]));

            assert!(entry.equiv(&cache.cached_entry(b"").unwrap()));
            assert!(entry.equiv(&cache.cached_entry(b"foo").unwrap()));
        }
        cache.clear();
        {
            let send = send.clone();
            cache.entry(b"some-key".as_ref().to_owned(), deadline(), move |entry| {
                send.send(entry).unwrap();
            });
            let entry = recv.recv().unwrap().unwrap();

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.partition_lower_bound());
            assert_eq!(b"", entry.partition_upper_bound());
        }
    }

    #[test]
    fn multi_tablet() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = client.create_table(table_builder, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();
        let cache = table.meta_cache().clone();

        let (send, recv) = sync_channel(1);

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 0, 1], deadline(), move |entry| {
            s.send(entry).unwrap();
        });
        let first = recv.recv().unwrap().unwrap();

        assert!(first.is_tablet());
        assert_eq!(b"", first.partition_lower_bound());
        assert_eq!(vec![0, 0, 0, 1], first.partition_upper_bound());

        let entries = cache.inner.entries.lock().clone();
        assert_eq!(10, entries.len());
        assert!(first.equiv(&entries[0]));

        assert!(first.equiv(&cache.cached_entry(b"").unwrap()));
        assert!(cache.cached_entry(b"foo").is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 11], deadline(), move |entry| {
            s.send(entry).unwrap();
        });
        let last = recv.recv().unwrap().unwrap();

        let entries = cache.inner.entries.lock().clone();
        assert_eq!(11, entries.len());
        assert!(entries[10].equiv(&last));

        assert!(cache.cached_entry(b"foo").unwrap().equiv(&last));
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10, 5]).is_none());

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 9], deadline(), move |entry| {
            let result = s.send(entry);
            result.unwrap();
        });
        let _ = recv.recv().unwrap().unwrap();
        assert_eq!(11, cache.inner.entries.lock().len());

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 10], deadline(), move |entry| {
            s.send(entry).unwrap();
        });
        let _ = recv.recv().unwrap().unwrap();
        assert_eq!(12, cache.inner.entries.lock().len());
    }

    #[test]
    fn multi_tablet_concurrent() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet_concurrent", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();
        let cache = table.meta_cache().clone();

        let (send, recv) = sync_channel(2);

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 0], deadline(), move |entry| {
            s.send(entry).unwrap();
        });

        let s = send.clone();
        cache.entry(vec![0, 0, 0, 8], deadline(), move |entry| {
            s.send(entry).unwrap();
        });

        recv.recv().unwrap().unwrap();
        recv.recv().unwrap().unwrap();

        let entries = cache.inner.entries.lock().clone();
        // Technically this could be 10 if the first request comes back before the second is
        // initiated, but in practice this doesn't really happen.
        assert!(entries.len() == 12);
    }

    #[test]
    fn non_covered_ranges() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

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

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();
        let cache = table.meta_cache().clone();
        let (send, recv) = sync_channel(10);

        let s = send.clone();
        cache.entry(b"\0".as_ref().to_owned(), deadline(), move |entry| {
            s.send(entry).unwrap();
        });
        recv.recv().unwrap().unwrap();
        let entries = cache.inner.entries.lock().clone();
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
            let s = send.clone();
            cache.entry(key.to_owned(), deadline(), move |entry| {
                s.send(entry).unwrap();
            });
            recv.recv().unwrap().unwrap();
            assert_eq!(&entries[6 - expected_entries..], &cache.inner.entries.lock().clone()[..],
                       "key: {:?}, expected entries: {}, entries: {:?}", key, expected_entries, &cache.inner.entries.lock().clone()[..]);
        }
    }
}
