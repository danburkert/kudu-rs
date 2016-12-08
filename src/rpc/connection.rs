use std::cmp;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::i32;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use futures::sync::mpsc;
use futures::{Async, Future, Poll, Stream};
use netbuf::Buf;
use protobuf::rt::ProtobufVarint;
use protobuf::{parse_length_delimited_from, Clear, CodedInputStream, Message};
use take_mut;
use tokio::net::{TcpStream, TcpStreamNew};
use tokio::reactor::{Handle, Timeout};

use Error;
use Result;
use backoff::Backoff;
use error::RpcError;
use kudu_pb::rpc_header::{SaslMessagePB_SaslState as SaslState};
use kudu_pb::rpc_header;
use rpc::Rpc;
use util::duration_to_ms;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionOptions {
    /// Maximum number of outstandings RPCs to allow in the connection.
    ///
    /// Defaults to 32.
    pub max_rpcs_in_flight: u32,

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

    /// Whether to disable Nagle's algorithm.
    ///
    /// Defaults to true.
    pub nodelay: bool,
}

impl Default for ConnectionOptions {
    fn default() -> ConnectionOptions {
        ConnectionOptions {
            nodelay: true,
            max_rpcs_in_flight: 32,
            backoff_initial: 10,
            backoff_max: 30_000,
            max_message_length: 5 * 1024 * 1024,
        }
    }
}

enum State {
    Connecting(TcpStreamNew),
    Negotiating(TcpStream),
    Connected(TcpStream),
    Reset(Timeout),
}

impl State {
    fn kind(&self) -> StateKind {
        match *self {
            State::Connecting(..) => StateKind::Connecting,
            State::Negotiating(..) => StateKind::Negotiating,
            State::Connected(..) => StateKind::Connected,
            State::Reset(..) => StateKind::Reset,
        }
    }

    fn stream(&mut self) -> &mut TcpStream {
        match *self {
            State::Negotiating(ref mut stream) | State::Connected(ref mut stream) => stream,
            _ => unreachable!(),
        }
    }

    fn stream_new(&mut self) -> &mut TcpStreamNew {
        match *self {
            State::Connecting(ref mut stream_new) => stream_new,
            _ => unreachable!(),
        }
    }

    fn timeout(&mut self) -> &mut Timeout {
        match *self {
            State::Reset(ref mut timeout) => timeout,
            _ => unreachable!(),
        }
    }

    fn transition_negotiating(&mut self, stream: TcpStream) {
        debug_assert_eq!(StateKind::Connecting, self.kind());
        *self = State::Negotiating(stream);
    }

    fn transition_connected(&mut self) {
        debug_assert_eq!(StateKind::Negotiating, self.kind());
        take_mut::take(self, |state| {
            match state {
                State::Negotiating(stream) => State::Connected(stream),
                _ => unreachable!(),
            }
        });
    }

