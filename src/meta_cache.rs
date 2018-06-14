use std::cmp::Ordering;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::collections::{
    BTreeMap,
    Bound,
    HashMap,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{
    Async,
    Future,
    Poll,
    Stream,
    stream,
};
use futures::sync::{mpsc, oneshot};
use parking_lot::Mutex;
use prost;
use krpc;
use tokio;

use backoff::Backoff;
use Error;
use HostPort;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TableId;
use TabletId;
use TabletServerId;
use pb::ExpectField;
use pb::master::{
    GetTableLocationsRequestPb,
    GetTableLocationsResponsePb,
    GetTableSchemaRequestPb,
    MasterFeatures,
    MasterService,
    TableIdentifierPb,
    TabletLocationsPb,
};
use table::Table;
use tablet::TabletInfo;
use partition::{
    IntoPartitionKey,
    PartitionKey,
};
use Options;
use replica::{
    Replica,
    ReplicaRpc,
    ReplicaSet,
    Selection,
    Speculation,
};
use retry::Retriable;

const MAX_RETURNED_TABLE_LOCATIONS: u32 = 10;

#[derive(Clone, Debug)]
pub(crate) enum Entry {
    Tablet(Arc<Tablet>),
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
            Entry::Tablet(ref tablet) => tablet.lower_bound(),
            Entry::NonCoveredRange { ref lower_bound, .. } => lower_bound,
        }
    }

    fn upper_bound(&self) -> &[u8] {
        match *self {
            Entry::Tablet(ref tablet) => tablet.upper_bound(),
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
                              (a.lower_bound() == b.lower_bound() &&
                               a.upper_bound() == b.upper_bound()));
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

#[derive(Clone)]
pub(crate) struct MetaCache {
    tables: Arc<Mutex<HashMap<TableId, TableLocations>>>,
    tablet_servers: Arc<Mutex<HashMap<TabletServerId, krpc::Proxy>>>,
    masters: Arc<Box<[MasterReplica]>>,
    options: Options,
}

// TODO: make fields private
pub(crate) struct MasterReplica {
    pub(crate) hostport: HostPort,
    pub(crate) proxy: krpc::Proxy,
    pub(crate) is_leader: AtomicBool,
}

impl MasterReplica {

    pub(crate) fn new(hostport: HostPort, options: &Options) -> impl Future<Item=MasterReplica, Error=Error> {
        krpc::Proxy::new(vec![hostport.clone()].into_boxed_slice(), options.rpc.clone())
            .map(move |proxy| {
                MasterReplica {
                    hostport,
                    proxy,
                    is_leader: AtomicBool::new(false),
                }
            })
            .map_err(Error::from)
    }
}

impl Replica for MasterReplica {

    fn proxy(&self) -> krpc::Proxy {
        self.proxy.clone()
    }

    fn is_leader(&self) -> bool {
        self.is_leader.load(Relaxed)
    }

    fn mark_leader(&self) {
        self.is_leader.store(true, Relaxed)
    }

    fn mark_follower(&self) {
        self.is_leader.store(false, Relaxed)
    }

    fn is_stale(&self) -> bool {
        false
    }

    fn mark_stale(&self) {
        panic!("Master replicas may not be stale")
    }
}

impl fmt::Debug for MasterReplica {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterReplica")
            .field("addr", &self.hostport)
            .field("is_leader", &self.is_leader)
            .finish()
    }
}

impl ReplicaSet for Arc<Box<[MasterReplica]>> {
    type Replica = MasterReplica;
    fn replicas(&self) -> &[MasterReplica] {
        &*self
    }
}

impl MetaCache {

    pub(crate) fn new(master_addrs: Vec<HostPort>,
                      options: Options) -> impl Future<Item=MetaCache, Error=Error> {
        connect_to_cluster(master_addrs, &options).map(|master_replicas| {
            MetaCache {
                tables: Arc::new(Mutex::new(HashMap::new())),
                tablet_servers: Arc::new(Mutex::new(HashMap::new())),
                masters: master_replicas,
                options,
            }
        })
    }

