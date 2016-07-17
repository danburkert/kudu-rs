use std::cmp;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use protobuf::Message;
use backoff::Backoff;
use dns::resolve_hosts_async;
use rpc::{
    Callback,
    Messenger,
    Rpc,
    master
};
use Error;
use MasterError;
use MasterErrorCode;
use MasterId;
use RaftRole;
use Result;
use Status;

use parking_lot::Mutex;
use kudu_pb::common::HostPortPB;
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};
use kudu_pb::master::{
    AlterTableRequestPB, AlterTableResponsePB,
    CreateTableRequestPB, CreateTableResponsePB,
    DeleteTableRequestPB, DeleteTableResponsePB,
    GetTableLocationsRequestPB, GetTableLocationsResponsePB,
    GetTableSchemaRequestPB, GetTableSchemaResponsePB,
    GetTabletLocationsRequestPB, GetTabletLocationsResponsePB,
    IsAlterTableDoneRequestPB, IsAlterTableDoneResponsePB,
    IsCreateTableDoneRequestPB, IsCreateTableDoneResponsePB,
    ListMastersRequestPB, ListMastersResponsePB,
    ListTablesRequestPB, ListTablesResponsePB,
    ListTabletServersRequestPB, ListTabletServersResponsePB,
    PingRequestPB, PingResponsePB,
};
use kudu_pb::wire_protocol::{ServerEntryPB as MasterEntry};

/// Maximum number of RPCs to queue in the master proxy during leader discovery. When the queue is
/// full, additional attempts to send RPCs will immediately fail with `RpcError::Backoff`.
const QUEUE_LEN: usize = 256;

/// The leader refresh ListMaster RPCs should have a long enough timeout that non-failed masters
/// can respond, but short enough that the RPCs don't stay queued forever.
const LEADER_REFRESH_TIMEOUT_SECS: u64 = 10;

/// The `MasterProxy` tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
#[derive(Clone)]
pub struct MasterProxy {
    inner: Arc<Inner>,
}

macro_rules! impl_master_rpc {
    ($fn_name:ident, $request_type:ident, $response_type:ident) => {
        pub fn $fn_name<F>(&self, deadline: Instant, request: $request_type, cb: F)
            where F: FnOnce(Result<$response_type>) + Send + 'static {
                // The real leader address will be filled in by `send_to_leader`.
                let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
                let mut rpc = master::$fn_name(addr, deadline, request);
                rpc.callback = Some(Box::new(CB(self.clone(), cb,
                                                Backoff::with_duration_range(8, 4096),
                                                PhantomData::<$response_type>)));
                self.send_to_leader(rpc);
            }
    };
}

impl MasterProxy {

    /// Creates a new `MasterProxy` with an initial seed of master addresses, and a `Messenger`
    /// instance to handle sending RPCs.
    pub fn new(masters: &[SocketAddr], messenger: Messenger) -> MasterProxy {
        assert!(masters.len() > 0);

        // Give it a bit more capacity since modifications are done under a mutex.
        let mut addrs = HashSet::with_capacity(masters.len() * 2);
        addrs.extend(masters.iter().cloned());

        let proxy = MasterProxy {
            inner: Arc::new(Inner {
                leader: Mutex::new(Leader::Unknown{ queue: Vec::with_capacity(QUEUE_LEN),
                                                    deadline: None }),
                masters: Mutex::new(addrs),
                messenger: messenger,
            }),
        };
        proxy.refresh_leader_cache();
        proxy
    }

    pub fn messenger(&self) -> &Messenger {
        &self.inner.messenger
    }

    /// Returns the masters which this `MasterProxy` has discovered.
    pub fn masters(&self) -> Vec<SocketAddr> {
        let mut masters = Vec::with_capacity(5);
        masters.extend(self.inner.masters.lock().iter().cloned());
        masters
    }

    /// Returns the leader master, if it is known.
    pub fn leader(&self) -> Option<SocketAddr> {
        match *self.inner.leader.lock() {
            Leader::Known(leader) => Some(leader),
            Leader::Unknown{ .. } => None,
        }
    }

