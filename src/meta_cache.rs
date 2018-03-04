use std::cmp::Ordering;
use std::collections::{
    BTreeMap,
    Bound,
    HashMap,
};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::future;
use futures::sync::{mpsc, oneshot};
use parking_lot::{
    Mutex,
    RwLock,
};

use Error;
use HostPort;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TableId;
use TabletId;
use TabletServerId;
use master::{
    MasterProxy,
    MasterFuture,
};
use pb::ExpectField;
use pb::master::{
    GetTableLocationsRequestPb,
    GetTableLocationsResponsePb,
    GetTableSchemaRequestPb,
    GetTableSchemaResponsePb,
    MasterService,
    TableIdentifierPb,
    TabletLocationsPb,
};
use table::Table;
use tablet::{
    Tablet,
    TabletReplica,
};
use tserver::TabletServer;
use partition::{
    IntoPartitionKey,
    PartitionKey,
};
use Options;

/// Todo:
/// - Rename MetaCache/MetaCacheTask to TableLocationsCache/TableLocationsTask
/// - Make a new MetaCache which holds the cache of TabletServers and table locations

const MAX_RETURNED_TABLE_LOCATIONS: u32 = 10;

#[derive(Clone)]
pub(crate) enum Entry {
    Tablet(Tablet),
    NonCoveredRange {
        lower_bound: PartitionKey,
        upper_bound: PartitionKey,
        deadline: Instant,
    }
}

impl Entry {

    fn non_covered_range(lower_bound: PartitionKey,
                         upper_bound: PartitionKey,
                         deadline: Instant) -> Entry {
        Entry::NonCoveredRange {
            lower_bound,
            upper_bound,
            deadline,
        }
    }

    fn is_tablet(&self) -> bool {
        match *self {
            Entry::Tablet(_) => true,
            Entry::NonCoveredRange { .. } => false,
        }
    }

    fn lower_bound(&self) -> &[u8] {
        match *self {
            Entry::Tablet(ref tablet) => tablet.partition().lower_bound_key(),
            Entry::NonCoveredRange { ref lower_bound, .. } => lower_bound,
        }
    }

    fn upper_bound(&self) -> &[u8] {
        match *self {
            Entry::Tablet(ref tablet) => tablet.partition().upper_bound_key(),
            Entry::NonCoveredRange { ref upper_bound, .. } => upper_bound,
        }
    }

    fn contains_partition_key(&self, partition_key: &[u8]) -> bool {
        let upper_bound = self.upper_bound();
        (upper_bound.is_empty() || partition_key < upper_bound)
            && partition_key >= self.lower_bound()
    }

    /// Returns `Ordering::Equal` if the entries intersect, `Ordering::Less` if this entry falls
    /// before the other entry, or `Ordering::Greater` if this entry falls after the other entry.
    fn cmp_entry(&self, other: &Entry) -> Ordering {
        if !self.upper_bound().is_empty() &&
            self.upper_bound() <= other.lower_bound() {
            Ordering::Less
        } else if !other.upper_bound().is_empty() &&
                   other.upper_bound() <= self.lower_bound() {
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
            (&Entry::NonCoveredRange { lower_bound: ref a_lower,
                                       upper_bound: ref a_upper, .. },
             &Entry::NonCoveredRange { lower_bound: ref b_lower,
                                       upper_bound: ref b_upper, .. }) => {
                a_lower == b_lower && a_upper == b_upper
            },
            _ => false,
        }
    }
}

type TabletServerCache = Arc<RwLock<HashMap<TabletServerId, TabletServer>>>;

#[derive(Clone)]
pub(crate) struct MetaCache(Arc<Mutex<HashMap<TableId, TableLocationsCache>>>);

impl MetaCache {

    pub(crate) fn new() -> MetaCache {
        MetaCache(Arc::new(Mutex::new(HashMap::new())))
    }

