use std::io::{
    self,
    Read,
    Write,
};
use std::net::SocketAddr;
use std::time::{
    Instant,
    Duration,
};
use std::u32;

use byteorder::{BigEndian, ByteOrder};
use bytes::{
    BufMut,
    Bytes,
    BytesMut,
    IntoBuf,
};
use futures::{
    Async,
    Future,
};
use prost::Message;
use prost::encoding::encoded_len_varint;
use tokio::net::{
    TcpStream,
    TcpStreamNew,
};
use tokio::reactor::Handle;

use RpcError;
use RpcErrorCode;
use Error;
use Request;
use RequestBody;
use pb::rpc::{
    ErrorStatusPb,
    RemoteMethodPb,
    RequestHeader,
    ResponseHeader,
};

const INITIAL_CAPACITY: usize = 8 * 1024;
const BACKPRESSURE_BOUNDARY: usize = INITIAL_CAPACITY;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportOptions {

    /// Maximum allowable message length.
    ///
    /// Defaults to 5 MiB.
    pub max_message_length: u32,

    /// Whether to disable Nagle's algorithm.
    ///
    /// Defaults to true.
    pub nodelay: bool,
}

impl Default for TransportOptions {
    fn default() -> TransportOptions {
        TransportOptions {
            max_message_length: 5 * 1024 * 1024,
            nodelay: true,
        }
    }
}

pub enum Response {
    Ok {
        call_id: i32,
        body: Bytes,
        sidecars: Vec<Bytes>,
    },
    Err {
        call_id: i32,
        error: RpcError,
    },
}

pub struct Transport {
    options: TransportOptions,
    stream: TcpStream,
    send_buf: BytesMut,
    recv_buf: BytesMut,
    request_header: RequestHeader,
    response_header: ResponseHeader,
}

impl Transport {

    pub fn connect(options: TransportOptions, addr: &SocketAddr, handle: &Handle) -> TransportNew {
        TransportNew {
            options,
            stream: TcpStream::connect(addr, handle),
        }
    }

    /// Start sending an RPC on the transport.
    ///
    /// If a fatal error is returned, the transport must be torn down. If a non-fatal error
    /// is returned, the RPC should be failed.
    pub fn start_send(&mut self, call_id: i32, request: &Request) -> Result<Async<()>, Error> {
        // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
        // *still* over 8KiB, then apply backpressure (reject the send).
        if self.send_buf.len() >= BACKPRESSURE_BOUNDARY {
            self.poll_flush()?;

            if self.send_buf.len() >= BACKPRESSURE_BOUNDARY {
                return Ok(Async::NotReady);
            }
        }

        self.send(call_id,
                  request.service,
                  request.method,
                  request.required_feature_flags,
                  &*request.body,
                  request.deadline)?;
        Ok(Async::Ready(()))
    }

    pub(crate) fn send(&mut self,
                       call_id: i32,
                       service: &str,
                       method: &str,
                       required_feature_flags: &[u32],
                       body: &RequestBody,
                       deadline: Instant) -> Result<(), Error> {
        let now = Instant::now();
        if deadline < now {
            return Err(Error::TimedOut);
        }

        // Set the header fields.
        self.request_header.call_id = call_id;
        {
            let remote_method = self.request_header.remote_method.get_or_insert(RemoteMethodPb::default());
            remote_method.clear();
            remote_method.service_name.push_str(service);
            remote_method.method_name.push_str(method);
        }
        self.request_header.timeout_millis = Some(duration_to_ms(deadline - now));
        self.request_header.required_feature_flags.clear();
        self.request_header.required_feature_flags.extend_from_slice(required_feature_flags);

        let header_len = Message::encoded_len(&self.request_header);
        let body_len = body.encoded_len();
        let len = encoded_len_varint(header_len as u64)
                + encoded_len_varint(body_len as u64)
                + header_len
                + body_len;

        if len > self.options.max_message_length as usize {
            return Err(RpcError {
                code: RpcErrorCode::ErrorInvalidRequest,
                message: format!("RPC request exceeds maximum length ({}/{})",
                len, self.options.max_message_length),
                unsupported_feature_flags: Vec::new(),
            }.into());
        }

        self.send_buf.put_u32::<BigEndian>(len as u32);
        Message::encode_length_delimited(&self.request_header, &mut self.send_buf).unwrap();
        body.encode_length_delimited(&mut self.send_buf);

        Ok(())
    }

    pub fn poll(&mut self) -> Result<Async<Response>, Error> {
        self.poll_flush()?;
        self.poll_recv()
    }

