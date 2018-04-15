use std::collections::{
    VecDeque,
    HashMap,
};
//use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
//use std::sync::Arc;
use std::time::Instant;

use bytes::Bytes;
use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use krpc::{
    self,
    Call,
    Proxy,
    RpcFuture,
};
use prost::Message;
use timer::{
    Sleep,
    Timer,
};

use RaftRole;
use Status;
use StatusCode;
use Error;
use HostPort;
use MasterError;
use MasterErrorCode;
use Options;
use backoff::Backoff;
use meta_cache::MasterReplica;
use pb::AppStatusPb;
use pb::master::{
    ConnectToMasterResponsePb,
    MasterFeatures,
    MasterService,
};
use util::ContextFuture;
//use retry::Retriable;

// TODO: rename something more generic like RpcHandler or RpcController.
pub(crate) trait ServerPicker {

    type Response: Message + Default;

    type ServerId;

    type Item;

    fn poll_next(&mut self) -> Poll<(Self::ServerId, Proxy), Error>;

    fn handle_result(&mut self,
                     server: Self::ServerId,
                     result: Result<(Self::Response, Vec<Bytes>), krpc::Error>)
                     -> Result<Option<Self::Item>, Error>;
}

pub(crate) fn connect_to_cluster(master_addrs: Vec<HostPort>,
                                 options: &Options)
                                 -> impl Future<Item=Vec<MasterReplica>, Error=Error> {
    let mut call = MasterService::connect_to_master(Default::default(),
                                                    Instant::now() + options.admin_timeout);
    call.set_required_feature_flags(&[MasterFeatures::ConnectToMaster as u32]);
    let sp = ConnectToClusterPicker::new(master_addrs, &options);
    RetryFuture::new(call, sp)
}

/// A `ServerPicker` which handles connecting to a set of masters.
pub(crate) struct ConnectToClusterPicker {
    timer: Timer,
    masters: HashMap<HostPort, (Proxy, Backoff, Option<Error>)>,
    queue: VecDeque<HostPort>,
    // TODO: rename to 'retries'?
    waiting: FuturesUnordered<ContextFuture<Sleep, HostPort>>,
    deadline: Sleep,
}

impl ConnectToClusterPicker {
    fn new(proxies: Vec<HostPort>, options: &Options) -> ConnectToClusterPicker {

        let queue = proxies.iter().cloned().collect::<VecDeque<_>>();
        let masters = proxies.into_iter().map(|hostport| {
            let proxy = Proxy::spawn(
                vec![hostport.clone()].into_boxed_slice(),
                options.rpc.clone(),
                options.threadpool.clone(),
                &options.remote);
            (hostport, (proxy, Backoff::with_duration_range(250, u32::max_value()), None))
        }).collect::<HashMap<_, _>>();

        ConnectToClusterPicker {
            timer: options.timer.clone(),
            masters,
            queue,
            waiting: FuturesUnordered::new(),
            deadline: options.timer.sleep(options.admin_timeout),
        }
    }

    fn process_errors(mut errors: Vec<Error>) -> Error {
        assert!(errors.len() > 0);
        if errors.len() == 1 {
            return errors.pop().unwrap();
        }

        return Error::Compound("failed to connect to cluster".to_string(), errors);
    }
}

impl ServerPicker for ConnectToClusterPicker {
    type Response = ConnectToMasterResponsePb;
    type ServerId = HostPort;
    type Item = Vec<MasterReplica>;

    fn poll_next(&mut self) -> Poll<(HostPort, Proxy), Error> {
        if let Async::Ready(()) = self.deadline.poll().expect("timer failed") {
            // TODO: this error should include the failed errors contexts.
            return Err(Error::TimedOut)
        }

        while let Ok(Async::Ready(Some((_, server)))) = self.waiting.poll() {
            self.queue.push_back(server);
        }

        if let Some(hostport) = self.queue.pop_front() {
            let proxy = self.masters[&hostport].0.clone();
            return Ok(Async::Ready((hostport, proxy)));
        }

        // If all masters have encountered a non-retriable error, fail fast.
        if self.queue.is_empty() &&
           self.waiting.is_empty() &&
           !self.masters.values().any(|&(_, _, ref error)| error.as_ref().map_or(true, Error::is_retriable)) {
            let errors: Vec<Error> = self.masters.values_mut().map(|&mut (_, _, ref mut error)| {
                error.take().unwrap()
            }).collect();
            return Err(ConnectToClusterPicker::process_errors(errors));
        }

        Ok(Async::NotReady)
    }

