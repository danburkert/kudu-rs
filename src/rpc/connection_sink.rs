use std::net::SocketAddr;
use std::collections::HashMap;

use tokio;
use netbuf::Buf;
use futures;

use backoff::Backoff;
use Error;
use Result;
use rpc::Rpc;

enum State {
    Initiating(Initiating),
    Negotiating(Negotiating),
    Connected(Connected),
    Reset(Reset),
}

struct Connection {
    state: State,

    /// The address of the remote Kudu server.
    addr: SocketAddr,

    /// Backoff tracker.
    reset_backoff: Backoff,
}

impl futures::sink::Sink for Connection {
    type SinkItem = Rpc;
    type SinkError = Error;
    fn start_send(&mut self, rpc: Rpc) -> futures::StartSend<Rpc, Error> {
        unimplemented!()
    }

    fn poll_complete(&mut self) -> futures::Poll<(), Error> {
        unimplemented!()
    }
}

struct Initiating {
    socket: tokio::net::TcpStreamNew,
    nodelay: bool,
}

impl futures::Future for Initiating {
    type Item = Negotiating;
    type Error = Error;
    fn poll(&mut self) -> futures::Poll<Negotiating, Error> {
        let socket = try_ready!(self.socket.poll());
        socket.set_nodelay(self.nodelay)?;
        Ok(futures::Async::Ready(Negotiating::new(socket)))
    }
}

struct Negotiating {
    socket: tokio::net::TcpStream,

    recv_buf: Buf,

    send_buf: Buf,
}

impl Negotiating {
    fn new(socket: tokio::net::TcpStream) -> Negotiating {
        Negotiating {
            socket: socket,
            recv_buf: Buf::new(),
            send_buf: Buf::new(),
        }
    }
}

impl futures::Future for Negotiating {
    type Item = Connected;
    type Error = Error;
    fn poll(&mut self) -> futures::Poll<Connected, Error> {
        unimplemented!()
    }
}

struct Connected {
    socket: tokio::net::TcpStream,
    recv_buf: Buf,
    send_buf: Buf,
    recv_queue: HashMap<usize, Rpc>,
}

impl futures::Future for Connected {
    type Item = ();
    type Error = Error;
    fn poll(&mut self) -> futures::Poll<(), Error> {
        unimplemented!()
    }
}

struct Reset {
    timer: tokio::reactor::Timeout,
}

impl futures::Future for Reset {
    type Item = ();
    type Error = Error;
    fn poll(&mut self) -> futures::Poll<(), Error> {
        self.timer.poll().map_err(From::from)
    }
}

struct FramedIo {
    buf: Buf,
}

impl futures::Future for FramedIo {
    type Item = Vec<u8>
    type Error = Error;
    fn poll(&mut self) -> futures::Poll<&'a [u8], Error> {
        unimplemented!()
    }
}
