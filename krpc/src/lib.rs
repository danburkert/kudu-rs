extern crate byteorder;
extern crate bytes;
extern crate fnv;
extern crate prost;
extern crate prost_types;

#[macro_use] extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate prost_derive;
#[macro_use] extern crate tokio_core as tokio;

mod connection;
mod error;
mod negotiator;
mod pb;
mod proxy;
mod transport;

use std::fmt;
use std::time::Instant;

use bytes::{
    Bytes,
    BytesMut,
};
use futures::sync::oneshot;
use prost::Message;

pub use error::{Error, RpcError, RpcErrorCode};
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

/// The response to an RPC request.
#[derive(Debug)]
pub enum Response<Body> {
    /// A successful RPC response.
    Ok {
        /// The response body.
        body: Body,
        /// The response sidecars.
        sidecars: Vec<Bytes>,
        /// The request.
        request: Request,
    },

    /// A failed RPC response.
    Err {
        /// The error.
        error: Error,
        /// The request.
        request: Request,
    },
}

/// An undecoded response.
pub type RawResponse = Response<Bytes>;
/// A future which resolves to an undecoded response.
pub type RawResponseFuture = oneshot::Receiver<RawResponse>;
/// A future which resolves to a response.
pub type ResponseFuture<T> = futures::Map<RawResponseFuture, fn (RawResponse) -> Response<T>>;

impl RawResponse {
    pub fn decode<T>(self) -> Response<T> where T: Message + Default {
        match self {
            Response::Ok { body, sidecars, request } => match T::decode_length_delimited(&body) {
                Ok(body) => Response::Ok { body, sidecars, request },
                Err(error) => Response::Err { error: error.into(), request },
            },
            Response::Err { error, request} => Response::Err { error, request },
        }
    }
}

/// An in-flight RPC.
#[derive(Debug)]
struct Rpc {
    /// The request.
    request: Request,

    /// The completer.
    completer: oneshot::Sender<RawResponse>,
}

impl Rpc {

    /// Returns `true` if the RPC has been cancelled by the caller.
    pub fn is_canceled(&self) -> bool {
        self.completer.is_canceled()
    }

    /// Returns `true` if the RPC is timed out.
    pub fn is_timed_out(&self, now: Instant) -> bool {
        self.request.deadline <= now
    }

    /// Completes the RPC.
    pub fn complete(self, body: Bytes, sidecars: Vec<Bytes>) {
        let _ = self.completer.send(Response::Ok { body, sidecars, request: self.request });
    }

    /// Fails the RPC.
    fn fail(self, error: Error) {
        let _ = self.completer.send(Response::Err {
            request: self.request,
            error,
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
}

impl Default for Options {
    fn default() -> Options {
        Options {
            max_rpcs_in_flight: 32,
            max_message_length: 5 * 1024 * 1024,
            nodelay: true,
        }
    }
}
