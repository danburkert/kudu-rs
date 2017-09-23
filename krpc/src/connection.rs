use std::cmp;
use std::convert::Into;
use std::fmt;
use std::net::SocketAddr;
use std::time::Instant;

use fnv::FnvHashMap;
use futures::{
    Async,
    Future,
    Poll,
};
use tokio::reactor::Handle;

use Options;
use Error;
use Rpc;
use negotiator::Negotiator;
use transport::Transport;

/// `Connection` manages in-flight RPCs and throttling on a single KRPC connection.
///
/// The connection wraps a transport. When the transport is shutdown, the connection should no
/// longer be used. Connection shutdown is indicated by a fatal error being returned from
/// `poll_ready()`, `send()`, or `poll()`.
pub(crate) struct Connection {

    /// The transport wraps the underlying TCP stream.
    transport: Transport,

    /// RPCs which have been sent and are awaiting response, indexed by call ID.
    in_flight_rpcs: FnvHashMap<i32, Rpc>,

    /// Maximum number of RPCs to allow to be in flight at once. `throttle` is halved every time
    /// `Connection::throttle` is called (which should be in response to a tablet server
    /// `Throttled` error). `throttle` is increased by one for every successful RPC up to a maximum
    /// of `Options::max_rpcs_in_flight`.
    throttle: u32,

    /// The next call id.
    next_call_id: i32,
}

pub(crate) type ConnectionNew = Box<Future<Item=Connection, Error=Error>>;

impl Connection {

    /// Creates a new connection from the provided transport. Negotiation must already be
    /// complete.
    pub fn new(transport: Transport) -> Connection {
        let throttle = transport.options().max_rpcs_in_flight;
        Connection {
            transport,
            in_flight_rpcs: FnvHashMap::default(),
            throttle,
            next_call_id: 0,
        }
    }

    /// Returns a future which will yield a new transport.
    pub fn connect(addr: SocketAddr,
                   options: Options,
                   handle: &Handle) -> ConnectionNew {

        fn new(transport: Transport) -> Connection {
            let throttle = transport.options().max_rpcs_in_flight;
            Connection {
                transport,
                in_flight_rpcs: FnvHashMap::default(),
                throttle,
                next_call_id: 0,
            }
        }

        Box::new(Transport::connect(addr, options, handle)
                           .map_err(Into::into)
                           .and_then(Negotiator::negotiate)
                           .map(new))
    }

    fn shutdown(&mut self, error: Error) -> Error {
        for (_, rpc) in &mut self.in_flight_rpcs.drain() {
            rpc.fail(error.clone());
        }
        error
    }

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.transport.options().max_rpcs_in_flight) / 2;
    }

    fn next_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow.
        let call_id = self.next_call_id;
        self.next_call_id = self.next_call_id.checked_add(1).unwrap_or(0);
        call_id
    }

    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_ready", self);
        // Make sure the transport is ready.
        try_ready!(self.transport.poll_ready().map_err(|error| self.shutdown(error)));

        // Check that the connection is not throttled.
        if self.in_flight_rpcs.len() < self.throttle as usize {
            Ok(Async::Ready(()))
        } else {
            // Try to clear the in-flight RPCs.
            self.poll()?;
            if self.in_flight_rpcs.len() < self.throttle as usize {
                Ok(Async::Ready(()))
            } else {
                Ok(Async::NotReady)
            }
        }
    }

    pub fn send(&mut self, rpc: Rpc) -> Result<(), Error> {
        trace!("{:?}: send: {:?}", self, rpc);
        let now = Instant::now();
        if rpc.request.deadline < now {
            rpc.fail(Error::TimedOut);
            return Ok(());
        }
        if rpc.is_canceled() {
            return Ok(());
        }

        let call_id = self.next_call_id();
        self.transport
            .send(call_id,
                  rpc.request.service,
                  rpc.request.method,
                  rpc.request.required_feature_flags,
                  &*rpc.request.body,
                  Some(rpc.request.deadline - now))
            .map_err(|error| self.shutdown(error))?;
        self.in_flight_rpcs.insert(call_id, rpc);
        Ok(())
    }

    pub fn poll(&mut self) -> Result<(), Error> {
        trace!("{:?}: poll", self);
        loop {
            match self.transport.poll() {
                Ok(Async::Ready((call_id, Ok((body, sidecars))))) => {
                    // TODO: unthrottle?
                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        rpc.complete(body, sidecars);
                    }
                },
                Ok(Async::Ready((call_id, Err(error)))) => {
                    // TODO: throttle.
                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        rpc.fail(error);
                    }
                },
                Ok(Async::NotReady) => return Ok(()),
                Err(error) => {
                    self.shutdown(error.clone());
                    return Err(error);
                },
            }
        }
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Connection");
        debug.field("addr", &self.transport.addr());
        debug.field("in-flight", &self.in_flight_rpcs.len());
        debug.field("buf (tx/rx)", &format_args!("{}/{}",
                                                 self.transport.send_buf_len(),
                                                 self.transport.recv_buf_len()));
        debug.finish()
    }
}
