use std::marker;
use std::mem;
use std::time::Instant;

use futures::{
    Async,
    AsyncSink,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use futures::sync::oneshot;
use krpc::{
    HostPort,
    Proxy,
    Request,
    Response,
};

use pb::master::*;
use Error;
use Options;
use retry::{
    Retry,
    KuduResponse,
};

pub struct MasterResponse<T> where T: KuduResponse {
    receiver: oneshot::Receiver<T>,
}

/// An in-flight master RPC.
pub struct MasterRpc {
}

pub struct MasterProxy {
}

impl MasterProxy {

    fn spawn(addrs: Vec<HostPort>) {
        unimplemented!()
    }

    /// Call a remote Master method asynchronously.
    pub fn send<R>(&mut self, request: Request) -> MasterResponse<R> where R: KuduResponse {
        let (completer, receiver) = oneshot::channel();
        let rpc = Rpc {
            request,
            completer,
        };

        match self.sender.start_send(rpc) {
            Ok(AsyncSink::Ready) => (),
            Ok(AsyncSink::NotReady(_)) => panic!("Proxy not ready"),
            Err(..) => unreachable!(),
        }

        Response {
            receiver,
            _marker: marker::PhantomData,
        }
    }
}

enum State {
    Connecting(ConnectToCluster),
    Connected(Proxy),
}

#[must_use = "futures do nothing unless polled"]
struct MasterProxyTask {
    addrs: Vec<HostPort>,
    options: Options,
}

impl Future for MasterProxyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
        }
    }
}

#[must_use = "futures do nothing unless polled"]
struct ConnectToCluster {
    responses: FuturesUnordered<Retry<ConnectToMasterResponsePb>>,
    errors: Vec<Error>,
}

impl ConnectToCluster {
    fn new(addrs: &[HostPort], options: &Options) -> ConnectToCluster {
        let now = Instant::now();
        let mut responses = FuturesUnordered::new();
        for addr in addrs {
            let mut proxy = Proxy::spawn(vec![addr.clone()],
                                         options.rpc.clone(),
                                         options.threadpool.clone(),
                                         &options.remote);

            let response = proxy.connect_to_master(Default::default(),
                                                   now + options.admin_timeout,
                                                   &[MasterFeatures::ConnectToMaster as u32]);

            responses.push(Retry::wrap(response, proxy, options.timer.clone()));
        }

        ConnectToCluster {
            responses,
            errors: Vec::new(),
        }
    }
}

impl Future for ConnectToCluster {
    type Item = Proxy;
    type Error = Error;
    fn poll(&mut self) -> Poll<Proxy, Error> {
        loop {
            match self.responses.poll() {
                Ok(Async::Ready(Some((_, _, proxy)))) => return Ok(Async::Ready(proxy)),
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
