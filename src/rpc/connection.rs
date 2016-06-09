use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, ErrorKind, Write};
use std::mem;
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::Instant;

use backoff::Backoff;
use kudu_pb::rpc_header::{SaslMessagePB_SaslState as SaslState};
use kudu_pb::rpc_header;
use rpc::messenger::{Loop, TimeoutKind};
use rpc::Rpc;
use util::duration_to_ms;
use Error;
use Result;
use error::RpcError;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use mio::{
    EventSet,
    PollOpt,
    Timeout,
    Token,
};
use mio::tcp::TcpStream;
use protobuf::{parse_length_delimited_from, Clear, CodedInputStream, Message};
use protobuf::rt::ProtobufVarint;
use netbuf::Buf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionOptions {
    /// Whether to disable Nagle's algorithm.
    ///
    /// Defaults to true.
    pub nodelay: bool,

    /// Maximum number of RPCs to queue in the connection.
    ///
    /// When the queue is full, additional attempts to send RPCs will immediately fail.
    ///
    /// Defaults to 256.
    pub rpc_queue_len: u32,

    /// Initial time in milliseconds to wait after an error before attempting to reconnect to the
    /// server.
    ///
    /// Defaults to 8ms.
    pub backoff_initial: u32,

    /// Maximum time in milliseconds to wait after an error before attempting to reconnect to the
    /// server.
    ///
    /// Defaults to 4 seconds.
    pub backoff_max: u32,

    /// Maximum time in milliseconds to keep an idle connection.
    ///
    /// If the connection is idle for longer than this period (no queued RPCs), then it will be
    /// shut down.
    ///
    /// Defaults to 1 minute.
    pub idle_timeout: u32,

    /// Maxmimum time in milliseconds to allow for initializing a new connection to a server.
    ///
    /// If the connection is unable to succefully connect to the server in this amount of time, it
    /// will be reset and another attempt will be made after a backoff period.
    pub negotiation_timeout: u32,

    /// Maximum allowable message length.
    pub max_message_length: u32,
}

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            nodelay: true,
            rpc_queue_len: 256,
            backoff_initial: 8,
            backoff_max: 4096,
            idle_timeout: 60 * 1000,
            negotiation_timeout: 4096,
            max_message_length: 5 * 1024 * 1024,
        }
    }
}

/// The state of a connection to a Kudu server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {

    /// The connection is initiating.
    Initiating,

    /// The connection is active.
    Connected,

    /// The connection has been shut down due to an error. It will be reestablished after a backoff
    /// period.
    Reset,
}

/// `Connection` is a state machine that manages a single client connection to a Kudu server.
///
/// A connection internally manages a TCP connection to the server, as well as a FIFO queue of
/// in-flight RPCs.
///
/// If an error occurs that requires tearing down the connection to the server (e.g. a
/// [de]serialization failure, or a fatal error response), the connection will automatically
/// attempt to reconnect after waiting for a backoff period. Consecutive retries will be delayed
/// with an exponentially increasing backoff with jitter.
///
/// If at any point the connection is idle for longer than the idle timeout, or if the connection
/// is reset and does not have any queued RPCs, then it will automatically be shut down.
pub struct Connection {
    /// The connection options.
    options: Rc<ConnectionOptions>,
    /// The current connection state.
    state: ConnectionState,
    /// The connection's TCP stream. `None` if the connection is in the `Reset` state.
    stream: Option<TcpStream>,
    /// The address of the remote Kudu server.
    addr: SocketAddr,
    /// Queue of RPCs to send.
    send_queue: VecDeque<Rpc>,
    /// RPCs which have been sent and are awaiting response.
    recv_queue: HashMap<i32, Rpc>,
    /// RPC request header, kept internally to reduce memory allocations.
    request_header: rpc_header::RequestHeader,
    /// RPC response header, kept internally to reduce memory allocations.
    response_header: rpc_header::ResponseHeader,
    /// Byte buffer holding the next incoming response.
    recv_buf: Buf,
    /// Byte buffer holding the next outgoing request.
    send_buf: Buf,
    /// The call ID of the next outbound request.
    next_call_id: i32,

    /// Backoff tracker.
    backoff: Backoff,

    /// The registered RPC timeout, and its deadline.
    ///
    /// Active while there are queued RPCs.
    rpc_timeout: Option<(Instant, Timeout)>,