    impl_master_rpc!(alter_table, AlterTableRequestPB, AlterTableResponsePB);
    impl_master_rpc!(create_table, CreateTableRequestPB, CreateTableResponsePB);
    impl_master_rpc!(delete_table, DeleteTableRequestPB, DeleteTableResponsePB);
    impl_master_rpc!(get_table_locations, GetTableLocationsRequestPB, GetTableLocationsResponsePB);
    impl_master_rpc!(get_table_schema, GetTableSchemaRequestPB, GetTableSchemaResponsePB);
    impl_master_rpc!(get_tablet_locations, GetTabletLocationsRequestPB, GetTabletLocationsResponsePB);
    impl_master_rpc!(is_alter_table_done, IsAlterTableDoneRequestPB, IsAlterTableDoneResponsePB);
    impl_master_rpc!(is_create_table_done, IsCreateTableDoneRequestPB, IsCreateTableDoneResponsePB);
    impl_master_rpc!(list_masters, ListMastersRequestPB, ListMastersResponsePB);
    impl_master_rpc!(list_tables, ListTablesRequestPB, ListTablesResponsePB);
    impl_master_rpc!(list_tablet_servers, ListTabletServersRequestPB, ListTabletServersResponsePB);
    impl_master_rpc!(ping, PingRequestPB, PingResponsePB);

    /// Sends the RPC if the leader master is known, otherwise queues the RPC to be sent when the
    /// leader is discovered.
    fn send_to_leader(&self, mut rpc: Rpc) {
        let now = Instant::now();
        // Make sure that the duration_since call below doesn't panic.
        if rpc.timed_out(now) {
            rpc.fail(Error::TimedOut);
            return;
        }

        let mut timer_deadline = None;
        match *self.inner.leader.lock() {
            Leader::Known(addr) => {
                rpc.addr = addr;
                self.inner.messenger.send(rpc);
            },
            Leader::Unknown{ ref mut queue, ref mut deadline } => {
                trace!("queueing rpc: {:?}", rpc);
                *deadline = match *deadline {
                    Some(existing) if existing < rpc.deadline => Some(existing),
                    _ => {
                        timer_deadline = Some(rpc.deadline);
                        Some(rpc.deadline)
                    }
                };
                queue.push(rpc);
            },
        }

        if let Some(deadline) = timer_deadline {
            let master = self.clone();
            self.inner.messenger.timer(deadline.duration_since(now), Box::new(move || {
                master.timeout_queued_rpcs();
            }));
        }
    }

    /// Clears the leader cache if the currently cached leader matches the provided address.
    /// If the cache is cleared, a refresh is initiated.
    fn reset_leader_cache(&self, stale_leader: SocketAddr) {
        {
            let mut master = self.inner.leader.lock();
            match *master {
                // Do nothing if the cached leader has already been refreshed.
                Leader::Unknown{ .. } => return,
                Leader::Known(leader) if leader != stale_leader => return,
                // Otherwise fall through to cache eviction and refresh.
                _ => (),
            }
            *master = Leader::Unknown { queue: Vec::new(), deadline: None };
        }
        self.refresh_leader_cache();
    }

    /// Asynchronously refreshes the cached leader master.
    ///
    /// This protocol has to handle failed leader replicas, as well as the possibility that the
    /// current leader master is not in the set of known master replicas. The control flow is:
    ///
    /// For every known master replica, create and send a `ListMasters` RPC. Each of these RPCs
    /// shares a single cancellation token, so each RPC can be notified of cancellation, and each
    /// can trigger a cancellation of all others.
    ///
    /// For each `ListMasters` response, extract the set of master addresses and the subset of
    /// leader addresses. For each master address, determine if we know about the master already.
    /// If not, add it to the list of known masters, and send a new `ListMasters` RPC to it. If a
    /// leader address is discovered, check if the leader is still unknown. If it is, then reset
    /// the cached leader master to the discovered leader, send all queued RPCs to the discovered
    /// leader, and cancel all other `ListMasters` RPCs.
    ///
    /// If the `ListMasters` RPC fails or if it does not discover a leader, then it is retried
    /// after a backoff period.
    fn refresh_leader_cache(&self) {
        debug_assert!(!self.inner.leader.lock().is_known());
        let cancel = Arc::new(AtomicBool::new(false));
        let masters = self.inner.masters.lock().iter().cloned().collect::<Vec<_>>();

        let deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
        for addr in masters {
            self.send_list_masters(addr, deadline, cancel.clone());
        }
    }

