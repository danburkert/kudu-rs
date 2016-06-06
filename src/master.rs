use std::collections::HashSet;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

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

/// The master proxy tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
///
/// This type is a thin public facade over the `Inner` type, which holds all state. The state must
/// be shareable among RPC callbacks, so it's wrapped in an `Arc`.
#[derive(Clone)]
pub struct MasterProxy {
    inner: Arc<Inner>,
}

impl MasterProxy {

    /// Creates a new `MasterProxy` with an initial seed of master addresses, and a `Messenger`
    /// instance to handle sending RPCs.
    pub fn new(masters: &[SocketAddr], messenger: Messenger) -> MasterProxy {
        assert!(masters.len() > 0);

        // Give it a bit more capacity since we modify it under a mutex and reallocations are
        // especially painful.
        let mut addrs = HashSet::with_capacity(masters.len() * 2);
        addrs.extend(masters.iter().cloned());

        let proxy = MasterProxy {
            inner: Arc::new(Inner {
                leader: Mutex::new(Leader::Unknown(Vec::with_capacity(QUEUE_LEN))),
                masters: Mutex::new(addrs),
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

    fn refresh_leader(&self) {
        debug_assert!(!self.inner.leader.lock().is_known());
        let cancel = Arc::new(AtomicBool::new(false));
        let masters = self.inner.masters.lock().iter().cloned().collect::<Vec<_>>();
        for addr in masters {
            self.send_list_masters(addr, cancel.clone());
        }
    }

    fn send_list_masters(&self, addr: SocketAddr, cancel: Arc<AtomicBool>) {
        let mut rpc = master::list_masters(addr, ListMastersRequestPB::new());
        rpc.cancel = Some(cancel);
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            proxy.handle_list_masters_response(result, rpc);
        }));
        self.inner.messenger.send(rpc);
    }

    fn retry_list_masters(self, rpc: Rpc) {
        // TODO: backoff wait
        self.inner.messenger.send(rpc);
    }

    /// The result should never fail, since it keeps looping forever.
    fn handle_list_masters_response(self, result: RpcResult, rpc: Rpc) {
        // Short circuit if the RPC has already been cancelled (indicating that the master has
        // already been found).
        if rpc.cancelled() { return; }

        // The only way the RPC can fail is if it is cancelled, which the check above has already
        // covered. Since we don't set a timeout, it should continue spinning in the connection
        // forever if it is not cancelled.
        //
        // TODO: switch this back to always having a timeout for RPCs, and making the timeout for
        // these requests something like 60 seconds.
        if let Err(error) = result {
            panic!("ListMasters RPC to master {} unexpectedly failed: {}", &rpc.addr, error);
        }

        {
            let response = rpc.response::<ListMastersResponsePB>();
            if response.has_error() {
                info!("ListMasters RPC to master {} failed: {:?}", &rpc.addr,
                      response.get_error());
                // Fall through to rety.
            } else {

                let hostports = Vec::new();
                for server_entry in response.get_masters() {
                    if server_entry.has_error()  { continue; }

                }
            }
        }

        self.retry_list_masters(rpc);

        // TODO: does this result in DNS lookups? we should probably be doing this on a threadpool.

        //let mut master_addrs = Vec::with_capacity(response.get_masters().len());
        /*
        for master in response.get_masters() {
            if master.has_registration() {
                for addr in master.get_registration().get_rpc_addresses() {
                    //master_addrs.push(SocketAddr::from_str(addr.get_
                }
            }
        }

        {
            let masters = inner.masters.lock();
        }

        */


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
    masters: Mutex<HashSet<SocketAddr>>,
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
/// 2. *`GetMasterRegistration` RPC callback handler* - Check if the RPC has been cancelled, and if
///    so do nothing. Cancellation can happen if another master has already reported being the
///    leader. Otherwise, branch on the result:
///      a. RPC completes, with state LEADER. goto 3.
///      b. RPC completes, with other state. goto 4.
///      c. RPC completes, but with a Master Error. goto retry.
///      d. RPC timed out. This is likely because the replica is unreachable. goto retry.
///
/// 3. *`GetMasterRegistration` LEADER state response handler* - Replace the leader cache with the
///    replica's address, send all queued RPCs, and cancel all other RPCs. Finally, issue a
///    `ListMasters` RPC to the leader, and register step 6 as the callback handler.
///
/// 4. *`GetMasterRegistration` non LEADER state response handler* - Create and send a
///    `ListMasters` RPC to the replica. Include the original shared cancellation token, and
///    register step 7. as the callback handler.
///
/// 5. *`ListMasters` response handler after not discovering the leader* - Check if the RPC has
///    been cancelled, and if so do nothing. Otherwise, branch on the result:
///      a. RPC completes. For every newly discovered master instance, create and send it a
///         `GetMasterRegistration` RPC including the original shared cancellation token, and with
///         step 2. as the callback handler. goto retry (in order to retry the original replica, in
///         case it has been elected in the meantime).
///      b. RPC completes, but with an AppStatus error. goto retry.
///      c. RPC timed out. Likely because the replica is no long reachable. goto retry.
///
/// 6. *`ListMasters` response handler after discovering the leader* - The goal of this
///    `ListMasters` is to make sure that we know about all of the master replicas that the leader
///    knows about. If the response fails (either because it timed out, or completed with an
///    `AppStatus` error), retry after a backoff period. Otherwise, add the newly discovered master
///    replicas to the known set.
///
/// retry. Wait for a backoff period, and then send a `GetMasterRegistration` RPC along with the
///        original cancellation token. Register step 2. as the callback handler.
///
/// This function should only be called once per instance of the cached leader master becoming
/// stale.
///
/// This is a free function instead of a method on `Inner`, because Rust does not have support for
/// using `Arc<Self>` as the `self` parameter.
fn refresh_leader(inner: &Arc<Inner>) {
    // Step 1.
    debug_assert!(!inner.leader.lock().is_known());
    let cancel = Arc::new(AtomicBool::new(false));
    let masters = inner.masters.lock().clone();
    for addr in masters {
        let mut rpc = master::get_master_registration(addr, GetMasterRegistrationRequestPB::new());
        rpc.cancel = Some(cancel.clone());
        let i = inner.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            handle_get_master_registration_response(i, result, rpc);
        }));
        inner.messenger.send(rpc);
    }
}

