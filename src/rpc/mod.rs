use std::any::Any;
use std::error;
use std::fmt;
use std::net::SocketAddr;
use std::result;
use std::time::Instant;

use futures::Async;
use futures::sync::oneshot;
use protobuf::Message;

pub use rpc::messenger::Messenger;

use Error;

mod connection;
mod connection_task;
pub mod master;
mod messenger;
pub mod tablet_server;

pub type RpcResult = result::Result<Rpc, RpcError>;

#[derive(Debug)]
pub struct RpcError {
    rpc: Rpc,
    error: Error,
}

impl error::Error for RpcError {
    fn description(&self) -> &str { self.error.description() }
    fn cause(&self) -> Option<&error::Error> { self.error.cause() }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// An `Rpc` contains all the state necessary to execute and retry an operation on a remote Kudu
/// server.
///
/// When the `Rpc` completes (either succesfully or unsuccessfully), the included oneshot future is
/// fired.
///
/// `Rpc`s impose a few restrictions to ensure eventual termination: all `Rpc`s have a timeout
/// deadline, and `Rpc`s may optionally be cancelled by the caller if they are no longer necessary.
/// Timing out or cancelling an `Rpc` should cause it to be completed with the corresponding error
/// message as soon as possible.
///
/// The [Kudu RPC protocol](https://github.com/cloudera/kudu/blob/master/docs/design-docs/rpc.md)
/// uses Protobuf as a standard serialization format. Each RPC to a Kudu server consists of a
/// request header message and a body message. The request header includes the service name and
/// method name, which together identify the type of the request and response body types. The
/// response includes a response header, a response body, and optional sidecars.
///
/// The `Rpc` struct holds both the request Protobuf message as well as the response. When the RPC
/// response is received from the wire, it is deserialized into the response message, and the `Rpc`
/// is completed.
///
/// `Rpc`s may be marked as 'fail-fast' to indicate that if any interruption in the connection to
/// the remote server occurs, the RPC should be immediately failed. If an `Rpc` is not marked as
/// fail-fast, it will be retried to the same destination until it either fails, or it times out.
/// Fail-fast RPCs are useful for situations where the RPC could be retried against an alternate
/// remote server.
pub struct Rpc {
    pub addr: SocketAddr,
    pub service_name: &'static str,
    pub method_name: &'static str,
    pub deadline: Instant,
    pub required_feature_flags: Vec<u32>,
    pub request: Box<Message>,
    pub response: Box<Message>,
    pub sidecars: Vec<Vec<u8>>,
    pub oneshot: Option<oneshot::Sender<RpcResult>>,
    pub fail_fast: bool,
}

impl Rpc {

    fn complete(mut self) {
        if let Some(oneshot) = self.oneshot.take() {
            oneshot.complete(Ok(self))
        }
    }

    pub fn fail(mut self, error: Error) {
        if let Some(oneshot) = self.oneshot.take() {
            oneshot.complete(Err(RpcError { rpc: self, error: error }))
        }
    }

    pub fn response<T>(&self) -> &T where T: Any {
        self.response.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn response_mut<T>(&mut self) -> &mut T where T: Any {
        self.response.as_any_mut().downcast_mut::<T>().unwrap()
    }

    pub fn take_response<T>(self) -> T where T: Any {
        *self.response.into_any().downcast::<T>().unwrap()
    }

    pub fn mut_response<T>(&mut self) -> &mut T where T: Any {
        self.response.as_any_mut().downcast_mut::<T>().unwrap()
    }

    /// Returns `true` if this RPC has been requested to be cancelled.
    pub fn cancelled(&mut self) -> bool {
        self.oneshot.as_mut().map_or(false, |f| {
            match f.poll_cancel().unwrap() {
                Async::Ready(_) => true,
                Async::NotReady => false,
            }
        })
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    pub fn timed_out(&self, now: Instant) -> bool {
        self.deadline <= now
    }

    pub fn fail_fast(&self) -> bool {
        self.fail_fast
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rpc {{ {}::{}, addr: {}, deadline: {:?} }}",
               self.service_name, self.method_name, self.addr, self.deadline)
    }
}
