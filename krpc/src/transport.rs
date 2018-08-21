use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr};
use std::time::Duration;
use std::u32;

use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, Bytes, BytesMut, IntoBuf};
use futures::{Async, Future, Poll};
use prost::{decode_length_delimiter, length_delimiter_len, Message};
use tokio::net::{ConnectFuture, TcpStream};

use pb::rpc::{ErrorStatusPb, RemoteMethodPb, RequestHeader, ResponseHeader};
use Error;
use Options;
use RequestBody;
use RpcError;
use RpcErrorCode;

const INITIAL_CAPACITY: usize = 8 * 1024;
const BACKPRESSURE_BOUNDARY: usize = INITIAL_CAPACITY;

pub type TransportResponse = (Bytes, Vec<BytesMut>);

/// `Transport` handles sending and receiving raw KRPC messages to a TCP stream.
///
/// The transport manages send and receive buffers, encoding and decoding of messages, message
/// framing, headers, and RPC errors.
///
/// The transport wraps a single TCP connection. When the TCP connection is shutdown or fails, the
/// transport should no longer be used. TCP connection shutdown is indicated by a fatal error being
/// returned from `poll_ready()`, `send()`, or `poll()`.
pub(crate) struct Transport {
    addr: SocketAddr,
    options: Options,
    stream: TcpStream,
    send_buf: BytesMut,
    recv_buf: BytesMut,
    request_header: RequestHeader,
    response_header: ResponseHeader,
}

impl Transport {
    /// Returns a future which will yield a new transport.
    pub fn connect(addr: SocketAddr, options: Options) -> TransportNew {
        let connect = TcpStream::connect(&addr);
        TransportNew {
            addr,
            options,
            connect,
        }
    }

    /// Returns `true` if the transport is ready to send an RPC to the peer.
    ///
    /// An error return indicates a fatal error.
    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        let result = || -> Poll<(), Error> {
            // If the buffer is already over 8KiB, then attempt to flush it. If after flushing it's
            // *still* over 8KiB, then apply backpressure (reject the send).
            if self.send_buf.len() >= BACKPRESSURE_BOUNDARY {
                self.poll_flush()?;

                if self.send_buf.len() >= BACKPRESSURE_BOUNDARY {
                    return Ok(Async::NotReady);
                }
            }
            Ok(Async::Ready(()))
        }();

        if result.is_err() {
            let _ = self.stream.shutdown(Shutdown::Both);
        }

