use std::mem;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};

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

/// Maximum number of RPCs to queue in the master proxy.
///
/// When the queue is full, additional attempts to send RPCs will immediately fail.
const QUEUE_LEN: usize = 256;

enum Leader {
    Known(SocketAddr),
    Unknown(Vec<Rpc>),
}

/// Thread-safe container for master metadata.
struct Inner {
    leader: Mutex<Leader>,
    masters: Mutex<Vec<SocketAddr>>,
    messenger: Messenger,
}

impl Inner {
    fn handle_leader_refresh_response(&self, result: RpcResult, rpc: Rpc) {
        let response = rpc.response::<GetMasterRegistrationResponsePB>();
        if let Err(error) = result {
            warn!("failed GetMasterRegistration RPC to master {}:  {}",
                  &rpc.addr, error);
        } else if response.has_error() {
            // This happens when the master is not yet initialized:
            // code: CATALOG_MANAGER_NOT_INITIALIZED
            // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
            warn!("failed GetMasterRegistration RPC to master {}: {:?}",
                  &rpc.addr, response.get_error());
        } else {
            if response.get_role() == Role::LEADER {
                debug!("discovered leader {}", &rpc.addr);
                // Swap out the leader cache with this one.
                let leader = mem::replace(&mut *self.leader.lock(),
                                          Leader::Known(rpc.addr));
                match leader {
                    Leader::Known(addr) => warn!("existing known leader {} swapped for {}",
                                                 addr, &rpc.addr),
                    Leader::Unknown(queued_rpcs) => {
                        for mut queued_rpc in queued_rpcs {
                            queued_rpc.addr = rpc.addr;
                            self.messenger.send(queued_rpc);
                        }
                    },
                }
            }
        }
    }

    /// Sends the RPC if the leader master is known, otherwise queues the RPC to be sent when the
    /// leader is discovered.
    fn send(&self, mut rpc: Rpc) {
        let addr = match *self.leader.lock() {
            Leader::Known(addr) => addr,
            Leader::Unknown(ref mut queue) => {
                debug!("queueing rpc: {:?}", rpc);
                queue.push(rpc);
                return;
            },
        };
        rpc.addr = addr;
        self.messenger.send(rpc);
    }
}

/// The master proxy tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
pub struct MasterProxy {
    inner: Arc<Inner>,
}

impl MasterProxy {

    pub fn new(masters: Vec<SocketAddr>, messenger: Messenger) -> MasterProxy {
        assert!(masters.len() > 0);
        let proxy = MasterProxy {
            inner: Arc::new(Inner {
                leader: Mutex::new(Leader::Unknown(Vec::with_capacity(QUEUE_LEN))),
                masters: Mutex::new(masters),
                messenger: messenger,
            }),
        };
        proxy.refresh_leader();
        proxy
    }

    /// Asynchronously refresh the cached leader master.
    fn refresh_leader(&self) {
        // The deadline should be long enough that any non-failed master can respond, but not so
        // long that we needlessly continue retrying RPCs to non-reachable masters.
        let deadline = Instant::now() + Duration::from_secs(60);
        let masters = self.inner.masters.lock().clone();
        for addr in masters {
            let mut rpc = master::get_master_registration(addr,
                                                          deadline,
                                                          GetMasterRegistrationRequestPB::new());
            let inner = self.inner.clone();
            rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
                inner.handle_leader_refresh_response(result, rpc);
            }));
            self.inner.messenger.send(rpc);
        }
    }

    fn list_tables(&self, deadline: Instant, request: ListTablesRequestPB, callback: Box<Callback>) {
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
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use mini_cluster::{MiniCluster, MiniClusterConfig};
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
}
