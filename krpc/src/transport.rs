use std::io;
use std::net::SocketAddr;

use bytes::BytesMut;
use futures::{
    Async,
    Future,
};
use tokio::net::TcpStream;
use tokio::reactor::Handle;

use pb::rpc::{
    RequestHeader,
    ResponseHeader,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportOptions {

    /// Maximum allowable message length.
    ///
    /// Defaults to 5 MiB.
    pub max_message_length: u32,

    /// Whether to disable Nagle's algorithm.
    ///
    /// Defaults to true.
    pub nodelay: bool,
}

impl Default for TransportOptions {
    fn default() -> TransportOptions {
        TransportOptions {
            max_message_length: 5 * 1024 * 1024,
            nodelay: true,
        }
    }
}

struct Transport {
    options: TransportOptions,
    stream: TcpStream,
    send_buf: BytesMut,
    recv_buf: BytesMut,
    request_header: RequestHeader,
    response_header: ResponseHeader,
}

impl Transport {
    pub fn connect(options: TransportOptions, addr: SocketAddr, handle: &Handle) -> NewTransport {

        NewTransport {

        }

    }
}

struct NewTransport {

}

impl Future for NewTransport {
    type Item = Transport;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Transport>, io::Error> {

        Ok(Async::NotReady)
    }
}