    pub(crate) fn options(&self) -> &Options {
        &self.options
    }

    pub(crate) fn master_rpc<Req, Resp>(&self, call: krpc::Call<Req, Resp>) -> impl Future<Item=Resp, Error=Error>
    where Req: prost::Message + 'static,
          Resp: Retriable {
        ReplicaRpc::new(self.masters.clone(),
                        call,
                        Speculation::Staggered(Duration::from_millis(32)),
                        Selection::Leader,
                        Backoff::with_duration_range(32, 2048))
            .map(|(_, resp, _)| resp)
    }

    pub(crate) fn open_table(&self, table: TableIdentifierPb, deadline: Instant) -> impl Future<Item=Table, Error=Error> {
        let call = MasterService::get_table_schema(Arc::new(GetTableSchemaRequestPb { table }), deadline);

        let tables = self.tables.clone();
        let tablet_servers = self.tablet_servers.clone();
        let options = self.options.clone();
        let masters = self.masters.clone();

        self.master_rpc(call).and_then(move |resp| -> Result<Table> {
            static MESSAGE: &'static str = "GetTableSchemaResponsePb";

            let num_replicas = resp.num_replicas() as u32;
            let name = resp.table_name.expect_field(MESSAGE, "table_name")?;

            let id = TableId::parse_bytes(&resp.table_id.expect_field(MESSAGE, "table_id")?)?;

            let schema = resp.schema.expect_field(MESSAGE, "schema")?;
            let partition_schema = PartitionSchema::from_pb(
                &resp.partition_schema.expect_field(MESSAGE, "partition_schema")?,
                &schema);
            let schema = Schema::from_pb(schema)?;

            let table_locations = tables
                .lock()
                .entry(id)
                .or_insert_with(|| TableLocations::new(options,
                                                       id,
                                                       schema.primary_key_projection(),
                                                       partition_schema.clone(),
                                                       masters,
                                                       tablet_servers))
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
pub(crate) struct TableLocations {
    entries: Arc<Mutex<BTreeMap<PartitionKey, Entry>>>,
    sender: mpsc::UnboundedSender<(PartitionKey, oneshot::Sender<Result<Entry>>)>,
}

impl TableLocations {
    pub(crate) fn new(options: Options,
                      table_id: TableId,
                      primary_key_schema: Schema,
                      partition_schema: PartitionSchema,
                      masters: Arc<Box<[MasterReplica]>>,
                      tablet_servers: Arc<Mutex<HashMap<TabletServerId, krpc::Proxy>>>)
                      -> TableLocations {

        let (sender, receiver) = mpsc::unbounded();
        let entries = Arc::new(Mutex::new(BTreeMap::new()));

        tokio::spawn(TableLocationsTask {
            options,
            entries: entries.clone(),
            tablet_servers,
            table_id,
            primary_key_schema,
            partition_schema,
            masters,
            receiver,
            requests: BTreeMap::new(),
            in_flight: None,
        });
        TableLocations { entries, sender }
    }

    pub(crate) fn entry(&self, partition_key: &[u8]) -> Lookup<Entry> {
        self.extract(partition_key, Entry::clone)
    }

    pub(crate) fn tablet_id(&self, partition_key: &[u8]) -> Lookup<Option<TabletId>> {
        self.extract(partition_key, |entry| {
            if let Entry::Tablet(ref tablet) = *entry {
                Some(tablet.id())
            } else {
                None
            }
        })
    }

    pub(crate) fn tablet(&self, partition_key: &[u8]) -> Lookup<Option<Arc<Tablet>>> {
        self.extract(partition_key, |entry| match *entry {
            Entry::Tablet(ref tablet) => Some(Arc::clone(tablet)),
            Entry::NonCoveredRange { .. } => None,
        })
    }

    fn extract<T>(&self, partition_key: &[u8], extractor: fn(&Entry) -> T) -> Lookup<T> {
        if let Some(entry) = get_entry(&self.entries.lock(), partition_key) {
            Lookup::Hit(Some(extractor(entry)))
        } else {
            let partition_key = partition_key.into_partition_key();
            let (send, recv) = oneshot::channel();
            self.sender.unbounded_send((partition_key, send)).expect("TableLocationsTask finished");
            Lookup::Miss { recv, extractor }
        }
    }

    pub(crate) fn clear(&self) {
        self.entries.lock().clear()
    }
}

pub(crate) enum Lookup<T> {
    Hit(Option<T>),
    Miss {
        recv: oneshot::Receiver<Result<Entry>>,
        extractor: fn(&Entry) -> T,
    },
}

impl <T> Future for Lookup<T> {
    type Item = T;
    type Error = Error;
    fn poll(&mut self) -> Poll<T, Error> {
        match self {
            Lookup::Hit(ref mut item) => Ok(Async::Ready(item.take().unwrap())),
            Lookup::Miss { ref mut recv, extractor } => {
                match recv.poll() {
                    Ok(Async::Ready(entry)) => Ok(Async::Ready(extractor(&entry?))),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Err(_) => unreachable!("TableLocationsTask finished"),
                }
            },
        }
    }
}

fn get_entry<'a, 'b>(entries: &'a BTreeMap<PartitionKey, Entry>,
                     partition_key: &'b [u8]) -> Option<&'a Entry> {
    // TODO: filter stale entries.
    match entries.range::<[u8], _>((Bound::Unbounded, Bound::Included(partition_key))).next_back() {
        Some((_, ref entry)) if entry.contains_partition_key(partition_key) => Some(entry),
        _ => None,
    }
}

/// A task which manages requesting locations from the master for a specific table.
///
/// TODO:
///     - Retry RPCS that fail with tablet not running.
struct TableLocationsTask {
    options: Options,

    /// Cache of tablet entries.
    entries: Arc<Mutex<BTreeMap<PartitionKey, Entry>>>,

    /// Cache of tablet server connections.
    tablet_servers: Arc<Mutex<HashMap<TabletServerId, krpc::Proxy>>>,

    table_id: TableId,
    primary_key_schema: Schema,
    partition_schema: PartitionSchema,

    /// Master repliacs..
    masters: Arc<Box<[MasterReplica]>>,
    /// Queue of incoming requests.
    receiver: mpsc::UnboundedReceiver<(PartitionKey, oneshot::Sender<Result<Entry>>)>,
    /// Collection of outstanding entry requests, ordered by partition key.
    requests: BTreeMap<PartitionKey, Vec<oneshot::Sender<Result<Entry>>>>,

    in_flight: Option<(PartitionKey, ReplicaRpc<Arc<Box<[MasterReplica]>>, GetTableLocationsRequestPb, GetTableLocationsResponsePb>)>,
}

impl TableLocationsTask {

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
        let mut entries = Vec::with_capacity(tablet_count);
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

        for pb in tablets {
            let id = TabletId::parse_bytes(&pb.tablet_id)?;

            let partition = pb.partition.expect_field("TabletLocationsPB", "partition")?;
            let lower_bound = partition.partition_key_start.expect_field("PartitionPb", "partition_key_start")?.into_partition_key();
            let upper_bound = partition.partition_key_end.expect_field("PartitionPb", "partition_key_end")?.into_partition_key();

            let replicas = pb.replicas.into_iter().map(move |pb| {
                let id = TabletServerId::parse_bytes(&pb.ts_info.permanent_uuid)?;
                let is_leader = pb.role() == RaftRole::Leader;
                let rpc_addrs = pb.ts_info
                                  .rpc_addresses
                                  .into_iter()
                                  .map(HostPort::from)
                                  .collect::<Vec<_>>()
                                  .into_boxed_slice();

                let proxy = self.tablet_servers
                                .lock()
                                .entry(id)
                                .or_insert_with(|| krpc::Proxy::spawn(rpc_addrs.clone(), self.options.rpc.clone()))
                                .clone();

                Ok(TabletReplica {
                    id,
                    rpc_addrs,
                    proxy,
                    is_stale: AtomicBool::new(false),
                    is_leader: AtomicBool::new(is_leader)
                })
            }).collect::<Result<Vec<TabletReplica>>>()?;

            let tablet = Arc::new(Tablet {
                id,
                lower_bound,
                upper_bound,
                replicas,
            });

            if tablet.lower_bound() > &last_upper_bound {
                entries.push(Entry::non_covered_range(last_upper_bound,
                                                      tablet.lower_bound().into_partition_key(),
                                                      deadline));
            }
            last_upper_bound = tablet.upper_bound().into_partition_key();
            entries.push(Entry::Tablet(tablet));
        }

        if !last_upper_bound.is_empty() && tablet_count < MAX_RETURNED_TABLE_LOCATIONS as usize {
            entries.push(Entry::non_covered_range(last_upper_bound,
                                                  PartitionKey::empty(),
                                                  deadline));
        }

        Ok(entries)
    }
}

impl Future for TableLocationsTask {
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

                        if let Some(entry) = get_entry(&self.entries.lock(), &*partition_key) {
                            let _ = sender.send(Ok(entry.clone()));
                        } else {
                            self.requests
                                .entry(partition_key)
                                .or_insert_with(Vec::new)
                                .push(sender);
                        }
                    },
                    // There are no more handles to the TableLocations instance, so shutdown.
                    None => return Ok(Async::Ready(())),
                }
            }

            // Step 2: Check if the currently outstanding tablet locations lookup is complete.
            if let Some((partition_key, mut in_flight)) = self.in_flight.take() {
                match in_flight.poll() {
                    Ok(Async::Ready((_, response, _))) => {
                        // TODO: error handling
                        let deadline = Instant::now() + Duration::from_millis(u64::from(response.ttl_millis()));
                        let entries = self.tablet_locations_to_entries(&*partition_key,
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

                        self.splice_entries(entries);
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
                        table: self.table_id.into(),
                        partition_key_start: Some(partition_key.as_ref().to_owned()),
                        partition_key_end: None,
                        max_returned_locations: Some(MAX_RETURNED_TABLE_LOCATIONS),
                        replica_type_filter: None,
                    });
                    let call = MasterService::get_table_locations(request, Instant::now() + self.options.admin_timeout);

                    let resp = ReplicaRpc::new(self.masters.clone(),
                                               call,
                                               Speculation::Staggered(Duration::from_millis(100)),
                                               Selection::Leader,
                                               Backoff::with_duration_range(250, u32::max_value()));

                    self.in_flight = Some((partition_key.clone(), resp));
                },
                None => return Ok(Async::NotReady),
            }
        }
    }
}

