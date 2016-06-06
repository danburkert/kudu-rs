use std::net::{lookup_host, SocketAddr};
use threadpool;

use kudu_pb::wire_protocol::ServerRegistrationPB;

/// Resolves
fn resolve_server_registrations<F>(server_registrations: &[ServerRegistrationPB], f: F) where F: FnOnce(Vec<SocketAddr>) {

}

//#[test]
//fn test_dns() {
    //let _ = env_logger::init();
    //for host in ::std::net::lookup_host("rust-lang.org").unwrap() {
        //info!("found address: {:?}", host.unwrap());
    //}
//}
