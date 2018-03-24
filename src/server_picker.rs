use std::collections::{
    VecDeque,
    HashMap,
};
use std::mem;
use std::time::{
    Duration,
    Instant,
};

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
use TabletServerError;
use backoff::Backoff;
use meta_cache::{
    MasterLocations,
    MasterReplica,
};
use pb::{AppStatusPb, master, tserver};
use pb::master::ConnectToMasterResponsePb;

// TODO: rename something more generic like RpcHandler or RpcController.
pub(crate) trait ServerPicker {

    type Response: Message + Default;

    type ServerId;

    type Item;

    fn poll_next(&mut self) -> Poll<(Self::ServerId, Proxy), Error>;

    /// 
    fn poll_result(&mut self,
                   server: Self::ServerId,
                   result: Result<(Self::Response, Vec<Bytes>), krpc::Error>)
                   -> Poll<Self::Item, Error>;
}

/// A `ServerPicker` which handles connecting to a set of masters.
pub(crate) struct ConnectToClusterPicker {
    masters: HashMap<HostPort, (Proxy, Backoff, Option<Error>)>,
    queue: VecDeque<HostPort>,
    waiting: FuturesUnordered<ContextFuture<Sleep, HostPort>>,
    deadline: Sleep,
    num_fatal_errors: usize,
}

impl ConnectToClusterPicker {
    fn new(proxies: Vec<HostPort>,
           options: &Options) -> ConnectToClusterPicker {

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
            masters,
            queue,
            waiting: FuturesUnordered::new(),
            deadline: options.timer.sleep(options.admin_timeout),
            num_fatal_errors: 0,
        }
    }
}

impl ServerPicker for ConnectToClusterPicker {
    type Response = ConnectToMasterResponsePb;
    type ServerId = HostPort;
    type Item = Vec<MasterReplica>;

    fn poll_next(&mut self) -> Poll<(HostPort, Proxy), Error> {
        if let Async::Ready(()) = self.deadline.poll().expect("timer failed") {
            // TODO: this error should include the failed errors contexts.
            Err(Error::TimedOut)
        } else if let Some(hostport) = self.queue.pop_front() {
            let proxy = self.masters[&hostport].0.clone();
            Ok(Async::Ready((hostport, proxy)))
        } else {
            Ok(Async::NotReady)
        }
    }

    fn poll_result(&mut self,
                   server: HostPort,
                   result: Result<(ConnectToMasterResponsePb, Vec<Bytes>), krpc::Error>)
                   -> Poll<Vec<MasterReplica>, Error> {

        let result: Result<ConnectToMasterResponsePb, Error> = match result {

            // Check if the response contains an application-level error.
            Ok((ConnectToMasterResponsePb { error: Some(error), .. }, _)) => {
                let error = MasterError::from(error);
                trace!("Failed to connect to master {}: {}", server, error);
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

                trace!("Connected to {:?} Master {}", response.role(), server);

                let mut status = AppStatusPb::default();
                status.set_code(StatusCode::NotFound);

                Err(MasterError {
                    code: MasterErrorCode::NotTheLeader,
                    status: Status::from(status),
                }.into())
            },
            Ok((response, _)) => {
                trace!("Connected to Leader Master {}", server);
                Ok(response)
            },
            Err(error) => {
                trace!("Failed to connect to master {}: {}", server, error);
                Err(error.into())
            },
        };

        match result {
            Ok(response) => {
                let master_addrs = response.master_addrs.into_iter().map(HostPort::from).collect::<Vec<_>>();

                unimplemented!()
            }
            Err(error) => {
                let (_, ref mut backoff, ref mut error_slot) = *self.masters.get_mut(&server).unwrap();

            },
        }

        Ok(Async::NotReady)
    }

