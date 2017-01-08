use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::mem;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use futures::{IntoFuture, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use kudu_pb::rpc_header::{self, ErrorStatusPB, SaslMessagePB_SaslState as SaslState};
use netbuf::Buf;
use protobuf::rt::ProtobufVarint;
use protobuf::{parse_length_delimited_from_bytes, Clear, CodedInputStream, Message};
use tokio::net::TcpStream;
use tokio::reactor::{Remote, Handle};

use Error;
use Result;
use error::RpcError;
use rpc::Rpc;
use util::duration_to_ms;

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

/// Writes the header and message to the buffer.
///
/// Does not flush the buffer.
///
/// If an error is returned, the connection should be torn down.
fn buffer_message(header: &rpc_header::RequestHeader, msg: &Message, buf: &mut Buf) -> Result<()> {
    let header_len = header.compute_size();
    let msg_len = msg.compute_size();
    let len = header_len + header_len.len_varint() + msg_len + msg_len.len_varint();
    buf.write_u32::<BigEndian>(len)?;
    header.write_length_delimited_to(buf)?;
    msg.write_length_delimited_to(buf).map_err(From::from)
}

/// Attempts to read at least `min` bytes from the socket into the buffer.
/// Fewer bytes may be read if there is no data available.
fn read_at_least(stream: &mut TcpStream, buf: &mut Buf, min: usize) -> Poll<(), Error> {
    let mut received = 0;
    while received < min {
        let n = try_nb!(buf.read_from(stream));
        if n == 0 {
            warn!("remote server hung up (read 0)");
            return Err(Error::ConnectionError);
        }
        received += n;
    }
    Ok(Async::Ready(()))
}

/// Reads the header and body of an RPC response message from the socket into the buffer, and then
/// decodes the header. Returns the header and body.
fn poll_read_message<'a>(options: &ConnectionOptions,
                         stream: &mut TcpStream,
                         buf: &'a mut Buf)
                         -> Poll<(rpc_header::ResponseHeader, Buf), Error> {
    // Read, or continue reading, an RPC response message from the socket into the read buffer.
    // Every RPC response is prefixed with a 4 bytes length header.
    if buf.len() < 4 {
        let needed = 4 - buf.len();
        try_ready!(read_at_least(stream, buf, needed));
    }

    let msg_len = BigEndian::read_u32(&buf[..4]) as usize;
    if msg_len > options.max_message_length as usize {
        return Err(RpcError::invalid_rpc_header(format!(
                    "RPC response message is too long; length: {}, max length: {}",
                    msg_len, options.max_message_length)).into());
    }
    if buf.len() - 4 < msg_len {
        let needed = msg_len + 4 - buf.len();
        try_ready!(read_at_least(stream, buf, needed));
    }

    let mut header = rpc_header::ResponseHeader::new();

    let header_len = {
        header.clear();
        let mut cis = CodedInputStream::from_bytes(&buf[4..]);
        cis.merge_message(&mut header)?;
        cis.pos() as usize
    };

    buf.consume(4 + header_len);
    let rest = buf.split_off(msg_len - header_len);
    let body = mem::replace(buf, rest);
    Ok(Async::Ready((header, body)))
}

/// Flushes a buffer to a TCP socket.
///
/// Returns:
///     * `Ok(Async::Ready(..))`
///         The entire write buffer is flushed to the socket.
///     * `Ok(Async::NotReady)`
///         The socket is not ready for the entire write.
///     * `Err(..)`
///         Fatal error. The caller should reset the connection.
fn poll_flush(buf: &mut Buf, stream: &mut TcpStream) -> Poll<(), Error> {
    while !buf.is_empty() {
        let n = try_nb!(buf.write_to(stream));
        if n == 0 {
            warn!("remote server hung up (write 0)");
            return Err(Error::ConnectionError);
        }
    }
    Ok(Async::Ready(()))
}

pub struct Connection {
    /// The connection options.
    options: ConnectionOptions,

    /// RPCs which have been sent and are awaiting response.
    recv_queue: HashMap<usize, Rpc>,

    /// RPCs which have failed, but haven't yet been returned by `poll()`.
    failed_rpcs: Vec<Rpc>,

    stream: TcpStream,

    /// Byte buffer holding the next incoming response.
    read_buf: Buf,
    /// Byte buffer holding the next outgoing request.
    write_buf: Buf,

    /// Maximum size of recv_queue. The throttle is halved every time `Connection::throttle` is
    /// called (which should be in response to a tablet server `Throttled` error), increased by
    /// one for every successful RPC, and bounded by `ConnectionOptions::max_rpcs_in_flight`.
    throttle: u32,

    /// The next call id.
    next_call_id: i32,
}

impl Connection {

    pub fn spawn<F, R>(reactor: &Remote, addr: SocketAddr, options: ConnectionOptions, f: F)
    where F: FnOnce(Result<Connection>) -> R + Send + 'static,
          R: IntoFuture<Item=(), Error=()> + 'static,
          R::Future: 'static
    {
        reactor.spawn(move |handle| Connection::new(&addr, handle, options).then(f))
    }

    pub fn new(addr: &SocketAddr,
               handle: &Handle,
               options: ConnectionOptions)
               -> Box<Future<Item=Connection, Error=Error> + Send> {
        TcpStream::connect(addr, handle)
                  .map_err(Error::from)
                  .and_then(move |stream| -> Result<Negotiate> {
                      stream.set_nodelay(options.nodelay)?;
                      negotiate(options, stream)
                  })
                  .flatten()
                  .and_then(|(options, stream)| -> Result<Connection> {
                      let throttle = options.max_rpcs_in_flight;
                      let mut connection = Connection {
                          options: options,
                          recv_queue: HashMap::new(),
                          failed_rpcs: Vec::new(),
                          stream: stream,
                          read_buf: Buf::new(),
                          write_buf: Buf::new(),
                          throttle: throttle,
                          next_call_id: 0,
                      };
                      connection.buffer_connection_context()?;
                      Ok(connection)
                  }).boxed()
    }

    pub fn throttle(&mut self) {
        self.throttle = cmp::min(self.throttle, self.options.max_rpcs_in_flight) / 2;
    }

    pub fn next_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow. The server will probably complain about this, but there
        // isn't much we can do other than shut down the connection anyway.
        let next = self.next_call_id.checked_add(1).unwrap_or(0);
        self.next_call_id = next;
        next
    }

    /// Writes a session context message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_connection_context(&mut self) -> Result<()> {
        trace!("{:?}: sending connection context to server", self);
        let mut header = rpc_header::RequestHeader::new();
        let mut message = rpc_header::ConnectionContextPB::new();
        header.set_call_id(-3);
        message.mut_user_info().set_effective_user("user".to_string());
        message.mut_user_info().set_real_user("user".to_string());
        buffer_message(&header, &message, &mut self.write_buf)
    }

    fn poll_flush(&mut self) -> Poll<(), Error> {
        poll_flush(&mut self.write_buf, &mut self.stream)
    }

    fn fail_rpc(&mut self, mut rpc: Rpc, error: Error) {
        rpc.fail(error);
        self.failed_rpcs.push(rpc);
    }

    fn shutdown(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both);
        self.failed_rpcs.extend(self.recv_queue.drain().map(|kv| {
            let mut rpc = kv.1;
            rpc.fail(Error::ConnectionError);
            rpc
        }));
    }
}