    fn handle_result(&mut self,
                     server: HostPort,
                     result: Result<(ConnectToMasterResponsePb, Vec<Bytes>), krpc::Error>)
                     -> Result<Option<Vec<MasterReplica>>, Error> {
        let result: Result<ConnectToMasterResponsePb, Error> = match result {

            // Check if the response contains an application-level error.
            Ok((ConnectToMasterResponsePb { error: Some(error), .. }, _)) => {
                let error = MasterError::from(error);
                debug!("Failed to connect to master {}: {}", server, error);
                Err(error.into())
            },

            // Check that the response contains the expected number of master replicas.
            Ok((ref response, _)) if response.master_addrs.len() != self.masters.len() => {
                // TODO: improve this error message based on C++ variant.
                unimplemented!("wrong masters configured");
            },

            // Check that the response contains the expected set of master replicas.
            // TODO: this appears to be hitting an overly cautious borrow check error.
            /*
            Ok((ref response, _)) if response.master_addrs().iter().any(|replica| {
                // !self.masters.contains_key(&HostPort::from(replica.clone()))
                true
            }) => {
                // TODO: improve this error message based on C++ variant.
                unimplemented!("wrong masters configured");
            },
            */

            Ok((ref response, _)) if response.role() != RaftRole::Leader => {
                // Create a synthetic error indicating that we could successfully connect to the
                // master, but it is currently not the leader.
                let mut status = AppStatusPb::default();
                status.set_code(StatusCode::NotFound);

                Err(MasterError {
                    code: MasterErrorCode::NotTheLeader,
                    status: Status::from(status),
                }.into())
            },
            Ok((response, _)) => Ok(response),
            Err(error) => Err(error.into()),
        };

        match result {
            Ok(_) => {
                let masters = self.masters
                    .drain()
                    .map(|(hostport, (proxy, _, _))| {
                        let is_leader = AtomicBool::new(hostport == server);
                        MasterReplica { hostport, proxy, is_leader }
                    })
                    .collect::<Vec<_>>();
                debug!("Connected to Leader Master {}", server);
                Ok(Some(masters))
            }
            Err(error) => {
                let (_, ref mut backoff, ref mut error_slot) = *self.masters.get_mut(&server).unwrap();

                if error.is_retriable() {
                    let sleep = self.timer.sleep(backoff.next_backoff());
                    debug!("Failed to connect to master {} (retriable in {:?}): {}", server, sleep.remaining(), error);
                    self.waiting.push(ContextFuture::new(sleep, server));
                } else {
                    debug!("Failed to connect to master {} (non-retriable): {}", server, error);
                }

                *error_slot = Some(error);
                Ok(None)
            },
        }
    }
}

/*
struct LeaderMasterPicker<Resp> where Resp: Retriable {
    master_replicas: Arc<Box<[MasterReplica]>>,
    failures: Box<[Option<(Instant, Error)>]>,
    _marker: PhantomData<Resp>,
}

impl <Resp> LeaderMasterPicker<Resp> where Resp: Retriable {

    fn new(master_replicas: Arc<Box<[MasterReplica]>>) -> LeaderMasterPicker<Resp> {

        let mut failures = Vec::with_capacity(master_replicas.len());
        for _ in 0..master_replicas.len() {
            failures.push(None);
        }
        let failures = failures.into_boxed_slice();

        LeaderMasterPicker {
            master_replicas,
            failures,
            _marker: Default::default(),
        }
    }
}

impl <Resp> ServerPicker for LeaderMasterPicker<Resp> where Resp: Retriable {
    type Response = Resp;
    type ServerId = usize;
    type Item = Resp;

    fn poll_next(&mut self) -> Poll<(usize, Proxy), Error> {
        Ok(Async::Ready((next_idx, self.master_replicas[next_idx].proxy.clone())))
    }

    fn handle_result(&mut self, server: usize, result: Result<(Resp, Vec<Bytes>), krpc::Error>) -> Result<Option<Resp>, Error> {
        match result.map_err(Error::from).and_then(|(resp, _)| resp.into_result()) {
            Ok(resp) => {
                self.master_replicas[server].mark_leader();
                Ok(Some(resp))
            },
            Err(error@Error::Master(MasterError { code: MasterErrorCode::NotTheLeader, .. }))
                | Err(error@Error::Master(MasterError { code: MasterErrorCode::CatalogManagerNotInitialized, .. })) => {
                self.master_replicas[server].mark_follower();
                self.failures[server] = Some((Instant::now(), error));
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}
*/

/// A future which wraps an in-flight Kudu RPC, and retries it after a backoff period if it fails
/// with a retriable error.
#[must_use = "futures do nothing unless polled"]
pub(crate) struct RetryFuture<Sp, Req>
where Sp: ServerPicker,
      Req: Message + 'static {
    call: Call<Req, Sp::Response>,
    server_picker: Sp,
    in_flight: FuturesUnordered<ContextFuture<RpcFuture<Sp::Response>, Sp::ServerId>>,
}