    pub(crate) fn open_table(&self,
                             table: TableIdentifierPb,
                             mut master_proxy: MasterProxy,
                             options: &Options) -> impl Future<Item=Table, Error=Error> {
        let meta_cache = self.clone();
        let call = MasterService::get_table_schema(Arc::new(GetTableSchemaRequestPb { table }),
                                                   Instant::now() + options.admin_timeout);

        master_proxy.send(call).and_then(move |resp: GetTableSchemaResponsePb| -> Result<Table> {
            static MESSAGE: &'static str = "GetTableSchemaResponsePb";

            let num_replicas = resp.num_replicas() as u32;
            let name = resp.table_name.expect_field(MESSAGE, "table_name")?;

            let id = TableId::parse_bytes(&resp.table_id.expect_field(MESSAGE, "table_id")?)?;

            let schema = resp.schema.expect_field(MESSAGE, "schema")?;
            let partition_schema = PartitionSchema::from_pb(
                &resp.partition_schema.expect_field(MESSAGE, "partition_schema")?,
                &schema);
            let schema = Schema::from_pb(schema)?;

            let table_locations = meta_cache.0
                .lock()
                .entry(id)
                .or_insert_with(|| TableLocationsCache::new(id,
                                                            schema.primary_key_projection(),
                                                            partition_schema.clone(),
                                                            master_proxy))
                .clone();

            Ok(Table::new(name,
                          id,
                          schema,
                          partition_schema,
                          num_replicas,
                          table_locations))
        })
    }
}

/// A cache of of table locations.
///
/// Table locations are either tablets or non-covered ranges.
#[derive(Clone)]
pub(crate) struct TableLocationsCache {
    inner: Arc<TableLocationsInner>,
    sender: mpsc::UnboundedSender<(PartitionKey, oneshot::Sender<Result<Entry>>)>,
}

impl TableLocationsCache {

    pub(crate) fn new(table: TableId,
                      primary_key_schema: Schema,
                      partition_schema: PartitionSchema,
                      master: MasterProxy)
                      -> TableLocationsCache {
        let (sender, receiver) = mpsc::unbounded();
        let inner = Arc::new(TableLocationsInner {
            table: table,
            primary_key_schema: primary_key_schema,
            partition_schema: partition_schema,
            entries: Mutex::new(BTreeMap::new()),
        });

        let remote = master.options().remote.clone();
        let task = MetaCacheTask {
            inner: inner.clone(),
            master,
            receiver,
            requests: BTreeMap::new(),
            in_flight: None,
        };
        remote.spawn(move |_| task);
        TableLocationsCache { inner, sender, }
    }

    pub(crate) fn entry(&self, partition_key: &[u8]) -> impl Future<Item=Entry, Error=Error> {
        self.extract(partition_key, Entry::clone)
    }

    pub(crate) fn cached_entry(&self, partition_key: &[u8]) -> Option<Entry> {
        self.inner.extract(partition_key, &Entry::clone)
    }

    pub(crate) fn tablet_id(&self, partition_key: &[u8]) -> impl Future<Item=Option<TabletId>, Error=Error> {
        self.extract(partition_key, |entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                Some(tablet.id())
            } else {
                None
            }
        })
    }

    pub(crate) fn tablet_leader(&self, partition_key: &[u8]) -> impl Future<Item=Option<Box<[HostPort]>>, Error=Error> {
        self.extract(partition_key, |entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                tablet.replicas()
                      .iter()
                      .find(|replica| replica.role() == RaftRole::Leader)
                      .map(TabletReplica::rpc_addrs)
            } else {
                None
            }
        })
    }

    fn extract<T>(&self,
                  partition_key: &[u8],
                  extractor: fn(&Entry) -> T)
                  -> impl Future<Item=T, Error=Error> {
        if let Some(value) = self.inner.extract(partition_key, &extractor) {
            return future::Either::A(future::ok(value));
        };

        let partition_key = partition_key.into_partition_key();
        let (send, recv) = oneshot::channel();

        self.sender.unbounded_send((partition_key, send)).expect("MetaCacheTask finished");
        future::Either::B(recv.then(move |result| match result {
            Ok(Ok(entry)) => Ok(extractor(&entry)),
            Ok(Err(error)) => Err(error),
            Err(_) => unreachable!("MetaCacheTask finished"),
        }))
    }

    pub(crate) fn table(&self) -> TableId {
        self.inner.table
    }

    pub(crate) fn primary_key_schema(&self) -> &Schema {
        &self.inner.primary_key_schema
    }

    pub(crate) fn partition_schema(&self) -> &PartitionSchema {
        &self.inner.partition_schema
    }

    pub(crate) fn clear(&self) {
        self.inner.entries.lock().clear()
    }
}

