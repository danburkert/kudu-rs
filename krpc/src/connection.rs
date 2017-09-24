use std::cmp;
use std::fmt;
use std::io;
use std::time::Instant;

use fnv::FnvHashMap;
use futures::{
    Async,
    Poll,
};
use tacho;

use Error;
use Options;
use Rpc;
use RpcError;
use RpcErrorCode;
use duration_to_us;
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

    metrics: Option<Metrics>,
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
            metrics: options.scope.as_ref().cloned().map(Metrics::new),
        }
    }

    /// Fails all in-flight RPCs.
    fn shutdown(&mut self, now: Instant) {
        for (_, rpc) in &mut self.in_flight_rpcs.drain() {
            if rpc.is_canceled() {
                if let Some(ref metrics) = self.metrics {
                    metrics.canceled_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                }
            } else if rpc.is_timed_out(now) {
                if let Some(ref metrics) = self.metrics {
                    metrics.timed_out_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                }
                rpc.fail(Error::TimedOut);
            } else {
                if let Some(ref metrics) = self.metrics {
                    metrics.failed_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                }
                rpc.fail(Error::Io(io::Error::from(io::ErrorKind::UnexpectedEof)));
            }
        }
    }

    fn throttle(&mut self) {
        self.throttle = cmp::max(self.throttle / 2, 1);
    }

    fn unthrottle(&mut self) {
        self.throttle = cmp::min(self.throttle + 1, self.transport.options().max_rpcs_in_flight);
    }

    fn next_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow.
        let call_id = self.next_call_id;
        self.next_call_id = self.next_call_id.checked_add(1).unwrap_or(0);
        call_id
    }

    /// Ensure that there is capacity to send an RPC. If an error is returned, the connection must
    /// be dropped.
    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_ready", self);
        // Make sure the transport is ready.
        try_ready!(self.transport.poll_ready().map_err(|error| {
            self.shutdown(Instant::now());
            error
        }));

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

    /// Send an RPC. If an error is returned, the connection must be dropped.
    pub fn send(&mut self, rpc: Rpc) -> Result<(), Error> {
        trace!("{:?}: send: {:?}", self, rpc.request);
        let now = Instant::now();
        if rpc.request.deadline < now {
            rpc.fail(Error::TimedOut);
            return Ok(());
        }
        if rpc.is_canceled() {
            return Ok(());
        }

        let call_id = self.next_call_id();
        let send = self.transport
                       .send(call_id,
                             rpc.request.service,
                             rpc.request.method,
                             rpc.request.required_feature_flags,
                             &*rpc.request.body,
                             Some(rpc.request.deadline - now));

        match send {
            Ok(()) => {
                self.in_flight_rpcs.insert(call_id, rpc);
                Ok(())
            },
            Err(error) => {
                self.shutdown(now);
                if let Some(ref metrics) = self.metrics {
                    metrics.failed_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                }
                rpc.fail(Error::Io(io::Error::from(io::ErrorKind::UnexpectedEof)));
                Err(error)
            }
        }
    }

    /// Poll the connection, completing in-flight RPCs if possible.
    /// If an error is returned, the connection must be dropped.
    pub fn poll(&mut self) -> Result<(), Error> {
        trace!("{:?}: poll", self);
        let now = Instant::now();
        loop {
            match self.transport.poll() {
                Ok(Async::Ready((call_id, Ok((body, sidecars))))) => {
                    self.unthrottle();
                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        if let Some(ref metrics) = self.metrics {
                            metrics.successful_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                        }
                        rpc.complete(body, sidecars);
                    }
                },
                Ok(Async::Ready((call_id, Err(error)))) => {
                    if let Error::Rpc(RpcError { code, .. }) = error {
                        if code == RpcErrorCode::ErrorServerTooBusy {
                            self.throttle()
                        }
                    }

                    // Ensure that fatal errors shut down the connection.
                    let fatal_error = if error.is_fatal() { Some(error.clone()) } else { None };

                    if let Some(rpc) = self.in_flight_rpcs.remove(&call_id) {
                        if let Some(ref metrics) = self.metrics {
                            metrics.failed_rpcs.add(duration_to_us(now - rpc.request.timestamp));
                        }
                        rpc.fail(error);
                    }

                    if let Some(error) = fatal_error {
                        self.shutdown(now);
                        return Err(error);
                    }
                },
                Ok(Async::NotReady) => return Ok(()),
                Err(error) => {
                    self.shutdown(now);
                    return Err(error);
                },
            }
        }
    }

    pub fn rpcs_in_flight(&self) -> usize {
        self.in_flight_rpcs.len()
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Connection");
        debug.field("addr", &format_args!("{}", &self.transport.addr()));
        debug.field("in-flight", &self.in_flight_rpcs.len());
        debug.field("buf (tx/rx)", &format_args!("{}/{}",
                                                 self.transport.send_buf_len(),
                                                 self.transport.recv_buf_len()));
        debug.finish()
    }
}

struct Metrics {

    // in_flight_rpcs: tacho::Gauge,
    // in_flight_capacity: tacho::Gauge,

    /// Successful RPC statistics.
    successful_rpcs: tacho::Stat,

    /// Failed RPC statistics.
    failed_rpcs: tacho::Stat,

    /// Timed-out RPC statistics.
    timed_out_rpcs: tacho::Stat,

    /// Canceled RPC statistics.
    canceled_rpcs: tacho::Stat,
}

impl Metrics {
    fn new(scope: tacho::Scope) -> Metrics {
        let successful_rpcs = scope.clone().labeled("result", "success").stat("rpc");
        let failed_rpcs = scope.clone().labeled("result", "failed").stat("rpc");
        let timed_out_rpcs = scope.clone().labeled("result", "timed_out").stat("rpc");
        let canceled_rpcs = scope.labeled("result", "canceled").stat("rpc");

        Metrics {
            successful_rpcs: successful_rpcs,
            failed_rpcs: failed_rpcs,
            timed_out_rpcs: timed_out_rpcs,
            canceled_rpcs: canceled_rpcs,
        }
    }
}
