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
mod transport;
pub mod pb; // TODO: make private

use std::fmt;
use std::time::Instant;

use bytes::{
    Bytes,
    BytesMut,
};
use prost::Message;

pub use connection::Connection;
pub use error::{Error, RpcError, RpcErrorCode};
pub use pb::rpc::{RequestIdPb as RequestId};

pub trait RequestBody {
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
    service: &'static str,
    method: &'static str,
    required_feature_flags: &'static [u32],
    body: Box<RequestBody>,
    deadline: Instant,
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
#[derive(Debug, Clone)]
pub struct Response {
    /// The response body.
    pub body: Bytes,
    /// The response sidecars.
    pub sidecars: Vec<Bytes>,
}

/// A completed RPC.
pub struct Rpc<S> {
    /// The request.
    pub request: Request,
    /// The paired state.
    pub state: S,
    /// The response.
    pub response: Result<Response, Error>,
}

/// An in-flight RPC.
struct InFlightRpc<S> {
    /// The request.
    pub request: Request,
    /// The paired state.
    pub state: S,
}

impl <S> InFlightRpc<S> {

    pub fn complete(self, body: Bytes, sidecars: Vec<Bytes>) -> Rpc<S> {
        Rpc {
            request: self.request,
            state: self.state,
            response: Ok(Response { body, sidecars }),
        }
    }

    pub fn fail(self, error: Error) -> Rpc<S> {
        Rpc {
            request: self.request,
            state: self.state,
            response: Err(error),
        }
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    pub fn timed_out(&self, now: Instant) -> bool {
        self.request.deadline <= now
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionOptions {
    /// Maximum number of outstandings RPCs to allow in the connection.
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

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            max_rpcs_in_flight: 32,
            max_message_length: 5 * 1024 * 1024,
            nodelay: true,
        }
    }
}

