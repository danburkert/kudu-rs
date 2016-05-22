use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::io::{self, ErrorKind, Write};
use std::thread::{self, JoinHandle};
use std::error;
use std::fmt;
use std::time::Instant;
use std::collections::hash_map::Entry;

use kudu_pb::rpc_header;
use kudu_pb::rpc_header::{SaslMessagePB_SaslState as SaslState};
use rpc::messenger::Loop;
use rpc::{Request, Response, RpcError, RpcResult};

use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use eventual::{Future, Complete};
use mio::{
    EventLoop,
    EventSet,
    Handler,
    PollOpt,
    Sender,
    Token,
};
use mio::tcp::TcpStream;
use protobuf::{parse_length_delimited_from, Clear, CodedInputStream, Message, ProtobufError};
use protobuf::rt::ProtobufVarint;
use slab::Slab;
use netbuf::Buf;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    Initiating,
    Connected
}

#[derive(Debug)]
pub struct Connection {
    state: ConnectionState,
    stream: TcpStream,
    addr: SocketAddr,
    send_queue: VecDeque<Request>,
    recv_queue: HashMap<i32, Request>,
    request_header: rpc_header::RequestHeader,
    response_header: rpc_header::ResponseHeader,
    recv_buf: Buf,
    send_buf: Buf,
}

impl Connection {

    pub fn new(event_loop: &mut Loop, token: Token, addr: SocketAddr) -> RpcResult<Connection> {
        debug!("initiating new new connection to remote address {:?}", addr);

        let mut cxn = Connection {
            state: ConnectionState::Initiating,
            stream: try!(TcpStream::connect(&addr)),
            addr: addr,
            send_queue: VecDeque::new(),
            recv_queue: HashMap::new(),
            request_header: rpc_header::RequestHeader::new(),
            response_header: rpc_header::ResponseHeader::new(),
            recv_buf: Buf::new(),
            send_buf: Buf::new(),
        };

        // Optimistically flush the connection header and SASL negotiation to the TCP socket. Even
        // though the socket hasn't yet been registered, and the connection is probably not yet
        // complete, this will usually succeed because the socket will have sufficient internal
        // buffer space.
        try!(cxn.send_connection_header());
        try!(cxn.send_sasl_negotiate());
        try!(cxn.flush());

        let event_set = cxn.event_set();
        let poll_opt = cxn.poll_opt();
        try!(event_loop.register(&mut cxn.stream, token, event_set, poll_opt));
        Ok(cxn)
    }

    /// Initiates message reads and writes bsaed on the provided event set, and connection state.
    /// If an error is returned, the connection should be torn down.
    pub fn ready(&mut self, events: EventSet) -> RpcResult<()> {
        match self.state {
            ConnectionState::Initiating => {
                if events.is_readable() {
                    assert!(!events.is_writable());
                    assert!(self.send_buf.is_empty());
                    try!(self.recv())
                } else if events.is_writable() {
                    assert!(!events.is_readable());
                    assert!(self.recv_buf.is_empty());
                    try!(self.send())
                }
            },
            ConnectionState::Connected => {

            },
        };
        Ok(())
    }

    pub fn send_request(&mut self, request: Request) -> RpcResult<()> {
        // TODO: implement maximum queue size
        self.send_queue.push_back(request);
        if self.state == ConnectionState::Connected && self.send_buf.is_empty() && self.send_queue.len() == 1 {
            try!(self.send());
        }
        Ok(())
    }

    /// Adds the message to the send buffer with connection's request header. Does not flush the
    /// buffer. If an error is returned, the connection should be torn down.
    fn send_message(&mut self, msg: &mut Message) -> RpcResult<()> {
        trace!("Adding message to send buffer; header: {:?}, message: {:?}",
               self.request_header, msg);
        let header_len = self.request_header.compute_size();
        let msg_len = msg.compute_size();
        let len = header_len + header_len.len_varint() + msg_len + msg_len.len_varint();

        try!(self.send_buf.write_u32::<BigEndian>(len));
        try!(self.request_header.write_to_with_cached_sizes(&mut self.send_buf));
        try!(msg.write_to_with_cached_sizes(&mut self.send_buf));
        Ok(())
    }

