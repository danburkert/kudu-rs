use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use cpupool::CpuPool;
use futures::{
    Async,
    AsyncSink,
    Future,
    Poll,
    Sink,
    Stream,
};
use futures::sync::{mpsc, oneshot};
use itertools::Itertools;
use prost::Message;
use tokio::reactor::{
    Handle,
    Remote,
};

use Descriptor;
use Error;
use HostPort;
use Options;
use Request;
use Rpc;
use RpcFuture;
use connection::Connection;
use connector::Connector;

#[derive(Clone)]
pub struct Proxy {
    sender: mpsc::Sender<Rpc>,
}

impl Proxy {

    pub fn spawn(hostports: Box<[HostPort]>,
                 options: Options,
                 threadpool: CpuPool,
                 remote: &Remote)
                 -> Proxy {
        let (sender, receiver) = mpsc::channel(options.max_rpcs_in_flight as usize);
        remote.spawn(move |handle| ProxyTask {
            hostports,
            options,
            threadpool,
            handle: handle.clone(),
            receiver,
            connection_state: ConnectionState::Quiesced,
            buffer: VecDeque::new(),
        });
        Proxy { sender }
    }

    /// Polls the proxy to determine if there is guaranteed to be capacity to send at least one
    /// RPC without waiting.
    ///
    /// Returns `Async::Ready(_)` if there is sufficient capacity, or returns `Async::NotReady` if
    /// the proxy is not guaranteed to have capacity.
    ///
    /// This method may only be called from inside the context of a task or future.
    pub fn poll_ready(&mut self) -> Async<()> {
        match self.sender.poll_ready() {
            Ok(async) => async,
            Err(_) => unreachable!(),
        }
    }

    /// Call a remote method asynchronously.
    ///
    /// Typically users will not call this directly, but rather through a generated service trait
    /// implemented by `Proxy`.
    pub fn send<Req, Resp>(&mut self,
                           descriptor: Descriptor<Req, Resp>,
                           request: Arc<Req>) -> RpcFuture<Resp>
    where Req: Message + 'static,
          Resp: Message + Default {
        let request = Request::new(request, descriptor);

        let (completer, receiver) = oneshot::channel();
        let rpc = Rpc {
            request,
            completer,
        };

        match self.sender.start_send(rpc) {
            Ok(AsyncSink::Ready) => (),
            Ok(AsyncSink::NotReady(_)) => panic!("Proxy not ready"),
            Err(..) => unreachable!(),
        }

        RpcFuture::new(receiver)
    }
}

enum ConnectionState {
    Quiesced,
    Connecting(Connector),
    Connected(Connection),
}

impl fmt::Debug for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionState::Quiesced => write!(f, "Quiesced"),
            ConnectionState::Connecting(_) => write!(f, "Connecting"),
            ConnectionState::Connected(ref connection) => connection.fmt(f),
        }
    }
}

struct ProxyTask {
    hostports: Box<[HostPort]>,
    options: Options,
    threadpool: CpuPool,
    handle: Handle,
    receiver: mpsc::Receiver<Rpc>,
    connection_state: ConnectionState,
    buffer: VecDeque<Rpc>,
}

impl Future for ProxyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        trace!("{:?}: poll", self);

        let now = Instant::now();

        // NLL hack.
        let ProxyTask { ref hostports,
                        ref options,
                        ref threadpool,
                        ref handle,
                        ref mut receiver,
                        ref mut connection_state,
                        ref mut buffer } = *self;

        use self::ConnectionState::*;
        loop {
            let state = match *connection_state {
                Quiesced => {
                    // Create a new connection if there are buffered or queued RPCs.
                    if buffer.is_empty() {
                        match try_ready!(receiver.poll()) {
                            Some(rpc) => buffer.push_back(rpc),
                            // No more senders; shutdown.
                            None => return Ok(Async::Ready(())),
                        }
                    }

                    Connecting(Connector::connect(hostports,
                                                  threadpool,
                                                  handle.clone(),
                                                  options.clone()))
                },
                Connecting(ref mut connector) => {
                    match connector.poll() {
                        Ok(Async::Ready(connection)) => Connected(connection),
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(error) => {
                            // Connecting to the server failed. Fail all buffered and queued
                            // RPCs, and return to the quiesced state.
                            for rpc in buffer.drain(..) {
                                rpc.fail(error.clone());
                            }
                            while let Ok(Async::Ready(Some(rpc))) = receiver.poll() {
                                rpc.fail(error.clone());
                            }
                            Quiesced
                        }
                    }
                },
                Connected(ref mut conn) => {

                    // Send all buffered and queued RPCs. The result of the loop is ok if either
                    // the connection has no more send capacity, or there are no more messages to
                    // send.  If any message fails to send, the result of the loop is the error.
                    let send_result: Result<(), ()> = loop {
                        match conn.poll_ready(now) {

                            // The connection has capacity to send an RPC.
                            Ok(Async::Ready(_)) => {

                                // Take an RPC from the buffer or queue.
                                let rpc = buffer.pop_front()
                                                .map(|rpc| Ok(Async::Ready(Some(rpc))))
                                                .unwrap_or_else(|| receiver.poll())?;

                                match rpc {
                                    // Attempt to send the RPC.
                                    Async::Ready(Some(rpc)) => match conn.send(rpc, now) {
                                        Ok(()) => continue,
                                        error => break error,
                                    },

                                    // No more senders; shutdown if there are no in-flight RPCs.
                                    Async::Ready(None) => if conn.in_flight_rpcs().is_empty() {
                                        return Ok(Async::Ready(()));
                                    } else {
                                        break Ok(());
                                    },

                                    // No messages to send.
                                    Async::NotReady => break Ok(()),
                                }
                            },

                            // The connection has no remaining capacity.
                            Ok(Async::NotReady) => break Ok(()),

                            // The connection is shutdown.
                            Err(()) => break Err(()),
                        }
                    };

                    // Poll the connection in order to complete in-flight RPCs.
                    match send_result.and_then(|_| conn.poll(now)) {
                        Ok(_) => return Ok(Async::NotReady),
                        Err(_) => {
                            buffer.extend(
                                conn.in_flight_rpcs()
                                    .drain()
                                    .map(|(_, rpc)| rpc)
                                    .flat_map(|rpc| {
                                        if rpc.is_canceled() {
                                            None
                                        } else if rpc.is_timed_out(now) {
                                            rpc.fail(Error::TimedOut);
                                            None
                                        } else {
                                            Some(rpc)
                                        }
                                    }));
                            ConnectionState::Quiesced
                        },
                    }
                },
            };
            *connection_state = state;
        }
    }
}

impl fmt::Debug for ProxyTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("ProxyTask");
        debug.field("hostports", &format_args!("{:?}", &self.hostports.iter().format(",")));
        debug.field("core", &self.handle.id());
        match self.connection_state {
            ConnectionState::Quiesced => debug.field("state", &self.connection_state),
            ConnectionState::Connecting(_) => debug.field("state", &self.connection_state),
            ConnectionState::Connected(ref connection) => debug.field("connection", connection),
        };
        debug.finish()
    }
}
