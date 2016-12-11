use std::marker::PhantomData;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use futures::{Async, AsyncSink, Future, Poll, Sink};
use parking_lot::Mutex;
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

use backoff::Backoff;
use io::Io;
use list_masters::find_leader_master_with_retry;
use protobuf::Message;
use rpc::{
    Rpc,
    RpcError,
    RpcFuture,
    master
};
use util;
use Error;
use MasterError;
use MasterErrorCode;
use MasterId;
use RaftRole;
use Result;
use Status;

enum Leader {
    /// The known leader.
    Known(SocketAddr, Vec<SocketAddr>),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    Unknown(Box<Future<Item=(SocketAddr, Vec<SocketAddr>), Error=!>>),
}

macro_rules! impl_master_rpc {
    ($fn_name:ident, $request_type:ident, $response_type:ident) => {
        pub fn $fn_name(&self, deadline: Instant, request: $request_type) -> MasterProxyFuture<$response_type>  {
            // The real leader address will be filled in after polling the current leader.
            let addr = util::dummy_addr();
            let mut rpc = master::$fn_name(addr, deadline, request);
            let future = rpc.future();
            MasterProxyFuture {
                proxy: self.clone(),
                rpc: Some(rpc),
                future: future,
                marker: PhantomData,
            }
        }
    };
}

/// The `MasterProxy` tracks the current leader master, and proxies RPCs to it. If any RPC fails
/// with `MasterErrorCode::NotTheLeader`, the cached leader is flushed (if it has not happened
/// already), and the RPC is retried after discovering the new leader.
#[derive(Clone)]
pub struct MasterProxy {
    leader: Arc<Mutex<Leader>>,
    io: Io,
}

impl MasterProxy {

