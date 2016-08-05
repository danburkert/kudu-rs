use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::i32;
use std::io::{self, ErrorKind, Write};
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::{Duration, Instant};

use Error;
use Result;
use backoff::Backoff;
use error::RpcError;
use kudu_pb::rpc_header::{SaslMessagePB_SaslState as SaslState};
use kudu_pb::rpc_header;
use queue_map::QueueMap;
use rpc::Rpc;
use rpc::messenger::{Loop, TimeoutKind};
use util::duration_to_ms;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use mio::{
    EventSet,
    PollOpt,
    Timeout,
    Token,
};
use mio::tcp::TcpStream;
use net2::TcpBuilder;
use netbuf::Buf;
use protobuf::{parse_length_delimited_from, Clear, CodedInputStream, Message};
use protobuf::rt::ProtobufVarint;

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
    /// Defaults to 10 ms.
    pub backoff_initial: u32,

    /// Maximum time in milliseconds to wait after an error before attempting to reconnect to the
    /// server.
    ///
    /// Defaults to 30 seconds.
    pub backoff_max: u32,

    /// Maximum allowable message length.
    ///
    /// Defaults to 5 MiB.
    pub max_message_length: u32,
}

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            nodelay: true,
            rpc_queue_len: 256,
            backoff_initial: 10,
            backoff_max: 30_000,
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

    /// The connection has been shut down due to an error.
    ///
    /// The connection will automatically be reestablished after a backoff period, if there are
    /// queued RPCs.
    Reset,
}

/// Wraps an `Rpc` and a timeout timer.
///
/// If the RPC completes before the timer fires, the timeout should be cleared.
struct QueuedRpc {
    rpc: Rpc,
    timer: Timeout,
}

/// Wraps an `Rpc`, a delay timer and the delay time.
///
/// If the RPC completes before the timer fires, the timeout should be cleared.
struct DelayedRpc {
    rpc: Rpc,
    timer: Timeout,
    delay: Instant,
}

