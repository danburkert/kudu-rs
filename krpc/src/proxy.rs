use std::fmt;
use std::collections::VecDeque;

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
use tokio::reactor::{
    Handle,
    Remote,
};

use Error;
use Options;
use RawResponse;
use RawResponseFuture;
use Request;
use Rpc;
use connection::Connection;
use connector::Connector;

#[derive(Clone)]
pub struct Proxy {
    sender: mpsc::Sender<Rpc>,
}

/// The result of an asynchronous remote method call.
#[must_use]
#[derive(Debug)]
pub enum AsyncSend {
    /// The RPC was sent. The response will be returned through the included future.
    Ready(oneshot::Receiver<RawResponse>),
    /// The connection is not ready.
    ///
    /// The current task will be scheduled to receive a notification when the `Proxy` is ready to
    /// send.
    NotReady(Request),
}

impl Proxy {

    pub fn spawn(hostports: Vec<String>,
                 options: Options,
                 threadpool: CpuPool,
                 remote: &Remote)
                 -> Proxy {
        trace!("spawn!");
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
    pub fn send(&mut self, request: Request) -> RawResponseFuture {
        let (completer, receiver) = oneshot::channel();
        let rpc = Rpc {
            request,
            completer
        };

        match self.sender.start_send(rpc) {
            Ok(AsyncSink::Ready) => (),
            Ok(AsyncSink::NotReady(_)) => panic!("Proxy not ready"),
            Err(..) => unreachable!(),
        }

        receiver
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
    hostports: Vec<String>,
    options: Options,
    threadpool: CpuPool,
    handle: Handle,
    receiver: mpsc::Receiver<Rpc>,
    connection_state: ConnectionState,
    buffer: VecDeque<Rpc>
}

impl Future for ProxyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        trace!("{:?}: poll", self);
        let ProxyTask { ref hostports,
                        ref options,
                        ref threadpool,
                        ref handle,
                        ref mut receiver,
                        ref mut connection_state,
                        ref mut buffer } = *self;
        use self::ConnectionState::*;
        // NLL hack.
        loop {
            let state = match *connection_state {
                Quiesced => {
                    // Assume wakeup due to an RPC being ready to send.
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
                            error!("{}", error);

                            // TODO: fail queued RPCs, backoff and retry
                            unimplemented!()
                        }
                    }
                },
                Connected(ref mut conn) => {

                    // Send all queued messages. The result of the loop is ok if either the
                    // connection has no more send capacity, or there are no more messages to send.
                    // If any message fails to send, the result of the loop is the error.
                    let send_result: Result<(), ()> = loop {
                        match conn.poll_ready() {

                            // The connection has capacity to send an RPC.
                            Ok(Async::Ready(_)) => {

                                // Take an RPC from the buffer, or from the receiver.
                                let rpc = buffer.pop_front()
                                                .map(|rpc| Ok(Async::Ready(Some(rpc))))
                                                .unwrap_or_else(|| receiver.poll());

                                match rpc {
                                    // Attempt to send the RPC.
                                    Ok(Async::Ready(Some(request))) => match conn.send(request) {
                                        Ok(()) => continue,
                                        error => break error,
                                    },

                                    // All proxy senders are dropped. If there are no RPCs in
                                    // flight, then shut down.
                                    Ok(Async::Ready(None)) => if conn.rpcs_in_flight() == 0 {
                                        return Ok(Async::Ready(()));
                                    } else {
                                        break Ok(());
                                    },

                                    // No messages to send.
                                    Ok(Async::NotReady) => break Ok(()),
                                    Err(()) => unreachable!(),
                                }
                            },

                            // The connection has no remaining capacity.
                            Ok(Async::NotReady) => break Ok(()),

                            // The connection is shutdown.
                            Err(()) => break Err(()),
                        }
                    };

                    match send_result.and_then(|_| conn.poll()) {
                        Ok(_) => return Ok(Async::NotReady),
                        Err(_) => {
                            buffer.extend(conn.in_flight_rpcs().drain().map(|(_, rpc)| rpc));
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
        debug.field("hostports", &format_args!("{}", &self.hostports.iter().format(",")));
        debug.field("core", &self.handle.id());
        match self.connection_state {
            ConnectionState::Quiesced => debug.field("state", &self.connection_state),
            ConnectionState::Connecting(_) => debug.field("state", &self.connection_state),
            ConnectionState::Connected(ref connection) => debug.field("connection", connection),
        };
        debug.finish()
    }
}
