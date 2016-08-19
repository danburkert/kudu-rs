use std::collections::HashSet;
use std::marker::PhantomData;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use dns;
use backoff::Backoff;
use itertools::Itertools;
use protobuf::Message;
use queue_map::QueueMap;
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
const LEADER_REFRESH_TIMEOUT_SECS: u64 = 60;

macro_rules! impl_master_rpc {
    ($fn_name:ident, $request_type:ident, $response_type:ident) => {
        pub fn $fn_name<F>(&self, deadline: Instant, request: $request_type, cb: F)
            where F: FnOnce(Result<$response_type>) + Send + 'static {
                // The real leader address will be filled in by `send_to_leader`.
                let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
                let mut rpc = master::$fn_name(addr, deadline, request);
                rpc.callback = Some(Box::new(CB(self.clone(), cb, PhantomData::<$response_type>)));
                self.send_to_leader(rpc);
            }
    };
}

/// The `MasterProxy` tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
#[derive(Clone)]
pub struct MasterProxy {
    inner: Arc<Mutex<Inner>>,
    messenger: Messenger,
}

/// Container for master metadata.
struct Inner {
    leader: Leader,
    replicas: HashSet<SocketAddr>,
}

impl MasterProxy {

    /// Creates a new `MasterProxy` with an initial seed of master addresses, and a `Messenger`
    /// instance to handle sending RPCs.
    pub fn new(replicas: &[SocketAddr], messenger: Messenger) -> MasterProxy {
        assert!(replicas.len() > 0);
        let replicas = replicas.iter().cloned().collect();
        let proxy = MasterProxy {
            inner: Arc::new(Mutex::new(Inner {
                leader: Leader::Unknown(QueueMap::with_capacity(QUEUE_LEN)),
                replicas: replicas,
            })),
            messenger: messenger,
        };
        proxy.refresh_leader_cache();
        proxy
    }

    pub fn messenger(&self) -> &Messenger {
        &self.messenger
    }

    /// Returns the masters which this `MasterProxy` has discovered.
    pub fn masters(&self) -> Vec<SocketAddr> {
        let mut masters = Vec::with_capacity(5);
        masters.extend(self.inner.lock().replicas.iter().cloned());
        masters
    }

