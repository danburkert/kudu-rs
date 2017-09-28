#![feature(proc_macro, conservative_impl_trait, generators)]
#![cfg(test)]

extern crate env_logger;
extern crate futures_await as futures;
extern crate futures_cpupool as cpupool;
extern crate krpc;
extern crate tacho;
extern crate tokio_core as tokio;

#[macro_use] extern crate prost_derive;

mod calculator_server;

use std::time::{Duration, Instant};

use cpupool::CpuPool;
use tokio::reactor::Core;

use krpc::{
    Error,
    Options,
    Proxy,
    RpcError,
    RpcErrorCode,
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

fn init(mut options: Options) -> (CalculatorServer, Core, Proxy, tacho::Reporter) {
    let _ = env_logger::init();

    let threadpool = CpuPool::new(4);
    let (scope, reporter) = tacho::new();
    options.scope = Some(scope);
    let server = CalculatorServer::start();
    let reactor = Core::new().unwrap();
    let proxy = Proxy::spawn(vec![server.addr().into()], options, threadpool, &reactor.remote());
    (server, reactor, proxy, reporter)
}

fn proxy_errors(report: &tacho::Report) -> usize {
    report.counters().iter().map(|(key, count)| -> usize {
        if key.name() == "proxy_errors" { *count } else { 0 }
    }).sum()
}

#[test]
fn call() {
    use rpc_tests::CalculatorService;
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    let request = Box::new(rpc_tests::AddRequestPb { x: 42, y: 18 });
    let deadline = Instant::now() + Duration::from_secs(10);

    let (response, sidecars, _) = reactor.run(proxy.add(request, deadline, &[]))
                                         .expect("add request failed");

    assert_eq!(0, sidecars.len());
    assert_eq!(60, response.result);
    assert_eq!(0, proxy_errors(&reporter.peek()));
}

#[test]
fn invalid_service() {
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    let now = Instant::now();
    let request = krpc::Request {
        service: "FooService",
        method: "foo",
        required_feature_flags: &[],
        body: Box::new(rpc_tests::AddRequestPb::default()),
        timestamp: now,
        deadline: now + Duration::from_secs(10),
    };

    match reactor.run(proxy.send::<()>(request)).unwrap_err().0 {
        Error::Rpc(RpcError { code: RpcErrorCode::ErrorNoSuchService, .. }) => (),
        error => panic!("unexpected error: {}", error),
    }

    assert_eq!(0, proxy_errors(&reporter.peek()));
}

#[test]
fn invalid_method() {
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    let now = Instant::now();
    let request = krpc::Request {
        service: "kudu.rpc_test.CalculatorService",
        method: "foo",
        required_feature_flags: &[],
        body: Box::new(rpc_tests::AddRequestPb::default()),
        timestamp: now,
        deadline: now + Duration::from_secs(10),
    };

    match reactor.run(proxy.send::<()>(request)).unwrap_err().0 {
        Error::Rpc(RpcError { code: RpcErrorCode::ErrorNoSuchMethod, .. }) => (),
        error => panic!("unexpected error: {}", error),
    }

    assert_eq!(0, proxy_errors(&reporter.peek()));
}

#[test]
fn timeout() {
    use rpc_tests::CalculatorService;
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    // Timeout expires before the RPC is sent.
    let request = Box::new(rpc_tests::AddRequestPb { x: 42, y: 18 });
    let deadline = Instant::now();

    match reactor.run(proxy.add(request, deadline, &[])).unwrap_err().0 {
        Error::TimedOut => (),
        error => panic!("unexpected error: {}", error),
    }

    assert_eq!(0, proxy_errors(&reporter.peek()));
}

#[test]
fn cancel() {
    use rpc_tests::CalculatorService;
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    // Timeout expires before the RPC is sent.
    let request = Box::new(rpc_tests::AddRequestPb { x: 42, y: 18 });
    let deadline = Instant::now() + Duration::from_secs(10);
    let _ = reactor.run(proxy.add(request, deadline, &[]));

    assert_eq!(0, proxy_errors(&reporter.peek()));
}

/*
#[test]
fn disconnect() {
    use std::thread;
    use rpc_tests::CalculatorService;
    let (_server, mut reactor, mut proxy, reporter) = init(Options::default());

    let request = Box::new(rpc_tests::AddRequestPb { x: 42, y: 18 });
    let deadline = Instant::now() + Duration::from_secs(10);

    let (response, sidecars) = match reactor.run(proxy.add(request.clone(), deadline, &[])).unwrap() {
        Response::Ok { body, sidecars, .. } => (body, sidecars),
        Response::Err { error, .. } => panic!("add request failed: {}", error),
    };

    thread::sleep(Duration::from_secs(70));

    let deadline = Instant::now() + Duration::from_secs(10);
    let (response, sidecars) = match reactor.run(proxy.add(request, deadline, &[])).unwrap() {
        Response::Ok { body, sidecars, .. } => (body, sidecars),
        Response::Err { error, .. } => panic!("add request failed: {}", error),
    };

    assert_eq!(0, sidecars.len());
    assert_eq!(60, response.result);
    assert_eq!(0, proxy_errors(&reporter.peek()));
}
*/

#[test]
fn server_shutdown() {
    use rpc_tests::CalculatorService;
    let (server, mut reactor, mut proxy, _reporter) = init(Options::default());

    let request = Box::new(rpc_tests::AddRequestPb { x: 42, y: 18 });
    let deadline = Instant::now() + Duration::from_secs(10);

    reactor.run(proxy.add(request.clone(), deadline, &[])).expect("add request failed");

    drop(server);

    let deadline = Instant::now() + Duration::from_secs(10);
    reactor.run(proxy.add(request.clone(), deadline, &[])).expect_err("add request failed");
}