    fn transition_connecting(&mut self, stream_new: TcpStreamNew) {
        debug_assert_eq!(StateKind::Reset, self.kind());
        *self = State::Connecting(stream_new);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateKind {
    Connecting,
    Negotiating,
    Connected,
    Reset,
}

/// `Connection` is a state machine that manages a TCP socket connection to a remote Kudu server.
///
/// The [Kudu RPC protocol](https://github.com/cloudera/kudu/blob/master/docs/design-docs/rpc.md)
/// allows multiple RPCs to be in-flight on a single connection at once, and allows responses to be
/// returned out of order. The `Connection` handles queuing, serializing, sending, receiving,
/// deserializing, and completing RPCs.
///
/// # Connection Errors
///
/// If an error occurs that requires reseting the socket connection to the server (e.g. a socket
/// error, a [de]serialization failure, or a fatal error response), the connection will
/// automatically attempt to reconnect after waiting for a backoff period. Consecutive retries
/// without a succesful RPC will be delayed with an exponentially increasing backoff with jitter.
/// See `Connection::reset()` for details.
///
/// When the connection is reset, all fail-fast RPCs will be failed with the error which caused the
/// reset. During the reconnection backoff period, new fail-fast RPCs will be failed immediately
/// instead of being queued.
///
/// # Back Pressure & Flow Control
///
/// Internally, the connection holds a queue of pending and in-flight `Rpc`s. The queue size is
/// limited by the `ConnectionOptions::max_rpcs_in_flight` option. If the queue is full, then
/// subsequent attempts to send an `Rpc` will fail with `Error::Backoff`.
///
/// The Kudu Tablet Server has a special error type, `Throttled`, to indicate that the server is
/// under memory pressure and is currently unable to handle RPCs. When an RPC fails due to
/// throttling, the `Connection` has a mechanism to artificially limit the in-flight queue, thus
/// reducing load to the server. This backoff mechanism is a cooperative effort between the RPC
/// sender and the `Connection`, since the error message is not part of the RPC header, and
/// therefore is not detectable by `Connection`. See `Connection::throttle()` for details.
pub struct Connection {
    /// The connection options.
    options: ConnectionOptions,
    /// The current connection state.
    state: State,
    /// The address of the remote Kudu server.
    addr: SocketAddr,

    handle: Handle,

    /// Queue of RPCs to send.
    send_queue: mpsc::Receiver<Rpc>,

    /// RPCs which have been sent and are awaiting response.
    recv_queue: HashMap<usize, Rpc>,

    /// RPCs which should be retried after reconnecting.
    retry_queue: Vec<Rpc>,

    /// RPC request header, kept internally to reduce memory allocations.
    request_header: rpc_header::RequestHeader,
    /// RPC response header, kept internally to reduce memory allocations.
    response_header: rpc_header::ResponseHeader,

    /// Byte buffer holding the next incoming response.
    read_buf: Buf,
    /// Byte buffer holding the next outgoing request.
    write_buf: Buf,

    /// Backoff tracker.
    reset_backoff: Backoff,

    /// Maximum size of recv_queue. The throttle is halved every time `Connection::throttle` is
    /// called (which should be in response to a tablet server `Throttled` error), increased by
    /// one for every successful RPC, and bounded by `ConnectionOptions::max_rpcs_in_flight`.
    throttle: u32,

    /// The next call id.
    next_call_id: i32,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection {{ state: {:?}, addr: {}, in-flight: {}, buf (tx/rx): {}/{} }}",
               self.state_kind(), self.addr, self.recv_queue.len(),
               self.write_buf.len(), self.read_buf.len())
    }
}

impl Connection {

    /// Creates a new connection.
    ///
    /// The connection automatically attempts to connect to the remote server.
    pub fn new(handle: Handle,
               addr: SocketAddr,
               options: ConnectionOptions,
               recv: mpsc::Receiver<Rpc>) -> Connection {
        trace!("Creating new connection to {:?}", addr);
        let reset_backoff = Backoff::with_duration_range(options.backoff_initial,
                                                         options.backoff_max);
        let throttle = options.max_rpcs_in_flight;
        let stream_new = TcpStream::connect(&addr, &handle);

        Connection {
            options: options,
            state: State::Connecting(stream_new),
            addr: addr,
            handle: handle.clone(),
            send_queue: recv,
            recv_queue: HashMap::new(),
            retry_queue: Vec::new(),
            request_header: rpc_header::RequestHeader::new(),
            response_header: rpc_header::ResponseHeader::new(),
            read_buf: Buf::new(),
            write_buf: Buf::new(),
            reset_backoff: reset_backoff,
            throttle: throttle,
            next_call_id: 0,
        }
    }

    fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.options.max_rpcs_in_flight) / 2;
    }

    pub fn next_call_id(&mut self) -> Result<i32> {
        let call_id = self.next_call_id;
        if let Some(next) = self.next_call_id.checked_add(1) {
            self.next_call_id = next;
            Ok(call_id)
        } else {
            warn!("{:?}: call id overflowed", self);
            Err(Error::ConnectionError)
        }
    }

    /// Poll the connection while in the `Connecting` state.
    ///
    /// If the TCP socket is successfully connect, the connection will be transition to the
    /// `Negotiating` state, and `poll_negotiating` will be called.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) on success.
    ///     * Err(..) on fatal error. The call should reset the connection.
    fn poll_connecting(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_connecting", self);
        // Check if the TCP stream has connected.
        let stream = try_ready!(self.stream_new().poll());

        // If it has, set the TCP socket options and start negotiating.
        stream.set_nodelay(self.options.nodelay)?;
        self.state.transition_negotiating(stream);
        self.buffer_connection_header()?;
        self.buffer_sasl_negotiate()?;
        self.poll_negotiating()
    }

    /// Poll the connection while in the `Negotiating` state.
    ///
    /// If the connection negotiation is completed, the connection will transition to the
    /// `Negotiating` state, and `poll_negotiating` will be called.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) on success.
    ///     * Err(..) on fatal error. The call should reset the connection.
    fn poll_negotiating(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_negotiating", self);
        loop {
            // Attempt to send any buffered negotiation messages.
            try_ready!(self.poll_flush());

            // We're waiting for a negotiation response, attempt to read it.
            let msg = try_ready!(self.poll_read_negotiation());

            trace!("{:?}: received SASL {:?} response from server", self, msg.get_state());
            match msg.get_state() {
                SaslState::NEGOTIATE => {
                    if msg.get_auths().iter().any(|auth| auth.get_mechanism() == "PLAIN") {
                        self.buffer_sasl_initiate()?;
                        // Fall through to another trip through the loop.
                    } else {
                        return Err(Error::NegotiationError("SASL PLAIN authentication not available"));
                    }
                },
                SaslState::SUCCESS => {
                    self.state.transition_connected();
                    self.reset_backoff.reset();
                    self.buffer_connection_context()?;
                    return self.poll_connected();
                },
                _ => unreachable!("Unexpected SASL message: {:?}", msg),
            }
        }
    }

    /// Poll the connection while in the `Connected` state.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) on success.
    ///     * Err(..) on fatal error. The call should reset the connection.
    fn poll_connected(&mut self) -> Poll<(), Error> {
        fn do_while_ready<F>(mut f: F) -> Result<()> where F: FnMut() -> Poll<(), Error> {
            while let Async::Ready(..) = f()? { }
            Ok(())
        }

        do_while_ready(|| self.poll_read_connected())?;
        do_while_ready(|| self.poll_write_connected())?;
        try_ready!(self.poll_flush());

        Ok(Async::NotReady)
    }

    /// Poll the connection while in the `Reset` state.
    ///
    /// If the reset period is over, the connection will transition to the `Connecting` state, and
    /// `poll_connecting` will be called.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) on success.
    ///     * Err(..) on fatal error. The call should reset the connection.
    fn poll_reset(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_reset", self);
        // Check if the timeout period is over.
        try_ready!(self.timeout().poll());

        self.state.transition_connecting(TcpStream::connect(&self.addr, &self.handle));
        self.poll_connecting()
    }

    /// Resets the connection following an error.
    fn reset(&mut self, error: Error) -> Poll<(), Error> {
        let backoff_ms = self.reset_backoff.next_backoff_ms();
        warn!("{:?}: reset, error: {}, backoff: {}ms", self, error, backoff_ms);
        self.state = State::Reset(Timeout::new(Duration::from_millis(backoff_ms), &self.handle).unwrap());

        let recv_buf_len = self.read_buf.len();
        self.read_buf.consume(recv_buf_len);
        let write_buf_len = self.write_buf.len();
        self.write_buf.consume(write_buf_len);

        self.retry_queue.extend(self.recv_queue.drain().map(|(_, rpc)| rpc));
        self.poll_reset()
    }

    /// Writes the message to the send buffer with a request header.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_message(&mut self, msg: &Message) -> Result<()> {
        let header_len = self.request_header.compute_size();
        let msg_len = msg.compute_size();
        let len = header_len + header_len.len_varint() + msg_len + msg_len.len_varint();
        try!(self.write_buf.write_u32::<BigEndian>(len));
        try!(self.request_header.write_length_delimited_to(&mut self.write_buf));
        msg.write_length_delimited_to(&mut self.write_buf).map_err(From::from)
    }

    /// Writes the KRPC connection header to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_connection_header(&mut self) -> Result<()> {
        trace!("{:?}: sending connection header to server", self);
        self.write_buf.write(b"hrpc\x09\0\0").map(|_| ()).map_err(From::from)
    }

    /// Writes a SASL negotiate message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_sasl_negotiate(&mut self) -> Result<()> {
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

    /// Reads the bytes for an RPC response message from the socket into the receive buffer, and
    /// then decodes the header into the response header. Returns the length of the message body.
    fn poll_read_header(&mut self) -> Poll<usize, Error> {
        /// Attempts to read at least `min` bytes from the socket into the receive buffer.
        /// Fewer bytes may be read if there is no data available.
        fn read_at_least(&mut Connection { ref mut state, ref mut read_buf, .. }: &mut Connection,
                         min: usize)
                         -> Poll<(), io::Error> {
            let mut received = 0;
            while received < min {
                received += try_nb!(read_buf.read_from(state.stream()));
            }
            Ok(Async::Ready(()))
        }

        // Read, or continue reading, an RPC response message from the socket into the read buffer.
        // Every RPC response is prefixed with a 4 bytes length header.
        if self.read_buf.len() < 4 {
            let needed = 4 - self.read_buf.len();
            try_ready!(read_at_least(self, needed));
        }

        let msg_len = BigEndian::read_u32(&self.read_buf[..4]) as usize;
        if msg_len > self.options.max_message_length as usize {
            return Err(RpcError::invalid_rpc_header(format!(
                       "RPC response message is too long; length: {}, max length: {}",
                       msg_len, self.options.max_message_length)).into());
        }
        if self.read_buf.len() - 4 < msg_len {
            let needed = msg_len + 4 - self.read_buf.len();
            try_ready!(read_at_least(self, needed));
        }

        let pos = {
            self.response_header.clear();
            let mut cis = CodedInputStream::from_bytes(&self.read_buf[4..]);
            cis.merge_message(&mut self.response_header)?;
            cis.pos() as usize
        };

        self.read_buf.consume(4 + pos);
        trace!("{:?}: received header from server: {:?}", self, self.response_header);
        Ok(Async::Ready(msg_len - pos))
    }

    /// Reads a negotiation message from the socket.
    ///
    /// If an error is returned, the connection should be torn down.
    fn poll_read_negotiation(&mut self) -> Poll<rpc_header::SaslMessagePB, Error> {
        trace!("{:?}: poll_read_negotiation", self);

        let body_len = try_ready!(self.poll_read_header());

        // SASL messages are required to have call ID -33.
        if self.response_header.get_call_id() != -33 {
            return Err(Error::Rpc(RpcError::invalid_rpc_header(
                        format!("negotiation RPC response header has illegal call id: {:?}",
                                self.response_header.get_call_id()))));

        }

        // SASL messages may not have sidecars.
        if !self.response_header.get_sidecar_offsets().is_empty() {
            return Err(Error::Rpc(RpcError::invalid_rpc_header(
                        "negotiation RPC response includes sidecars".to_string())));
        }

        // We only expect a single response message to be in flight during negotiation.
        if body_len != self.read_buf.len() {
            return Err(Error::NegotiationError(
                    "detected multiple in-flight RPC responses during negotiation"))
        }

        let msg = {
            let mut cis = CodedInputStream::from_bytes(&self.read_buf[..]);

            if self.response_header.get_is_error() {
                // All errors during SASL negotiation are fatal.
                let err = parse_length_delimited_from::<rpc_header::ErrorStatusPB>(&mut cis)?;
                return Err(Error::Rpc(RpcError::from(err)));
            }

            let msg = parse_length_delimited_from(&mut cis)?;

            if body_len != cis.pos() as usize {
                return Err(Error::NegotiationError(
                        "decoded message length does not match the header length"));
            }

            msg
        };

        self.read_buf.consume(body_len);
        Ok(Async::Ready(msg))
    }

    /// Reads an RPC response messsage from the socket, and completes the corresponding `Rpc` in
    /// the receive queue.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) if a message is not ready to be read from the socket.
    ///     * Ok(Async::Ready(..)) if a message is successfully read from the socket.
    ///     * Err(..) if a fatal error occurs. The caller should reset the connection.
    fn poll_read_connected(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_read_connected", self);

        let poll_read_header = self.poll_read_header();
        trace!("{:?}: poll_read_header result: {:?}", self, poll_read_header);
        let body_len = try_ready!(poll_read_header);
        let call_id = self.response_header.get_call_id() as usize;
        if self.response_header.get_is_error() {
            let error = RpcError::from(
                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                    &mut CodedInputStream::from_bytes(&self.read_buf[..body_len]))?);

            // Remove the RPC from the read queue, and fail it. The message may not be
            // in the receive queue if it has already timed out or been cancelled.
            if let Some(rpc) = self.recv_queue.remove(&call_id) {
                rpc.fail(Error::Rpc(error.clone()));
            }
            // If the message is fatal, then return an error in order to have the
            // connection torn down.
            if error.is_fatal() {
                return Err(Error::Rpc(error))
            }
        } else if let Entry::Occupied(mut entry) = self.recv_queue.entry(call_id) {
            // Use the entry API so that the RPC is not removed from the read queue
            // if the protobuf decode step fails. Since it isn't removed, it has the
            // opportunity to be retried when the error is bubbled up and the
            // connection is reset.
            //
            // The message may not be in the read queue if it has already been
            // cancelled.
            CodedInputStream::from_bytes(&self.read_buf[..body_len])
                             .merge_message(&mut *entry.get_mut().response)?;

            if !self.response_header.get_sidecar_offsets().is_empty() {
                panic!("sidecar decoding not implemented");
            }

            let rpc = entry.remove();
            rpc.complete();
            if self.throttle < self.options.max_rpcs_in_flight {
                self.throttle += 1;
            }
        }

        self.read_buf.consume(body_len);
        Ok(Async::Ready(()))
    }

    /// Send messages until either there are no more messages to send, or the connection can not
    /// accept any more writes.
    ///
    /// Returns:
    ///     * Ok(Async::NotReady) if a message is not ready to be sent, or the connection can not
    ///       accept any more writes.
    ///     * Ok(Async::Ready(..)) if a message is successfully sent.
    ///     * Err(..) if a fatal error occurs. The caller should reset the connection.
    fn poll_write_connected(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_write_connected", self);

        // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
        // *still* over 8KiB, then stop sending messages until the buffer clears.
        if self.write_buf.len() > 8 * 1024 {
            self.poll_flush()?;
            if self.write_buf.len() > 8 * 1024 {
                return Ok(Async::NotReady);
            }
        }

        // Check if the connection is throttled.
        if self.recv_queue.len() >= self.throttle as usize {
            return Ok(Async::NotReady);
        }

        let mut rpc = if let Some(rpc) = self.retry_queue.pop() {
            rpc
        } else if let Async::Ready(Some(rpc)) = self.send_queue.poll().unwrap() {
            rpc
        } else {
            // No more messages to send!
            return Ok(Async::NotReady);
        };

        let now = Instant::now();
        if rpc.cancelled() {
            trace!("{:?}: cancelling {:?}", self, rpc);
            rpc.fail(Error::Cancelled);
        } else if rpc.timed_out(now) {
            trace!("{:?}: timing out {:?}", self, rpc);
            rpc.fail(Error::TimedOut);
        } else {
            let call_id = self.next_call_id()?;
            self.request_header.clear();
            self.request_header.set_call_id(call_id);
            self.request_header.mut_remote_method().mut_service_name().push_str(rpc.service_name);
            self.request_header.mut_remote_method().mut_method_name().push_str(rpc.method_name);
            self.request_header.set_timeout_millis(duration_to_ms(&rpc.deadline.duration_since(now)) as u32);
            self.request_header.mut_required_feature_flags().extend_from_slice(&rpc.required_feature_flags);

            trace!("{:?}: sending rpc to server; call ID: {}, rpc: {:?}", self, call_id, rpc);
            self.buffer_message(&*rpc.request)?;
            self.recv_queue.insert(call_id as usize, rpc);
        }
        Ok(Async::Ready(()))
    }

    /// Flushes the write buffer to the socket.
    ///
    /// Returns:
    ///     * Ok(Async::Ready) if the entire write buffer is flushed to the socket.
    ///     * Ok(Async::NotReady) if the socket is not ready for the entire write buffer.
    ///     * Err(..) on fatal error. The caller should reset the connection.
    fn poll_flush(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_flush", self);
        let Connection { ref mut state, ref mut write_buf, .. } = *self;
        while !write_buf.is_empty() {
            let n = try_nb!(write_buf.write_to(state.stream()));
            if n == 0 {
                return Err(Error::Io(io::Error::new(io::ErrorKind::WriteZero,
                                                    "failed to flush to socket")));
            }
        }
        Ok(Async::Ready(()))
    }

    fn state_kind(&self) -> StateKind {
        self.state.kind()
    }

    fn stream(&mut self) -> &mut TcpStream {
        self.state.stream()
    }

    fn stream_new(&mut self) -> &mut TcpStreamNew {
        self.state.stream_new()
    }

    fn timeout(&mut self) -> &mut Timeout {
        self.state.timeout()
    }
}

impl Future for Connection {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<(), ()> {
        trace!("{:?}: poll", self);
        let poll = match self.state_kind() {
            StateKind::Connecting => self.poll_connecting(),
            StateKind::Negotiating => self.poll_negotiating(),
            StateKind::Connected => self.poll_connected(),
            StateKind::Reset => self.poll_reset(),
        };
        match poll {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => {
                info!("{:?} error during poll: {}", self, error);
                self.reset(error).unwrap();
                Ok(Async::NotReady)
            },
            Ok(Async::Ready(())) => unreachable!(),
        }
    }
}