    /// Returns the leader master, if it is known.
    pub fn leader(&self) -> Option<SocketAddr> {
        match self.inner.lock().leader {
            Leader::Known(leader) => Some(leader),
            Leader::Unknown(_) => None,
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
    pub fn send_to_leader(&self, rpc: Rpc) {
        /// Returns the queue index if the RPC is queued.
        fn inner(master_proxy: &MasterProxy, mut rpc: Rpc) -> Option<usize> {
            // Keep critical section short by performing the send outside the lock.
            let leader_addr = {
                let mut inner = master_proxy.inner.lock();
                match inner.leader {
                    Leader::Known(addr) => Some(addr),
                    Leader::Unknown(ref mut queue) => {
                        if queue.len() < QUEUE_LEN {
                            return Some(queue.push(rpc));
                        }
                        None
                    },
                }
            };
            if let Some(leader_addr) = leader_addr {
                rpc.addr = leader_addr;
                master_proxy.messenger.send(rpc);
            } else {
                rpc.fail(Error::Backoff);
            }
            None
        }

        // This control flow is a bit funky to keep the critical section short and still appease the
        // borrow checker.

        let now = Instant::now();
        // Make sure that the duration_since call below doesn't panic.
        if rpc.timed_out(now) {
            rpc.fail(Error::TimedOut);
            return;
        }
        let duration = rpc.deadline.duration_since(now);

        if let Some(queue_idx) = inner(self, rpc) {
            let master_proxy = self.clone();
            self.messenger.timer(duration, Box::new(move || {
                master_proxy.timeout_queued_rpc(queue_idx)
            }));
        }
    }

    /// Times out the queued RPC with the given index.
    fn timeout_queued_rpc(&self, queue_idx: usize) {
        // Keep the critical section short.
        let mut rpc = None;
        if let Leader::Unknown(ref mut queue) = self.inner.lock().leader {
            rpc = queue.remove(queue_idx);
        }
        if let Some(rpc) = rpc {
            // Warning: extreme hack. We can get 'false positive' timeout callbacks when we
            // transition from Unknown -> Known -> Unknown, because the queue resets the idx
            // counter back to 0, so we may get the callback from a previous Unkown era. To
            // properly fix this we either need to keep the timeout handle with the queued RPCs,
            // like in Connection, or keep a transition counter or something. Easier than all that
            // is to just check if the RPC is actually timed out.
            if rpc.timed_out(Instant::now()) {
                rpc.fail(Error::TimedOut);
            } else {
                self.send_to_leader(rpc);
            }
        }
    }

    /// Clears the leader cache if the currently cached leader matches the provided address.
    /// If the cache is cleared, a refresh is initiated.
    fn reset_leader_cache(&self, stale_leader: SocketAddr) {
        let queue = QueueMap::with_capacity(QUEUE_LEN);
        {
            let mut inner = self.inner.lock();
            match inner.leader {
                // Do nothing if the cached leader has already been refreshed.
                Leader::Unknown(_) => return,
                Leader::Known(leader) if leader != stale_leader => return,
                // Otherwise fall through to cache eviction and refresh.
                _ => (),
            }
            inner.leader = Leader::Unknown(queue);
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
    /// For each `ListMasters` response, extract the set of replica addresses and the set of leader
    /// addresses. If a leader address is discovered, update the cached leader address, cancel all
    /// outstanding `ListMaster` RPCs, and dispatch all queued RPCs. Otherwise dispatch a new
    /// `ListMasters` RPC for each newly discovered replica, and retry the original RPC after a
    /// backoff.
    fn refresh_leader_cache(&self) {
        debug_assert!(!self.inner.lock().leader.is_known());
        let cancel = Arc::new(AtomicBool::new(false));
        let replicas = self.inner.lock().replicas.iter().cloned().collect::<Vec<_>>();
        debug!("refreshing leader master from known replicas: {:?}", replicas);

        let deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
        for addr in replicas {
            self.send_list_masters(addr, deadline, cancel.clone());
        }
    }

    /// Sends a `ListMasters` RPC to a master with the provided deadline and cancellation token.
    /// The completion handler is set to `handle_list_masters_response`.
    fn send_list_masters(&self, addr: SocketAddr, deadline: Instant, cancel: Arc<AtomicBool>) {
        trace!("sending ListMasters RPC to {}", addr);

        // Backoff that manages backoffs between retries.
        let backoff = Backoff::with_duration_range(10, 30_000);

        let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
        rpc.cancel = Some(cancel);
        // This RPC can't be retried at another replica, so the RPC layer should retry it
        // automatically.
        rpc.fail_fast = false;
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            thread::spawn(move || proxy.handle_list_masters_response(result, rpc, backoff));
        }));
        self.messenger.send(rpc);
    }

    /// Retries a `ListMasters` RPC after a backoff period.
    fn retry_list_masters(self, mut rpc: Rpc, mut backoff: Backoff) {
        // Short circuit if the leader has already been found.
        if rpc.cancelled() { return; }
        let delay_ms = backoff.next_backoff_ms();
        let delay = Duration::from_millis(delay_ms);
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            thread::spawn(move || proxy.handle_list_masters_response(result, rpc, backoff));
        }));
        rpc.deadline = Instant::now() +
                       Duration::from_millis(delay_ms) +
                       Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
        trace!("retrying ListMasters RPC to {} after delay of {}ms", rpc.addr, delay_ms);
        self.messenger.delayed_send(delay, rpc);
    }

    /// Handles the response to a `ListMasters` RPC.
    /// This should *not* be called on the Event Loop thread.
    fn handle_list_masters_response(self, result: Result<()>, mut rpc: Rpc, backoff: Backoff) {
        // Short circuit if the master has already been found.
        if rpc.cancelled() { return; }
        let addr = rpc.addr;

        // Holds the resolved addresses for all replicas listed by the master response.
        let mut replicas: HashSet<SocketAddr> = HashSet::new();

        // Holds the resolved addresses for the leader master.
        let mut leader: HashSet<SocketAddr> = HashSet::new();

        if let Err(error) = result {
            info!("ListMasters RPC to master {} failed: {}", addr, error);
            // Fall through to retry.
        } else {
            let response = rpc.mut_response::<ListMastersResponsePB>();
            if response.has_error() {
                // This can happen when the master is not yet initialized, e.g.:
                // code: CATALOG_MANAGER_NOT_INITIALIZED
                // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
                info!("ListMasters RPC to master {} failed: {:?}", addr, response.get_error());
                // Fall through to retry.
            } else {
                debug!("ListMasters RPC to master {} response: {:?}", addr, response);

                for server_entry in response.mut_masters().iter_mut() {
                    if server_entry.has_error()  { continue; }
                    let addrs = dns::resolve_hosts(server_entry.get_registration()
                                                               .get_rpc_addresses());
                    replicas.extend(addrs.iter().cloned());

                    if server_entry.get_role() == Role::LEADER {
                        // Check that an individual ListMasters response contains at most a single
                        // leader node.
                        assert!(leader.is_empty());
                        leader = addrs;
                    }
                }
            }
        }

        // Short circuit if the master has already been found.
        if rpc.cancelled() { return; }
        let cancel = rpc.cancel.as_ref().unwrap().clone();
        if replicas.is_empty() {
            // The ListMasters call either completely failed, or we weren't able to resolve any
            // master addresses.
            self.retry_list_masters(rpc, backoff);
        } else if leader.is_empty() {
            // We found some replicas, but no leader addresses.
            self.handle_discovered_replicas(replicas, cancel);
            self.retry_list_masters(rpc, backoff);
        } else {
            self.handle_discovered_leaders(leader, replicas, cancel);
        }
    }

    /// Handler executed when a set of master addresses is returned from a `ListMasters` RPC.
    fn handle_discovered_replicas(&self, addrs: HashSet<SocketAddr>, cancel: Arc<AtomicBool>) {
        let mut discovered_replicas = Vec::with_capacity(addrs.len());
        {
            // Take the master lock, then check the cancellation token to make sure we haven't
            // been cancelled. The check must be done under the lock to close a race with a
            // concurrent handle_discovered_leaders call.
            let mut inner = self.inner.lock();
            if cancel.load(Ordering::Relaxed) { return; }
            for addr in addrs {
                if inner.replicas.insert(addr) { discovered_replicas.push(addr); }
            }
        }

        if !discovered_replicas.is_empty() {
            info!("discovered replicas: [{}]", discovered_replicas.iter().format_default(", "));
            let deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
            for addr in discovered_replicas {
                self.send_list_masters(addr, deadline, cancel.clone());
            }
        }
    }

    /// Handler executed when a set of master leader addresses is returned from a `ListMasters` RPC.
    fn handle_discovered_leaders(&self,
                                 leader: HashSet<SocketAddr>,
                                 replicas: HashSet<SocketAddr>,
                                 cancel: Arc<AtomicBool>) {
        let addr = match leader.iter().next() {
            Some(&addr) => addr,
            None => return,
        };

        let inner = {
            // Swap out the Unknown leader queue with the new Known leader. We also attempt to
            // cancel all other RPCs while holding the lock, so that the cancellation happens
            // atomically with changing the leader state. This prevents a race with a concurrent
            // handle_discovered_replicas call.
            let mut inner = self.inner.lock();
            if cancel.compare_and_swap(false, true, Ordering::Relaxed) {
                // Another replica has already claimed to be the leader.
                return;
            }

            // We replace the entire replica set instead of adding to it so that we retain only the
            // replicas that the current leader knows about. This serves to filter out old master
            // replicas after they are no longer around.
            mem::replace(&mut *inner, Inner { leader: Leader::Known(addr), replicas: replicas })
        };

        if leader.len() > 1 {
            info!("discovered leader master {}, chosen from resolved addresses [{}]",
                  addr, leader.iter().format_default(", "));
        } else {
            info!("discovered leader master {}", addr);
        }

        match inner.leader {
            Leader::Unknown(mut queue) => {
                while let Some((_, mut rpc)) = queue.pop() {
                    rpc.addr = addr;
                    self.messenger.send(rpc);
                }
            },
            // This can't happen, since `refresh_leader_cache` is only executed once per leader
            // cache invalidation, and only a single callback can replace the cache due to the
            // cancellation token.
            Leader::Known(prev_addr) => unreachable!("existing known leader {} swapped for {}",
                                                     prev_addr, &addr),
        }
    }
}

