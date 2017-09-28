use futures::{
    Async,
    Future,
    Poll,
};
use futures::stream::FuturesUnordered;

use krpc::{
    HostPort,
    Proxy,
    ResponseFuture,
};

use pb::master::ConnectToMasterResponsePb;
use Error;

pub struct MasterProxy {


}

impl MasterProxy {
    fn spawn(master_addresses: Vec<HostPort>) {
        unimplemented!()
    }
}

struct MasterTask {

}

impl MasterTask {

}

struct ConnectToMaster {
    response: ResponseFuture<ConnectToMasterResponsePb>,
    proxy: Option<Proxy>,
}

impl Future for ConnectToMaster {
    type Item = (ConnectToMasterResponsePb, Proxy);
    type Error = Error;

    fn poll(&mut self) -> Poll<(ConnectToMasterResponsePb, Proxy), Error> {
        let response = try_ready!(self.response.poll());
        Ok(Async::Ready((response, self.proxy.take().unwrap())))
    }
}

impl ConnectToMaster {


}

/*
pub struct ConnectToCluster {
    rpcs: FuturesUnordered<krpc::ResponseFuture<ConnectToMasterResponsePb>>,
}

impl struct ConnectToCluster {

    fn spawn(hostports: Vec<String>, 
             options: Options, 
             threadpool: CpuPool, 
             remote: &Remote) {

    }


    fn connect(&mut proxy: Proxy,
               request: Box<ConnectToMasterRequestPb>, 
                deadline: Instant, 
                required_feature_flags: &'static [u32]
}
*/
