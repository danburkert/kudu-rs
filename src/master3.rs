use futures::{
    Future,
}
use futures::stream::FuturesUnordered;

pub struct MasterProxy {
    fn spawn(hostports: Vec<String>)


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