    fn poll_recv(&mut self) -> Result<Async<Response>, Error> {
        // Read, or continue reading, an RPC response message from the socket into the receive
        // buffer. Every RPC response is prefixed with a 4 bytes length header.
        if self.recv_buf.len() < 4 {
            let needed = 4 - self.recv_buf.len();
            try_ready!(self.poll_fill(needed));
        }

        let msg_len = BigEndian::read_u32(&self.recv_buf[..4]) as usize;
        if msg_len > self.options.max_message_length as usize {
            return Err(Error::Serialization(format!(
                       "RPC response exceeds maximum length ({}/{})",
                       msg_len, self.options.max_message_length)));
        }
        if self.recv_buf.len() - 4 < msg_len {
            let needed = msg_len + 4 - self.recv_buf.len();
            try_ready!(self.poll_fill(needed));
        }
        let _ = self.recv_buf.split_to(4);
        let buf = self.recv_buf.split_to(msg_len).freeze();

        // Decode the header.
        let header_len = {
            let mut cursor = buf.clone().into_buf();
            self.response_header.clear();
            self.response_header.merge_length_delimited(&mut cursor)?;
            cursor.position() as usize
        };

        let mut buf = buf.slice_from(header_len);
        let call_id = self.response_header.call_id;
        if self.response_header.is_error() {
            let error = ErrorStatusPb::decode_length_delimited(buf)?.into();
            Ok(Async::Ready(Response::Err {
                call_id,
                error,
            }))
        } else if self.response_header.sidecar_offsets.is_empty() {
            Ok(Async::Ready(Response::Ok {
                call_id,
                body: buf,
                sidecars: Vec::new(),
            }))
        } else {
            let mut prev_offset = self.response_header.sidecar_offsets[0] as usize;
            let body = buf.split_to(prev_offset);

            let mut sidecars = Vec::new();
            for &offset in &self.response_header.sidecar_offsets[1..] {
                let offset = offset as usize;
                sidecars.push(buf.split_to(offset - prev_offset));
                prev_offset = offset;
            }
            sidecars.push(buf);

            Ok(Async::Ready(Response::Ok {
                call_id,
                body,
                sidecars,
            }))
        }
    }

    /// Reads at least `at_least` bytes into the receive buffer.
    ///
    /// Based on tokio-io's [`FramedRead`][1] and [`AsyncRead`][2].
    /// [1]: https://github.com/tokio-rs/tokio-io/blob/master/src/framed_read.rs#L259-L294
    /// [2]: https://github.com/tokio-rs/tokio-io/blob/0.1.3/src/lib.rs#L138-L157
    fn poll_fill(&mut self, mut at_least: usize) -> Result<Async<()>, io::Error> {
        self.recv_buf.reserve(at_least);

        while at_least > 0 {
            let n = unsafe {
                let n = try_nb!(self.stream.read(self.recv_buf.bytes_mut()));
                self.recv_buf.advance_mut(n);
                n
            };

            at_least = at_least.saturating_sub(n);

            if n == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
        }

        Ok(Async::Ready(()))
    }

    /// Flushes bytes from send buffer to the stream.
    ///
    /// Based on tokio-io's [`FramedWrite`][1].
    /// [1]: https://github.com/tokio-rs/tokio-io/blob/0.1.3/src/framed_write.rs#L202-L225
    fn poll_flush(&mut self) -> Result<Async<()>, io::Error> {
        while !self.send_buf.is_empty() {
            let n = try_nb!(self.stream.write(&self.send_buf));

            if n == 0 {
                return Err(io::Error::from(io::ErrorKind::WriteZero));
            }

            let _ = self.send_buf.split_to(n);
        }

        Ok(Async::Ready(()))
    }
}

/// Future returned by `Transport::connect` which will resolve to a `Transport` when the TCP stream
/// is connected.
pub struct TransportNew {
    options: TransportOptions,
    stream: TcpStreamNew,
}

impl Future for TransportNew {
    type Item = Transport;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Transport>, io::Error> {
        let stream = try_ready!(self.stream.poll());
        stream.set_nodelay(self.options.nodelay)?;

        // Write the connection header to the send buffer.
        let mut send_buf = BytesMut::with_capacity(INITIAL_CAPACITY);
        send_buf.put_slice(b"hrpc\x09\0\0");

        Ok(Async::Ready(Transport {
            options: self.options.clone(),
            stream,
            send_buf,
            recv_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
            request_header: RequestHeader::default(),
            response_header: ResponseHeader::default(),
        }))
    }
}

fn duration_to_ms(duration: Duration) -> u32 {
    let millis = duration.as_secs()
                         .saturating_mul(1000)
                         .saturating_add(duration.subsec_nanos() as u64 / 1000_000);
    if millis > u32::MAX as u64 {
        u32::MAX
    } else {
        millis as u32
    }
}