    /// Sends a `ListMasters` RPC to a master with the provided deadline and cancellation token.
    /// The completion handler is set to `handle_list_masters_response`.
    fn send_list_masters(&self, addr: SocketAddr, deadline: Instant, cancel: Arc<AtomicBool>) {

        // Backoff that manages backoffs between retries.
        let backoff = Backoff::with_duration_range(128, 8192);

        let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
        rpc.cancel = Some(cancel);
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            proxy.handle_list_masters_response(result, rpc, backoff);
        }));
        self.inner.messenger.send(rpc);
    }

    /// Retries a `ListMasters` RPC after a backoff period.
    fn retry_list_masters(self, mut rpc: Rpc, mut backoff: Backoff) {
        let delay = Duration::from_millis(backoff.next_backoff_ms());
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            proxy.handle_list_masters_response(result, rpc, backoff);
        }));
        rpc.deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
        trace!("retrying ListMasters RPC to {} after delay of {:?}", &rpc.addr, &delay);
        self.inner.messenger.delayed_send(delay, rpc);
    }

    /// Handles the response to a `ListMasters` RPC.
    fn handle_list_masters_response(self, result: Result<()>, mut rpc: Rpc, backoff: Backoff) {
        // Short circuit if the master has already been found.
        if rpc.cancelled() { return; }
        let addr = rpc.addr;
        if let Err(error) = result {
            info!("ListMasters RPC to master {} failed: {}", &addr, error);
            // Fall through to retry.
        } else {
            let cancel = rpc.cancel.as_ref().unwrap().clone();
            let response = rpc.mut_response::<ListMastersResponsePB>();
            if response.has_error() {
                // This can happen when the master is not yet initialized, e.g.:
                // code: CATALOG_MANAGER_NOT_INITIALIZED
                // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
                info!("ListMasters RPC to master {} failed: {:?}", &addr, response.get_error());
                // Fall through to retry.
            } else {
                debug!("ListMasters RPC to master {} response: {:?}", &addr, response);
                let mut masters: Vec<HostPortPB> = Vec::new();
                let mut leaders: Vec<HostPortPB> = Vec::new();

                for server_entry in response.mut_masters().iter_mut() {
                    if server_entry.has_error()  { continue; }
                    if server_entry.get_role() == Role::LEADER {
                        leaders.extend(server_entry.get_registration()
                                                   .get_rpc_addresses()
                                                   .iter()
                                                   .cloned());
                    }
                    masters.extend(server_entry.mut_registration()
                                               .take_rpc_addresses()
                                               .into_iter());
                }

                if !masters.is_empty() {
                    let proxy = self.clone();
                    let cancel = cancel.clone();
                    resolve_hosts_async(masters, move |addrs| {
                        proxy.handle_discovered_masters(addrs, cancel)
                    });
                }

                if !leaders.is_empty() {
                    let proxy = self.clone();
                    resolve_hosts_async(leaders, move |addrs| {
                        proxy.handle_discovered_leaders(addrs, cancel)
                    });
                }

                // Fall through to retry. We retry even if a leader registration is found because
                // the DNS resolution can fail.
            }
        }

        self.retry_list_masters(rpc, backoff);
    }

    /// Handler executed when a set of master addresses is returned from a `ListMasters` RPC.
    fn handle_discovered_masters(self, addrs: HashSet<SocketAddr>, cancel: Arc<AtomicBool>) {
        if addrs.is_empty() { return; }

        let mut discovered_masters = Vec::with_capacity(addrs.len());
        {
            let mut masters = self.inner.masters.lock();
            for addr in addrs {
                if masters.insert(addr) { discovered_masters.push(addr); }
            }
        }

        if !discovered_masters.is_empty() {
            info!("discovered masters: [{}]",
                  discovered_masters.iter().map(ToString::to_string).collect::<Vec<_>>().join(","));
            if cancel.load(Ordering::Relaxed) { return; }
            let deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
            for addr in discovered_masters {
                self.send_list_masters(addr, deadline, cancel.clone());
            }
        }
    }

    /// Handler executed when a set of master leader addresses is returned from a `ListMasters` RPC.
    fn handle_discovered_leaders(self, addrs: HashSet<SocketAddr>, cancel: Arc<AtomicBool>) {
        // Short circuit if the leader has already been found.
        if addrs.is_empty() || cancel.load(Ordering::Relaxed) { return; }

        let addr = *addrs.iter().next().unwrap();

        let leader = {
            // Swap out the Unknown leader queue with the new Known leader. We also attempt to
            // cancel all other RPCs while holding the lock, so that the cancellation happens
            // atomically with changing the leader state. This prevents a TOCTOU race between the
            // cancellation check at the start of this function and now.
            let mut leader = self.inner.leader.lock();
            if cancel.compare_and_swap(false, true, Ordering::Relaxed) {
                // Another replica has already claimed to be the leader.
                debug!("another replica already claimed it!");
                return;
            }

            mem::replace(&mut *leader, Leader::Known(addr))
        };

        if addrs.len() > 1 {
            info!("discovered leader master {}, chosen randomly from possibilities [{}]",
                  addr, addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "));
        } else {
            info!("discovered leader master {}", addr);
        }

        match leader {
            Leader::Unknown { queue, .. } => {
                for mut rpc in queue {
                    rpc.addr = addr;
                    self.inner.messenger.send(rpc);
                }
            },
            // This can't happen, since `refresh_leader_cache` is only executed once per leader
            // cache invalidation, and only a single callback can replace the cache due to the
            // cancellation token.
            Leader::Known(prev_addr) => unreachable!("existing known leader {} swapped for {}",
                                                     prev_addr, &addr),
        }
    }

    /// Times out all queued rpcs.
    fn timeout_queued_rpcs(&self) {
        let now = Instant::now();
        let mut next_deadline = None;

        let mut retained_rpcs = Vec::with_capacity(QUEUE_LEN);
        let mut failed_rpcs = Vec::new();

        match *self.inner.leader.lock() {
            Leader::Unknown { ref mut queue, ref mut deadline } => {
                for rpc in queue.drain(..) {
                    if rpc.cancelled() || rpc.timed_out(now) {
                        failed_rpcs.push(rpc);
                    } else {
                        next_deadline = Some(next_deadline.map_or(rpc.deadline, |t| cmp::min(t, rpc.deadline)));
                        retained_rpcs.push(rpc);
                    }
                }
                *queue = retained_rpcs;
                *deadline = next_deadline;
            },
            Leader::Known(_) => (),
        }

        for rpc in failed_rpcs {
            if rpc.cancelled() {
                rpc.fail(Error::Cancelled);
            } else {
                rpc.fail(Error::TimedOut);
            }
        }

        if let Some(deadline) = next_deadline {
            let master = self.clone();
            self.inner.messenger.timer(deadline.duration_since(now), Box::new(move || {
                master.timeout_queued_rpcs();
            }));
        }
    }
}

