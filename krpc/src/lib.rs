extern crate byteorder;
extern crate bytes;
extern crate fnv;
extern crate itertools;
extern crate prost;
extern crate prost_types;
extern crate tacho;
extern crate tokio;
extern crate tokio_threadpool as threadpool;

#[macro_use] extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate prost_derive;
#[macro_use] extern crate tokio_io;

mod call;
mod connection;
mod connector;
mod error;
mod hostport;
mod negotiator;
mod pb;
mod proxy;
mod rpc;
mod transport;

use std::marker;

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

pub use call::Call;
pub use error::{Error, RpcError, RpcErrorCode};
pub use hostport::HostPort;
pub use pb::rpc::{RequestIdPb as RequestId};
pub use proxy::Proxy;
use rpc::Rpc;

trait RequestBody: Send + Sync {
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

type RpcResult = Result<(Bytes, Vec<Bytes>), Error>;

#[must_use = "futures do nothing unless polled"]
pub struct RpcFuture<Resp> where Resp: Message + Default {
    receiver: oneshot::Receiver<RpcResult>,
    _marker: marker::PhantomData<Resp>,
}

impl <Resp> RpcFuture<Resp> where Resp: Message + Default {
    /// Returns a new `RpcFuture` wrapping the provided oneshot receiver.
    fn new(receiver: oneshot::Receiver<RpcResult>) -> RpcFuture<Resp> {
        RpcFuture {
            receiver,
            _marker: marker::PhantomData::default(),
        }
    }
}

impl <Resp> Future for RpcFuture<Resp> where Resp: Message + Default {
    type Item = (Resp, Vec<Bytes>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (bytes, sidecars) = try_ready!(self.receiver.poll().map_err(|_| -> Error { unreachable!("RPC dropped"); }))?;
        let body = Resp::decode_length_delimited(&bytes)?;

        trace!("RpcFuture Complete: {:?}", body);

        Ok(Async::Ready((body, sidecars)))
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