    /// The registered state timeout.
    ///
    /// Inactive if the connection is connected and there are queued RPCs.
    state_timeout: Option<Timeout>,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection {{ state: {:?}, addr: {}, queue (tx/rx): {}/{}, buf (tx/rx): {}/{} }}",
               self.state, self.addr, self.send_queue.len(), self.recv_queue.len(),
               self.send_buf.len(), self.recv_buf.len())
    }
}

impl Connection {

    /// Creates a new connection.
    ///
    /// The connection automatically attempts to connect to the remote server.
    pub fn new(event_loop: &mut Loop,
               token: Token,
               addr: SocketAddr,
               options: Rc<ConnectionOptions>)
               -> Connection {
        let backoff = Backoff::with_duration_range(options.backoff_initial, options.backoff_max);
        let mut connection = Connection {
            options: options,
            state: ConnectionState::Initiating,
            stream: None,
            addr: addr,
            send_queue: VecDeque::new(),
            recv_queue: HashMap::new(),
            request_header: rpc_header::RequestHeader::new(),
            response_header: rpc_header::ResponseHeader::new(),
            recv_buf: Buf::new(),
            send_buf: Buf::new(),
            next_call_id: 0,
            backoff: backoff,
            rpc_timeout: None,
            state_timeout: None,
        };
        connection.connect(event_loop, token);
        connection
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    /// Notifies the connection of event readiness.
    pub fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        fn inner(cxn: &mut Connection, event_loop: &mut Loop, token: Token, events: EventSet) -> Result<()> {
            match cxn.state {
                ConnectionState::Initiating => {
                    if events.is_readable() {
                        assert!(!events.is_writable());
                        assert!(cxn.send_buf.is_empty());
                        try!(cxn.recv(event_loop, token))
                    } else if events.is_writable() {
                        assert!(!events.is_readable());
                        assert!(cxn.recv_buf.is_empty());
                        try!(cxn.flush());
                    }
                },
                ConnectionState::Connected => {
                    if events.is_readable() {
                        try!(cxn.recv(event_loop, token))
                    } else if events.is_writable() {
                        try!(cxn.send())
                    }
                },
                _ => unreachable!("{:?}: unexpected ready, events: {:?}", cxn, events),
            };
            Ok(())
        }

        debug!("{:?}: ready; events: {:?}", self, events);
        if events.is_hup() || events.is_error() {
            self.reset(event_loop, token);
        } else {
            inner(self, event_loop, token, events).and_then(|_| self.reregister(event_loop, token))
                                                  .unwrap_or_else(|error| {
                                                      info!("{:?} error: {}", self, error);
                                                      self.reset(event_loop, token)
                                                  })
        }
    }

    /// Send an RPC to the connected Kudu server.
    pub fn send_rpc(&mut self, event_loop: &mut Loop, token: Token, rpc: Rpc) {
        trace!("{:?}: queueing rpc: {:?}", self, rpc);

        let queue_len = self.send_queue.len() + self.recv_queue.len();
        if rpc.cancelled() {
            trace!("{:?}: cancelling {:?}", self, rpc);
            return rpc.fail(Error::Cancelled);
        } else if queue_len > self.options.rpc_queue_len as usize {
            debug!("{:?}: send queue full; failing {:?}", self, rpc);
            return rpc.fail(Error::Backoff);
        } else {
            // If the RPC's deadline is before the currently scheduled timeout, then reset the
            // timeout to the earlier time.
            if self.rpc_timeout.map(|(deadline, _)| rpc.deadline < deadline).unwrap_or(true) {
                let now = Instant::now();
                if rpc.timed_out(now) {
                    trace!("{:?}: timing out {:?}", self, rpc);
                    return rpc.fail(Error::TimedOut);
                } else {
                    self.clear_rpc_timeout(event_loop);
                    let timeout = duration_to_ms(&rpc.deadline.duration_since(now));
                    trace!("{:?}: setting new RPC timeout: {:?}ms", self, timeout);
                    self.rpc_timeout = Some((rpc.deadline,
                                             event_loop.timeout_ms((TimeoutKind::ConnectionRpc, token), timeout).unwrap()));
                }
            }

            self.send_queue.push_back(rpc);
            // If this is the only message in the queue, then optimistically try to write it to the
            // socket.
            if self.state == ConnectionState::Connected && queue_len == 0 {
                self.send()
                    .and_then(|_| self.reregister(event_loop, token))
                    .unwrap_or_else(|error| {
                        info!("{:?} error sending RPC: {}", self, error);
                        self.reset(event_loop, token)
                    });
            }
        }
    }

