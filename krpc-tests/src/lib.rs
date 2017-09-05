extern crate bytes;
extern crate krpc;
extern crate prost;
extern crate tokio_core as tokio;

#[macro_use] extern crate prost_derive;

mod calculator_server;

use std::net::SocketAddr;
use std::str::FromStr;
use std::time::{Duration, Instant};

use tokio::reactor::Core;

use krpc::{
    Options,
    Proxy,
    Request,
};

use calculator_server::CalculatorServer;

pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc.rs"));
}
pub mod rpc_tests {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test.rs"));
}
pub mod rpc_test_diff_package {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test_diff_package.rs"));
}
pub mod security {
    include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
}

#[test]
fn ping() {
    //let _server = CalculatorServer::start();

    let reactor = Core::new().unwrap();

    let mut proxy = Proxy::spawn(SocketAddr::from_str("127.0.0.1:12345").unwrap(),
                                 Options::default(),
                                 &reactor.remote());

    let request = Request {
        service: "FooService",
        method: "foo",
        required_feature_flags: &[],
        body: Box::new(rpc_tests::AddRequestPb::default()),
        deadline: Instant::now() + Duration::from_secs(10),
    };

    proxy.send(request);
}
