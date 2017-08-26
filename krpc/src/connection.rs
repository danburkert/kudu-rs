use std::cmp;
use std::fmt;
use std::io::Write;
use std::io;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use fnv::FnvHashMap;
use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};

use Error;
use error::RpcError;
use Rpc;
use transport::Transport;

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

pub struct Connection<S> {

    /// The connection options.
    options: ConnectionOptions,

    /// The transport wraps the underlying TCP stream.
    transport: Transport,

    /// RPCs which have been sent and are awaiting response, indexed by call ID.
    in_flight_rpcs: FnvHashMap<i32, InFlightRpc<S>>,

    /// RPCs which have failed, but haven't yet been returned by `poll()`.
    failed_rpcs: Vec<Rpc<S>>,

    /// Maximum number of RPCs to allow to be in flight at once. `throttle` is halved every time
    /// `Connection::throttle` is called (which should be in response to a tablet server
    /// `Throttled` error). `throttle` is increased by one for every successful RPC up to a maximum
    /// of `ConnectionOptions::max_rpcs_in_flight`.
    throttle: u32,

    /// The next call id.
    next_call_id: i32,

    is_shutdown: bool,
}

impl <S> Connection <S> {

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.options.max_rpcs_in_flight) / 2;
    }

    fn next_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow. The server will probably complain about this, but there
        // isn't much we can do other than shut down the connection anyway.
        let next = self.next_call_id.checked_add(1).unwrap_or(0);
        self.next_call_id = next;
        next
    }

    /*
    fn fail_rpc(&mut self, mut rpc: Rpc, error: Error) {
        rpc.fail(error);
        self.failed_rpcs.push(rpc);
    }

    /// Shutdown the connection. In flight RPCs will be failed with an IO error and
    /// returned in subsequent calls to `poll`.
    pub fn shutdown(&mut self) {
        self.is_shutdown = true;
        let _ = self.stream.shutdown(Shutdown::Both);
        self.failed_rpcs.extend(self.recv_queue.drain().map(|kv| {
            let mut rpc = kv.1;
            rpc.fail(Error::from(io::Error::new(io::ErrorKind::ConnectionAborted,
                                                "connection shutdown")));
            rpc
        }));
    }
    */
}

impl <S> Sink for Connection<S> {
    type SinkItem = Rpc;
    type SinkError = Error;

    /// Attempt to send an RPC to a remote server. The RPC may only be buffered by the connection;
    /// the caller should call `poll_complete` after calling this method one or more times to
    /// ensure that the RPC is dispatched to the remote server.
    ///
    /// Returns:
    ///     * `Ok(AsyncSink::Ready)`
    ///         The RPC has been successfully buffered.
    ///     * `Ok(AsyncSink::NotReady)`
    ///         The connection has insufficient capacity to send an additional RPC.
    ///     * `Err(..)`
    ///         The connection has been shutdown and no further RPCs may be sent. The RPC can be
    ///         retrieved via `poll`.
    fn start_send(&mut self, rpc: Rpc) -> StartSend<Rpc, Error> {
        trace!("{:?}: start_send", self);
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_complete", self);
        unimplemented!()
    }
}

impl <S> Stream for Connection<S> {
    type Item = Rpc;
    type Error = Error;

    /// Reads an RPC messsage from the socket.
    ///
    /// Returns:
    ///     * `Ok(Async::Ready(Some(rpc))`
    ///         a completed RPC is ready.
    ///     * `Ok(Async::Ready(None))`
    ///         no RPCs are currently in flight.
    ///
    ///         TODO: should Ready(None) only be returned when the connection is shutdown?
    ///     * `Ok(Async::NotReady)`
    ///         RPC(s) are in flight, but not currently available.
    fn poll(&mut self) -> Poll<Option<Rpc<S>>, Error> {
        trace!("{:?}: poll", self);

        if let Some(failed) = self.failed_rpcs.pop() {
            return Ok(Async::Ready(Some(failed)));
        }

        unimplemented!()
    }
}

impl <S> fmt::Debug for Connection<S>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Connection");

         .field("addr", 
        match self.stream.peer_addr() {
            Ok(addr) => write!(f, "Connection {{ addr: {}, in-flight: {}, buf (tx/rx): {}/{} }}",
                               addr, self.recv_queue.len(), self.write_buf.len(), self.read_buf.len()),
            Err(error) => write!(f, "Connection {{ addr: {}, in-flight: {}, buf (tx/rx): {}/{} }}",
                                 error, self.recv_queue.len(), self.write_buf.len(), self.read_buf.len()),
        }
    }
}