    /// Creates a new `MasterProxy` with an initial seed of master addresses, and an `Io`
    /// instance to handle scheduling and sending RPCs.
    pub fn new(replicas: Vec<SocketAddr>, io: Io) -> MasterProxy {
        assert!(replicas.len() > 0);
        let find_leader = find_leader_master_with_retry(io.clone(),
                                                        Backoff::with_duration_range(1_000, 60_000),
                                                        replicas);
        MasterProxy {
            leader: Arc::new(Mutex::new(Leader::Unknown(find_leader))),
            io: io,
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

    fn poll_replicas(&self) -> Poll<Vec<SocketAddr>, !> {
        let mut lock = self.leader.lock();
        let (leader, replicas) = match *lock {
            Leader::Known(_, ref replicas) => return Ok(Async::Ready(replicas.clone())),
            Leader::Unknown(ref mut f) => try_ready!(f.poll()),
        };
        *lock = Leader::Known(leader, replicas.clone());
        Ok(Async::Ready(replicas))
    }

    /// Returns the leader master, if it is known.
    fn poll_leader(&self) -> Poll<SocketAddr, !> {
        let mut lock = self.leader.lock();
        let (leader, replicas) = match *lock {
            Leader::Known(leader, ..) => return Ok(Async::Ready(leader)),
            Leader::Unknown(ref mut f) => try_ready!(f.poll()),
        };
        *lock = Leader::Known(leader, replicas);
        Ok(Async::Ready(leader))
    }

    /// Clears the leader cache if the currently cached leader matches the provided address.
    /// If the cache is cleared, a refresh is initiated.
    fn reset_leader(&self, stale_leader: SocketAddr) {
        let mut lock = self.leader.lock();
        let future = match *lock {
            Leader::Known(leader, ref mut replicas) if leader == stale_leader => {
                let replicas = mem::replace(replicas, Vec::new());
                find_leader_master_with_retry(self.io.clone(),
                                              Backoff::with_duration_range(1_000, 60_000),
                                              replicas)
            },
            _ => return,
        };
        *lock = Leader::Unknown(future);
    }
}

pub trait MasterResponse: Message {
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

pub struct MasterProxyFuture<Resp> where Resp: MasterResponse {
    proxy: MasterProxy,
    rpc: Option<Rpc>,
    future: RpcFuture,
    marker: PhantomData<Resp>,
}

impl <Resp> Future for MasterProxyFuture<Resp> where Resp: MasterResponse {
    type Item = Resp;
    type Error = Error;
    fn poll(&mut self) -> Poll<Resp, Error> {
        loop {
            if self.rpc.is_some() {
                let leader = match self.proxy.poll_leader().unwrap() {
                    Async::NotReady => return Ok(Async::NotReady),
                    Async::Ready(leader) => leader,
                };
                let mut rpc = self.rpc.take().unwrap();
                rpc.addr = leader;

                if let AsyncSink::NotReady(rpc) = self.proxy.io.messenger().start_send(rpc).unwrap() {
                    self.rpc = Some(rpc);
                    return Ok(Async::NotReady);
                }
            }

            match self.future.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(mut rpc)) => match rpc.mut_response::<Resp>().error() {
                    Some(ref error) if error.code() == MasterErrorCode::NotTheLeader ||
                                       error.code() == MasterErrorCode::CatalogManagerNotInitialized => {
                        self.proxy.reset_leader(rpc.addr);
                        self.future = rpc.future();
                        self.rpc = Some(rpc);
                        // Loop.
                    },
                    Some(error) => return Err(Error::from(MasterError::from(error))),
                    None => return Ok(Async::Ready(rpc.take_response::<Resp>())),
                },
                Err(RpcError { mut rpc, error }) => if error.is_network_error() {
                    // On connection error, reset the leader cache and resend.
                    self.proxy.reset_leader(rpc.addr);
                    self.future = rpc.future();
                    self.rpc = Some(rpc);
                } else {
                    return Err(error);
                },
            }
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
    use std::sync::mpsc::sync_channel;
    use std::thread;
    use std::time::{Duration, Instant};

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

    fn replicas(proxy: &MasterProxy) -> Vec<SocketAddr> {
        // TODO: use poll_fn
        let mut replicas = proxy.poll_replicas().unwrap();
        while let Async::NotReady = replicas {
            thread::sleep_ms(100);
            replicas = proxy.poll_replicas().unwrap();
        }

        match replicas {
            Async::Ready(replicas) => replicas,
            Async::NotReady => panic!(),
        }
    }

    fn leader(proxy: &MasterProxy) -> SocketAddr {
        // TODO: use poll_fn
        let mut leader = proxy.poll_leader().unwrap();
        while let Async::NotReady = leader {
            thread::sleep_ms(100);
            leader = proxy.poll_leader().unwrap();
        }

        match leader {
            Async::Ready(leader) => leader,
            Async::NotReady => panic!(),
        }
    }

    /// Tests that the leader is discovered in a single-master cluster.
    #[test]
    fn leader_discovery_single_master() {
        let _ = env_logger::init();
        let test_reactor = util::TestReactor::default();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), test_reactor.io().clone());
        let result = proxy.list_tables(Instant::now() + Duration::from_secs(5),
                                       ListTablesRequestPB::new())
                          .wait();

        check_list_tables_response(result);
    }

    /// Tests that the leader is discovered in a multi-master cluster.
    #[test]
    fn leader_discovery_multi_master() {
        let _ = env_logger::init();
        let test_reactor = util::TestReactor::default();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(3).num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), test_reactor.io().clone());
        let result = proxy.list_tables(Instant::now() + Duration::from_secs(5),
                                       ListTablesRequestPB::new())
                          .wait();

        check_list_tables_response(result);
    }

    /// Tests that masters which are not part of the original seed list are discovered in a
    /// multi-master cluster.
    #[test]
    fn master_discovery() {
        let _ = env_logger::init();
        let test_reactor = util::TestReactor::default();
        let cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(2).num_tservers(0));

        let proxy = MasterProxy::new(cluster.master_addrs().to_owned(), test_reactor.io().clone());
        let result = proxy.list_tables(Instant::now() + Duration::from_secs(5),
                                       ListTablesRequestPB::new())
                          .wait();
        check_list_tables_response(result);

        let mut discovered_masters = HashSet::new();
        let mut cluster_masters = HashSet::new();
        discovered_masters.extend(replicas(&proxy));
        cluster_masters.extend(cluster.master_addrs().iter().cloned());

        assert_eq!(cluster_masters, discovered_masters);
    }

    /// Tests that the `MasterProxy` will remove masters not known by the leader.
    fn master_cleanup() {
    }

    /*
    /// Tests that RPCs are timed out when the leader is unavailable.
    #[test]
    fn timeout() {
        let _ = env_logger::init();
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
    */

    /// Tests that the `MasterProxy` will discover and reroute RPCs to a new leader when the
    /// current leader becomes unreachable.
    #[test]
    fn leader_failover() {
        let _ = env_logger::init();
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
}
