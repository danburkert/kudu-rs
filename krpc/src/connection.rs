use std::cmp;
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

use ConnectionOptions;
use Error;
use Request;
use Rpc;
use negotiator::Negotiator;
use transport::Transport;

pub struct Connection<S> {

    /// The transport wraps the underlying TCP stream.
    transport: Transport,

    /// RPCs which have been sent and are awaiting response, indexed by call ID.
    in_flight_rpcs: FnvHashMap<i32, Request<S>>,

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
        // Wrap back to 0 on overflow.
        let call_id = self.next_call_id;
        self.next_call_id = self.next_call_id.checked_add(1).unwrap_or(0);
        call_id
    }

    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        // Make sure the transport is ready.
        try_ready!(self.transport.poll_ready());

        // Check that the connection is not throttled.
        if self.in_flight_rpcs.len() < self.throttle as usize {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }

    pub fn send(&mut self, request: Request<S>) -> Result<(), Error> {
        let call_id = self.next_call_id();

        self.transport.send(call_id,
                            request.service,
                            request.method,
                            request.required_feature_flags,
                            &*request.body,
                            request.deadline)?;

        self.in_flight_rpcs.insert(call_id, request);

        Ok(())
    }

    pub fn poll(&mut self) -> Async<Rpc<S>> {
        match self.transport.poll() {
            Ok(Async::Ready((call_id, Ok(response)))) => {
                if let Some(request) = self.in_flight_rpcs.remove(&call_id) {
                    return Async::Ready(request.complete(response))
                }
            },
            Ok(Async::Ready((call_id, Err(error)))) => {
                if let Some(request) = self.in_flight_rpcs.remove(&call_id) {
                    return Async::Ready(request.fail(error))
                }
            },
            Ok(Async::NotReady) => (),
            Err(error) => {
                // The transport is shutdown. Pick a random RPC and fail it.
                if let Some(&call_id) = self.in_flight_rpcs.keys().next() {
                    return Async::Ready(self.in_flight_rpcs.remove(&call_id).unwrap().fail(error));
                }
            },
        }
        Async::NotReady
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
