use std::any::Any;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{SyncSender, SendError};
use std::time::Instant;

use protobuf::Message;

pub use rpc::messenger::Messenger;

use Error;
use Result;

mod connection;
pub mod master;
mod messenger;
pub mod tablet_server;

/// A callback that will be executed when an RPC is complete. If the RPC succeeds, the result will
/// be `Ok`, and the RPC will contain the response and sidecars. Othewise, the result will contain
/// the failure. `Callback` is used instead of the built-in function traits so that the `self`
/// parameter can be passed by move when firing the callback, which allows for efficient retry
/// handlers.
pub trait Callback: Send {
    fn callback(self: Box<Self>, result: Result<()>, rpc: Rpc);
}

impl<F> Callback for F where F: FnOnce(Result<()>, Rpc) + Send {
    fn callback(self: Box<F>, result: Result<()>, rpc: Rpc) {
        (*self)(result, rpc)
    }
}

/// An `Rpc` contains all the state necessary to execute and retry an operation on a remote Kudu
/// server.
///
/// When the `Rpc` completes (either succesfully or unsuccessfully), the included callback is
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
    pub callback: Option<Box<Callback>>,
    pub cancel: Option<Arc<AtomicBool>>,
    pub fail_fast: bool,
}

impl Rpc {
    fn complete(mut self) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Ok(()), self)
        }
    }

    pub fn fail(mut self, error: Error) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Err(error), self)
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
    pub fn cancelled(&self) -> bool {
        self.cancel.as_ref().map_or(false, |b| b.load(Ordering::Relaxed))
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

/// A callback which retries the RPC on network error.
#[derive(Clone)]
pub struct RetryNetworkErrorCB<F>{
    messenger: Messenger,
    f: F,
}
impl <F> RetryNetworkErrorCB<F> where F: FnMut(Result<()>, Rpc) + Send + 'static {
    fn new(messenger: Messenger, f: F) -> RetryNetworkErrorCB<F> {
        RetryNetworkErrorCB {
            messenger: messenger,
            f: f
        }
    }
}
impl <F> Callback for RetryNetworkErrorCB<F> where F: FnMut(Result<()>, Rpc) + Send + 'static {
    fn callback(mut self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
        if result.as_ref().err().map_or(false, |error| error.is_network_error()) {
            trace!("retrying RPC {:?} after network error: {}", rpc, result.unwrap_err());
            let messenger = self.messenger.clone();
            rpc.response.clear();
            rpc.callback = Some(self);
            messenger.send(rpc);
        } else {
            (self.f)(result, rpc)
        }
    }
}

/// Returns a callback which will send the result and RPC to a sync channel on completion. If the
/// RPC fails, it will be retried until it times out. The caller should ensure that the channel has
/// sufficient capcity for the callback, otherwise the RPC I/O thread will be blocked.
pub fn channel_callback(sender: SyncSender<(Result<()>, Rpc)>) -> Box<Callback> {
    Box::new(move |result, rpc| {
        if let Err(SendError((result, rpc))) = sender.send((result, rpc)) {
            warn!("callback channel disconnected, result: {:?}, rpc: {:?}", result, rpc)
        }
    })
}

/// Returns a callback which will retry the RPC on connection error, and send the result and RPC to
/// a sync channel on completion. If the RPC fails, it will be retried until it times out. The
/// caller should ensure that the channel has sufficient capcity for the callback, otherwise the
/// RPC I/O thread will be blocked.
pub fn retry_channel_callback(messenger: Messenger, sender: SyncSender<(Result<()>, Rpc)>) -> Box<Callback> {
    Box::new(RetryNetworkErrorCB::new(messenger, move |result, rpc| {
        if let Err(SendError((result, rpc))) = sender.send((result, rpc)) {
            warn!("callback channel disconnected, result: {:?}, rpc: {:?}", result, rpc)
        }
    }))
}