enum Leader {
    /// The known leader.
    Known(SocketAddr),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    /// holds the queue of RPCs, and the next registered timeout deadline.
    Unknown {
        queue: Vec<Rpc>,
        deadline: Option<Instant>,
    }
}

impl Leader {
    fn is_known(&self) -> bool {
        match *self {
            Leader::Known(_) => true,
            Leader::Unknown { .. } => false,
        }
    }
}

/// Container for master metadata.
struct Inner {
    leader: Mutex<Leader>,
    masters: Mutex<HashSet<SocketAddr>>,
    messenger: Messenger,
}

trait MasterResponse: Message {
    fn error(&mut self) -> Option<MasterError>;
}
macro_rules! impl_master_response {
    ($response_type:ident) => {
        impl MasterResponse for $response_type {
            fn error(&mut self) -> Option<MasterError> {
                if self.has_error() { Some(MasterError::from(self.take_error())) } else { None }
            }
        }
    };
    ($response_type:ident, no_error) => {
        impl MasterResponse for $response_type {
            fn error(&mut self) -> Option<MasterError> { None }
        }
    };
}
impl_master_response!(AlterTableResponsePB);
impl_master_response!(CreateTableResponsePB);
impl_master_response!(DeleteTableResponsePB);
impl_master_response!(GetTableLocationsResponsePB);
impl_master_response!(GetTableSchemaResponsePB);
impl_master_response!(GetTabletLocationsResponsePB);
impl_master_response!(IsAlterTableDoneResponsePB);
impl_master_response!(IsCreateTableDoneResponsePB);
impl_master_response!(ListTablesResponsePB);
impl_master_response!(ListTabletServersResponsePB);
impl_master_response!(PingResponsePB, no_error);

