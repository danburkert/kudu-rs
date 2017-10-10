use std::mem;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use futures::future;
use futures::future::Shared;
use futures::{
    Async,
    AsyncSink,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use krpc::{
    HostPort,
    Proxy,
    Request,
    Response,
};

use pb::master::*;
use Error;
use MasterError;
use MasterErrorCode;
use Options;
use retry::{
    Retry,
    KuduResponse,
};

struct MasterProxy {
    cache: Option<(usize, Proxy)>,
    inner: Arc<Inner>,
}

impl MasterProxy {

    fn new(addrs: Vec<HostPort>, options: Options) -> MasterProxy {
        MasterProxy {
            cache: None,
            inner: Arc::new(Inner::new(addrs, options)),
        }
    }

    fn send<T>(&mut self, request: Request) -> Box<Future<Item=T, Error=Error>> where T: KuduResponse + 'static {
        if let Some((epoch, ref mut proxy)) = self.cache {
            if self.inner.epoch.load(Ordering::Relaxed) == epoch {
                /*
                return proxy.send(request)
                            .and_then(|(value, _, request)| {
                                match value.into_result() {
                                    Ok(value) => {
                                    },
                                    Err(error) => {

                                    },
                                }
                                Ok(value)
                            })
                            .map_err(|(error, _)| error.into())
                            .boxed();
                            */
                unreachable!()
            }
        }
        self.cache = None;

        let (shared_proxy, epoch) = {
            let shared = self.inner.proxy.read().unwrap();
            (shared.clone(), self.inner.epoch.load(Ordering::Relaxed))
        };

        match shared_proxy.peek() {
            Some(Ok(proxy)) => {
                self.cache = Some((epoch, (*proxy).clone()));
                return self.send(request);
            },
            Some(Err(error)) => return future::err((*error).clone()).boxed(),
            None => (),
        }

        unimplemented!()
    }
}

struct Inner {
    addrs: Vec<HostPort>,
    options: Options,
    epoch: AtomicUsize,
    proxy: RwLock<Shared<ConnectToCluster>>,
}

impl Inner {
    fn new(addrs: Vec<HostPort>, options: Options) -> Inner {
        let proxy = ConnectToCluster::new(&addrs, &options).shared();

        Inner {
            addrs: addrs,
            options: options,
            epoch: AtomicUsize::new(0),
            proxy: RwLock::new(proxy),
        }
    }

    fn reset(&self, reset_epoch: usize) {
        let current_epoch = self.epoch.load(Ordering::Relaxed);
        if current_epoch != reset_epoch {
            return;
        }

        let mut shared = self.proxy.write().unwrap();
        if self.epoch.compare_and_swap(current_epoch,
                                       current_epoch.wrapping_add(1),
                                       Ordering::Relaxed) != current_epoch {
            return;
        }

        *shared = ConnectToCluster::new(&self.addrs, &self.options).shared();
    }

    fn poll_send<T>(&self, request: Request) -> Poll<MasterResponse<T>, Error> where T: KuduResponse {
        unimplemented!()
    }
}

struct MasterResponse<T> where T: KuduResponse {
    response: Response<T>,
    epoch: usize,
    inner: Arc<Inner>,
}

impl <T> Future for MasterResponse<T> where T: KuduResponse {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let (error, request) = match self.response.poll() {
            Ok(Async::Ready((response, _, request))) => match response.into_result() {
                Ok(response) => return Ok(Async::Ready(response)),
                Err(error) => (error, request),
            },
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err((error, request)) => (error.into(), request),
        };

        if let Error::Master(MasterError { code: MasterErrorCode::NotTheLeader, .. }) = error {
            self.inner.reset(self.epoch);
        };


        unimplemented!()
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
