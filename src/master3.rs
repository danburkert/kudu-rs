use std::mem;
use std::time::Instant;

use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use krpc::{
    HostPort,
    Proxy,
};

use pb::master::*;
use Error;
use Options;
use retry::{
    Retry,
};

pub struct MasterProxy {

}

impl MasterProxy {
    fn spawn(master_addrs: Vec<HostPort>) {
        unimplemented!()
    }
}

enum State {
    Connecting(ConnectToCluster),
    Connected(Proxy),
}

struct MasterTask {
    master_addrs: Vec<HostPort>,
    options: Options,
}

impl MasterTask {


}

struct ConnectToCluster {
    responses: FuturesUnordered<Retry<ConnectToMasterResponsePb>>,
    errors: Vec<Error>,
}

impl ConnectToCluster {
    fn new(master_addrs: &[HostPort], options: &Options) -> ConnectToCluster {
        let now = Instant::now();
        let mut responses = FuturesUnordered::new();
        for master_addr in master_addrs {
            let mut proxy = Proxy::spawn(vec![master_addr.clone()],
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
