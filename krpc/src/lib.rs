extern crate byteorder;
extern crate bytes;
extern crate fnv;
extern crate futures_cpupool as cpupool;
extern crate itertools;
extern crate prost;
extern crate prost_types;
extern crate tacho;

#[macro_use] extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate prost_derive;
#[macro_use] extern crate tokio_core as tokio;

mod connection;
mod connector;
mod error;
mod hostport;
mod negotiator;
mod pb;
mod proxy;
mod transport;

use std::fmt;
use std::marker;
use std::time::{Duration, Instant};

use bytes::{
    Bytes,
    BytesMut,
};
use futures::sync::oneshot;
use futures::{
    Future,
    Async,
    Poll,
};
use prost::Message;

pub use error::{Error, RpcError, RpcErrorCode};
pub use hostport::HostPort;
pub use pb::rpc::{RequestIdPb as RequestId};
pub use proxy::{Proxy, AsyncSend};

pub trait RequestBody: Send {
    fn encoded_len(&self) -> usize;
    fn encode_length_delimited(&self, dst: &mut BytesMut);
}

impl <M> RequestBody for M where M: Message {
    fn encoded_len(&self) -> usize {
        Message::encoded_len(self)
    }
    fn encode_length_delimited(&self, dst: &mut BytesMut) {
        Message::encode_length_delimited(self, dst).unwrap()
    }
}

/// An RPC request builder.
pub struct Request {
    pub service: &'static str,
    pub method: &'static str,
    pub required_feature_flags: &'static [u32],
    pub body: Box<RequestBody>,
    pub timestamp: Instant,
    pub deadline: Instant,
}

impl Request {
    /// Creates a new [Request].
    pub fn new(service: &'static str,
               method: &'static str,
               body: Box<RequestBody>,
               deadline: Instant) -> Request {
        Request {
            service,
            method,
            required_feature_flags: &[],
            body,
            timestamp: Instant::now(),
            deadline,
        }
    }

    /// Sets the required feature flags of the request.
    pub fn required_feature_flags(&mut self, required_feature_flags: &'static [u32]) -> &mut Request {
        self.required_feature_flags = required_feature_flags;
        self
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Request")
         .field("service", &self.service)
         .field("method", &self.method)
         .field("deadline", &self.deadline)
         .finish()
    }
}

type RpcResult = Result<(Bytes, Vec<Bytes>, Request),
                        (Error, Request)>;

#[must_use = "futures do nothing unless polled"]
pub struct Response<T> where T: Message + Default {
    receiver: oneshot::Receiver<RpcResult>,
    _marker: marker::PhantomData<T>,
}

impl <T> Response<T> where T: Message + Default {
    pub fn failed(request: Request, error: Error) -> Response<T> {
        let (completer, receiver) = oneshot::channel();
        completer.send(Err((error, request))).unwrap();

        Response {
            receiver,
            _marker: marker::PhantomData,
        }
    }
}

impl <T> Future for Response<T> where T: Message + Default {
    type Item = (T, Vec<Bytes>, Request);
    type Error = (Error, Request);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.poll().expect("RPC dropped") {
            Async::Ready(Ok((bytes, sidecars, request))) => match T::decode_length_delimited(&bytes) {
                Ok(body) => Ok(Async::Ready((body, sidecars, request))),
                Err(error) => Err((error.into(), request)),
            },
            Async::Ready(Err((error, request))) => Err((error, request)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}

/// An in-flight RPC.
struct Rpc {
    /// The request.
    request: Request,

    /// The completer.
    completer: oneshot::Sender<RpcResult>,
}

impl Rpc {

    /// Returns `true` if the RPC has been canceled by the caller.
    pub fn is_canceled(&self) -> bool {
        self.completer.is_canceled()
    }

    /// Returns `true` if the RPC is timed out.
    pub fn is_timed_out(&self, now: Instant) -> bool {
        self.request.deadline <= now
    }

    /// Completes the RPC.
    pub fn complete(self, body: Bytes, sidecars: Vec<Bytes>) {
        let _ = self.completer.send(Ok((body, sidecars, self.request)));
    }

    /// Fails the RPC.
    fn fail(self, error: Error) {
        let _ = self.completer.send(Err((error, self.request)));
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    /// Maximum number of outstandings RPCs to allow per connection.
    ///
    /// Defaults to 32.
    pub max_rpcs_in_flight: u32,

    /// Maximum allowable message length.
    ///
    /// Defaults to 5 MiB.
    pub max_message_length: u32,

    /// Whether to disable Nagle's algorithm.
    ///
    /// Defaults to true.
    pub nodelay: bool,

    pub scope: Option<tacho::Scope>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            max_rpcs_in_flight: 32,
            max_message_length: 5 * 1024 * 1024,
            nodelay: true,
            scope: None,
        }
    }
}

fn duration_to_us(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000 + duration.subsec_nanos() as u64 / 1000
}