    /// Notifies the connection that the timeout has fired. Returns `true` if the connection should
    /// be closed, or `false` if it should continue.
    pub fn timeout(&mut self, event_loop: &mut Loop, token: Token, kind: TimeoutKind) -> bool {
        trace!("{:?}: {:?} timeout", self, kind);

        self.short_circuit_rpcs(event_loop, token);
        let queue_len = self.queue_len();
        if queue_len == 0 && self.state != ConnectionState::Connected { return true; }
        if kind == TimeoutKind::ConnectionState {
            // If we don't have any active RPCs, then close the connection.
            if queue_len == 0 { return true; }

            match self.state {
                ConnectionState::Initiating => {
                    info!("{:?} negotiation timed out", self);
                    self.reset(event_loop, token)
                },
                ConnectionState::Reset => self.connect(event_loop, token),
                ConnectionState::Connected => unreachable!("idle timeout fired with active RPCs"),
            }
        };

        false
    }

    /// Fails all inflight RPCs which are timed out, cancels all inflight RPCs which have
    /// cancellation requests, and resets the active RPC timeout.
    ///
    /// There is a balance between constantly calling this in the hope that there are cancelled
    /// RPCs (e.g. on every `ready` event), and the overhead that would require. Right now we
    /// settle for calling it whenever an RPC timeout fires.
    fn short_circuit_rpcs(&mut self, event_loop: &mut Loop, token: Token) {
        // TODO: there is likely a way to do this in O(n) without reallocating both queues.

        let now = Instant::now();
        let mut next_deadline = None;
        let mut send_queue = VecDeque::with_capacity(self.send_queue.len());
        for rpc in self.send_queue.drain(..) {
            if rpc.cancelled() {
                trace!("cancelling {:?}", rpc);
                rpc.fail(Error::Cancelled);
            } else if rpc.timed_out(now) {
                trace!("now: {:?}, timing out {:?}", now, rpc);
                rpc.fail(Error::TimedOut);
            } else {
                next_deadline = Some(next_deadline.map_or(rpc.deadline, |t| cmp::min(t, rpc.deadline)));
                send_queue.push_back(rpc);
            }
        }
        mem::replace(&mut self.send_queue, send_queue);

        let mut recv_queue = HashMap::with_capacity(self.recv_queue.len());
        for (call_id, rpc) in self.recv_queue.drain() {
            if rpc.cancelled() {
                trace!("cancelling {:?}", rpc);
                rpc.fail(Error::Cancelled);
            } else if rpc.timed_out(now) {
                trace!("now: {:?}, timing out {:?}", now, rpc);
                rpc.fail(Error::TimedOut);
            } else {
                next_deadline = Some(next_deadline.map_or(rpc.deadline, |t| cmp::min(t, rpc.deadline)));
                recv_queue.insert(call_id, rpc);
            }
        }
        mem::replace(&mut self.recv_queue, recv_queue);

        self.clear_rpc_timeout(event_loop);
        if let Some(deadline) = next_deadline {
            let timeout = event_loop.timeout_ms((TimeoutKind::ConnectionRpc, token),
                                                duration_to_ms(&deadline.duration_since(now)))
                                    .unwrap();
            self.rpc_timeout = Some((deadline, timeout));
        }
    }

    /// Connects an inactive connection to the server.
    fn connect(&mut self, event_loop: &mut Loop, token: Token) {
        fn inner(cxn: &mut Connection, event_loop: &mut Loop, token: Token) -> Result<()> {
            assert!(cxn.recv_queue.is_empty());
            debug!("{:?}: connecting", cxn);
            let stream = try!(TcpStream::connect(&cxn.addr));
            try!(stream.set_nodelay(cxn.options.nodelay));
            cxn.stream = Some(stream);
            cxn.state = ConnectionState::Initiating;

            let timeout = cxn.options.negotiation_timeout as u64;
            cxn.set_state_timeout(event_loop, token, timeout);

            // Optimistically flush the connection header and SASL negotiation to the TCP socket.
            // Even though the socket has not yet been registered, this will usually succeed
            // because the socket will have sufficient internal buffer space. This sometimes fails
            // on OS X with Not Connected errors, but they are safe to ignore since the flush will
            // be retried after then next writeable event.
            try!(cxn.send_connection_header());
            try!(cxn.send_sasl_negotiate());
            let _ = cxn.flush();
            cxn.register(event_loop, token)
        };
        inner(self, event_loop, token).unwrap_or_else(|error| {
            info!("{:?} unable to connect: {}", self, error);
            self.reset(event_loop, token)
        });
    }

