use std::cmp;
use std::fmt;
use std::net::SocketAddr;
use std::time::Instant;

use fnv::FnvHashMap;
use futures::{
    Async,
    Future,
    Poll,
    Sink,
    StartSend,
    Stream,
};
use tokio::reactor::Handle;

use Error;
use Rpc;
use transport::Transport;
use InFlightRpc;
use ConnectionOptions;
use negotiator::Negotiator;

pub struct Connection<S> {

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
}

impl <S> Connection <S> {

    pub fn connect(addr: SocketAddr,
                   options: ConnectionOptions,
                   deadline: Instant,
                   handle: &Handle) -> Box<Future<Item=Connection<S>, Error=Error>> {
        Box::new(
            Transport::connect(addr, options, handle)
                .map_err(Into::into)
                .and_then(move |transport| {
                    Negotiator::negotiate(transport, deadline)
                })
                .map(|transport| {
                    let throttle = transport.options().max_rpcs_in_flight;
                    Connection {
                        transport,
                        in_flight_rpcs: FnvHashMap::default(),
                        failed_rpcs: Vec::new(),
                        throttle,
                        next_call_id: 0,
                    }
                }))
    }

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.transport.options().max_rpcs_in_flight) / 2;
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
    type SinkItem = Rpc<S>;
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
    fn start_send(&mut self, rpc: Rpc<S>) -> StartSend<Rpc<S>, Error> {
        trace!("{:?}: start_send", self);
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_complete", self);
        unimplemented!()
    }
}

impl <S> Stream for Connection<S> {
    type Item = Rpc<S>;
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
        debug.field("addr", &self.transport.addr());
        debug.field("in-flight", &self.in_flight_rpcs.len());

        let failed_rpcs = self.failed_rpcs.len();
        if failed_rpcs > 0 {
            debug.field("failed-rpcs", &failed_rpcs);
        }

        debug.field("buf (tx/rx)", &format_args!("{}/{}", self.transport.send_buf_len(), self.transport.recv_buf_len()));
        debug.finish()
    }
}
