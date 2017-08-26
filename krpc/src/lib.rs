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
use futures::sync::oneshot::{Sender, Receiver};
use futures::{Async, Sink};

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
pub struct Response<S> {
    /// The response body.
    pub body: Bytes,
    /// The response sidecars.
    pub sidecars: Vec<Bytes>,
}

/// A completed RPC.
pub struct Rpc {
    /// The request.
    pub request: Request,
    /// The paired state.
    pub state: S,
    /// The response.
    pub response: Result<Response, Error>,
}

/// An in-flight RPC.
struct <S> InFlightRpc<S> {
    /// The request.
    pub request: Request,
    /// The paired state.
    pub state: S,
}

impl <S> InFlightRpc<S> {

    fn fail(self, error: Error) {
        let rpc = Rpc {
            request: self.request,
            response: Err(error),
        };
        let _ = self.sender.send(rpc);
    }

    fn complete(self, body: Bytes, sidecars: Vec<Bytes>) {
        let rpc = Rpc {
            request: self.request,
            response: Ok(Response { body, sidecars }),
        };
        let _ = self.sender.send(rpc);
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    pub fn timed_out(&self, now: Instant) -> bool {
        self.request.deadline <= now
    }

    /// Returns `true` if the RPC has been cancelled.
    ///
    /// This method must be called in the context of a task (like `poll`).
    pub fn cancelled(&mut self) -> bool {
        self.sender.poll_cancel().unwrap() == Async::Ready(())
    }
}