    /// Registers the connection with the event loop in order to enable readiness notifications.
    fn register(&mut self, event_loop: &mut Loop, token: Token) -> Result<()> {
        let event_set = self.event_set();
        let poll_opt = self.poll_opt();
        trace!("{:?}: register event_set: {:?}, poll_opt: {:?}", self, event_set, poll_opt);
        try!(event_loop.register(self.stream.as_mut().unwrap(), token, event_set, poll_opt));
        Ok(())
    }

    /// Reregisters the connection with the event loop in order to enable readiness notifications.
    fn reregister(&mut self, event_loop: &mut Loop, token: Token) -> Result<()> {
        let event_set = self.event_set();
        let poll_opt = self.poll_opt();
        trace!("{:?}: reregister event_set: {:?}, poll_opt: {:?}", self, event_set, poll_opt);
        try!(event_loop.reregister(self.stream.as_mut().unwrap(), token, event_set, poll_opt));
        Ok(())
    }

    /// Resets the connection following an error.
    fn reset(&mut self, event_loop: &mut Loop, token: Token) {
        trace!("{:?}: reset", self);
        self.state = ConnectionState::Reset;
        self.stream.take();
        let recv_buf_len = self.recv_buf.len();
        self.recv_buf.consume(recv_buf_len);
        let send_buf_len = self.send_buf.len();
        self.send_buf.consume(send_buf_len);

        let timeout = self.backoff.next_backoff_ms();
        self.set_state_timeout(event_loop, token, timeout);
    }

    /// Adds the message to the send buffer with connection's request header. Does not flush the
    /// buffer. If an error is returned, the connection should be torn down.
    fn send_message(&mut self, msg: &Message) -> Result<()> {
        trace!("{:?}: send message: {:?}", self, msg);
        let header_len = self.request_header.compute_size();
        let msg_len = msg.compute_size();
        let len = header_len + header_len.len_varint() + msg_len + msg_len.len_varint();
        try!(self.send_buf.write_u32::<BigEndian>(len));
        try!(self.request_header.write_length_delimited_to(&mut self.send_buf));
        try!(msg.write_length_delimited_to(&mut self.send_buf));
        Ok(())
    }