enum Leader {
    /// The known leader.
    Known(SocketAddr),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    Unknown(QueueMap<Rpc>),
}

impl Leader {
    fn is_known(&self) -> bool {
        match *self {
            Leader::Known(_) => true,
            Leader::Unknown(_) => false,
        }
    }
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

struct CB<Resp, F>(MasterProxy, F, PhantomData<Resp>)
where Resp: MasterResponse, F: FnOnce(Result<Resp>) + Send + 'static;
impl <Resp, F> Callback for CB<Resp, F>
where Resp: MasterResponse, F: FnOnce(Result<Resp>) + Send + 'static {
    fn callback(self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
        match result {
            Ok(_) => match rpc.mut_response::<Resp>().error() {
                Some(ref error) if error.code() == MasterErrorCode::NotTheLeader ||
                                   error.code() == MasterErrorCode::CatalogManagerNotInitialized => {
                    self.0.reset_leader_cache(rpc.addr);
                    let proxy: MasterProxy = self.0.clone();
                    rpc.callback = Some(self);
                    proxy.send_to_leader(rpc);
                },
                Some(error) => self.1(Err(Error::from(MasterError::from(error)))),
                None => self.1(Ok(rpc.take_response::<Resp>())),
            },
            Err(ref error) if error.is_network_error() => {
                // On connection error, reset the leader cache and resend.
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
        proxy.list_tables(now + Duration::from_millis(100),
                          ListTablesRequestPB::new(), move |resp| send.send(resp).unwrap());

        let result = recv.recv().unwrap();

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
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(0));

        let proxy = MasterProxy::new(&cluster.master_addrs()[0..1], Messenger::new().unwrap());

        let (send, recv) = sync_channel(1);
        let send_copy = send.clone();
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), move |resp| send_copy.send(resp).unwrap());

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
        // TODO: this check occasionally causes tests failures when only two of three masters comes
        // up before the initial election is decided, and we filter the master address of the
        // not-yet available replica.
        assert_eq!(3, proxy.masters().len());

        info!("Stopping leader {}", proxy.leader().unwrap());
        cluster.stop_node(proxy.leader().unwrap());

        // Reelection can take a disapointingly long time...
        proxy.list_tables(Instant::now() + Duration::from_secs(10),
                          ListTablesRequestPB::new(), move |resp| {
                              send.send(resp).unwrap()
                          });

        let result = recv.recv().unwrap();
        check_list_tables_response(result);
    }
}