impl <Sp, Req> RetryFuture<Sp, Req>
where Sp: ServerPicker,
      Req: Message + 'static {

    fn new(call: Call<Req, Sp::Response>, server_picker: Sp) -> RetryFuture<Sp, Req> {
        RetryFuture {
            call,
            server_picker,
            in_flight: FuturesUnordered::new(),
        }
    }

    fn poll_in_flight(&mut self) -> Poll<Sp::Item, Error> {
        loop {
            let (server_id, result) = match self.in_flight.poll() {
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(Some((result, server_id)))) => (server_id, Ok(result)),
                Err((error, server_id)) => (server_id, Err(error)),
            };
            if let Some(item) = self.server_picker.handle_result(server_id, result)? {
                return Ok(Async::Ready(item));
            }
        }
    }
}

impl <Sp, Req> Future for RetryFuture<Sp, Req>
where Sp: ServerPicker,
      Req: Message + 'static {

    type Item = Sp::Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Sp::Item, Error> {
        match self.poll_in_flight() {
            Ok(Async::NotReady) => (),
            other => return other,
        }

        while let Async::Ready((server_id, mut proxy)) = self.server_picker.poll_next()? {
            self.in_flight.push(ContextFuture::new(proxy.send(self.call.clone()), server_id));
        }

        self.poll_in_flight()
    }
}

#[cfg(test)]
mod tests {

    use std::net::TcpListener;
    use std::time::Duration;

    use cpupool::CpuPool;
    use env_logger;
    use krpc;
    use tokio::reactor::Core;

    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use replica::Replica;
    use super::*;

    // TODO(tests):
    //  - Shutdown single master and ensure ConnectToCluster immediately fails.
    //  - Shutdown one master replica and ensure ConnectToCluster succeeds.
    //  - Shutdown all master replicas and ensure ConnectToCluster immediately fails.
    //  - Manipulate master election to make sure there's no leader, ensure ConnectToCluster waits,
    //    then elect a leader and ensure it completes.
    //  - Connect to cluster stress test: have a thread spawning a bunch of concurrent connect to
    //    cluster calls, and have the cluster undergo constant elections and failures.

    fn setup(cluster_config: &MiniClusterConfig) -> (MiniCluster, Core, Options) {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(cluster_config);
        let reactor = Core::new().unwrap();

        let options = Options {
            rpc: krpc::Options::default(),
            remote: reactor.remote(),
            threadpool: CpuPool::new_num_cpus(),
            timer: Timer::default(),
            admin_timeout: Duration::from_secs(10),
        };

        (cluster, reactor, options)
    }

    #[test]
    fn test_connect_to_cluster() {
        let _ = env_logger::init();

        let run = |num_masters: u32| {
            let (mut cluster, mut reactor, options) =
                setup(MiniClusterConfig::default().num_masters(num_masters).num_tservers(0));
            let masters = reactor.run(connect_to_cluster(cluster.master_addrs(), &options)).unwrap();

            assert_eq!(masters.len(), cluster.master_addrs().len());
            assert_eq!(1, masters.iter().filter(|master| master.is_leader()).count());
        };

        run(1);
        run(3);
    }

    #[test]
    fn test_connect_to_cluster_unavailable() {
        let _ = env_logger::init();

        let run = |num_masters: u32| -> Error {
            let mut reactor = Core::new().unwrap();

            let options = Options {
                rpc: krpc::Options::default(),
                remote: reactor.remote(),
                threadpool: CpuPool::new_num_cpus(),
                timer: Timer::default(),
                admin_timeout: Duration::from_secs(10),
            };

            let hostports = {
                let listeners = (0..num_masters).map(|_| TcpListener::bind("127.0.0.1:0").unwrap())
                                                .collect::<Vec<_>>();
                listeners.iter().flat_map(TcpListener::local_addr).map(HostPort::from).collect::<Vec<HostPort>>()
            };

            reactor.run(connect_to_cluster(hostports, &options)).unwrap_err()
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
        let _ = env_logger::init();

        let (mut cluster, mut reactor, options) =
            setup(MiniClusterConfig::default().num_masters(3).num_tservers(0));

        cluster.stop_master(0);

        let masters = reactor.run(connect_to_cluster(cluster.master_addrs(), &options)).unwrap();

        assert_eq!(masters.len(), cluster.master_addrs().len());
        assert_eq!(1, masters.iter().filter(|master| master.is_leader()).count());
    }

    // TODO:
    /*
    // Test cluster connection when the cluster is starting up.
    #[test]
    fn test_connect_to_cluster_startup() {
        let _ = env_logger::init();

        let (mut cluster, mut reactor, options) =
            setup(MiniClusterConfig::default().num_masters(3).num_tservers(0));
    }
    */
}