    /*
    fn register_success(&mut self, server: Self::ServerId) -> Result<Option<Item>, Error> {
        self.leader = Some(server);
    }


    fn register_error(&mut self, server: Self::ServerId, error: Error) {
        self.masters.get_mut(&server).unwrap().2 = Some(error);
    }
    */
}

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
        // TODO: this should return a ! error type.
        loop {
            let (server_id, result) = match self.in_flight.poll() {
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(Some((result, server_id)))) => (server_id, Ok(result)),
                Err((error, server_id)) => (server_id, Err(error)),
            };
            match self.server_picker.poll_result(server_id, result) {
                Ok(Async::NotReady) => (),
                other => return other,
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

/*
fn fail<Req, Resp>(errors: &mut Vec<Error>, call: &Call<Req, Resp>) -> Result<(), Error>
where Req: Message + 'static,
      Resp: Retriable {
    let errors = mem::replace(errors, Vec::new());
    let description = format!("RPC failed: {:?}", call);
    Err(Error::Compound(description, errors))
}

impl <Req, Resp> Future for RetryFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    type Item = (Resp, Vec<Bytes>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            // Poll the in-flight RPC to see if it's complete.
            // NLL hack.
            let state = match self.state {
                State::InFlight(ref mut response) => {
                    let error = match response.poll() {
                        Ok(Async::Ready((response, sidecars))) => match response.into_result() {
                            Ok(response) => return Ok(Async::Ready((response, sidecars))),
                            Err(error) => error,
                        },
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(error) => error.into(),
                    };

                    // The in-flight RPC failed. Check to see if it's retriable.
                    let is_retriable = error.is_retriable();
                    self.errors.push(error);
                    if !is_retriable {
                        fail(&mut self.errors, &self.call)?;
                    }
                    State::Waiting
                },
                State::Waiting =>  {
                    try_ready!(self.sleep.poll().map_err(|_| -> Error { unreachable!() }));
                    let backoff = self.backoff.next_backoff();
                    if self.call.deadline() < Instant::now() + backoff {
                        self.errors.push(Error::TimedOut);
                        fail(&mut self.errors, &self.call)?;
                    }

                    let response = self.proxy.send(self.call.clone());
                    let sleep = self.timer.sleep(backoff);
                    self.sleep = sleep;
                    State::InFlight(response)
                },
            };
            self.state = state;
        }
    }
}
*/

struct ContextFuture<F, C> {
    future: F,
    context: Option<C>,
}

impl <F, C> ContextFuture<F, C> {
    fn new(future: F, context: C) -> ContextFuture<F, C> {
        ContextFuture {
            future,
            context: Some(context),
        }
    }
}

impl <F, C> Future for ContextFuture<F, C> where F: Future {
    type Item = (F::Item, C);
    type Error = (F::Error, C);

    fn poll(&mut self) -> Poll<(F::Item, C), (F::Error, C)> {
        match self.future.poll() {
            Ok(Async::Ready(item)) => Ok(Async::Ready((item, self.context.take().expect("future already complete")))),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err((error, self.context.take().expect("future already complete"))),
        }
    }
}

#[cfg(test)]
mod tests {

    use cpupool::CpuPool;
    use env_logger;
    use krpc;
    use tokio::reactor::Core;

    use AlterTableBuilder;
    use Column;
    use DataType;
    use RaftRole;
    use RangePartitionBound;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use pb::master::*;
    use schema::tests::simple_schema;
    use super::*;

    // TODO(tests):
    //  - Shutdown single master and ensure ConnectToCluster immediately fails.
    //  - Shutdown one master replica and ensure ConnectToCluster succeeds.
    //  - Shutdown all master replicas and ensure ConnectToCluster immediately fails.
    //  - Manipulate master election to make sure there's no leader, ensure ConnectToCluster waits,
    //    then elect a leader and ensure it completes.
    //  - Connect to cluster stress test: have a thread spawning a bunch of concurrent connect to
    //    cluster calls, and have the cluster undergo constant elections and failures.

    #[test]
    fn test_connect_to_master() {
        let _ = env_logger::init();

        let mut cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(1).num_tservers(0));
        let mut reactor = Core::new().unwrap();

        let options = Options {
            rpc: krpc::Options::default(),
            remote: reactor.remote(),
            threadpool: CpuPool::new_num_cpus(),
            timer: Timer::default(),
            admin_timeout: Duration::from_secs(10),
        };

        let mut call = MasterService::connect_to_master(Default::default(),
                                                        Instant::now() + options.admin_timeout);
        call.set_required_feature_flags(&[MasterFeatures::ConnectToMaster as u32]);
        let sp = ConnectToClusterPicker::new(cluster.master_addrs(), &options);
        let future = RetryFuture::new(call.clone(), sp);

        let masters = reactor.run(future).unwrap();

        panic!("masters: {:?}", masters);
        /*
        assert!(sidecars.is_empty());
        assert!(response.error.is_none());
        assert!(!response.ca_cert_der.is_empty());
        assert!(response.authn_token.is_some());
        assert_eq!(response.role(), RaftRole::Leader);
        assert_eq!(cluster.master_addrs(),
                   response.master_addrs.into_iter().map(HostPort::from).collect::<Vec<_>>());
        */

    }

    #[test]
    fn test_connect_to_multi_master() {
        let _ = env_logger::init();

        let mut cluster = MiniCluster::new(MiniClusterConfig::default().num_masters(3).num_tservers(0));
        let mut reactor = Core::new().unwrap();

        let options = Options {
            rpc: krpc::Options::default(),
            remote: reactor.remote(),
            threadpool: CpuPool::new_num_cpus(),
            timer: Timer::default(),
            admin_timeout: Duration::from_secs(10),
        };

        let mut call = MasterService::connect_to_master(Default::default(),
                                                        Instant::now() + options.admin_timeout);
        call.set_required_feature_flags(&[MasterFeatures::ConnectToMaster as u32]);
        let sp = ConnectToClusterPicker::new(cluster.master_addrs(), &options);
        let future = RetryFuture::new(call.clone(), sp);
        let response = reactor.run(future).unwrap();

        panic!("response: {:?}", response);
    }
}
