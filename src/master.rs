use std::collections::HashSet;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use backoff::Backoff;
use dns::resolve_hosts_async;
use rpc::{
    Callback,
    Messenger,
    Rpc,
    RpcResult,
    master
};

use parking_lot::Mutex;
use kudu_pb::common::HostPortPB;
use kudu_pb::master::{
    GetMasterRegistrationRequestPB,
    GetMasterRegistrationResponsePB,
    ListMastersRequestPB,
    ListMastersResponsePB,
    ListTablesRequestPB,
    ListTablesResponsePB,
    MasterErrorPB_Code as MasterErrorCode
};
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};

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

// TODO:
//
// * Test leader cache invalidation.
// * Test master discovery.
// * Implement queued RPC timeout.
// * Invalidate masters when they leave the consensus group

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
                leader: Mutex::new(Leader::Unknown(Vec::with_capacity(QUEUE_LEN))),
                masters: Mutex::new(addrs),
                messenger: messenger,
            }),
        };
        proxy.refresh_leader_cache();
        proxy
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
            Leader::Unknown(_) => None,
        }
    }

    /// Sends a `ListTables` RPC to the leader master, and executes the provided callback on
    /// completion.
    pub fn list_tables(&self,
                       deadline: Instant,
                       request: ListTablesRequestPB,
                       callback: Box<Callback>) {

        struct ListTablesCB(MasterProxy, Box<Callback>, Backoff);
        impl Callback for ListTablesCB {
            fn callback(mut self: Box<Self>, result: RpcResult, mut rpc: Rpc) {
                if result.is_ok() {
                    let error_code = {
                        let resp = rpc.response::<ListTablesResponsePB>();
                        debug!("{:?} callback, response: {:?}", rpc, resp);
                        if resp.has_error() { Some(resp.get_error().get_code()) }
                        else { None }
                    };
                    match error_code {
                        Some(MasterErrorCode::NOT_THE_LEADER) => {
                            self.0.reset_leader_cache(rpc.addr);
                            let proxy: MasterProxy = self.0.clone();
                            rpc.callback = Some(self);
                            return proxy.send_to_leader(rpc);
                        },
                        Some(MasterErrorCode::CATALOG_MANAGER_NOT_INITIALIZED) => {
                            // This is a transient error which occurs when the master is starting up.
                            let delay = Duration::from_millis(self.2.next_backoff_ms());
                            info!("{:?}: Catalog manager not initialized, retrying after delay of {:?}",
                                   rpc, delay);
                            let messenger = self.0.inner.messenger.clone();
                            rpc.callback = Some(self);
                            return messenger.delayed_send(delay, rpc);
                        },
                        _ => (), // fall through to callback completion
                    }
                }

                // Complete the callback
                self.1.callback(result, rpc);
            }
        }

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let mut rpc = master::list_tables(addr, deadline, request);
        rpc.callback = Some(Box::new(ListTablesCB(self.clone(), callback,
                                                  Backoff::with_duration_range(8, 4096))));
        self.send_to_leader(rpc);
    }

    /// Sends the RPC if the leader master is known, otherwise queues the RPC to be sent when the
    /// leader is discovered.
    fn send_to_leader(&self, mut rpc: Rpc) {
        match *self.inner.leader.lock() {
            Leader::Known(addr) => {
                rpc.addr = addr;
                self.inner.messenger.send(rpc);
            },
            Leader::Unknown(ref mut queue) => {
                trace!("queueing rpc: {:?}", rpc);
                queue.push(rpc);
            },
        }
    }

    /// Clears the leader cache if the currently cached leader matches the provided address.
    /// If the cache is cleared, a refresh is initiated.
    fn reset_leader_cache(&self, stale_leader: SocketAddr) {
        {
            let mut master = self.inner.leader.lock();
            match *master {
                // Do nothing if the cached leader has already been refreshed.
                Leader::Unknown(_) => return,
                Leader::Known(leader) if leader != stale_leader => return,
                // Otherwise fall through to cache eviction and refresh.
                _ => (),
            }
            *master = Leader::Unknown(Vec::new());
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
    fn handle_list_masters_response(self, result: RpcResult, mut rpc: Rpc, backoff: Backoff) {
        // Short circuit if the master has already been found.
        if rpc.cancelled() { return; }
        let addr = rpc.addr;
        if let Err(error) = result {
            // Typically due to an RPC timeout because the master is not reachable.
            warn!("ListMasters RPC to master {} failed: {}", &addr, error);
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
            info!("discovered leader master {}, chosen randomly from possibilities {:?}",
                  addr, addrs);
        } else {
            info!("discovered leader master {}", addr);
        }

        match leader {
            Leader::Unknown(queued_rpcs) => {
                for mut queued_rpc in queued_rpcs {
                    queued_rpc.addr = addr;
                    self.inner.messenger.send(queued_rpc);
                }
            },
            // This can't happen, since `refresh_leader_cache` is only executed once per leader cache
            // invalidation, and only a single callback can replace the cache due to the
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
    Unknown(Vec<Rpc>),
}

impl Leader {
    fn is_known(&self) -> bool {
        match *self {
            Leader::Known(_) => true,
            Leader::Unknown(_) => false,
        }
    }
}

/// Container for master metadata.
struct Inner {
    leader: Mutex<Leader>,
    masters: Mutex<HashSet<SocketAddr>>,
    messenger: Messenger,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use mini_cluster::{get_unbound_address, MiniCluster, MiniClusterConfig};
    use rpc::{
        channel_callback,
        Messenger,
        Rpc,
        RpcResult,
    };
    use super::*;

    use env_logger;
    use kudu_pb::master::{ListTablesRequestPB, ListTablesResponsePB};

    fn check_list_tables_response(result: RpcResult, rpc: Rpc) {
        assert!(result.is_ok(), "failed result: {:?}", result);
        let response = rpc.response::<ListTablesResponsePB>();
        assert!(!response.has_error(), "failed response: {:?}", response);
        assert!(response.get_tables().is_empty(), "response: {:?}", response);
    }

    /// Tests that the leader is discovered in a single-master cluster.
    #[test]
    fn leader_discovery_single_master() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_tservers(1));

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), channel_callback(send));

        let (result, rpc) = recv.recv().unwrap();
        check_list_tables_response(result, rpc);
    }

    /// Tests that the leader is discovered in a multi-master cluster.
    #[test]
    fn leader_discovery_multi_master() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(3).num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_secs(5),
                          ListTablesRequestPB::new(), channel_callback(send));

        let (result, rpc) = recv.recv().unwrap();
        check_list_tables_response(result, rpc);
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
                          ListTablesRequestPB::new(), channel_callback(send));

        let (result, rpc) = recv.recv().unwrap();
        check_list_tables_response(result, rpc);

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
    }
}