/// Step 2. Handle the response to a leader refresh `GetMasterRegistration` RPC.
fn handle_get_master_registration_response(inner: Arc<Inner>, result: RpcResult, mut rpc: Rpc) {
    // Short circuit if the RPC has already been cancelled (indicating that another replica was
    // already found to be the leader).
    let cancel = rpc.cancel.take().unwrap();
    if cancel.load(Ordering::Relaxed) { return; }

    if let Err(error) = result {
        // Typically due to an RPC timeout because the master is not reachable.
        info!("failed GetMasterRegistration RPC to master {}: {}", &rpc.addr, &error);
        return retry_get_master_registration(inner, rpc.addr, cancel);
    }

    // TODO(perf): Once Rust has non-lexical lifetimes, reduce the number of rpc::response calls.

    if rpc.response::<GetMasterRegistrationResponsePB>().has_error() {
        // This can happen when the master is not yet initialized, e.g.:
        // code: CATALOG_MANAGER_NOT_INITIALIZED
        // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
        info!("failed GetMasterRegistration RPC to master {}: {:?}",
              &rpc.addr, rpc.response::<GetMasterRegistrationResponsePB>().get_error());
        return retry_get_master_registration(inner, rpc.addr, cancel);
    }

    if rpc.response::<GetMasterRegistrationResponsePB>().get_role() == Role::LEADER {
        // Step 3.
        let leader = {
            // Swap out the Unknown leader queue with the new Known leader. We also attempt to
            // cancel all other RPCs while holding the lock, so that the cancel happens atomically
            // with changing the leader state. This prevents a race between the cancellation check
            // at the start of this function and now.
            let mut leader = inner.leader.lock();
            if !cancel.compare_and_swap(false, true, Ordering::Relaxed) {
                // Another replica has already claimed to be the leader.
                return;
            }

            mem::replace(&mut *leader, Leader::Known(rpc.addr))
        };
        debug!("new master leader discovered: {}", &rpc.addr);
        match leader {
            Leader::Unknown(queued_rpcs) => {
                for mut queued_rpc in queued_rpcs {
                    queued_rpc.addr = rpc.addr;
                    inner.messenger.send(queued_rpc);
                }
            },
            Leader::Known(addr) => unreachable!("existing known leader {} swapped for {}",
                                                addr, &rpc.addr),
        }
    } else {
        // Step 4.
        let mut list_masters_rpc = master::list_masters(rpc.addr, ListMastersRequestPB::new());
        list_masters_rpc.cancel = Some(cancel);
        let i = inner.clone();
        list_masters_rpc.callback = Some(Box::new(move |result, rpc| {
            handle_list_masters_response_from_follower(i, result, rpc);
        }));
        inner.send(list_masters_rpc);
    }
}

/// Step 5.
fn handle_list_masters_response_from_follower(inner: Arc<Inner>, result: RpcResult, mut rpc: Rpc) {
    // Short circuit if the RPC has already been cancelled (indicating that another replica was
    // already found to be the leader).
    let cancel = rpc.cancel.take().unwrap();
    if cancel.load(Ordering::Relaxed) { return; }

    if let Err(error) = result {
        // Typically due to an RPC timeout because the master is not reachable.
        info!("failed ListMasters RPC to master {}: {}", &rpc.addr, &error);
        return retry_get_master_registration(inner, rpc.addr, cancel);
    }

    let response = rpc.response::<ListMastersResponsePB>();
    if response.has_error() {
        info!("failed ListMasters RPC to master {}: {:?}", &rpc.addr, response.get_error());
        return retry_get_master_registration(inner, rpc.addr, cancel);
    }

    // TODO: does this result in DNS lookups? we should probably be doing this on a threadpool.

    //let mut master_addrs = Vec::with_capacity(response.get_masters().len());
    for master in response.get_masters() {
        if master.has_registration() {
            for addr in master.get_registration().get_rpc_addresses() {
                //master_addrs.push(SocketAddr::from_str(addr.get_
            }
        }
    }

    {
        let masters = inner.masters.lock();
    }


}

/// Step 6.
fn handle_list_masters_response_from_leader(inner: Arc<Inner>, result: RpcResult, rpc: Rpc) {
}

/// Retry step.
fn retry_get_master_registration(inner: Arc<Inner>,
                                 master: SocketAddr,
                                 cancel: Arc<AtomicBool>) {
    unimplemented!()
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