        result
    }

    /// Sends an RPC to the peer.
    ///
    /// This method does not provide backpressure, so callers should always check that `poll_ready`
    /// indicates that there is send capacity available.
    ///
    /// If a fatal error is returned the transport is shut down. If a non-fatal error is returned,
    /// the RPC should be failed.
    pub fn send(
        &mut self,
        call_id: i32,
        service: &str,
        method: &str,
        required_feature_flags: &[u32],
        body: &RequestBody,
        timeout: Option<Duration>,
    ) -> Result<(), Error> {
        let result = || -> Result<(), Error> {
            // Set the header fields.
            self.request_header.call_id = call_id;
            {
                let remote_method = self
                    .request_header
                    .remote_method
                    .get_or_insert(RemoteMethodPb::default());
                remote_method.clear();
                remote_method.service_name.push_str(service);
                remote_method.method_name.push_str(method);
            }
            if let Some(timeout) = timeout {
                self.request_header.timeout_millis = Some(duration_to_ms(timeout));
            }
            self.request_header.required_feature_flags.clear();
            self.request_header
                .required_feature_flags
                .extend_from_slice(required_feature_flags);

            let header_len = Message::encoded_len(&self.request_header);
            let body_len = body.encoded_len();
            let len = length_delimiter_len(header_len)
                + length_delimiter_len(body_len)
                + header_len
                + body_len;

            if len > self.options.max_message_length as usize {
                return Err(RpcError {
                    code: RpcErrorCode::ErrorInvalidRequest,
                    message: format!(
                        "RPC request exceeds maximum length ({}/{})",
                        len, self.options.max_message_length
                    ),
                    unsupported_feature_flags: Vec::new(),
                }.into());
            }

            self.send_buf.put_u32_be(len as u32);
            Message::encode_length_delimited(&self.request_header, &mut self.send_buf).unwrap();
            body.encode_length_delimited(&mut self.send_buf);
            Ok(())
        }();

        if let Err(ref error) = result {
            if error.is_fatal() {
                let _ = self.stream.shutdown(Shutdown::Both);
            }
        }
        result
    }

    /// Attempts to receive a response from the peer.
    pub fn poll(&mut self) -> Poll<(i32, Result<TransportResponse, Error>), Error> {
        self.poll_flush()?;
        self.poll_recv()
    }

    /// Attempts to read a response from the TCP stream.
    fn poll_recv(&mut self) -> Poll<(i32, Result<TransportResponse, Error>), Error> {
        let result = || -> Poll<(i32, Result<TransportResponse, Error>), Error> {
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
                    msg_len, self.options.max_message_length
                )));
            }
            if self.recv_buf.len() - 4 < msg_len {
                let needed = msg_len + 4 - self.recv_buf.len();
                try_ready!(self.poll_fill(needed));
            }
            let _ = self.recv_buf.split_to(4);
            let mut buf = self.recv_buf.split_to(msg_len);

            // Decode the header.
            let header_len = {
                let mut cursor = buf.clone().into_buf();
                self.response_header.clear();
                self.response_header.merge_length_delimited(&mut cursor)?;
                cursor.position() as usize
            };

            buf.split_to(header_len);
            let call_id = self.response_header.call_id;
            if self.response_header.is_error() {
                let error = Error::Rpc(ErrorStatusPb::decode_length_delimited(buf)?.into());
                Ok(Async::Ready((call_id, Err(error))))
            } else {
                // KRPC inserts a len integer before the main message whose value is the length of
                // the main message and sidecars. This is completely useless since this can be
                // solved for easily using the header length and the overall length. Unfortunately
                // stripping this integer is not trivial, since it's variable length. In order to
                // know its width we are forced to read it. Who designed this crap?
                //
                // There's probably a way to solve for the width of the varint based on the
                // remaining length of the buffer, but it's unfortunately not as simple as just
                // calling length_delimiter_len since the buffer contains the varint itself.
                let main_message_len = decode_length_delimiter(&mut (&buf).into_buf()).unwrap();
                buf.split_to(length_delimiter_len(main_message_len));

                let mut sidecars = Vec::new();
                let body;

                if self.response_header.sidecar_offsets.is_empty() {
                    body = buf.freeze();
                } else {
                    let mut prev_offset = self.response_header.sidecar_offsets[0] as usize;
                    body = buf.split_to(prev_offset).freeze();

                    for &offset in &self.response_header.sidecar_offsets[1..] {
                        let offset = offset as usize;
                        sidecars.push(buf.split_to(offset - prev_offset));
                        prev_offset = offset;
                    }
                    sidecars.push(buf);
                }

                Ok(Async::Ready((call_id, Ok((body, sidecars)))))
            }
        }();

        let is_fatal = match result {
            Ok(Async::Ready((_, Err(ref error)))) => error.is_fatal(),
            Err(_) => true,
            _ => false,
        };
        if is_fatal {
            let _ = self.stream.shutdown(Shutdown::Both);
        }

        result
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
    ///
    /// An error return indicates a fatal error.
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

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn options(&self) -> &Options {
        &self.options
    }

    pub fn send_buf_len(&self) -> usize {
        self.send_buf.len()
    }

    pub fn recv_buf_len(&self) -> usize {
        self.recv_buf.len()
    }
}

/// Future returned by `Transport::connect` which will resolve to a `Transport` when the TCP stream
/// is connected.
pub(crate) struct TransportNew {
    addr: SocketAddr,
    options: Options,
    connect: ConnectFuture,
}

impl Future for TransportNew {
    type Item = Transport;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Transport>, io::Error> {
        let stream = try_ready!(self.connect.poll());
        stream.set_nodelay(self.options.nodelay)?;

        // Write the connection header to the send buffer.
        let mut send_buf = BytesMut::with_capacity(INITIAL_CAPACITY);
        send_buf.put_slice(b"hrpc\x09\0\0");

        Ok(Async::Ready(Transport {
            addr: self.addr,
            options: self.options.clone(),
            stream,
            send_buf,
            recv_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
            request_header: RequestHeader::default(),
            response_header: ResponseHeader::default(),
        }))
    }
}

/// Converts a duration to milliseconds.
fn duration_to_ms(duration: Duration) -> u32 {
    let millis = duration
        .as_secs()
        .saturating_mul(1000)
        .saturating_add(u64::from(duration.subsec_nanos()) / 1_000_000);
    if millis > u64::from(u32::MAX) {
        u32::MAX
    } else {
        millis as u32
    }
}
