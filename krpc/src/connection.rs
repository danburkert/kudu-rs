use std::cmp;
use std::fmt;
use std::io;
use std::time::Instant;

use fnv::FnvHashMap;
use futures::{Async, Poll};

use transport::Transport;
use Error;
use Options;
use Rpc;
use RpcError;
use RpcErrorCode;

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

impl Connection {
    /// Creates a new connection from the provided transport. Negotiation must already be
    /// complete.
    pub fn new(transport: Transport, options: &Options) -> Connection {
        let throttle = transport.options().max_rpcs_in_flight;
        Connection {
            transport,
            in_flight_rpcs: FnvHashMap::default(),
            throttle,
            next_call_id: 0,
        }
    }

    /// This is a total hack to allow Proxy to reach in and grab the RPCs.
    /// Would probably be better to return an `impl Iterator<Item=Rpc>`
    pub fn in_flight_rpcs(&mut self) -> &mut FnvHashMap<i32, Rpc> {
        &mut self.in_flight_rpcs
    }

    fn throttle(&mut self) {
        self.throttle = cmp::max(self.throttle / 2, 1);
    }

    fn unthrottle(&mut self) {
        self.throttle = cmp::min(
            self.throttle + 1,
            self.transport.options().max_rpcs_in_flight,
        );
    }

    fn next_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow.
        let call_id = self.next_call_id;
        self.next_call_id = self.next_call_id.checked_add(1).unwrap_or(0);
        call_id
    }

    fn log_error(&self, error: Error) -> Result<(), ()> {
        match error {
            Error::Io(ref error) if error.kind() == io::ErrorKind::UnexpectedEof => {
                info!("{:?}: shutdown by remote", self);
            }
            error => {
                warn!("{:?}: error: {}", self, error);
            }
        }
        Err(())
    }

    /// Ensure that there is capacity to send an RPC. If an error is returned, the connection must
    /// be dropped.
    pub fn poll_ready(&mut self, now: Instant) -> Poll<(), ()> {
        trace!("{:?}: poll_ready", self);

        // Make sure the transport is ready.
        try_ready!(self.transport.poll_ready().map_err(|error| {
            let _ = self.log_error(error);
        }));

        // Check that the connection is not throttled.
        if self.in_flight_rpcs.len() < self.throttle as usize {
            Ok(Async::Ready(()))
        } else {
            // Try to clear the in-flight RPCs.
            self.poll(now)?;
            if self.in_flight_rpcs.len() < self.throttle as usize {
                Ok(Async::Ready(()))
            } else {
                Ok(Async::NotReady)
            }
        }
    }

    /// Send an RPC. If an error is returned, the connection must be dropped.
    pub fn send(&mut self, rpc: Rpc, now: Instant) -> Result<(), ()> {
        trace!("{:?}: send: {:?}", self, rpc);
        if rpc.deadline < now {
            rpc.fail(Error::TimedOut);
            return Ok(());
        }
        if rpc.is_canceled() {
            return Ok(());
        }

        let call_id = self.next_call_id();
        let send = self.transport.send(
            call_id,
            rpc.service,
            rpc.method,
            rpc.required_feature_flags,
            &*rpc.request,
            Some(rpc.deadline - now),
        );

        // Regardless of the result of the send, add the RPC to the in-flight queue. The upstream
        // Proxy will remove it and retry it if the send failed.
        self.in_flight_rpcs.insert(call_id, rpc);
        match send {
            Ok(()) => Ok(()),
            Err(error) => self.log_error(error),
        }
    }

    /// Poll the connection, completing in-flight RPCs if possible.
    /// If an error is returned, the connection must be dropped.
    pub fn poll(&mut self, now: Instant) -> Result<(), ()> {
        trace!("{:?}: poll", self);
        loop {
            match self.transport.poll() {
                Ok(Async::Ready((call_id, Ok((body, sidecars))))) => {
                    self.unthrottle();
                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        rpc.complete(body, sidecars);
                    }
                }
                Ok(Async::Ready((call_id, Err(error)))) => {
                    if let Error::Rpc(RpcError { code, .. }) = error {
                        if code == RpcErrorCode::ErrorServerTooBusy {
                            self.throttle()
                        }
                    }

                    // Ensure that fatal errors shut down the connection.
                    let is_fatal = error.is_fatal();

                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        error!("{:?}: {:?} failed: {}", self, rpc, error);
                        rpc.fail(error);
                    } else {
                        error!("{:?}: RPC failed: {}", self, error);
                    }

                    if is_fatal {
                        return Err(());
                    }
                }
                Ok(Async::NotReady) => return Ok(()),
                Err(error) => return self.log_error(error),
            }
        }
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Connection");
        debug.field("addr", &format_args!("{}", &self.transport.addr()));
        debug.field("in-flight", &self.in_flight_rpcs.len());
        debug.field(
            "buf (tx/rx)",
            &format_args!(
                "{}/{}",
                self.transport.send_buf_len(),
                self.transport.recv_buf_len()
            ),
        );
        debug.finish()
    }
}