impl fmt::Debug for TableLocationsCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TableLocationsCache")
         .field("table", &self.table())
         .finish()
    }
}

struct TableLocationsInner {
    table: TableId,
    primary_key_schema: Schema,
    partition_schema: PartitionSchema,
    entries: Mutex<BTreeMap<PartitionKey, Entry>>,
}

impl TableLocationsInner {

    fn extract<Extractor, T>(&self, partition_key: &[u8], extractor: &Extractor) -> Option<T>
    where Extractor: Fn(&Entry) -> T {
        let entries = self.entries.lock();
        match entries.range::<[u8], _>((Bound::Unbounded, Bound::Included(partition_key))).next_back() {
            Some(ref entry) if entry.1.contains_partition_key(partition_key) => Some(extractor(entry.1)),
            _ => None,
        }
    }

    fn splice_entries(&self, entries: Vec<Entry>) {
        let mut left = self.entries.lock();

        // NLL hack.
        let mut right = {
            let lower_bound = entries[0].lower_bound();
            let upper_bound = entries[entries.len() - 1].lower_bound();

            let mut right = left.split_off(lower_bound);

            if upper_bound.is_empty() {
                BTreeMap::new()
            } else {
                right.split_off(upper_bound)
            }
        };

        for entry in entries {
            left.insert(entry.lower_bound().into_partition_key(), entry);
        }

        left.append(&mut right);
    }

    /// Converts the results of a `GetTableLocations` RPC to a set of meta cache entries. The
    /// entries are guaranteed to be contiguous in the partition key space. The partition key must
    /// match the partition key of the get table locations request. The request must not have an
    /// end key.
    fn tablet_locations_to_entries(&self,
                                   partition_key: &[u8],
                                   tablets: Vec<TabletLocationsPb>,
                                   deadline: Instant)
                                   -> Result<Vec<Entry>> {
        if tablets.is_empty() {
            // If there are no tablets in the response, then the table is empty. If
            // there were any tablets in the table they would have been returned, since
            // the master guarantees that the if the partition key falls in a
            // non-covered range, the previous tablet will be returned, and we did not
            // set an upper bound partition key on the request.
            return Ok(vec![Entry::non_covered_range(PartitionKey::empty(),
                                                    PartitionKey::empty(),
                                                    deadline)]);
        }

        let tablet_count = tablets.len();
        let mut entries = Vec::with_capacity(tablets.len());
        let mut last_upper_bound = tablets[0].partition
                                             .as_ref()
                                             .expect_field("TabletLocationsPb", "partition")?
                                             .partition_key_start()
                                             .into_partition_key();

        if partition_key < &*last_upper_bound {
            // If the first tablet is past the requested partition key, then the partition key fell
            // in an initial non-covered range.
            entries.push(Entry::non_covered_range(PartitionKey::empty(),
                                                  last_upper_bound.clone(),
                                                  deadline));
        }

        for tablet in tablets {
            let tablet = Tablet::from_pb(&self.primary_key_schema,
                                         self.partition_schema.clone(),
                                         tablet,
                                         deadline)?;
            if tablet.partition().lower_bound_key() > &*last_upper_bound {
                entries.push(Entry::non_covered_range(last_upper_bound,
                                                      tablet.partition().lower_bound_key().into_partition_key(),
                                                      deadline));
            }
            last_upper_bound = tablet.partition().upper_bound_key().into_partition_key();
            entries.push(Entry::Tablet(tablet));
        }

        if !last_upper_bound.is_empty() && tablet_count < MAX_RETURNED_TABLE_LOCATIONS as usize {
            entries.push(Entry::non_covered_range(last_upper_bound,
                                                  PartitionKey::empty(),
                                                  deadline));
        }

        //trace!("{:?}: discovered entries: {:?}", self, entries);
        Ok(entries)
    }
}