/// Metadata pertaining to a Kudu tablet.
// TODO: when replacing a tablet in the table locations map, carry through the leader flag.
#[derive(Debug)]
pub(crate) struct Tablet {

    /// The tablet ID.
    id: TabletId,

    /// The tablet lower-bound partition key.
    lower_bound: PartitionKey,

    /// The tablet upper-bound partition key.
    upper_bound: PartitionKey,

    /// The tablet replicas.
    replicas: Vec<TabletReplica>,
}

impl Tablet {

    /// Returns the unique tablet ID.
    pub fn id(&self) -> TabletId {
        self.id
    }

    /// Returns the tablet partition.
    pub fn lower_bound(&self) -> &PartitionKey {
        &self.lower_bound
    }

    pub fn upper_bound(&self) -> &PartitionKey {
        &self.upper_bound
    }

    pub fn info(&self) -> Result<TabletInfo> {
        unimplemented!()
    }
}

impl ReplicaSet for Arc<Tablet> {
    type Replica = TabletReplica;

    fn replicas(&self) -> &[TabletReplica] {
        &self.replicas
    }
}

/// Tablet replica belonging to a tablet server.
pub(crate) struct TabletReplica {
    id: TabletServerId,
    rpc_addrs: Box<[HostPort]>,
    proxy: krpc::Proxy,
    is_leader: AtomicBool,
    is_stale: AtomicBool,
}

