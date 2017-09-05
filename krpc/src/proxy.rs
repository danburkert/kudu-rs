use std::net::SocketAddr;

use either::Either;
use futures::{
    AsyncSink,
    Future,
    Poll,
    Sink,
};
use futures::sync::{mpsc, oneshot};
use tokio::reactor::{
    Handle,
    Remote,
};

use Options;
use Request;
use Response;
use Rpc;
use connection::{Connection, ConnectionNew};
use negotiator::Negotiator;

#[derive(Clone, Debug)]
pub struct Proxy {
    sender: mpsc::Sender<Rpc>,
}

/// The result of an asynchronous remote method call.
#[must_use]
#[derive(Debug)]
pub enum AsyncSend {
    /// The RPC was sent. The response will be returned through the included future.
    Ready(oneshot::Receiver<Response>),
    /// The connection is not ready.
    ///
    /// The current task will be scheduled to receive a notification when the `Proxy` is ready to
    /// send.
    NotReady(Request),
}

impl Proxy {

    pub fn spawn(addr: SocketAddr, options: Options, remote: &Remote) -> Proxy {
        let (sender, receiver) = mpsc::channel(options.max_rpcs_in_flight as usize);
        remote.spawn(move |handle| Inner {
            addr: addr.clone(),
            options: options.clone(),
            handle: handle.clone(),
            receiver,
            connection: Either::Right(Connection::connect(addr, options, handle)),
        });
        Proxy { sender }
    }

    /// Call a remote method asynchronously.
    ///
    /// Typically users will not call this directly, but rather through a generated service trait
    /// implemented by `Proxy`.
    pub fn send(&mut self, request: Request) -> AsyncSend {
        let (completer, receiver) = oneshot::channel();
        let rpc = Rpc {
            request,
            completer
        };
        match self.sender.start_send(rpc) {
            Ok(AsyncSink::Ready) => AsyncSend::Ready(receiver),
            Ok(AsyncSink::NotReady(rpc)) => AsyncSend::NotReady(rpc.request),
            Err(..) => unreachable!(),
        }
    }
}

type Connecting = Negotiator;

struct Inner {
    addr: SocketAddr,
    options: Options,
    handle: Handle,
    receiver: mpsc::Receiver<Rpc>,
    connection: Either<Connection, ConnectionNew>,
}

impl Future for Inner {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        match self.connection {
            Either::Left(ref mut connection) => {
            },
            Either::Right(ref mut negotiator) => {
            },
        }
        unimplemented!()
    }
}
