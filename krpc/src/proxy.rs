use std::collections::VecDeque;
use std::fmt;
use std::time::Instant;

use futures::future;
use futures::sync::{mpsc, oneshot};
use futures::{Async, AsyncSink, Future, Poll, Sink, Stream};
use itertools::Itertools;
use prost::Message;
use tokio;

use connection::Connection;
use connector::Connector;
use Call;
use Error;
use HostPort;
use Options;
use Rpc;
use RpcFuture;

/// `Proxy` is a handle to a remote server which allows clients to send and receive RPCs.
#[derive(Clone)]
pub struct Proxy {
    sender: mpsc::Sender<Rpc>,
}

impl Proxy {
    pub fn new(
        hostports: Box<[HostPort]>,
        options: Options,
    ) -> impl Future<Item = Proxy, Error = Error> {
        future::lazy(|| Ok(Proxy::spawn(hostports, options)))
    }

    pub fn spawn(hostports: Box<[HostPort]>, options: Options) -> Proxy {
        let (sender, receiver) = mpsc::channel(options.max_rpcs_in_flight as usize);
        tokio::spawn(ProxyTask {
            hostports,
            options,
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
    pub fn send<Req, Resp>(&mut self, call: Call<Req, Resp>) -> RpcFuture<Resp>
    where
        Req: Message + 'static,
        Resp: Message + Default,
    {
        let (completer, receiver) = oneshot::channel();
        let rpc = Rpc {
            service: call.service,
            method: call.method,
            required_feature_flags: call.required_feature_flags,
            timestamp: Instant::now(),
            deadline: call.deadline,
            request: call.request,
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

/// `ConnectionState` is a state-machine which drives the `ProxyTask`.
///
/// ## States
///
/// - Quiesced
///     The initial state. The `Quiesced` state is used when the proxy is not connected to the
///     remote server. When the task has buffered RPCs, or when an RPC is received from the
///     channel, the task transitions to `Connecting`.
/// - Connecting
///     The task is attempting to connect to the remote server. No RPCs are received from the RPC
///     channel while connecting, so senders will (eventually) encounter backpressure.
/// - `Connected`
///     RPCs are received from the channel and forwarded to the connection, as connection capacity
///     permits. If a fatal RPC error occurs, all non-failed in-flight RPCs are buffered
///     internally, and the task transitions to `Quiesced`.
/// - `Failed`
///     When connecting to the remote server fails, the task is put into the `Failed` state. All
///     buffered RPCs and RPCs received from the channel are immediately failed. After waiting for
///     a backoff timeout period, the task will transition back to `Quiesced`.
///
/// ## State Diagram
///
/// ```
///                ┌──────────────┐
///                │              │
///           ┌────│    Failed    │◀────┐
///           │    │              │     │
///           e    └──────────────┘     c
///           │                         │
///           ▼                         │
///   ┌──────────────┐          ┌──────────────┐
///   │              │          │              │
///   │   Quiesced   │──── a ──▶│  Connecting  │
///   │              │          │              │
///   └──────────────┘          └──────────────┘
///           ▲                         │
///           │                         │
///           d    ┌──────────────┐     b
///           │    │              │     │
///           └────│  Connected   │◀────┘
///                │              │
///                └──────────────┘
/// ```
///
/// ## State Transitions
///
/// |   | current state | event            | next state   | actions                          |
/// | - | ------------- | ---------------- | ------------ | -------------------------------- |
/// | a | `Quiesced`    | RPC enqueued     | `Connecting` | create a new `Connector`         |
/// | b | `Connecting`  | connect succeeds | `Connected`  |                                  |
/// | c | `Connecting`  | connect fails    | `Failed`     | fail buffered RPCs               |
/// | d | `Connected`   | fatal RPC error  | `Quiesced`   | buffer non-failed in-flight RPCs |
/// | e | `Failed`      | backoff elapsed  | `Quiesced`   |                                  |
///
/// TODO: add the Failed state.
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

/// `ProxyTask` is a task which manages the lifecycle of TCP connections to a remote server.
///
///
/// `ProxyTask` handles reconnections, buffering, will automatically attempt to reconnect to the
/// remote server, buffer RPCs while reconnecting (when appropriate).
struct ProxyTask {
    hostports: Box<[HostPort]>,
    options: Options,
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
        let ProxyTask {
            ref hostports,
            ref options,
            ref mut receiver,
            ref mut connection_state,
            ref mut buffer,
        } = *self;

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

                    Connecting(Connector::connect(hostports, options.clone()))
                }
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
                }
                Connected(ref mut conn) => {
                    // Send all buffered and queued RPCs. The result of the loop is ok if either
                    // the connection has no more send capacity, or there are no more messages to
                    // send.  If any message fails to send, the result of the loop is the error.
                    let send_result: Result<(), ()> = loop {
                        match conn.poll_ready() {
                            // The connection has capacity to send an RPC.
                            Ok(Async::Ready(_)) => {
                                // Take an RPC from the buffer or queue.
                                let rpc = buffer
                                    .pop_front()
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
                            }

                            // The connection has no remaining capacity.
                            Ok(Async::NotReady) => break Ok(()),

                            // The connection is shutdown.
                            Err(()) => break Err(()),
                        }
                    };

                    // Poll the connection in order to complete in-flight RPCs.
                    match send_result.and_then(|_| conn.poll()) {
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
                                    }),
                            );
                            ConnectionState::Quiesced
                        }
                    }
                }
            };
            *connection_state = state;
        }
    }
}

impl fmt::Debug for ProxyTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("ProxyTask");
        debug.field(
            "hostports",
            &format_args!("{:?}", &self.hostports.iter().format(",")),
        );
        match self.connection_state {
            ConnectionState::Quiesced => debug.field("state", &self.connection_state),
            ConnectionState::Connecting(_) => debug.field("state", &self.connection_state),
            ConnectionState::Connected(ref connection) => debug.field("connection", connection),
        };
        debug.finish()
    }
}