impl fmt::Debug for TableLocationsInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TableLocationsCache")
         .field("table", &self.table)
         .finish()
    }
}

/// A task which manages requesting locations from the master for a table.
///
/// TODO:
///     - Retry RPCS that fail with tablet not running.
struct MetaCacheTask {
    /// Cache of tablet entries.
    inner: Arc<TableLocationsInner>,
    /// Master connection.
    master: MasterProxy,
    /// Queue of incoming requests.
    receiver: mpsc::UnboundedReceiver<(PartitionKey, oneshot::Sender<Result<Entry>>)>,
    /// Collection of outstanding entry requests, ordered by partition key.
    requests: BTreeMap<PartitionKey, Vec<oneshot::Sender<Result<Entry>>>>,

    in_flight: Option<(PartitionKey, MasterFuture<GetTableLocationsRequestPb, GetTableLocationsResponsePb>)>,
}

impl Future for MetaCacheTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            // Step 1: Receive as many requests as possible, adding them to the requests map. Since
            // requests are asynchronous we also take the opportunity to check that they haven't
            // been cancelled, and haven't already been fulfilled.
            while let Async::Ready(item) = self.receiver.poll()? {
                match item {
                    Some((partition_key, sender)) => {
                        if sender.is_canceled() {
                            continue;
                        }
                        if let Some(entry) = self.inner.extract(&*partition_key, &Entry::clone) {
                            let _ = sender.send(Ok(entry));
                        } else {
                            self.requests
                                .entry(partition_key)
                                .or_insert_with(Vec::new)
                                .push(sender);
                        }
                    },
                    // There are no more handles to the TableLocationsCache instance, so shutdown.
                    None => return Ok(Async::Ready(())),
                }
            }

            // Step 2: Check if the currently outstanding tablet locations lookup is complete.
            if let Some((partition_key, mut in_flight)) = self.in_flight.take() {
                match in_flight.poll() {
                    Ok(Async::Ready(response)) => {
                        // TODO: error handling
                        let deadline = Instant::now() + Duration::from_millis(u64::from(response.ttl_millis()));
                        let entries = self.inner
                                          .tablet_locations_to_entries(&*partition_key,
                                                                       response.tablet_locations,
                                                                       deadline)
                                          .unwrap();
                        let mut requests = self.requests.split_off(entries[0].lower_bound());
                        { // NLL hack
                            let upper_bound = entries[entries.len() - 1].upper_bound();
                            if !upper_bound.is_empty() {
                                self.requests.extend(requests.split_off(upper_bound));
                            }
                        }

                        for (partition_key, senders) in requests {
                            let index = match entries.binary_search_by_key(&&*partition_key, |entry| entry.lower_bound()) {
                                Ok(index) => index,
                                Err(index) => index - 1,
                            };
                            debug_assert!(entries[index].contains_partition_key(&*partition_key));
                            for sender in senders {
                                let _ = sender.send(Ok(entries[index].clone()));
                            }
                        }

                        self.inner.splice_entries(entries);
                    }
                    Ok(Async::NotReady) => {
                        self.in_flight = Some((partition_key, in_flight));
                        return Ok(Async::NotReady);
                    },
                    Err(error) => {
                        for sender in self.requests.remove(&partition_key).unwrap_or_default() {
                            let _ = sender.send(Err(error.clone()));
                        }
                    },
                }
            }

            // Step 3: kick off another tablet locations lookup if necessary.
            match self.requests.keys().next() {
                Some(partition_key) => {
                    let request = Arc::new(GetTableLocationsRequestPb {
                        table: self.inner.table.into(),
                        partition_key_start: Some(partition_key.as_ref().to_owned()),
                        partition_key_end: None,
                        max_returned_locations: Some(MAX_RETURNED_TABLE_LOCATIONS),
                        replica_type_filter: None,
                    });
                    let call = MasterService::get_table_locations(
                        request, Instant::now() + self.master.options().admin_timeout);
                    let resp = self.master.send(call);

                    self.in_flight = Some((partition_key.clone(), resp));
                },
                None => return Ok(Async::NotReady),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};

    use env_logger;
    use tokio::reactor::Core;

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
            let entry = reactor.run(cache.entry(&[])).expect("entry");

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.lower_bound());
            assert_eq!(b"", entry.upper_bound());

            let entries = cache.inner.entries.lock().clone();
            assert_eq!(1, entries.len());
            assert!(entry.equiv(entries.values().next().unwrap()));

            assert!(entry.equiv(&cache.cached_entry(b"").unwrap()));
            assert!(entry.equiv(&cache.cached_entry(b"foo").unwrap()));
        }

        cache.clear();
        {
            let entry = reactor.run(cache.entry(b"some-key")).expect("entry");

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.lower_bound());
            assert_eq!(b"", entry.upper_bound());
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

        let first = reactor.run(cache.entry(&[0, 0, 0, 0, 1])).expect("entry");

        assert!(first.is_tablet());
        assert_eq!(b"", first.lower_bound());
        assert_eq!(vec![0, 0, 0, 1], first.upper_bound());

        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        assert_eq!(10, entries.len());
        assert!(first.equiv(&entries[0]));

        assert!(first.equiv(&cache.cached_entry(b"").unwrap()));
        assert!(cache.cached_entry(b"foo").is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());

        let last = reactor.run(cache.entry(&[0, 0, 0, 11])).expect("entry");

        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();

        assert_eq!(11, entries.len());
        assert!(entries[10].equiv(&last));

        assert!(cache.cached_entry(b"foo").unwrap().equiv(&last));
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10, 5]).is_none());

        reactor.run(cache.entry(&[0, 0, 0, 9])).expect("entry");
        assert_eq!(11, cache.inner.entries.lock().len());

        reactor.run(cache.entry(&[0, 0, 0, 10])).expect("entry");
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

        reactor.run(cache.entry(&[0, 0, 0, 0]).join(cache.entry(&[0, 0, 0, 9])))
               .expect("entry");
        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        assert_eq!(entries.len(), 10);
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

        reactor.run(cache.entry(&[0])).expect("entry");
        let entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();
        assert_eq!(6, entries.len());

        let expected: Vec<(&[u8], &[u8], bool)> = vec![ (b"",  b"a", false),
                                                        (b"a", b"c",  true),
                                                        (b"c", b"m",  true),
                                                        (b"m", b"p", false),
                                                        (b"p", b"s",  true),
                                                        (b"s", b"",  false) ];

        for (entry, &(lower, upper, covered)) in entries.iter().zip(expected.iter()) {
            assert_eq!(lower, entry.lower_bound());
            assert_eq!(upper, entry.upper_bound());
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
            reactor.run(cache.entry(key)).expect("entry");

            let new_entries: Vec<Entry> = cache.inner.entries.lock().values().cloned().collect();

            assert_eq!(&entries[6 - expected_entries..], &new_entries[..],
                       "key: {:?}, expected entries: {}, entries: {:?}",
                       key, expected_entries, &new_entries[..]);
        }
    }
}