    /// Adds the KRPC connection header to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_connection_header(&mut self) -> RpcResult<()> {
        try!(self.send_buf.write(b"hrpc\x09\0\0"));
        Ok(())
    }

    /// Adds a SASL negotiate message to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_sasl_negotiate(&mut self) -> RpcResult<()> {
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::NEGOTIATE);
        self.send_message(&mut msg)
    }

    /// Adds a SASL initiate message to the send buffer. Does not flush the buffer. If an error is
    /// returned, the connection should be torn down.
    fn send_sasl_initiate(&mut self) -> RpcResult<()> {
        self.request_header.clear();
        self.request_header.set_call_id(-33);
        let mut msg = rpc_header::SaslMessagePB::new();
        msg.set_state(SaslState::INITIATE);
        let mut auth = rpc_header::SaslMessagePB_SaslAuth::new();
        auth.mut_mechanism().push_str("PLAIN");
        msg.mut_auths().push(auth);
        self.send_message(&mut msg)
    }

    /// Adds a session context message to the send buffer. Does not flush the buffer. If an error
    /// is returned, the connection should be torn down.
    fn send_connection_context(&mut self) -> RpcResult<()> {
        self.request_header.clear();
        self.request_header.set_call_id(-3);
        let mut msg = rpc_header::ConnectionContextPB::new();
        msg.mut_user_info().mut_effective_user().push_str("user");
        msg.mut_user_info().mut_real_user().push_str("user");
        self.send_message(&mut msg)
    }

    fn handle_sasl_message(&mut self, msg: rpc_header::SaslMessagePB) -> RpcResult<()> {
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
                // Set the call ID to -1, so that the the next message sent will increment it to 0.
                self.request_header.set_call_id(-1);

                // Optimistically flush the connection context and send any queued messages. The
                // connection has not necessarily received a writeable event at this point, but it
                // is highly likely that there is space available in the socket's write buffer.
                self.send()
            },
            _ => panic!("Unexpected SASL message: {:?}", msg),
        }
    }

    /// Receive messages until no more messages are available on the socket. Should be called when
    /// the connection's socket is readable. If an error is returned, the connection should be torn
    /// down.
    fn recv(&mut self) -> RpcResult<()> {
        loop {
            // Read, or continue reading, a message from the socket into the receive buffer.
            if self.recv_buf.len() < 4 {
                let needed = 4 - self.recv_buf.len();
                if try!(self.read(needed)) < needed {
                    debug!("incomplete message length read");
                    return Ok(());
                }
            }

            let msg_len = BigEndian::read_u32(&self.recv_buf[..4]) as usize;
            // TODO: inject max message length configuration
            if self.recv_buf.len() - 4 < msg_len {
                let needed = msg_len + 4 - self.recv_buf.len();
                if try!(self.read(needed)) < needed {
                    debug!("incomplete message read");
                    return Ok(());
                }
            }

            // The whole message has been read
            self.recv_buf.consume(4);

            // Read the response header into self.response_header
            self.response_header.clear();
            let header_len = {
                let mut coded_stream = CodedInputStream::from_bytes(&self.recv_buf[..]);
                coded_stream.merge_message(&mut self.response_header);
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
                        let error = RpcError::from(try!(
                                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                    &mut CodedInputStream::from_bytes(&self.recv_buf[..]))));
                        self.recv_buf.consume(msg_len - header_len);
                        // All errors during SASL negotiation should result in tearing down the
                        // connection.
                        return Err(error)
                    }

                    let msg: rpc_header::SaslMessagePB = try!(parse_length_delimited_from(
                            &mut CodedInputStream::from_bytes(&self.recv_buf[..])));
                    self.recv_buf.consume(msg_len - header_len);
                    self.handle_sasl_message(msg);
                },
                ConnectionState::Connected => {
                    if self.response_header.get_is_error() {
                        let error = RpcError::from(try!(
                                parse_length_delimited_from::<rpc_header::ErrorStatusPB>(
                                    &mut CodedInputStream::from_bytes(&self.recv_buf[..]))));
                        // Remove the request from the recv queue, and fail the completion.
                        let request = self.recv_queue.remove(&self.response_header.get_call_id());
                        if let Some(request) = request {
                            request.complete.fail(error.clone());
                        }
                        // If the message is fatal, then return an error in order to have the
                        // connection torn down.
                        if error.is_fatal() {
                            return Err(error.clone())
                        }
                    } else {
                        // Use the entry API so that the request is not removed from the recv queue
                        // if the protobuf decode step fails. Since it isn't removed, it will be
                        // retried when the error is bubbled up to the MessengerHandler.
                        match self.recv_queue.entry(self.response_header.get_call_id()) {
                            Entry::Occupied(mut entry) => {
                                try!(entry.get_mut().response_message.merge_from(
                                        &mut CodedInputStream::from_bytes(&self.recv_buf[..])));

                                let Request { request_message, mut response_message, mut complete, .. } = entry.remove();
                                if !self.response_header.get_sidecar_offsets().is_empty() {
                                    panic!("sidecar decoding not implemented");
                                }
                                let sidecars = Vec::new();

                                complete.complete(Response {
                                    request_message: request_message,
                                    response_message: response_message,
                                    sidecars: sidecars,
                                });
                            },
                            _ => {
                                // The request has already been removed from the recv queue, most
                                // likely due to a timeout.
                            }
                        }
                    }
                },
            };
            self.recv_buf.consume(msg_len - header_len);
        }
    }

    /// Send messages until either there are no more messages to send, or the socket can not accept
    /// any more writes. If an error is returned, the connection should be torn down.
    fn send(&mut self) -> RpcResult<()> {
        loop {
            match self.state {
                ConnectionState::Initiating => {
                    try!(self.flush());
                    return Ok(());
                },
                ConnectionState::Connected => {
                    try!(self.flush());
                    if !self.send_buf.is_empty() { return Ok(()); }

                    if let Some(req) = self.send_queue.pop_front() {
                        let Request { service_name, method_name, timeout,
                                      required_feature_flags, request_message,
                                      response_message, complete } = req;

                        // TODO: handle timeout

                        let call_id = self.request_header.get_call_id() + 1;
                        self.request_header.set_call_id(call_id);
                        self.request_header.mut_remote_method().mut_service_name().clear();
                        self.request_header.mut_remote_method().mut_method_name().clear();
                        self.request_header.mut_remote_method().mut_service_name().push_str(service_name);
                        self.request_header.mut_remote_method().mut_method_name().push_str(method_name);
                        self.request_header.set_timeout_millis(10000);
                        self.request_header.set_required_feature_flags(required_feature_flags);

                        let header_len = self.request_header.compute_size();
                        let msg_len = request_message.compute_size();
                        let len = header_len + header_len.len_varint() +
                                  msg_len + msg_len.len_varint();

                        try!(self.send_buf.write_u32::<BigEndian>(len));
                        try!(self.request_header.write_to_with_cached_sizes(&mut self.send_buf));
                        try!(request_message.write_to_with_cached_sizes(&mut self.send_buf));
                        assert_eq!(4 + len as usize, self.send_buf.len());
                    }
                },
            };
        }
    }

    /// Attempts to read at least `min` bytes from the socket into the receive buffer.
    /// Fewer bytes may be read if there is no data available.
    fn read(&mut self, min: usize) -> io::Result<usize> {
        let Connection { ref mut stream, ref mut recv_buf, .. } = *self;
        let mut received = 0;
        while received < min {
            match recv_buf.read_from(stream) {
                Ok(amount) => received += amount,
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => return Err(error),
            }
        }
        Ok(received)
    }

    /// Flushes the send buffer to the socket, returning the total number of bytes sent.
    fn flush(&mut self) -> io::Result<usize> {
        let Connection { ref mut stream, ref mut send_buf, .. } = *self;
        let mut sent = 0;
        while !send_buf.is_empty() {
            match send_buf.write_to(stream) {
                Ok(amount) => sent += amount,
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => return Err(error),
            }
        }
        Ok(sent)
    }

    fn poll_opt(&self) -> PollOpt {
        PollOpt::edge() | PollOpt::oneshot()
    }

    fn event_set(&self) -> EventSet {
        let mut event_set = EventSet::hup() | EventSet::error() | EventSet::readable();
        if !self.send_buf.is_empty() || !self.send_queue.is_empty() {
            event_set | EventSet::writable()
        } else { event_set }
    }
}
