extern crate bytes;
extern crate krpc;
extern crate prost;

#[macro_use] extern crate prost_derive;

mod calculator_server;
use calculator_server::CalculatorServer;

mod rpc {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc.rs"));
}
mod rpc_tests {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test.rs"));
}
mod rpc_test_diff_package {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test_diff_package.rs"));
}
mod security {
    include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
}

#[test]
fn ping() {
    let _server = CalculatorServer::start();
}