impl MasterResponse for ListMastersResponsePB {
    fn error(&mut self) -> Option<MasterError> {
        if self.has_error() {
            Some(MasterError::new(MasterErrorCode::UnknownError, Status::from(self.take_error())))
        } else { None }
    }
}

struct CB<Resp, F>(MasterProxy, F, Backoff, PhantomData<Resp>)
where Resp: MasterResponse, F: FnOnce(Result<Resp>) + Send + 'static;
impl <Resp, F> Callback for CB<Resp, F>
where Resp: MasterResponse, F: FnOnce(Result<Resp>) + Send + 'static {
    fn callback(mut self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
        match result {
            Ok(_) => match rpc.mut_response::<Resp>().error() {
                Some(ref error) if error.code() == MasterErrorCode::NotTheLeader => {
                    self.0.reset_leader_cache(rpc.addr);
                    let proxy: MasterProxy = self.0.clone();
                    rpc.callback = Some(self);
                    proxy.send_to_leader(rpc);
                },
                Some(ref error) if error.code() == MasterErrorCode::CatalogManagerNotInitialized => {
                    // This is a transient error which occurs when the master is starting up.
                    let delay = Duration::from_millis(self.2.next_backoff_ms());
                    info!("{:?}: Catalog manager not initialized, retrying after delay of {:?}",
                            rpc, delay);
                    let messenger = self.0.inner.messenger.clone();
                    rpc.callback = Some(self);
                    messenger.delayed_send(delay, rpc);
                },
                Some(error) => self.1(Err(Error::from(MasterError::from(error)))),
                None => self.1(Ok(rpc.take_response::<Resp>())),
            },
            Err(ref error) if error.is_network_error() => {
                // On connection error, reset the master cache and resend.
                self.0.reset_leader_cache(rpc.addr);
                let proxy: MasterProxy = self.0.clone();
                rpc.callback = Some(self);
                proxy.send_to_leader(rpc);
            }
            Err(error) => self.1(Err(error)),
        }
    }
}

/// Master metadata.
///
/// This information should be considered 'point-in-time', and may change as the cluster topology
/// changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Master {
    id: MasterId,
    rpc_addrs: Vec<(String, u16)>,
    http_addrs: Vec<(String, u16)>,
    seqno: i64,
    role: RaftRole,
}

impl Master {
    pub fn id(&self) -> &MasterId {
        &self.id
    }

    pub fn rpc_addrs(&self) -> &[(String, u16)] {
        &self.rpc_addrs
    }

    pub fn http_addrs(&self) -> &[(String, u16)] {
        &self.http_addrs
    }

    pub fn seqno(&self) -> i64 {
        self.seqno
    }