impl Replica for TabletReplica {

    fn proxy(&self) -> krpc::Proxy {
        self.proxy.clone()
    }

    fn is_leader(&self) -> bool {
        self.is_leader.load(Relaxed)
    }

    fn mark_leader(&self) {
        self.is_leader.store(true, Relaxed)
    }

    fn mark_follower(&self) {
        self.is_leader.store(false, Relaxed)
    }

    fn is_stale(&self) -> bool {
        self.is_stale.load(Relaxed)
    }

    fn mark_stale(&self) {
        self.is_stale.store(true, Relaxed)
    }
}

impl fmt::Debug for TabletReplica {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TabletReplica")
            .field("id", &self.id)
            .field("is_leader", &self.is_leader)
            .field("is_stale", &self.is_stale)
            .finish()
    }
}

impl TabletReplica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> TabletServerId {
        self.id
    }
}

pub(crate) fn connect_to_cluster(master_addrs: Vec<HostPort>,
                                 options: &Options)
                                 -> impl Future<Item=Arc<Box<[MasterReplica]>>, Error=Error> {
    let admin_timeout = options.admin_timeout;
    stream::futures_unordered(master_addrs.into_iter()
                                          .map(|hostport| MasterReplica::new(hostport, options)))
           .collect()
           .and_then(move |replicas: Vec<MasterReplica>| {
               let replica_set = Arc::new(replicas.into_boxed_slice());
               let mut call = MasterService::connect_to_master(Default::default(),
                                                               Instant::now() + admin_timeout);
               call.set_required_feature_flags(&[MasterFeatures::ConnectToMaster as u32]);

               ReplicaRpc::new(replica_set.clone(),
                               call,
                               Speculation::Full,
                               Selection::Leader,
                               Backoff::with_duration_range(250, u32::max_value()))
                   .map(move |_| replica_set)
           })
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use env_logger;
    use tokio::runtime::current_thread::Runtime;

    use Client;
    use RangePartitionBound;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use replica::Replica;
    use schema::tests::simple_schema;
    use super::*;

    // TODO(tests):
    //  - Shutdown single master and ensure ConnectToCluster immediately fails.
    //  - Shutdown one master replica and ensure ConnectToCluster succeeds.
    //  - Shutdown all master replicas and ensure ConnectToCluster immediately fails.
    //  - Manipulate master election to make sure there's no leader, ensure ConnectToCluster waits,
    //    then elect a leader and ensure it completes.
    //  - Connect to cluster stress test: have a thread spawning a bunch of concurrent connect to
    //    cluster calls, and have the cluster undergo constant elections and failures.

    #[test]
    fn test_connect_to_cluster() {
        let _ = env_logger::try_init();
        let run = |num_masters: u32| {
            let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                                .num_masters(num_masters)
                                                                .num_tservers(0));
            let mut runtime = Runtime::new().unwrap();

            let masters = runtime.block_on(connect_to_cluster(cluster.master_addrs(),
                                                              &Options::default()))
                                 .unwrap();

            assert_eq!(masters.len(), num_masters as usize);
            assert_eq!(1, masters.iter().filter(|master| master.is_leader()).count());
        };

        run(1);
        run(3);
    }

    #[test]
    fn test_connect_to_cluster_unavailable() {
        let _ = env_logger::try_init();

        let run = |num_masters: u32| -> Error {
            let mut runtime = Runtime::new().unwrap();

            let hostports = {
                let listeners = (0..num_masters).map(|_| TcpListener::bind("127.0.0.1:0").unwrap())
                                                .collect::<Vec<_>>();
                listeners.iter().flat_map(TcpListener::local_addr).map(HostPort::from).collect::<Vec<HostPort>>()
            };

            runtime.block_on(connect_to_cluster(hostports, &Options::default())).unwrap_err()
        };

        match run(1) {
            Error::Io(..) => (),
            other => panic!("unexpected error: {}", other),
        }

        // TODO: Compound is a terrible error here.  The errors really need to be overhauled.
        match run(3) {
            Error::Compound(..) => (),
            other => panic!("unexpected error: {}", other),
        }
    }

    // Test cluster connection when one of the replicas is down.
    #[test]
    fn test_connect_to_cluster_failover() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(0));
        let mut runtime = Runtime::new().unwrap();

        let masters = runtime.block_on(connect_to_cluster(cluster.master_addrs(),
                                                            &Options::default()))
                                .unwrap();

        assert_eq!(3, cluster.master_addrs().len());
        assert_eq!(1, masters.iter().filter(|master| master.is_leader()).count());
    }

    /*
    /// Tests that RPCs are timed out when the leader is unavailable.
    #[test]
    fn timeout() {
        let _ = env_logger::try_init();
        let test_reactor = util::TestReactor::default();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(2)
                                                             .num_tservers(0)
                                                             .log_rpc_negotiation_trace(true)
                                                             .rpc_negotiation_delay(1000));
        let addr = cluster.master_addrs()[0];
        cluster.stop_node(addr);

        let now = Instant::now();
        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), test_reactor.io().clone());
        let result = proxy.list_tables(now + Duration::from_millis(100),
                                       ListTablesRequestPB::new())
                          .wait();

        let elapsed = Instant::now().duration_since(now);

        assert_eq!(Err(Error::TimedOut), result);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(100), "expected: 100ms, elapsed: {:?}", elapsed);
        assert!(elapsed < Duration::from_millis(150), "expected: 100ms, elapsed: {:?}", elapsed);
    }

    /// Tests that the `MasterProxy` will discover and reroute RPCs to a new leader when the
    /// current leader becomes unreachable.
    #[test]
    fn leader_failover() {
        let _ = env_logger::try_init();
        let test_reactor = util::TestReactor::default();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), test_reactor.io().clone());
        let result = proxy.list_tables(Instant::now() + Duration::from_secs(5),
                                       ListTablesRequestPB::new())
                          .wait();
        check_list_tables_response(result);

        // TODO: this check occasionally causes tests failures when only two of three masters comes
        // up before the initial election is decided, and we filter the master address of the
        // not-yet available replica.
        assert_eq!(3, replicas(&proxy).len());

        info!("Stopping leader {}", leader(&proxy));
        cluster.stop_node(leader(&proxy));

        info!("Sending list tables");
        // Reelection can take a disapointingly long time...
        let result = proxy.list_tables(Instant::now() + Duration::from_secs(10),
                                       ListTablesRequestPB::new())
                          .wait();
        check_list_tables_response(result);
    }
    */


    #[test]
    fn single_tablet() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime.block_on(Client::new(cluster.master_addrs(), Options::default()))
                                .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("single_tablet", schema.clone());
        table_builder.set_range_partition_columns(vec!["key".to_owned()]);
        table_builder.set_num_replicas(1);

        let table_id = runtime.block_on(client.create_table(table_builder)).expect("create_table");

        let table = runtime.block_on(client.open_table_by_id(table_id)).expect("open_table");
        let locations = table.table_locations().clone();

        {
            let entries = locations.entries.lock().clone();
            assert!(entries.is_empty());
        }

        {
            let entry = runtime.block_on(locations.entry(&[])).expect("entry");

            assert!(entry.is_tablet());
            assert_eq!(b"", entry.lower_bound());
            assert_eq!(b"", entry.upper_bound());

            let entries = locations.entries.lock().clone();
            assert_eq!(1, entries.len());
            assert!(entry.equiv(entries.values().next().unwrap()));
        }

        locations.clear();
        {
            let entry = runtime.block_on(locations.entry(b"some-key")).expect("entry");
            assert!(entry.is_tablet());
            assert_eq!(b"", entry.lower_bound());
            assert_eq!(b"", entry.upper_bound());
        }
    }

    #[test]
    fn multi_tablet() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime.block_on(Client::new(cluster.master_addrs(), Options::default()))
                                .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = runtime.block_on(client.create_table(table_builder)).expect("create_table");

        let table = runtime.block_on(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.table_locations().clone();

        let first = runtime.block_on(cache.entry(&[0, 0, 0, 0, 1])).expect("entry");

        assert!(first.is_tablet());
        assert_eq!(b"", first.lower_bound());
        assert_eq!(vec![0, 0, 0, 1], first.upper_bound());

        let entries: Vec<Entry> = cache.entries.lock().values().cloned().collect();
        assert_eq!(10, entries.len());
        assert!(first.equiv(&entries[0]));

        /*
        assert!(first.equiv(&cache.cached_entry(b"").unwrap()));
        assert!(cache.cached_entry(b"foo").is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());
        */

        let last = runtime.block_on(cache.entry(&[0, 0, 0, 11])).expect("entry");

        let entries: Vec<Entry> = cache.entries.lock().values().cloned().collect();

        assert_eq!(11, entries.len());
        assert!(entries[10].equiv(&last));

        /*
        assert!(cache.cached_entry(b"foo").unwrap().equiv(&last));
        assert!(cache.cached_entry(&vec![0, 0, 0, 10]).is_none());
        assert!(cache.cached_entry(&vec![0, 0, 0, 10, 5]).is_none());
        */

        runtime.block_on(cache.entry(&[0, 0, 0, 9])).expect("entry");
        assert_eq!(11, cache.entries.lock().len());

        runtime.block_on(cache.entry(&[0, 0, 0, 10])).expect("entry");
        assert_eq!(12, cache.entries.lock().len());
    }

    #[test]
    fn multi_tablet_concurrent() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime.block_on(Client::new(cluster.master_addrs(), Options::default()))
                                .expect("client");

        let schema = simple_schema();

        let mut table_builder = TableBuilder::new("multi_tablet_concurrent", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 12);
        table_builder.set_num_replicas(1);

        let table_id = runtime.block_on(client.create_table(table_builder)).expect("create_table");

        let table = runtime.block_on(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.table_locations().clone();

        runtime.block_on(cache.entry(&[0, 0, 0, 0]).join(cache.entry(&[0, 0, 0, 9])))
               .expect("entry");
        let entries: Vec<Entry> = cache.entries.lock().values().cloned().collect();
        assert_eq!(entries.len(), 10);
    }

    #[test]
    fn non_covered_ranges() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime.block_on(Client::new(cluster.master_addrs(), Options::default()))
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

        let table_id = runtime.block_on(client.create_table(table_builder)).expect("create_table");

        let table = runtime.block_on(client.open_table_by_id(table_id)).expect("open_table");
        let cache = table.table_locations().clone();

        runtime.block_on(cache.entry(&[0])).expect("entry");
        let entries: Vec<Entry> = cache.entries.lock().values().cloned().collect();
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
            runtime.block_on(cache.entry(key)).expect("entry");

            let new_entries: Vec<Entry> = cache.entries.lock().values().cloned().collect();
            for (expected_entry, new_entry) in entries[6 - expected_entries..].iter().zip(new_entries.iter()) {
                assert!(expected_entry.equiv(new_entry),
                       "key: {:?}, expected entry: {:?}, new entry: {:?}",
                       key, expected_entry, new_entry);
            }
        }
    }
}