impl Sink for Connection {
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
    fn start_send(&mut self, mut rpc: Rpc) -> StartSend<Rpc, Error> {
        trace!("{:?}: start_send", self);
        rpc.clear();

        // Internally, this method avoids using the try/try_ready/? operators since the RPC needs
        // to be added to the `failed_rpcs` buffer on error, so that it can be returned during a
        // subsequent call to `poll`.

        // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
        // *still* over 8KiB, then stop sending messages until the buffer clears.
        if self.write_buf.len() > 8 * 1024 {
            if let Err(error) = self.poll_flush() {
                self.shutdown();
                self.fail_rpc(rpc, Error::ConnectionError);
                return Err(error);
            }
            if self.write_buf.len() > 8 * 1024 {
                return Ok(AsyncSink::NotReady(rpc));
            }
        }

        // Check if the connection is throttled.
        if self.recv_queue.len() >= self.throttle as usize {
            // Flush to avoid a situation where the connection is throttled due
            // to buffered messages.
            if let Err(error) = self.poll_flush() {
                self.shutdown();
                self.fail_rpc(rpc, Error::ConnectionError);
                return Err(error);
            }
            return Ok(AsyncSink::NotReady(rpc));
        }

        let now = Instant::now();
        if rpc.timed_out(now) {
            trace!("{:?}: timing out {:?}", self, rpc);
            self.fail_rpc(rpc, Error::TimedOut);
        } else {
            let call_id = self.next_call_id();
            let mut header = rpc_header::RequestHeader::new();
            header.set_call_id(call_id);
            header.mut_remote_method().mut_service_name().push_str(rpc.service());
            header.mut_remote_method().mut_method_name().push_str(rpc.method());
            header.set_timeout_millis(duration_to_ms(&rpc.deadline().duration_since(now)) as u32);
            header.mut_required_feature_flags().extend_from_slice(&rpc.required_feature_flags);

            trace!("{:?}: sending rpc to server; call ID: {}, rpc: {:?}", self, call_id, rpc);
            if let Err(error) = buffer_message(&header, &*rpc.request, &mut self.write_buf) {
                self.shutdown();
                self.fail_rpc(rpc, error);
                return Err(Error::ConnectionError);
            }
            self.recv_queue.insert(call_id as usize, rpc);
        }

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        trace!("{:?}: poll_complete", self);
        if let Err(error) = self.poll_flush() {
            self.shutdown();
            return Err(error);
        }

        if self.recv_queue.is_empty() {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl Stream for Connection {
    type Item = Rpc;
    type Error = !;

    /// Reads an RPC messsage from the socket.
    ///
    /// Returns:
    ///     * `Ok(Async::Ready(Some(rpc))`
    ///         a completed RPC is ready.
    ///     * `Ok(Async::Ready(None))`
    ///         no RPCs are currently in flight.
    ///     * `Ok(Async::NotReady)`
    ///         RPC(s) are in flight, but not currently available.
    fn poll(&mut self) -> Poll<Option<Rpc>, !> {
        trace!("{:?}: poll", self);

        if let Some(failed) = self.failed_rpcs.pop() {
            return Ok(Async::Ready(Some(failed)));
        }

        /// Shuts down the connection, and returns a failed RPC, if there are any.
        fn shutdown_and_return(cxn: &mut Connection) -> Poll<Option<Rpc>, !> {
            cxn.shutdown();
            if let Some(failed) = cxn.failed_rpcs.pop() {
                return Ok(Async::Ready(Some(failed)));
            } else {
                return Ok(Async::NotReady);
            }
        }

        // Parse the next header and body from the socket.
        let (header, body) = match poll_read_message(&self.options,
                                                     &mut self.stream,
                                                     &mut self.read_buf) {
            Ok(Async::Ready((header, body))) => (header, body),
            Ok(Async::NotReady) => if self.recv_queue.is_empty() {
                return Ok(Async::Ready(None));
            } else {
                return Ok(Async::NotReady);
            },
            Err(error) => {
                trace!("{:?}: error reading message: {:?}", self, error);
                return shutdown_and_return(self);
            },
        };

        // Retrieve the RPC corresponding to the message from the receive queue.
        let call_id = header.get_call_id() as usize;
        let mut rpc = match self.recv_queue.remove(&call_id) {
            Some(rpc) => rpc,
            None => {
                warn!("{:?}: received response for non-existent request; header: {:?}",
                      self, header);
                return shutdown_and_return(self);
            },
        };

        if header.get_is_error() {
            // If the RPC is an error, then parse the cause and fail it.
            let error = match parse_length_delimited_from_bytes::<ErrorStatusPB>(&body[..]) {
                Ok(pb) => pb,
                Err(error) => {
                    warn!("{:?}: unable to parse error body; header: {:?}, error: {:?}",
                          self, header, error);
                    return shutdown_and_return(self);
                },
            };
            let error = RpcError::from(error);

            // If the error is fatal, shut down the connection.
            if error.is_fatal() {
                self.shutdown();
            }

            rpc.fail(Error::from(error));
        } else {
            // Parse the response.

            if !header.get_sidecar_offsets().is_empty() {
                panic!("sidecar decoding not implemented");
            }

            if let Err(error) = CodedInputStream::from_bytes(&body[..]).merge_message(rpc.response_mut()) {
                warn!("{:?}: unable to parse response body; header: {:?}, error: {:?}",
                      self, header, error);
                rpc.fail(Error::from(error));
                self.shutdown();
            };
        }

        if self.throttle < self.options.max_rpcs_in_flight {
            self.throttle += 1;
        }

        Ok(Async::Ready(Some(rpc)))
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.stream.peer_addr() {
            Ok(addr) => write!(f, "Connection {{ addr: {}, in-flight: {}, buf (tx/rx): {}/{} }}",
                               addr, self.recv_queue.len(), self.write_buf.len(), self.read_buf.len()),
            Err(error) => write!(f, "Connection {{ addr: {}, in-flight: {}, buf (tx/rx): {}/{} }}",
                                 error, self.recv_queue.len(), self.write_buf.len(), self.read_buf.len()),
        }
    }
}

fn negotiate(options: ConnectionOptions, stream: TcpStream) -> Result<Negotiate> {
    let mut negotiate = Negotiate {
        options: options,
        stream: Some(stream),
        buf: Buf::new(),
    };
    negotiate.buffer_connection_header()?;
    negotiate.buffer_sasl_negotiate()?;
    Ok(negotiate)
}

struct Negotiate {
    options: ConnectionOptions,
    buf: Buf,
    stream: Option<TcpStream>,
}

impl Negotiate {

    /// Writes the KRPC connection header to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_connection_header(&mut self) -> Result<()> {
        trace!("{:?}: sending connection header to server", self);
        self.buf.write_all(b"hrpc\x09\0\0").map_err(From::from)
    }

    /// Writes a SASL negotiate message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_sasl_negotiate(&mut self) -> Result<()> {
        // Write the SASL negotiate message to the send buffer.
        let mut header = rpc_header::RequestHeader::new();
        let mut message = rpc_header::SaslMessagePB::new();
        header.set_call_id(-33);
        message.set_state(SaslState::NEGOTIATE);
        buffer_message(&header, &message, &mut self.buf)
    }

    /// Writes a SASL initiate message to the send buffer.
    ///
    /// Does not flush the buffer.
    ///
    /// If an error is returned, the connection should be torn down.
    fn buffer_sasl_initiate(&mut self) -> Result<()> {
        trace!("{:?}: sending SASL INITIATE request to server", self);
        let mut header = rpc_header::RequestHeader::new();
        let mut message = rpc_header::SaslMessagePB::new();
        header.set_call_id(-33);
        message.set_state(SaslState::INITIATE);
        message.mut_token().extend_from_slice(b"\0user\0");
        let mut auth = rpc_header::SaslMessagePB_SaslAuth::new();
        auth.mut_mechanism().push_str("PLAIN");
        message.mut_auths().push(auth);
        buffer_message(&header, &message, &mut self.buf)
    }

    /// Reads a negotiation message from the socket.
    ///
    /// If an error is returned, the connection should be torn down.
    fn poll_recv(&mut self) -> Poll<rpc_header::SaslMessagePB, Error> {
        let (header, body) = try_ready!(poll_read_message(&self.options,
                                                          self.stream.as_mut().unwrap(),
                                                          &mut self.buf));

        // SASL messages are required to have call ID -33.
        if header.get_call_id() != -33 {
            return Err(Error::Rpc(RpcError::invalid_rpc_header(
                        format!("negotiation RPC response header has illegal call id: {:?}",
                                header.get_call_id()))));

        }

        // SASL messages may not have sidecars.
        if !header.get_sidecar_offsets().is_empty() {
            return Err(Error::Rpc(RpcError::invalid_rpc_header(
                        "negotiation RPC response includes sidecars".to_string())));
        }

        // We only expect a single response message to be in flight during negotiation.
        if !self.buf.is_empty() {
            return Err(Error::NegotiationError(
                    "detected multiple in-flight RPC responses during negotiation"))
        }

        let msg = parse_length_delimited_from_bytes(&body[..])?;
        Ok(Async::Ready(msg))
    }

    fn stream(&mut self) -> &mut TcpStream {
        self.stream.as_mut().take().unwrap()
    }
}

impl Future for Negotiate {
    type Item = (ConnectionOptions, TcpStream);
    type Error = Error;
    fn poll(&mut self) -> Poll<(ConnectionOptions, TcpStream), Error> {
        trace!("{:?}: poll", self);
        loop {
            // Flush buffered negotiation messages.
            try_ready!(poll_flush(&mut self.buf, self.stream.as_mut().unwrap()));

            // Read and handle the negotiation response message.
            let msg = try_ready!(self.poll_recv());
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
                    return Ok(Async::Ready((self.options.clone(), self.stream.take().unwrap())))
                },
                _ => unreachable!("Unexpected SASL message: {:?}", msg),
            }
        }
    }
}

impl fmt::Debug for Negotiate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.stream.as_ref().unwrap().peer_addr() {
            Ok(addr) => write!(f, "Negotiate {{ addr: {}, buf: {} }}", addr, self.buf.len()),
            Err(error) => write!(f, "Negotiate {{ addr: {}, buf: {} }}", error, self.buf.len()),
        }
    }
}