    pub fn role(&self) -> RaftRole {
        self.role
    }

    #[doc(hidden)]
    pub fn from_pb(mut master: MasterEntry) -> Result<Master> {
        if master.has_error() {
            return Err(Error::from(MasterError::new(MasterErrorCode::UnknownError,
                                                    Status::from(master.take_error()))))
        }

        let id = try!(MasterId::parse_bytes(master.get_instance_id().get_permanent_uuid()));
        let seqno = master.get_instance_id().get_instance_seqno();

        // TODO: check bounds on port casts.
        let rpc_addrs = master.mut_registration()
                                     .take_rpc_addresses()
                                     .into_iter()
                                     .map(|mut host_port| (host_port.take_host(),
                                                           host_port.get_port() as u16))
                                     .collect::<Vec<_>>();
        let http_addrs = master.mut_registration()
                                      .take_http_addresses()
                                      .into_iter()
                                      .map(|mut host_port| (host_port.take_host(),
                                                            host_port.get_port() as u16))
                                      .collect::<Vec<_>>();

        let role = RaftRole::from_pb(master.get_role());

        Ok(Master {
            id: id,
            rpc_addrs: rpc_addrs,
            http_addrs: http_addrs,
            seqno: seqno,
            role: role,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use rpc:: Messenger;
    use Error;
    use Result;
    use super::*;

    use env_logger;
    use kudu_pb::master::{ListTablesRequestPB, ListTablesResponsePB};

    fn check_list_tables_response(result: Result<ListTablesResponsePB>) {
        let response = result.unwrap();
        assert!(!response.has_error(), "failed response: {:?}", response);
        assert!(response.get_tables().is_empty(), "response: {:?}", response);
    }

    /// Tests that the leader is discovered in a single-master cluster.
    #[test]
    fn leader_discovery_single_master() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
    }

    /// Tests that the leader is discovered in a multi-master cluster.
    #[test]
    fn leader_discovery_multi_master() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(3).num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
    }

    /// Tests that masters which are not part of the original seed list are discovered in a
    /// multi-master cluster.
    #[test]
    fn master_discovery() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(2).num_tservers(0));

        let proxy = MasterProxy::new(&cluster.master_addrs()[0..1], Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);

        let mut discovered_masters = HashSet::new();
        let mut cluster_masters = HashSet::new();
        discovered_masters.extend(proxy.masters());
        cluster_masters.extend(cluster.master_addrs().iter().cloned());
        assert_eq!(cluster_masters, discovered_masters);
    }

    /// Tests that the `MasterProxy` will remove masters not known by the leader.
    fn master_cleanup() {
    }

    /// Tests that RPCs are timed out when the leader is unavailable.
    #[test]
    fn timeout() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(2)
                                                             .num_tservers(0)
                                                             .log_rpc_negotiation_trace(true)
                                                             .rpc_negotiation_delay(1000));
        let addr = cluster.master_addrs()[0];
        cluster.stop_node(addr);

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());
        let now = Instant::now();

        let (send, recv) = sync_channel(0);
        proxy.list_tables(now + Duration::from_millis(300),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();

        let elapsed = Instant::now().duration_since(now);

        assert_eq!(Err(Error::TimedOut), result);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(275), "expected: 300ms, elapsed: {:?}", elapsed);
        assert!(elapsed < Duration::from_millis(325), "expected: 300ms, elapsed: {:?}", elapsed);
    }

    /// Tests that the `MasterProxy` will discover and reroute RPCs to a new leader when the
    /// current leader becomes unreachable.
    #[test]
    fn leader_failover() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(0));

        let proxy = MasterProxy::new(&cluster.master_addrs()[0..1], Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        let send_copy = send.clone();
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), move |resp| send_copy.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
        assert_eq!(3, proxy.masters().len());

        info!("Stopping leader {}", proxy.leader().unwrap());
        cluster.stop_node(proxy.leader().unwrap());

        proxy.list_tables(Instant::now() + Duration::from_secs(10),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
    }
}
