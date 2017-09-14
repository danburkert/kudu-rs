#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate bytes;
extern crate futures_await as futures;
extern crate krpc;
extern crate tokio_core as tokio;
extern crate env_logger;

#[macro_use] extern crate prost_derive;

mod calculator_server;

use std::net::SocketAddr;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};

use futures::prelude::*;
use futures::future;
use tokio::reactor::Core;

use krpc::{
    Options,
    Proxy,
    Request,
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
fn ping() {
    let _ = env_logger::init();
    let server = CalculatorServer::start();
    thread::sleep(::std::time::Duration::from_secs(2));

    let mut reactor = Core::new().unwrap();
    let mut proxy = Proxy::spawn(server.addr(), Options::default(), &reactor.remote());

    let request = Request {
        service: "FooService",
        method: "foo",
        required_feature_flags: &[],
        body: Box::new(rpc_tests::AddRequestPb::default()),
        deadline: Instant::now() + Duration::from_secs(10),
    };

    let response = reactor.run(proxy.send(request)).unwrap();

    panic!()
}