    /// Adds the KRPC connection header to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_connection_header(&mut self) -> Result<()> {
        trace!("{:?}: sending connection header to server", self);
        try!(self.send_buf.write(b"hrpc\x09\0\0"));
        Ok(())
    }

    /// Adds a SASL negotiate message to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_sasl_negotiate(&mut self) -> Result<()> {
        trace!("{:?}: sending SASL NEGOTIATE request to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::NEGOTIATE);
        self.send_message(&msg)
    }

    /// Adds a SASL initiate message to the send buffer. Does not flush the buffer. If an error is
    /// returned, the connection should be torn down.
    fn send_sasl_initiate(&mut self) -> Result<()> {
        trace!("{:?}: sending SASL INITIATE request to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::INITIATE);
        msg.mut_token().extend_from_slice(b"\0user\0");
        let mut auth = rpc_header::SaslMessagePB_SaslAuth::new();
        auth.mut_mechanism().push_str("PLAIN");
        msg.mut_auths().push(auth);
        self.send_message(&msg)
    }

    /// Adds a session context message to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_connection_context(&mut self) -> Result<()> {
        trace!("{:?}: sending connection context to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-3);
        let mut msg = rpc_header::ConnectionContextPB::new();
        msg.mut_user_info().set_effective_user("user".to_string());
        msg.mut_user_info().set_real_user("user".to_string());
        self.send_message(&msg)
    }

    fn handle_sasl_message(&mut self,
                           event_loop: &mut Loop,
                           token: Token,
                           msg: rpc_header::SaslMessagePB)
                           -> Result<()> {
        trace!("{:?}: received SASL {:?} response from server", self, msg.get_state());
        match msg.get_state() {
            SaslState::NEGOTIATE => {
                if msg.get_auths().iter().any(|auth| auth.get_mechanism() == "PLAIN") {
                    try!(self.send_sasl_initiate());
                    try!(self.flush());
                    Ok(())
                } else {
                    panic!("SASL PLAIN authentication not available: {:?}", msg)
                }
            },
            SaslState::SUCCESS => {
                try!(self.send_connection_context());
                self.state = ConnectionState::Connected;
                self.backoff.reset();

                if self.queue_len() == 0 {
                    let timeout = self.options.idle_timeout as u64;
                    self.set_state_timeout(event_loop, token, timeout);
                } else {
                    self.clear_state_timeout(event_loop);
                }

                // Optimistically flush the connection context and send any queued messages. The
                // connection has not necessarily received a writeable event at this point, but it
                // is highly likely that there is space available in the socket's write buffer.
                self.send()
            },
            _ => unreachable!("Unexpected SASL message: {:?}", msg),
        }
    }

    /// Receive messages until no more messages are available on the socket. Should be called when
    /// the connection's socket is readable. If an error is returned, the connection should be torn
    /// down.
    fn recv(&mut self, event_loop: &mut Loop, token: Token) -> Result<()> {
        trace!("{:?}: recv", self);

        let maybe_start_idle_timer =
            self.state == ConnectionState::Connected && self.queue_len() == 0;

        loop {
            // Read, or continue reading, a message from the socket into the receive buffer.
            if self.recv_buf.len() < 4 {
                let needed = 4 - self.recv_buf.len();
                let read = try!(self.read(needed));
                if read < needed { break; }
            }

            let msg_len = BigEndian::read_u32(&self.recv_buf[..4]) as usize;
            if msg_len > self.options.max_message_length as usize {
                return Err(RpcError::invalid_rpc_header(format!(
                           "RPC message is too long; length: {}, max length: {}",
                            msg_len, self.options.max_message_length)).into());
            }
            if self.recv_buf.len() - 4 < msg_len {
                let needed = msg_len + 4 - self.recv_buf.len();
                let read = try!(self.read(needed));
                if read < needed { break; }
            }

            // The whole message has been read
            self.recv_buf.consume(4);

            // Read the response header into self.response_header
            self.response_header.clear();
            let header_len = {
                let mut coded_stream = CodedInputStream::from_bytes(&self.recv_buf[..]);
                try!(coded_stream.merge_message(&mut self.response_header));
                coded_stream.pos() as usize
            };
            self.recv_buf.consume(header_len);

            match self.state {
                ConnectionState::Initiating => {
                    // All SASL messages are required to have call ID -33.
                    debug_assert_eq!(-33, self.response_header.get_call_id());
                    // Only one response should be in flight during SASL negotiation.
                    debug_assert_eq!(msg_len - header_len, self.recv_buf.len());

                    if self.response_header.get_is_error() {
                        // All errors during SASL negotiation should result in tearing down the
                        // connection.
                        return Err(Error::Rpc(RpcError::from(try!(
                                        parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                            &mut CodedInputStream::from_bytes(&self.recv_buf[..]))))));
                    }

                    let msg: rpc_header::SaslMessagePB = try!(parse_length_delimited_from(
                            &mut CodedInputStream::from_bytes(&self.recv_buf[..])));
                    try!(self.handle_sasl_message(event_loop, token, msg));
                },
                ConnectionState::Connected => {
                    trace!("{:?}: received response from server: {:?}", self, self.response_header);
                    if self.response_header.get_is_error() {
                        let error = RpcError::from(try!(
                                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                    &mut CodedInputStream::from_bytes(&self.recv_buf[..]))));
                        // Remove the RPC from the recv queue, and fail it.
                        if let Some(rpc) = self.recv_queue.remove(&self.response_header.get_call_id()) {
                            rpc.fail(Error::Rpc(error.clone()));
                        }
                        // If the message is fatal, then return an error in order to have the
                        // connection torn down.
                        if error.is_fatal() {
                            return Err(Error::Rpc(error.clone()))
                        }
                    } else {
                        // Use the entry API so that the RPC is not removed from the recv queue
                        // if the protobuf decode step fails. Since it isn't removed, it will be
                        // retried when the error is bubbled up to the MessengerHandler.
                        match self.recv_queue.entry(self.response_header.get_call_id()) {
                            Entry::Occupied(mut entry) => {
                                {
                                    try!(CodedInputStream::from_bytes(&self.recv_buf[..])
                                                          .merge_message(&mut *entry.get_mut().response));
                                }

                                if !self.response_header.get_sidecar_offsets().is_empty() {
                                    panic!("sidecar decoding not implemented");
                                }

                                entry.remove().complete();
                            },
                            _ => {
                                // The rpc has already been removed from the recv queue, most
                                // likely due to a timeout or cancellation.
                            }
                        }
                    }
                },
                _ => unreachable!("{:?}: recv"),
            };
            self.recv_buf.consume(msg_len - header_len);
        };

        // Register the idle timer if all queued RPCs have been completed.
        if maybe_start_idle_timer && self.queue_len() == 0 {
            let timeout = self.options.idle_timeout as u64;
            self.set_state_timeout(event_loop, token, timeout)
        }
        Ok(())
    }

    /// Send messages until either there are no more messages to send, or the socket can not accept
    /// any more writes. If an error is returned, the connection should be torn down.
    fn send(&mut self) -> Result<()> {
        trace!("{:?}: send", self);
        assert_eq!(self.state, ConnectionState::Connected);

        let now = Instant::now();
        while !self.send_buf.is_empty() || !self.send_queue.is_empty() {
            while self.send_buf.len() < 4096 && !self.send_queue.is_empty() {
                let rpc = self.send_queue.pop_front().unwrap();

                if rpc.cancelled() {
                    trace!("{:?}: cancelling {:?}", self, rpc);
                    rpc.fail(Error::Cancelled);
                    break;
                } else if rpc.timed_out(now) {
                    trace!("{:?}: timing out {:?}", self, rpc);
                    rpc.fail(Error::TimedOut);
                    break;
                }

                let call_id = self.next_call_id;
                self.next_call_id += 1;

                self.request_header.clear();
                self.request_header.set_call_id(call_id);
                self.request_header.mut_remote_method().mut_service_name().push_str(&rpc.service_name);
                self.request_header.mut_remote_method().mut_method_name().push_str(&rpc.method_name);
                self.request_header.set_timeout_millis(duration_to_ms(&rpc.deadline.duration_since(now)) as u32);
                self.request_header.mut_required_feature_flags().extend_from_slice(&rpc.required_feature_flags);

                trace!("{:?}: sending rpc to server; call ID: {}", self, call_id);
                try!(self.send_message(&*rpc.request));
                self.recv_queue.insert(call_id, rpc);
            }

            if try!(self.flush()) == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Attempts to read at least `min` bytes from the socket into the receive buffer.
    /// Fewer bytes may be read if there is no data available.
    fn read(&mut self, min: usize) -> io::Result<usize> {
        let Connection { ref mut stream, ref mut recv_buf, .. } = *self;
        let mut received = 0;
        while received < min {
            match recv_buf.read_from(stream.as_mut().unwrap()) {
                Ok(amount) => received += amount,
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => return Err(error),
            }
        }
        Ok(received)
    }

    /// Flushes the send buffer to the socket, returning the total number of bytes sent.
    fn flush(&mut self) -> io::Result<usize> {
        trace!("{:?}: flush", self);
        let Connection { ref mut stream, ref mut send_buf, .. } = *self;
        let mut sent = 0;
        while !send_buf.is_empty() {
            match send_buf.write_to(stream.as_mut().unwrap()) {
                Ok(amount) => sent += amount,
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => return Err(error),
            }
        }
        Ok(sent)
    }

    fn queue_len(&self) -> usize {
        self.send_queue.len() + self.recv_queue.len()
    }

    fn set_state_timeout(&mut self, event_loop: &mut Loop, token: Token, timeout: u64) {
        self.clear_state_timeout(event_loop);
        let timeout_handle = event_loop.timeout_ms((TimeoutKind::ConnectionState, token), timeout as u64);
        self.state_timeout = Some(timeout_handle.unwrap());
    }

    fn clear_state_timeout(&mut self, event_loop: &mut Loop) {
        if let Some(timeout) = self.state_timeout.take() {
            event_loop.clear_timeout(timeout);
        }
    }

    fn clear_rpc_timeout(&mut self, event_loop: &mut Loop) {
        if let Some((_, timeout)) = self.rpc_timeout.take() {
            event_loop.clear_timeout(timeout);
        }
    }

    fn poll_opt(&self) -> PollOpt {
        PollOpt::edge() | PollOpt::oneshot()
    }

    fn event_set(&self) -> EventSet {
        let mut event_set = EventSet::hup() | EventSet::error() | EventSet::readable();

        if self.state == ConnectionState::Initiating {
            if !self.send_buf.is_empty() {
                event_set = event_set | EventSet::writable();
            }
        } else {
            if !self.send_buf.is_empty() || !self.send_queue.is_empty() {
                event_set = event_set | EventSet::writable();
            }
        }

        event_set
    }
}