/// `Connection` is a state machine that manages connections to a Kudu server.
///
/// A connection internally manages a single TCP connection to the server, and keeps track of
/// in-flight RPCs.
///
/// If an error occurs that requires reseting the socket connection to the server (e.g. a
/// [de]serialization failure, or a fatal error response), the connection will automatically
/// attempt to reconnect after waiting for a backoff period. Consecutive retries will be delayed
/// with an exponentially increasing backoff with jitter. In-flight RPCs will be failed when the
/// connection is reset.
///
/// If the connection is reset and has no queued RPCs, then it will automatically be shut down.
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
    send_queue: QueueMap<QueuedRpc>,
    /// RPCs which have been sent and are awaiting response.
    recv_queue: HashMap<u32, QueuedRpc>,
    /// Queue of RPCs to send after a delay.
    delay_queue: QueueMap<DelayedRpc>,

    /// RPC request header, kept internally to reduce memory allocations.
    request_header: rpc_header::RequestHeader,
    /// RPC response header, kept internally to reduce memory allocations.
    response_header: rpc_header::ResponseHeader,

    /// Byte buffer holding the next incoming response.
    recv_buf: Buf,
    /// Byte buffer holding the next outgoing request.
    send_buf: Buf,

    /// Backoff tracker.
    backoff: Backoff,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection {{ state: {:?}, addr: {}, queue (tx/rx/d): {}/{}/{}, buf (tx/rx): {}/{} }}",
               self.state, self.addr, self.send_queue.len(), self.recv_queue.len(),
               self.delay_queue.len(), self.send_buf.len(), self.recv_buf.len())
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
            send_queue: QueueMap::new(),
            recv_queue: HashMap::new(),
            delay_queue: QueueMap::new(),
            request_header: rpc_header::RequestHeader::new(),
            response_header: rpc_header::ResponseHeader::new(),
            recv_buf: Buf::new(),
            send_buf: Buf::new(),
            backoff: backoff,
        };
        connection.connect(event_loop, token);
        connection
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    /// Notifies the connection of socket events.
    pub fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        fn inner(cxn: &mut Connection, event_loop: &mut Loop, events: EventSet) -> Result<()> {
            match cxn.state {
                ConnectionState::Initiating => {
                    if events.is_readable() {
                        assert!(!events.is_writable());
                        assert!(cxn.send_buf.is_empty());
                        try!(cxn.recv(event_loop))
                    } else if events.is_writable() {
                        assert!(!events.is_readable());
                        assert!(cxn.recv_buf.is_empty());
                        try!(cxn.flush());
                    }
                },
                ConnectionState::Connected => {
                    if events.is_readable() {
                        try!(cxn.recv(event_loop))
                    } else if events.is_writable() {
                        try!(cxn.send(event_loop))
                    }
                },
                _ => unreachable!("{:?}: unexpected ready, events: {:?}", cxn, events),
            };
            Ok(())
        }

        debug!("{:?}: ready; events: {:?}", self, events);
        if events.is_error() || events.is_hup() {
            self.reset(event_loop, token, Error::ConnectionError);
        } else {
            inner(self, event_loop, events).and_then(|_| self.reregister(event_loop, token))
                                           .unwrap_or_else(|error| {
                                               info!("{:?} error: {}", self, &error);
                                               self.reset(event_loop, token, error)
                                           })
        }
    }

    /// Send an RPC to the Kudu server.
    pub fn send_rpc(&mut self, event_loop: &mut Loop, token: Token, rpc: Rpc) {
        trace!("{:?}: queueing rpc: {:?}", self, rpc);

        let now = Instant::now();
        if rpc.cancelled() {
            return rpc.fail(Error::Cancelled);
        } else if rpc.timed_out(now) {
            return rpc.fail(Error::TimedOut);
        } else if self.queue_len() > self.options.rpc_queue_len as usize {
            return rpc.fail(Error::Backoff);
        }

        self.send_queue.push_with(|call_id| {
            let timer = event_loop.timeout_ms((token, TimeoutKind::Rpc(call_id)),
                                              duration_to_ms(&rpc.deadline.duration_since(now)));
            QueuedRpc { rpc: rpc, timer: timer.unwrap() }
        });

        // If this is the only message in the queue, optimistically try to write it to the socket.
        if self.state == ConnectionState::Connected &&
           self.send_buf.is_empty() &&
           self.send_queue.len() == 1 {
            self.send(event_loop)
                .and_then(|_| self.reregister(event_loop, token))
                .unwrap_or_else(|error| {
                    info!("{:?} error sending RPC: {}", self, error);
                    self.reset(event_loop, token, error)
                });
        }
    }

    /// Send an RPC to the Kudu server after waiting for a delay period.
    pub fn send_delayed_rpc(&mut self,
                            event_loop: &mut Loop,
                            token: Token,
                            rpc: Rpc,
                            delay: Duration) {
        trace!("{:?}: queueing delayed rpc: {:?}, delay: {:?}", self, rpc, delay);
        let now = Instant::now();
        if rpc.cancelled() {
            return rpc.fail(Error::Cancelled);
        } else if rpc.timed_out(now + delay) {
            return rpc.fail(Error::TimedOut);
        } else if self.queue_len() > self.options.rpc_queue_len as usize {
            return rpc.fail(Error::Backoff);
        }

        self.delay_queue.push_with(|id| {
            let timer = event_loop.timeout_ms((token, TimeoutKind::DelayedRpc(id)),
                                              duration_to_ms(&delay))
                                  .unwrap();
            DelayedRpc {
                rpc: rpc,
                timer: timer,
                delay: now + delay,
            }
        });
    }

    /// Notifies the connection that the timeout has fired. Returns `true` if the connection should
    /// be closed, or `false` if it should continue.
    pub fn timeout(&mut self, event_loop: &mut Loop, token: Token, kind: TimeoutKind) -> bool {
        trace!("{:?}: {:?} timeout", self, kind);

        match kind {
            TimeoutKind::ConnectionReset => {
                assert!(self.state == ConnectionState::Reset, "{:?}: illegal {:?}", self, kind);
                if self.queue_len() == 0 {
                    // If the queue is empty then tear down the connection. This prevents a
                    // connection to a permanently partitioned tablet server from attempting to
                    // reconnect indefinitely. Eventually all RPCs will timeout and the retries
                    // will cease.
                    return true;
                }
                self.connect(event_loop, token);
            },
            TimeoutKind::Rpc(call_id) => {
                // No need to cancel the timeout here, since it fired.
                self.send_queue
                    .remove(call_id)
                    .or_else(|| self.recv_queue.remove(&(call_id as u32)))
                    .expect("timed out RPC not found in send or recv queue")
                    .rpc
                    .fail(Error::TimedOut);
            },
            TimeoutKind::DelayedRpc(id) => {
                // No need to cancel the timeout here, since it fired.
                let rpc = self.delay_queue
                              .remove(id)
                              .expect("delayed RPC not found in delay queue")
                              .rpc;
                self.send_rpc(event_loop, token, rpc);
            },
            TimeoutKind::Timer => unreachable!(),
        }

        false
    }

    /// Connects an inactive connection to the server.
    fn connect(&mut self, event_loop: &mut Loop, token: Token) {
        fn inner(cxn: &mut Connection, event_loop: &mut Loop, token: Token) -> Result<()> {
            assert!(cxn.recv_queue.is_empty());
            debug!("{:?}: connecting", cxn);

            // Create the stream via the net2 StreamBuilder API so that set_nodelay can be called
            // before connect as a workaround to mio#446. This is mostly copied from mio.
            let stream_builder = try!(match cxn.addr {
                SocketAddr::V4(..) => TcpBuilder::new_v4(),
                SocketAddr::V6(..) => TcpBuilder::new_v6(),
            });
            // Required on Windows for a future `connect_overlapped` operation to be
            // executed successfully.
            if cfg!(windows) {
                try!(stream_builder.bind(&inaddr_any(&cxn.addr)));
            }
            let stream = try!(stream_builder.to_tcp_stream());
            try!(stream.set_nodelay(cxn.options.nodelay));

            cxn.stream = Some(try!(TcpStream::connect_stream(stream, &cxn.addr)));
            cxn.state = ConnectionState::Initiating;

            // Write the connection header and SASL negotiation messages to the send buffer.
            try!(cxn.buffer_connection_header());
            try!(cxn.buffer_sasl_negotiation());
            // Optimistically flush the connection header and SASL negotiation to the TCP socket.
            // Even though the socket has not yet been registered, this will usually succeed
            // because the socket will have sufficient internal buffer space. This sometimes fails
            // on OS X with Not Connected errors, but they are safe to ignore, since the flush will
            // be retried on the next writeable event.
            let _ = cxn.flush();
            cxn.register(event_loop, token)
        };
        inner(self, event_loop, token).unwrap_or_else(|error| {
            info!("{:?} unable to connect: {}", self, error);
            self.reset(event_loop, token, error)
        });
    }

    /// Registers the connection with the event loop in order to enable readiness notifications.
    fn register(&mut self, event_loop: &mut Loop, token: Token) -> Result<()> {
        let event_set = self.event_set();
        let poll_opt = self.poll_opt();
        trace!("{:?}: register event_set: {:?}, poll_opt: {:?}", self, event_set, poll_opt);
        event_loop.register(self.stream.as_mut().unwrap(), token, event_set, poll_opt)
                  .map_err(From::from)
    }

    /// Reregisters the connection with the event loop in order to enable readiness notifications.
    fn reregister(&mut self, event_loop: &mut Loop, token: Token) -> Result<()> {
        let event_set = self.event_set();
        let poll_opt = self.poll_opt();
        trace!("{:?}: reregister event_set: {:?}, poll_opt: {:?}", self, event_set, poll_opt);
        event_loop.reregister(self.stream.as_mut().unwrap(), token, event_set, poll_opt)
                  .map_err(From::from)
    }

    /// Resets the connection following an error.
    fn reset(&mut self, event_loop: &mut Loop, token: Token, error: Error) {
        let backoff_ms = self.backoff.next_backoff_ms();
        warn!("{:?}: reset, error: {}, backoff: {}ms", self, error, backoff_ms);
        self.state = ConnectionState::Reset;
        self.stream.take();
        let recv_buf_len = self.recv_buf.len();
        self.recv_buf.consume(recv_buf_len);
        let send_buf_len = self.send_buf.len();
        self.send_buf.consume(send_buf_len);

        let now = Instant::now();

        let fail = |rpc: Rpc| {
            let error = if rpc.cancelled() { Error::Cancelled }
                        else if rpc.timed_out(now) { Error::TimedOut }
                        else { error.clone() };
            rpc.fail(error);
        };

        for (_, QueuedRpc { rpc, timer }) in self.recv_queue.drain() {
            event_loop.clear_timeout(timer);
            fail(rpc);
        }

        while let Some((_, QueuedRpc { rpc, timer })) = self.send_queue.pop() {
            event_loop.clear_timeout(timer);
            fail(rpc);
        }

        while let Some((_, DelayedRpc { rpc, timer, .. })) = self.delay_queue.pop() {
            event_loop.clear_timeout(timer);
            fail(rpc);
        }

        event_loop.timeout_ms((token, TimeoutKind::ConnectionReset), backoff_ms).unwrap();
    }

    /// Writes the message to the send buffer with a request header.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_message(&mut self, msg: &Message) -> Result<()> {
        trace!("{:?}: send message: {:?}", self, msg);
        let header_len = self.request_header.compute_size();
        let msg_len = msg.compute_size();
        let len = header_len + header_len.len_varint() + msg_len + msg_len.len_varint();
        try!(self.send_buf.write_u32::<BigEndian>(len));
        try!(self.request_header.write_length_delimited_to(&mut self.send_buf));
        msg.write_length_delimited_to(&mut self.send_buf).map_err(From::from)
    }

    /// Writes the KRPC connection header to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_connection_header(&mut self) -> Result<()> {
        trace!("{:?}: sending connection header to server", self);
        self.send_buf.write(b"hrpc\x09\0\0").map(|_| ()).map_err(From::from)
    }

    /// Writes a SASL negotiate message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_sasl_negotiation(&mut self) -> Result<()> {
        trace!("{:?}: sending SASL NEGOTIATE request to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::NEGOTIATE);
        self.buffer_message(&msg)
    }

    /// Writes a SASL initiate message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_sasl_initiate(&mut self) -> Result<()> {
        trace!("{:?}: sending SASL INITIATE request to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::INITIATE);
        msg.mut_token().extend_from_slice(b"\0user\0");
        let mut auth = rpc_header::SaslMessagePB_SaslAuth::new();
        auth.mut_mechanism().push_str("PLAIN");
        msg.mut_auths().push(auth);
        self.buffer_message(&msg)
    }

    /// Writes a session context message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_connection_context(&mut self) -> Result<()> {
        trace!("{:?}: sending connection context to server", self);
        self.request_header.clear();
        self.request_header.set_call_id(-3);
        let mut msg = rpc_header::ConnectionContextPB::new();
        msg.mut_user_info().set_effective_user("user".to_string());
        msg.mut_user_info().set_real_user("user".to_string());
        self.buffer_message(&msg)
    }

    /// Handles a SASL handshake response message.
    fn handle_sasl_message(&mut self,
                           event_loop: &mut Loop,
                           msg: rpc_header::SaslMessagePB)
                           -> Result<()> {
        trace!("{:?}: received SASL {:?} response from server", self, msg.get_state());
        match msg.get_state() {
            SaslState::NEGOTIATE => {
                if msg.get_auths().iter().any(|auth| auth.get_mechanism() == "PLAIN") {
                    try!(self.buffer_sasl_initiate());
                    try!(self.flush());
                    Ok(())
                } else {
                    Err(Error::NegotiationError("SASL PLAIN authentication not available"))
                }
            },
            SaslState::SUCCESS => {
                try!(self.buffer_connection_context());
                self.state = ConnectionState::Connected;
                self.backoff.reset();

                // Optimistically flush the connection context and send any queued messages. The
                // connection has not necessarily received a writeable event at this point, but it
                // is highly likely that there is space available in the socket's write buffer.
                self.send(event_loop)
            },
            _ => unreachable!("Unexpected SASL message: {:?}", msg),
        }
    }

    /// Reads available messages from the socket.
    ///
    /// If this connection is in the `Initiating` state, the message is assumed to be a SASL
    /// handshake message, and is passed to `Connection::handle_sasl_message`. If this connection
    /// is in the `Connected` state, the message is assumed to be an RPC response, and the
    /// corresponding queued `Rpc` is completed with the deserialized response.
    ///
    /// This method should be called when the connection's socket is readable.
    ///
    /// If an error is returned, the connection should be torn down.
    fn recv(&mut self, event_loop: &mut Loop) -> Result<()> {
        trace!("{:?}: recv", self);

        loop {
            // Read, or continue reading, an RPC response message from the socket into the receive
            // buffer. Every RPC response is prefixed with a 4 bytes length header.
            if self.recv_buf.len() < 4 {
                let needed = 4 - self.recv_buf.len();
                let read = try!(self.read(needed));
                if read < needed { break; }
            }

            let msg_len = BigEndian::read_u32(&self.recv_buf[..4]) as usize;
            if msg_len > self.options.max_message_length as usize {
                return Err(RpcError::invalid_rpc_header(format!(
                           "RPC response message is too long; length: {}, max length: {}",
                            msg_len, self.options.max_message_length)).into());
            }
            if self.recv_buf.len() - 4 < msg_len {
                let needed = msg_len + 4 - self.recv_buf.len();
                let read = try!(self.read(needed));
                if read < needed { break; }
            }

            // The whole message has been read. Skip the length prefix.
            self.recv_buf.consume(4);

            // TODO: the below would be a lot simpler if a single CodedInputStream wrapping
            // self.recv_buf could be used, but unfortunately that trips the borrow checker.
            // Perhaps once non-lexical lifetimes land it can be cleaned up.

            // Read the response header into self.response_header
            self.response_header.clear();
            let header_len = {
                let mut coded_stream = CodedInputStream::from_bytes(&self.recv_buf[..msg_len]);
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

                    let sasl_msg = try!(parse_length_delimited_from(
                            &mut CodedInputStream::from_bytes(&self.recv_buf[..])));
                    try!(self.handle_sasl_message(event_loop, sasl_msg));
                },
                ConnectionState::Connected => {
                    trace!("{:?}: received response from server: {:?}", self, self.response_header);
                    if self.response_header.get_is_error() {
                        let error = RpcError::from(try!(
                                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                    &mut CodedInputStream::from_bytes(&self.recv_buf[..msg_len - header_len]))));
                        // Remove the RPC from the recv queue, and fail it. The message may not be
                        // in the recv queue if it has already timed out.
                        if let Some(QueuedRpc { rpc, timer }) = self.recv_queue.remove(&(self.response_header.get_call_id() as u32)) {
                            event_loop.clear_timeout(timer);
                            rpc.fail(Error::Rpc(error.clone()));
                        }
                        // If the message is fatal, then return an error in order to have the
                        // connection torn down.
                        if error.is_fatal() {
                            return Err(Error::Rpc(error.clone()))
                        }
                    } else if let Entry::Occupied(mut entry) = self.recv_queue.entry(self.response_header.get_call_id() as u32) {
                        // Use the entry API so that the RPC is not removed from the recv queue
                        // if the protobuf decode step fails. Since it isn't removed, it has the
                        // opportunity to be retried when the error is bubbled up and the
                        // connection is reset.
                        //
                        // The message may not be in the recv queue if it has already timed out.
                        try!(CodedInputStream::from_bytes(&self.recv_buf[..msg_len - header_len])
                             .merge_message(&mut *entry.get_mut().rpc.response));

                        if !self.response_header.get_sidecar_offsets().is_empty() {
                            panic!("sidecar decoding not implemented");
                        }

                        let QueuedRpc { rpc, timer } = entry.remove();
                        event_loop.clear_timeout(timer);
                        rpc.complete();
                    }
                },
                _ => unreachable!("{:?}: recv"),
            };
            self.recv_buf.consume(msg_len - header_len);
        };

        Ok(())
    }

    /// Send messages until either there are no more messages to send, or the socket can not accept
    /// any more writes. If an error is returned, the connection should be torn down.
    fn send(&mut self, event_loop: &mut Loop) -> Result<()> {
        trace!("{:?}: send", self);
        assert_eq!(self.state, ConnectionState::Connected);

        let now = Instant::now();
        while !self.send_buf.is_empty() || !self.send_queue.is_empty() {
            while self.send_buf.len() < 4096 && !self.send_queue.is_empty() {
                let (call_id, QueuedRpc { rpc, timer }) = self.send_queue.pop().unwrap();

                if rpc.cancelled() {
                    trace!("{:?}: cancelling {:?}", self, rpc);
                    event_loop.clear_timeout(timer);
                    rpc.fail(Error::Cancelled);
                    break;
                } else if rpc.timed_out(now) {
                    trace!("{:?}: timing out {:?}", self, rpc);
                    event_loop.clear_timeout(timer);
                    rpc.fail(Error::TimedOut);
                    break;
                }

                if call_id > i32::MAX as usize {
                    warn!("{:?}: call id overflowed", self);
                    return Err(Error::ConnectionError);
                }

                self.request_header.clear();
                self.request_header.set_call_id(call_id as i32);
                self.request_header.mut_remote_method().mut_service_name().push_str(rpc.service_name);
                self.request_header.mut_remote_method().mut_method_name().push_str(rpc.method_name);
                self.request_header.set_timeout_millis(duration_to_ms(&rpc.deadline.duration_since(now)) as u32);
                self.request_header.mut_required_feature_flags().extend_from_slice(&rpc.required_feature_flags);

                trace!("{:?}: sending rpc to server; call ID: {}", self, call_id);
                try!(self.buffer_message(&*rpc.request));
                self.recv_queue.insert(call_id as u32, QueuedRpc { rpc: rpc, timer: timer });
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

    /// Returns the number of queued RPCs.
    fn queue_len(&self) -> usize {
        self.send_queue.len() + self.recv_queue.len() + self.delay_queue.len()
    }

    fn poll_opt(&self) -> PollOpt {
        PollOpt::edge() | PollOpt::oneshot()
    }

    /// Returns the event set that the connection should register to handle.
    fn event_set(&self) -> EventSet {
        let mut event_set = EventSet::hup() | EventSet::error() | EventSet::readable();

        if self.state == ConnectionState::Initiating {
            if !self.send_buf.is_empty() {
                event_set = event_set | EventSet::writable();
            }
        } else if !self.send_buf.is_empty() || !self.send_queue.is_empty() {
            event_set = event_set | EventSet::writable();
        }

        event_set
    }
}

/// Copied from mio.
fn inaddr_any(other: &SocketAddr) -> SocketAddr {
    use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
    match *other {
        SocketAddr::V4(..) => {
            let any = Ipv4Addr::new(0, 0, 0, 0);
            let addr = SocketAddrV4::new(any, 0);
            SocketAddr::V4(addr)
        }
        SocketAddr::V6(..) => {
            let any = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
            let addr = SocketAddrV6::new(any, 0, 0, 0);
            SocketAddr::V6(addr)
        }
    }
}
