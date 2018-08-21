#![cfg(test)]

extern crate env_logger;
extern crate futures;
extern crate krpc;
extern crate tacho;
extern crate tokio;

#[macro_use]
extern crate prost_derive;

mod calculator_server;

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::lazy;
use futures::Future;
use krpc::{Call, Error, Options, Proxy, RpcError, RpcErrorCode};
use tokio::executor::current_thread::CurrentThread;

use calculator_server::CalculatorServer;
use rpc_tests::*;

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

fn init(mut options: Options) -> (CalculatorServer, Proxy, tacho::Reporter) {
    let _ = env_logger::try_init();

    let (scope, reporter) = tacho::new();
    options.scope = Some(scope);
    let server = CalculatorServer::start();
    let proxy = Proxy::spawn(vec![server.addr().into()].into_boxed_slice(), options);
    (server, proxy, reporter)
}

fn proxy_errors(report: &tacho::Report) -> usize {
    report
        .counters()
        .iter()
        .map(|(key, count)| -> usize {
            if key.name() == "proxy_errors" {
                *count
            } else {
                0
            }
        }).sum()
}

#[test]
fn call() {
    tokio::run(lazy(|| {
        let (_server, mut proxy, reporter) = init(Options::default());

        let call = CalculatorService::add(
            Arc::new(AddRequestPb { x: 42, y: 18 }),
            Instant::now() + Duration::from_secs(10),
        );

        let (response, sidecars) = proxy.send(call).wait().expect("add request failed");
        assert_eq!(0, sidecars.len());
        assert_eq!(60, response.result);
        assert_eq!(0, proxy_errors(&reporter.peek()));
        Ok(())
    }))
}

#[test]
fn invalid_service() {
    tokio::run(lazy(|| {
        let (_server, mut proxy, reporter) = init(Options::default());

        let now = Instant::now();
        let call = Call::<AddRequestPb, AddResponsePb>::new(
            "FooService",
            "foo",
            Arc::new(AddRequestPb::default()),
            now + Duration::from_secs(10),
        );

        match proxy.send(call).wait().unwrap_err() {
            Error::Rpc(RpcError {
                code: RpcErrorCode::ErrorNoSuchService,
                ..
            }) => (),
            error => panic!("unexpected error: {}", error),
        }
        assert_eq!(0, proxy_errors(&reporter.peek()));
        Ok(())
    }))
}

#[test]
fn invalid_method() {
    tokio::run(lazy(|| {
        let (_server, mut proxy, reporter) = init(Options::default());

        let call = Call::<AddRequestPb, AddResponsePb>::new(
            "kudu.rpc_test.CalculatorService",
            "foo",
            Arc::new(AddRequestPb::default()),
            Instant::now() + Duration::from_secs(10),
        );

        match proxy.send(call).wait().unwrap_err() {
            Error::Rpc(RpcError {
                code: RpcErrorCode::ErrorNoSuchMethod,
                ..
            }) => (),
            error => panic!("unexpected error: {}", error),
        }

        assert_eq!(0, proxy_errors(&reporter.peek()));
        Ok(())
    }))
}

#[test]
fn timeout() {
    tokio::run(lazy(|| {
        let (_server, mut proxy, reporter) = init(Options::default());

        // Timeout expires before the RPC is sent.
        let call = CalculatorService::add(Arc::new(AddRequestPb { x: 42, y: 18 }), Instant::now());

        match proxy.send(call).wait().unwrap_err() {
            Error::TimedOut => (),
            error => panic!("unexpected error: {}", error),
        }

        assert_eq!(0, proxy_errors(&reporter.peek()));
        Ok(())
    }))
}

#[test]
fn cancel() {
    tokio::run(lazy(|| {
        let (_server, mut proxy, reporter) = init(Options::default());

        // Timeout expires before the RPC is sent.
        let call = CalculatorService::add(
            Arc::new(AddRequestPb { x: 42, y: 18 }),
            Instant::now() + Duration::from_secs(10),
        );

        let _ = proxy.send(call);

        assert_eq!(0, proxy_errors(&reporter.peek()));
        Ok(())
    }))
}

/*
#[test]
fn disconnect() {
    use std::thread;
    use rpc_tests::CalculatorService;
    let (_server, mut proxy, reporter) = init(Options::default());

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
    tokio::run(lazy(|| {
        let (server, mut proxy, _reporter) = init(Options::default());

        let call = CalculatorService::add(
            Arc::new(AddRequestPb { x: 42, y: 18 }),
            Instant::now() + Duration::from_secs(20),
        );

        proxy.send(call.clone()).wait().expect("add request failed");

        drop(server);

        match proxy.send(call).wait().unwrap_err() {
            Error::Io(_) => (),
            error => panic!("unexpected error: {}", error),
        }
        Ok(())
    }))
}
