use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};
use std::u32;

use byteorder::{
    BigEndian,
    ByteOrder,
};
use bytes::{
    Buf,
    BufMut,
    BytesMut,
    IntoBuf,
};
use fnv::FnvHashMap;
use tokio_io::codec::{
    Encoder,
    Decoder,
};

use prost::encoding::encoded_len_varint;
use prost::Message;

use RpcError;
use RpcErrorCode;
use Error;
use Request;
use Rpc;
use pb::rpc::{
    RemoteMethodPb as RemoteMethod,
    RequestHeader,
    ResponseHeader,
};

struct Codec {
    /// Maximum allowable message length.
    ///
    /// Defaults to 5 MiB.
    pub max_message_length: u32,

    pub next_call_id: i32,

    pub request_header: RequestHeader,
    pub response_header: ResponseHeader,

    pub in_flight_rpcs: FnvHashMap<i32, Request>,

    /// Failed RPCs.
    pub failed_rpcs: Vec<Rpc>,
}

impl Codec {
    fn new() -> Codec {
        Codec {
            max_message_length: 5 * 1024 * 1024,
            next_call_id: 0,
            request_header: RequestHeader::default(),
            response_header: ResponseHeader::default(),
            in_flight_rpcs: FnvHashMap::default(),
            failed_rpcs: Vec::new(),
        }
    }

    /// Increments `next_call_id` and returns the current call ID.
    pub fn increment_call_id(&mut self) -> i32 {
        // Wrap back to 0 on overflow. The server will probably complain about this, but there
        // isn't much we can do other than shut down the connection anyway.
        let call_id = self.next_call_id;
        self.next_call_id = call_id.checked_add(1).unwrap_or(0);
        call_id
    }
}

impl Encoder for Codec {
    type Item = Request;
    type Error = io::Error;
    fn encode(&mut self, mut request: Request, dst: &mut BytesMut) -> Result<(), io::Error> {
        let now = Instant::now();
        if request.deadline < now {
            self.failed_rpcs.push(Rpc {
                request,
                response: Err(Error::TimedOut),
            });
            return Ok(());
        }

        // Set the header fields.
        self.request_header.call_id = self.increment_call_id();
        {
            let mut remote_method = self.request_header.remote_method.get_or_insert(RemoteMethod::default());
            remote_method.clear();
            remote_method.service_name.push_str(request.service);
            remote_method.method_name.push_str(request.method);
        }
        self.request_header.timeout_millis = Some(duration_to_ms(request.deadline - now));
        self.request_header.required_feature_flags.clear();
        self.request_header.required_feature_flags.extend_from_slice(request.required_feature_flags);

        let header_len = self.request_header.encoded_len();
        let body_len = request.body.encoded_len();
        let len = encoded_len_varint(header_len as u64)
                + encoded_len_varint(body_len as u64)
                + header_len
                + body_len;

        if len > self.max_message_length as usize {
            self.failed_rpcs.push(Rpc {
                request,
                response: Err(RpcError {
                    code: RpcErrorCode::ErrorInvalidRequest,
                    message: format!("RPC request is too long: length: {}, max length: {}",
                                    len, self.max_message_length),
                    unsupported_feature_flags: Vec::new(),
                }.into()),
            });
            return Ok(())
        }

        dst.put_u32::<BigEndian>(len as u32);
        self.request_header.encode_length_delimited(dst);
        request.body.encode_length_delimited(dst);
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Rpc;
    type Error = Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Rpc>, Error> {
        if let Some(rpc) = self.failed_rpcs.pop() {
            return Ok(Some(rpc));
        }

        // Read the length delimiter, and remove it from `src`.
        if src.len() < 4 {
            return Ok(None);
        }
        let msg_len = BigEndian::read_u32(&*src) as usize;
        if src.len() - 4 < msg_len {
            return Ok(None);
        }
        let _ = src.split_to(4);

        // Decode the message.
        let mut buf = src.split_off(msg_len).into_buf();

        let header = ResponseHeader::decode(&mut buf);

        Ok(None)
    }
}

fn duration_to_ms(duration: Duration) -> u32 {
    let millis = duration.as_secs().saturating_mul(1000)
                         .saturating_add(duration.subsec_nanos() as u64 / 1000_000);
    if millis > u32::MAX as u64 {
        u32::MAX
    } else {
        millis as u32
    }
}
