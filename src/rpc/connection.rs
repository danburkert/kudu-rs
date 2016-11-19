use std::cmp;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::i32;
use std::io::{self, ErrorKind, Write};
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::{Duration, Instant};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use futures::{self, Async, AsyncSink, Future, Poll, Sink, StartSend};
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
use queue_map::QueueMap;
use rpc::Rpc;
use util::duration_to_ms;

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
/// limited by the `ConnectionOptions::rpc_queue_len` option. If the queue is full, then subsequent
/// attempts to send an `Rpc` will fail with `Error::Backoff`.
///
/// The Kudu Tablet Server has a special error type, `Throttled`, to indicate that the server is
/// under memory pressure and is currently unable to handle RPCs. When an RPC fails due to
/// throttling, the `Connection` has a mechanism to artificially limit the in-flight queue, thus
/// reducing load to the server. This backoff mechanism is a cooperative effort between the RPC
/// sender and the `Connection`, since the error message is not part of the RPC header, and
/// therefore is not detectable by `Connection`. See `Connection::throttle()` for details.
pub struct Connection {
    /// The connection options.
    options: Rc<ConnectionOptions>,
    /// The current connection state.
    state: State,
    /// The address of the remote Kudu server.
    addr: SocketAddr,

    handle: Handle,

    /// Queue of RPCs to send.
    send_queue: QueueMap<Rpc>,
    /// RPCs which have been sent and are awaiting response.
    recv_queue: HashMap<usize, Rpc>,

    /// RPC request header, kept internally to reduce memory allocations.
    request_header: rpc_header::RequestHeader,
    /// RPC response header, kept internally to reduce memory allocations.
    response_header: rpc_header::ResponseHeader,

    /// Byte buffer holding the next incoming response.
    recv_buf: Buf,
    /// Byte buffer holding the next outgoing request.
    write_buf: Buf,

    /// Backoff tracker.
    reset_backoff: Backoff,

    /// Maximum size of recv_queue. The throttle is halved every time `Connection::throttle` is
    /// called (which should be in response to a tablet server `Throttled` error), increased by
    /// one for every successful RPC, and bounded by `ConnectionOptions::rpc_queue_len`.
    throttle: u32,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection {{ state: {:?}, addr: {}, queue (tx/rx): {}/{}, buf (tx/rx): {}/{} }}",
               self.state_kind(), self.addr, self.send_queue.len(), self.recv_queue.len(),
               self.write_buf.len(), self.recv_buf.len())
    }
}

impl Connection {

    /// Creates a new connection.
    ///
    /// The connection automatically attempts to connect to the remote server.
    pub fn new(handle: Handle,
               addr: SocketAddr,
               options: Rc<ConnectionOptions>)
               -> Connection {
        trace!("Creating new connection to {:?}", addr);
        let reset_backoff = Backoff::with_duration_range(options.backoff_initial, options.backoff_max);
        let throttle = options.rpc_queue_len;

        let stream_new = TcpStream::connect(&addr, &handle);

        Connection {
            options: options,
            state: State::Connecting(stream_new),
            addr: addr,
            handle: handle,
            send_queue: QueueMap::new(),
            recv_queue: HashMap::new(),
            request_header: rpc_header::RequestHeader::new(),
            response_header: rpc_header::ResponseHeader::new(),
            recv_buf: Buf::new(),
            write_buf: Buf::new(),
            reset_backoff: reset_backoff,
            throttle: throttle,
        }
    }

    fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    /// Notifies the connection of socket events.
    pub fn tick(&mut self) {
    }

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.options.rpc_queue_len) / 2;
    }

    /// Resets the connection following an error.
    fn reset(&mut self, error: Error) {
        let backoff_ms = self.reset_backoff.next_backoff_ms();
        warn!("{:?}: reset, error: {}, backoff: {}ms", self, error, backoff_ms);
        self.state = State::Reset(Timeout::new(Duration::from_millis(backoff_ms), &self.handle).unwrap());

        let recv_buf_len = self.recv_buf.len();
        self.recv_buf.consume(recv_buf_len);
        let write_buf_len = self.write_buf.len();
        self.write_buf.consume(write_buf_len);

        let now = Instant::now();
        let mut retries = Vec::new();
        for (call_id, mut rpc) in self.recv_queue.drain().chain(self.send_queue.drain()) {
            if rpc.cancelled() {
                continue;
            } else if rpc.timed_out(now) {
                rpc.fail(Error::TimedOut);
            } else if rpc.fail_fast() {
                rpc.fail(error.clone());
            } else {
                retries.push((call_id, rpc));
            }
        }

        for (call_id, rpc) in retries {
            self.send_queue.insert(call_id, rpc);
        }
        trace!("{:?}: retrying rpcs: {:?}", self, self.send_queue);
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
    fn handle_sasl_message(&mut self, msg: rpc_header::SaslMessagePB) -> Result<()> {
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
                take_mut::take(&mut self.state, |state| {
                    match state {
                        State::Negotiating(stream) => State::Connected(stream),
                        _ => unreachable!(),
                    }
                });
                self.reset_backoff.reset();
                try!(self.buffer_connection_context());

                // Optimistically flush the connection context and send any queued messages. The
                // connection has not necessarily received a writeable event at this point, but it
                // is highly likely that there is space available in the socket's write buffer.
                self.poll_write()?;
                Ok(())
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
    fn poll_read(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_read", self);

        loop {
            // Read, or continue reading, an RPC response message from the socket into the receive
            // buffer. Every RPC response is prefixed with a 4 bytes length header.
            if self.recv_buf.len() < 4 {
                let needed = 4 - self.recv_buf.len();
                try_ready!(self.read_at_least(needed));
            }

            let msg_len = BigEndian::read_u32(&self.recv_buf[..4]) as usize;
            if msg_len > self.options.max_message_length as usize {
                return Err(RpcError::invalid_rpc_header(format!(
                           "RPC response message is too long; length: {}, max length: {}",
                            msg_len, self.options.max_message_length)).into());
            }
            if self.recv_buf.len() - 4 < msg_len {
                let needed = msg_len + 4 - self.recv_buf.len();
                try_ready!(self.read_at_least(needed));
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

            match self.state_kind() {
                StateKind::Negotiating => {
                    // All SASL messages are required to have call ID -33.
                    if self.response_header.get_call_id() != 33 {
                        return Err(Error::Rpc(RpcError::invalid_rpc_header(
                                    format!("Negotiation RPC response header has illegal call id: {:?}",
                                            self.response_header))));
                    }
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
                    try!(self.handle_sasl_message(sasl_msg));
                },
                StateKind::Connected => {
                    trace!("{:?}: received response from server: {:?}", self, self.response_header);
                    if self.response_header.get_is_error() {
                        let error = RpcError::from(try!(
                                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                    &mut CodedInputStream::from_bytes(&self.recv_buf[..msg_len - header_len]))));
                        // Remove the RPC from the read queue, and fail it. The message may not be
                        // in the read queue if it has already timed out.
                        if let Some(rpc) = self.recv_queue.remove(&(self.response_header.get_call_id() as usize)) {
                            rpc.fail(Error::Rpc(error.clone()));
                        }
                        // If the message is fatal, then return an error in order to have the
                        // connection torn down.
                        if error.is_fatal() {
                            return Err(Error::Rpc(error.clone()))
                        }
                    } else if let Entry::Occupied(mut entry) = self.recv_queue.entry(self.response_header.get_call_id() as usize) {
                        // Use the entry API so that the RPC is not removed from the read queue
                        // if the protobuf decode step fails. Since it isn't removed, it has the
                        // opportunity to be retried when the error is bubbled up and the
                        // connection is reset.
                        //
                        // The message may not be in the read queue if it has already been
                        // cancelled.
                        try!(CodedInputStream::from_bytes(&self.recv_buf[..msg_len - header_len])
                             .merge_message(&mut *entry.get_mut().response));

                        if !self.response_header.get_sidecar_offsets().is_empty() {
                            panic!("sidecar decoding not implemented");
                        }

                        let rpc = entry.remove();
                        rpc.complete();
                        if self.throttle < self.options.rpc_queue_len {
                            self.throttle += 1;
                        }
                    }
                },
                _ => unreachable!("{:?}: poll_read"),
            };
            self.recv_buf.consume(msg_len - header_len);
        }
    }

    /// Send messages until either there are no more messages to send, or the socket can not accept
    /// any more writes. If an error is returned, the connection should be torn down.
    fn poll_write(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll", self);

        let now = Instant::now();

        // While we aren't throttled
        while self.recv_queue.len() < self.throttle as usize {

            // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
            // *still* over 8KiB, then apply backpressure (stop pulling from the write queue).
            if self.write_buf.len() > 8 * 1024 {
                try!(self.flush());
                if self.write_buf.len() > 8 * 1024 {
                    return Ok(Async::NotReady);
                }
            }

            if let Some((call_id, mut rpc)) = self.send_queue.pop() {
                let (call_id, mut rpc) = self.send_queue.pop().unwrap();

                if rpc.cancelled() {
                    trace!("{:?}: cancelling {:?}", self, rpc);
                    rpc.fail(Error::Cancelled);
                    continue;
                } else if rpc.timed_out(now) {
                    trace!("{:?}: timing out {:?}", self, rpc);
                    rpc.fail(Error::TimedOut);
                    continue;
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

                trace!("{:?}: sending rpc to server; call ID: {}, rpc: {:?}", self, call_id, rpc);
                self.buffer_message(&*rpc.request)?;
                self.recv_queue.insert(call_id, rpc);
            } else {
                break;
            }
        }

        self.flush()
    }

    /// Flushes the write buffer to the socket.
    fn flush(&mut self) -> Poll<(), Error> {
        trace!("{:?}: flush", self);
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

    /// Attempts to read at least `min` bytes from the socket into the receive buffer.
    /// Fewer bytes may be read if there is no data available.
    fn read_at_least(&mut self, min: usize) -> Poll<(), io::Error> {
        let Connection { ref mut state, ref mut recv_buf, .. } = *self;
        let mut received = 0;
        while received < min {
            received += try_nb!(recv_buf.read_from(state.stream()));
        }
        Ok(Async::Ready(()))
    }

    /// Returns the number of queued RPCs.
    fn queue_len(&self) -> usize {
        self.send_queue.len() + self.recv_queue.len()
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
        fn inner(cxn: &mut Connection) -> Result<()> {
            trace!("{:?}: tick", cxn);
            loop {
                match cxn.state_kind() {
                    StateKind::Connecting => {
                        match cxn.stream_new().poll()? {
                            Async::Ready(stream) => {
                                stream.set_nodelay(cxn.options.nodelay)?;
                                cxn.state = State::Negotiating(stream);
                                cxn.buffer_connection_header()?;
                                cxn.buffer_sasl_negotiation()?;
                            },
                            Async::NotReady => return Ok(()),
                        }
                    },
                    StateKind::Negotiating => {
                        cxn.poll_read()?;
                        cxn.flush()?;
                        return Ok(());
                    },
                    StateKind::Connected => {
                        cxn.poll_read()?;
                        cxn.poll_write()?;
                        return Ok(());
                    },
                    StateKind::Reset => {
                        if let Async::Ready(..) = cxn.timeout().poll()? {
                            cxn.state = State::Connecting(TcpStream::connect(&cxn.addr, &cxn.handle));
                        } else {
                            return Ok(());
                        }
                    },
                }
            }
        }
        inner(self).unwrap_or_else(|error| self.reset(error));
        Ok(Async::NotReady)
    }
}

/*
impl Sink for Connection {
    type SinkItem = Rpc;
    type SinkError = ();

    fn start_send(&mut self, mut rpc: Rpc) -> StartSend<Rpc, ()> {
        trace!("{:?}: start_send; rpc: {:?}, task: {:?}", self, rpc, futures::task::park());
        let now = Instant::now();
        if rpc.cancelled() {
            trace!("{:?}: rpc cancelled before queue: {:?}", self, rpc);
            rpc.fail(Error::Cancelled);
            return Ok(AsyncSink::Ready);
        } else if rpc.timed_out(now) {
            trace!("{:?}: rpc timed out before queue: {:?}", self, rpc);
            rpc.fail(Error::TimedOut);
            return Ok(AsyncSink::Ready);
        } else if self.queue_len() >= self.options.rpc_queue_len as usize ||
                  self.queue_len() >= self.throttle as usize {
            trace!("{:?}: connection not ready for rpc: {:?}", self, rpc);
            return Ok(AsyncSink::NotReady(rpc));
        }

        trace!("{:?}: queueing rpc: {:?}", self, rpc);

        self.send_queue.push(rpc);

        // If this is the only message in the queue, optimistically try to write it to the socket.
        if self.state_kind() == StateKind::Connected &&
           self.write_buf.is_empty() &&
           self.send_queue.len() == 1 {
            self.poll_write()
                .unwrap_or_else(|error| {
                    info!("{:?} error sending RPC: {}", self, error);
                    self.reset(error)
                });
        }

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), ()> {
        if self.queue_len() == 0 {
            return Ok(Async::Ready(()));
        }

        //self.tick();

        if self.queue_len() == 0 {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}
*/
