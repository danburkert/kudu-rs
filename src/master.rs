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
        proxy.refresh_leader();
        proxy
    }

    /// Sends a `ListTables` RPC to the leader master, and executes the provided callback on
    /// completion.
    pub fn list_tables(&self,
                       deadline: Instant,
                       request: ListTablesRequestPB,
                       callback: Box<Callback>) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let mut rpc = master::list_tables(addr, deadline, request);
        let inner = self.inner.clone();
        rpc.callback = Some(Box::new(move |result: RpcResult, rpc: Rpc| {
            if result.is_ok() {
                let is_not_leader_error = {
                    let resp = rpc.response::<ListTablesResponsePB>();
                    resp.has_error() &&
                        resp.get_error().get_code() == MasterErrorCode::NOT_THE_LEADER
                };
                if is_not_leader_error {
                    inner.send(rpc);
                    return;
                }
            };
            callback.callback(result, rpc);
        }));
        self.inner.send(rpc);
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
    fn refresh_leader(&self) {
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
        //self.inner.messenger.send(rpc);
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
            info!("discovered masters: {:?}", discovered_masters);
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
            // This can't happen, since `refresh_leader` is only executed once per leader cache
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

impl Inner {
    /// Sends the RPC if the leader master is known, otherwise queues the RPC to be sent when the
    /// leader is discovered.
    fn send(&self, mut rpc: Rpc) {
        let addr = match *self.leader.lock() {
            Leader::Known(addr) => addr,
            Leader::Unknown(ref mut queue) => {
                trace!("queueing rpc: {:?}", rpc);
                queue.push(rpc);
                return;
            },
        };
        rpc.addr = addr;
        self.messenger.send(rpc);
    }
}

struct LeaderRefreshCallback {
    inner: Arc<Inner>,
    backoff: Backoff,
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use mini_cluster::{get_unbound_address, MiniCluster, MiniClusterConfig};
    use super::*;
    use kudu_pb::master::{ListTablesRequestPB};
    use rpc::Messenger;

    use env_logger;
    use kudu_pb;

    #[test]
    fn list_tables() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(2).num_tservers(0));

        //::std::thread::sleep_ms(5000);

        let proxy = MasterProxy::new(cluster.master_addrs(), Messenger::new().unwrap());

        let (send, recv) = sync_channel(0);
        proxy.list_tables(Instant::now() + Duration::from_millis(500),
                          ListTablesRequestPB::new(), Box::new(move |result, rpc| {
                              send.send((result, rpc)).unwrap();
                          }));

        let (response, rpc) = recv.recv().unwrap();
    }

    //#[test]
    //fn list_tables_with_bogus_masters() {
        //let _ = env_logger::init();
        //let cluster = MiniCluster::new(MiniClusterConfig::default().num_tservers(0));

        //let masters = vec![get_unbound_address(), get_unbound_address()];
        //let proxy = MasterProxy::new(masters, Messenger::new().unwrap());


        //let (send, recv) = sync_channel(0);
        //proxy.list_tables(Instant::now() + Duration::from_millis(500),
                          //ListTablesRequestPB::new(), Box::new(move |result, rpc| {
                              //send.send((result, rpc)).unwrap();
                          //}));

        //let (response, rpc) = recv.recv().unwrap();
    //}
}
