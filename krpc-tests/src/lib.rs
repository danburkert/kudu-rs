#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
extern crate krpc;
extern crate tokio_core as tokio;
extern crate env_logger;

#[macro_use] extern crate prost_derive;

mod calculator_server;

use std::time::{Duration, Instant};

use tokio::reactor::Core;

use krpc::{
    Options,
    Proxy,
    Response,
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
fn add() {
    use rpc_tests::CalculatorService;
    let _ = env_logger::init();
    let server = CalculatorServer::start();

    let mut reactor = Core::new().unwrap();
    let mut proxy = Proxy::spawn(server.addr(), Options::default(), &reactor.remote());

    let request = rpc_tests::AddRequestPb { x: 42, y: 18 };
    let deadline = Instant::now() + Duration::from_secs(10);

    let (response, sidecars) = match reactor.run(proxy.add(request, deadline, &[])).unwrap() {
        Response::Ok { body, sidecars, .. } => (body, sidecars),
        Response::Err { error, .. } => panic!("add request failed: {}", error),
    };

    assert_eq!(0, sidecars.len());
    assert_eq!(60, response.result);
}
