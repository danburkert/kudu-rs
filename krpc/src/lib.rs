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
mod transport;

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
pub struct Request<S> {
    service: &'static str,
    method: &'static str,
    required_feature_flags: &'static [u32],
    body: Box<RequestBody>,
    state: S,
    deadline: Instant,
}

impl <S> Request<S> {
    /// Creates a new [Request].
    pub fn new(service: &'static str,
               method: &'static str,
               body: Box<RequestBody>,
               state: S,
               deadline: Instant) -> Request<S> {
        Request {
            service,
            method,
            required_feature_flags: &[],
            body,
            state,
            deadline,
        }
    }

    /// Sets the required feature flags of the request.
    pub fn required_feature_flags(&mut self, required_feature_flags: &'static [u32]) -> &mut Request<S> {
        self.required_feature_flags = required_feature_flags;
        self
    }

    pub(crate) fn complete(self, response: Response) -> Rpc<S> {
        Rpc {
            request: self,
            response: Ok(response),
        }
    }

    pub(crate) fn fail(self, error: Error) -> Rpc<S> {
        Rpc {
            request: self,
            response: Err(error),
        }
    }
}

impl <S> fmt::Debug for Request<S>  {
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
    pub request: Request<S> ,
    /// The response.
    pub response: Result<Response, Error>,
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

