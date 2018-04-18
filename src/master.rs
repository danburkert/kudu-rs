use std::mem;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use futures::future::Shared;
use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use krpc::{
    Call,
    HostPort,
    Proxy,
};
use prost::Message;

use pb::master::*;
use Error;
use MasterError;
use MasterErrorCode;
use Options;
use retry::{
    Retriable,
    RetryFuture,
    RetryProxy,
};

#[derive(Clone)]
pub(crate) struct MasterProxy {
    cache: Option<(Proxy, usize)>,
    inner: Arc<Inner>,
}

impl MasterProxy {

    pub fn new(addrs: Vec<HostPort>, options: Options) -> MasterProxy {
        MasterProxy {
            cache: None,
            inner: Arc::new(Inner::new(addrs, options)),
        }
    }

    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        unimplemented!()
    }

    pub fn send<Req, Resp>(&mut self, call: Call<Req, Resp>) -> MasterFuture<Req, Resp>
    where Req: Message + 'static,
          Resp: Retriable {
        let epoch = self.inner.epoch();

        if let Some((ref mut proxy, cache_epoch)) = self.cache {
            if epoch == cache_epoch {
                let response = proxy.clone().send_retriable(call.clone());
                return MasterFuture {
                    inner: Arc::clone(&self.inner),
                    call,
                    state: State::InFlight(response),
                    epoch,
                };
            }
        };
        self.cache = None;

        let (connection, epoch) = self.inner.connection();

        match connection.peek() {
            Some(Ok(proxy)) => {
                self.cache = Some((proxy.deref().clone(), epoch));
                return self.send(call);
            },
            Some(Err(_)) => {
                self.inner.reset(epoch);
            },
            None => (),
        }

        MasterFuture {
            inner: Arc::clone(&self.inner),
            call,
            state: State::Connecting(connection),
            epoch,
        }
    }

    pub fn options(&self) -> &Options {
        &self.inner.options
    }
}

struct Inner {
    addrs: Vec<HostPort>,
    options: Options,
    connection: RwLock<(Shared<ConnectToCluster>, usize)>,
}

impl Inner {

    fn new(addrs: Vec<HostPort>, options: Options) -> Inner {
        let connection = ConnectToCluster::new(&addrs, &options).shared();
        Inner {
            addrs,
            options,
            connection: RwLock::new((connection, 0)),
        }
    }

    /// Resets the current connection, if the current epoch matches `reset_epoch`.
    /// Returns the current connection and connection epoch.
    fn reset(&self, reset_epoch: usize) -> (Shared<ConnectToCluster>, usize) {
        let mut connection = self.connection.write().unwrap();
        if reset_epoch == connection.1 {
            connection.0 = ConnectToCluster::new(&self.addrs, &self.options).shared();
            connection.1 += 1;
        }
        connection.clone()
    }

    /// Returns the current connection epoch.
    fn epoch(&self) -> usize {
        self.connection.read().unwrap().1
    }

    /// Returns the current connection and connection epoch.
    fn connection(&self) -> (Shared<ConnectToCluster>, usize) {
        self.connection.read().unwrap().clone()
    }
}

enum State<Req, Resp>
where Req: Message + 'static, Resp: Retriable {
    Connecting(Shared<ConnectToCluster>),
    InFlight(RetryFuture<Req, Resp>),
}

#[must_use = "futures do nothing unless polled"]
pub(crate) struct MasterFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    inner: Arc<Inner>,
    call: Call<Req, Resp>,
    state: State<Req, Resp>,
    epoch: usize,
}

impl <Req, Resp> Future for MasterFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    type Item = Resp;
    type Error = Error;

    fn poll(&mut self) -> Poll<Resp, Error> {
        loop {
            // NLL hack.
            let state = match self.state {
                State::Connecting(ref mut connecting) => {
                    let proxy = try_ready!(connecting.poll().map_err(|error| error.deref().clone()));
                    let rpc = proxy.deref().clone().send_retriable(self.call.clone());
                    State::InFlight(rpc)
                },
                State::InFlight(ref mut rpc) => {
                    let (response, _) = try_ready!(rpc.poll());
                    match response.into_result() {
                        Ok(value) => return Ok(Async::Ready(value)),
                        Err(Error::Master(MasterError { code: MasterErrorCode::NotTheLeader, .. })) => {
                            let (connection, epoch) = self.inner.reset(self.epoch);
                            self.epoch = epoch;
                            State::Connecting(connection)
                        },
                        Err(error) => return Err(error),
                    }
                }
            };
            self.state = state;
        }
    }
}

#[must_use = "futures do nothing unless polled"]
struct ConnectToCluster {
    responses: FuturesUnordered<Box<Future<Item=Proxy, Error=Error>>>,
    errors: Vec<Error>,
}

impl ConnectToCluster {
    fn new(addrs: &[HostPort], options: &Options) -> ConnectToCluster {
        let mut connect = ConnectToCluster {
            responses: FuturesUnordered::new(),
            errors: Vec::new(),
        };

        let mut call = MasterService::connect_to_master(Default::default(),
                                                        Instant::now() + options.admin_timeout);
        call.set_required_feature_flags(&[MasterFeatures::ConnectToMaster as u32]);
        for addr in addrs.iter().cloned() {
            let mut proxy = Proxy::spawn(Box::new([addr]), options.rpc.clone());
            connect.responses.push(Box::new(proxy.clone().send_retriable(call.clone()).map(move |_| proxy)));
        }

        connect
    }
}

impl Future for ConnectToCluster {
    type Item = Proxy;
    type Error = Error;
    fn poll(&mut self) -> Poll<Proxy, Error> {
        loop {
            match self.responses.poll() {
                Ok(Async::Ready(Some(proxy))) => return Ok(Async::Ready(proxy)),
                Ok(Async::Ready(None)) => {
                    let errors = mem::replace(&mut self.errors, Vec::new());
                    return Err(Error::Compound("failed to connect to cluster".to_string(), errors));
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(error) => {
                    // TODO: scope this warning down if it's a Not Leader error.
                    warn!("failed to connect to master: {}", error);
                    self.errors.push(error);
                },
            }
        }
    }
}

/*
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
*/
