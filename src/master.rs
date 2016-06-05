use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use backoff::Backoff;
use rpc::{
    Callback,
    Messenger,
    Rpc,
    RpcResult,
    master
};

use parking_lot::Mutex;
use kudu_pb::master::{
    GetMasterRegistrationRequestPB,
    GetMasterRegistrationResponsePB,
    ListTablesRequestPB,
    ListTablesResponsePB,
    MasterErrorPB_Code as MasterErrorCode
};
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};

/// Maximum number of RPCs to queue in the master proxy during leader discovery. When the queue is
/// full, additional attempts to send RPCs will immediately fail with `RpcError::Backoff`.
const QUEUE_LEN: usize = 256;

/// The master proxy tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
///
/// This type is a thin public facade over the `Inner` type, which holds all state. The state must
/// be shareable among RPC callbacks, so it's wrapped in an `Arc`.
pub struct MasterProxy {
    inner: Arc<Inner>,
}

impl MasterProxy {

    /// Creates a new `MasterProxy` with an initial seed of master addresses, and a `Messenger`
    /// instance to handle sending RPCs.
    pub fn new(masters: Vec<SocketAddr>, messenger: Messenger) -> MasterProxy {
        assert!(masters.len() > 0);
        let proxy = MasterProxy {
            inner: Arc::new(Inner {
                leader: Mutex::new(Leader::Unknown(Vec::with_capacity(QUEUE_LEN))),
                masters: Mutex::new(masters),
                messenger: messenger,
            }),
        };
        refresh_leader(&proxy.inner);
        proxy
    }

    /// Sends a `ListTables` RPC to the leader master, and executes the provided callback on
    /// completion.
    fn list_tables(&self, deadline: Instant, request: ListTablesRequestPB, callback: Box<Callback>) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let mut rpc = master::list_tables(addr, request);
        rpc.set_deadline(deadline);
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

/// Thread-safe container for master proxy state.
struct Inner {
    leader: Mutex<Leader>,
    masters: Mutex<Vec<SocketAddr>>,
    messenger: Messenger,
}

/// Asynchronously refresh the cached leader master.
///
/// This process has to handle failed leader replicas, as well as the possibility that the current
/// leader master is not in the set of known master replicas. The control flow is:
///
/// 1. For every known master replica, create and send a `GetMasterRegistration` RPC. Each of these
///    RPCs shares a single cancellation token, so each RPC can be notified of cancellation, and
///    each can trigger a cancellation of all others. Each RPC registers step 2. as the callback
///    handler.
///
/// 2. *`GetMasterRegistration` RPC callback handler* - Branch depending on the result:
///      a. RPC completes, with state LEADER. goto 3.
///      b. RPC completes, with other state. goto 4.
///      c. RPC completes, but with a Master Error. goto retry.
///      d. RPC timed out. This is likely because the replica is unreachable. goto retry.
///
/// 3. *`GetMasterRegistration` LEADER state response handler* - Check if the RPC has been cancelled,
///    and if so, do nothing. Cancellation can happen if another master has already reported being
///    the leader. Otherwise, replace the leader cache with the replica's address, send all queued
///    RPCs, and cancel all other RPCs. Finally, issue a `ListMasters` RPC to the leader, and
///    register step 6 as the callback handler.
///
/// 4. *`GetMasterRegistration` non LEADER state response handler* - Check if the RPC has been
///    cancelled, and if so do nothing. Create and send a `ListMasters` RPC to the leader with
///    the original shared cancellation token, and register step 7. as the callback handler.
///
/// 5. *`ListMasters` response handler after not discovering the leader* - Branch depending on the result:
///     a. RPC completes. For every newly discovered master instance, create and send it a
///        `GetMasterRegistration` RPC including the original shared cancellation token, and with
///        step 2. as the callback handler. goto retry (in order to retry the original non-leader
///        master, in case it has been elected in the meantime).
///     b. RPC completes, but with an AppStatus error. goto retry.
///     c. RPC timed out. Likely because the replica is no long reachable. goto retry.
///
/// 6. *`ListMasters` response handler after discovering the leader* - The goal of this
///    `ListMasters` is to make sure that we know about all of the master replicas that the leader
///    knows about. If the response fails (either because it timed out, or completed with an
///    `AppStatus` error), retry after a backoff period. Otherwise, add the newly discovered master
///    replicas to the known set.
///
/// This is a free function instead of a method on `Inner` because Rust does not have support for
/// using `Arc<Self>` as the `self` parameter.
fn refresh_leader(inner: &Arc<Inner>) {
    debug_assert!(!inner.leader.lock().is_known());
    // The deadline should be long enough that non-failed masters can respond, but short enough
    // that we don't waste a lot of time waiting for non-reachable masters before retrying.
    // TODO: should this use a backoff?
    let deadline = Instant::now() + Duration::from_secs(5);
    let masters = inner.masters.lock().clone();
    let inflight = Arc::new(AtomicUsize::new(masters.len()));
    for addr in masters {
        let mut rpc = master::get_master_registration(addr, GetMasterRegistrationRequestPB::new());
        rpc.set_deadline(deadline);
        let i = inner.clone();
        let inflight = inflight.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            handle_leader_refresh_response(i, inflight, result, rpc);
        }));
        inner.messenger.send(rpc);
    }
}

/// Handles the response to a leader refresh `GetMasterRegistration` RPC.
///
/// This is a free function instead of a method on `Inner` because Rust does not have support for
/// using `Arc<Self>` as the `self` parameter.
fn handle_leader_refresh_response(inner: Arc<Inner>,
                                  inflight: Arc<AtomicUsize>,
                                  result: RpcResult,
                                  rpc: Rpc) {
    let response = rpc.response::<GetMasterRegistrationResponsePB>();
    if let Err(error) = result {
        // Typically due to an RPC timeout because the master is not reachable.
        info!("failed GetMasterRegistration RPC to master {}:  {}",
                &rpc.addr, &error);
        handle_leader_refresh_failure(inner, inflight);
    } else if response.has_error() {
        // This can happen when the master is not yet initialized:
        // code: CATALOG_MANAGER_NOT_INITIALIZED
        // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
        info!("failed GetMasterRegistration RPC to master {}: {:?}",
              &rpc.addr, response.get_error());
        handle_leader_refresh_failure(inner, inflight);
    } else {
        if response.get_role() == Role::LEADER {
            debug!("discovered leader {}", &rpc.addr);
            // Swap out the leader cache with this one.
            let leader = mem::replace(&mut *inner.leader.lock(), Leader::Known(rpc.addr));
            match leader {
                Leader::Known(addr) => warn!("existing known leader {} swapped for {}",
                                             addr, &rpc.addr),
                Leader::Unknown(queued_rpcs) => {
                    for mut queued_rpc in queued_rpcs {
                        queued_rpc.addr = rpc.addr;
                        inner.messenger.send(queued_rpc);
                    }
                },
            }
            inflight.fetch_sub(1, Ordering::Relaxed);
        } else {
            handle_leader_refresh_failure(inner, inflight);
        }
    }
}

fn handle_leader_refresh_failure(inner: Arc<Inner>, inflight: Arc<AtomicUsize>) {
    if inflight.fetch_sub(1, Ordering::Relaxed) == 1 {
        warn!("unable to find leader master, retrying...");
        refresh_leader(&inner);
    }
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
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), Messenger::new().unwrap());


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
