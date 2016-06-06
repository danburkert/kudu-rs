use std::any::Any;
use std::error;
use std::fmt;
use std::io;
use std::mem;
use std::net::SocketAddr;
use std::result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use kudu_pb::rpc_header;
use kudu_pb::rpc_header::{ErrorStatusPB_RpcErrorCodePB as RpcErrorCodePB};

use protobuf::{Message, ProtobufError};

pub use rpc::messenger::Messenger;

mod connection;
pub mod master;
mod messenger;
pub mod tablet_server;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RpcErrorCode {
    // Non-fatal RPC errors. Connection should be left open for future RPC calls.

    /// The application generated an error status. See the message field for
    /// more details.
    ApplicationError,
    /// The specified method was not valid.
    NoSuchMethod,
    /// The specified service was not valid.
    NoSuchService,
    /// The server is overloaded - the client should try again shortly.
    ServerTooBusy,
    /// The request parameter was not parseable, was missing required fields,
    /// or the server does not support the required feature flags.
    InvalidRequest,

    // Fatal errors indicate that the client should shut down the connection.

    FatalUnknown,
    /// The RPC server is already shutting down.
    FatalServerShuttingDown,
    /// Fields of RpcHeader are invalid.
    FatalInvalidRpcHeader,
    /// Could not deserialize RPC request.
    FatalDeserializingRequest,
    /// IPC Layer version mismatch.
    FatalVersionMismatch,
    /// Auth failed.
    FatalUnauthorized,
}

impl RpcErrorCode {
    pub fn is_fatal(&self) -> bool {
        match *self {
            RpcErrorCode::FatalUnknown |
            RpcErrorCode::FatalServerShuttingDown |
            RpcErrorCode::FatalInvalidRpcHeader |
            RpcErrorCode::FatalDeserializingRequest |
            RpcErrorCode::FatalVersionMismatch |
            RpcErrorCode::FatalUnauthorized => true,
            _ => false,
        }
    }
}

/// An internal error type returned by RPC operations.
#[derive(Debug)]
pub enum RpcError {
    /// An IO error.
    Io(io::Error),
    /// A Protobuf error.
    Pb(String),
    /// The RPC completed, but the server was not able to service the request.
    Rpc {
        code: RpcErrorCode,
        message: String,
        unsupported_feature_flags: Vec<u32>,
    },
    /// The send queue is full.
    Backoff,
    /// The RPC timed out.
    TimedOut,
    /// The RPC was cancelled.
    Cancelled,
}

impl RpcError {
    pub fn is_fatal(&self) -> bool {
        match *self {
            RpcError::Io(..) => true,
            RpcError::Pb(..) => true,
            RpcError::Rpc { code, .. } => code.is_fatal(),
            RpcError::Backoff => false,
            RpcError::TimedOut => false,
            RpcError::Cancelled => false,
        }
    }
    pub fn invalid_rpc_header(message: String) -> RpcError {
        RpcError::Rpc {
            code: RpcErrorCode::FatalInvalidRpcHeader,
            message: message,
            unsupported_feature_flags: Vec::new(),
        }
    }
}

impl Clone for RpcError {
    fn clone(&self) -> RpcError {
        match *self {
            RpcError::Io(ref e) => RpcError::Io(io::Error::new(e.kind(),
                                                               error::Error::description(e).clone())),
            RpcError::Pb(ref msg) => RpcError::Pb(msg.clone()),
            RpcError::Rpc { ref code, ref message, ref unsupported_feature_flags } => {
                RpcError::Rpc {
                    code: code.clone(),
                    message: message.clone(),
                    unsupported_feature_flags: unsupported_feature_flags.clone()
                }
            },
            RpcError::Backoff => RpcError::Backoff,
            RpcError::TimedOut => RpcError::TimedOut,
            RpcError::Cancelled => RpcError::Cancelled,
        }
    }
}


impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for RpcError {
    fn description(&self) -> &str {
        match *self {
            RpcError::Io(ref error) => error.description(),
            RpcError::Pb(ref msg) => msg,
            RpcError::Rpc { ref message, .. } => message,
            RpcError::Backoff => "backoff",
            RpcError::TimedOut => "RPC timed out",
            RpcError::Cancelled => "RPC cancelled",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RpcError::Io(ref error) => error.cause(),
            _ => None,
        }
    }
}

impl From<rpc_header::ErrorStatusPB> for RpcError {
    fn from(mut error: rpc_header::ErrorStatusPB) -> RpcError {
        let code = match error.get_code() {
            RpcErrorCodePB::FATAL_UNKNOWN => RpcErrorCode::FatalUnknown,
            RpcErrorCodePB::ERROR_APPLICATION => RpcErrorCode::ApplicationError,
            RpcErrorCodePB::ERROR_NO_SUCH_METHOD => RpcErrorCode::NoSuchMethod,
            RpcErrorCodePB::ERROR_NO_SUCH_SERVICE => RpcErrorCode::NoSuchService,
            RpcErrorCodePB::ERROR_SERVER_TOO_BUSY => RpcErrorCode::ServerTooBusy,
            RpcErrorCodePB::ERROR_INVALID_REQUEST => RpcErrorCode::InvalidRequest,
            RpcErrorCodePB::FATAL_SERVER_SHUTTING_DOWN => RpcErrorCode::FatalServerShuttingDown,
            RpcErrorCodePB::FATAL_INVALID_RPC_HEADER => RpcErrorCode::FatalInvalidRpcHeader,
            RpcErrorCodePB::FATAL_DESERIALIZING_REQUEST => RpcErrorCode::FatalDeserializingRequest,
            RpcErrorCodePB::FATAL_VERSION_MISMATCH => RpcErrorCode::FatalVersionMismatch,
            RpcErrorCodePB::FATAL_UNAUTHORIZED => RpcErrorCode::FatalUnauthorized,
        };

        let message = mem::replace(error.mut_message(), String::new());
        let unsupported_feature_flags = mem::replace(error.mut_unsupported_feature_flags(), Vec::new());

        RpcError::Rpc {
            code: code,
            message: message,
            unsupported_feature_flags: unsupported_feature_flags,
        }
    }
}

impl From<io::Error> for RpcError {
    fn from(error: io::Error) -> RpcError {
        RpcError::Io(error)
    }
}

impl From<ProtobufError> for RpcError {
    fn from(error: ProtobufError) -> RpcError {
        match error {
            ProtobufError::IoError(error) => RpcError::Io(error),
            ProtobufError::WireError(msg) => RpcError::Pb(msg),
            ProtobufError::MessageNotInitialized { message } =>
                // This should never happen, all Protobuf messages are initialized internally.
                panic!("Protobuf message not initialized: {}", message),
        }
    }
}

pub type RpcResult = result::Result<(), RpcError>;

/// A callback that will be executed when the RPC is complete. If the RPC succeeds, the result will
/// be `Ok`, and the RPC will contain the response and sidecars. Othewise, the result will contain
/// the failure.
pub trait Callback: Send {
    fn callback(self: Box<Self>, result: RpcResult, rpc: Rpc);

}

impl<F> Callback for F where F: FnOnce(RpcResult, Rpc) + Send {
    fn callback(self: Box<F>, result: RpcResult, rpc: Rpc) {
        (*self)(result, rpc)
    }
}

pub struct Rpc {
    pub addr: SocketAddr,
    pub service_name: &'static str,
    pub method_name: &'static str,
    pub deadline: Instant,
    pub required_feature_flags: Vec<u32>,
    pub request: Box<Message>,
    pub response: Box<Message>,
    pub sidecars: Vec<Vec<u8>>,
    pub callback: Option<Box<Callback>>,
    pub cancel: Option<Arc<AtomicBool>>,
}

impl Rpc {
    fn complete(mut self) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Ok(()), self)
        }
    }

    fn fail(mut self, error: RpcError) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Err(error), self)
        }
    }

    pub fn response<T>(&self) -> &T where T: Any {
        self.response.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn mut_response<T>(&mut self) -> &mut T where T: Any {
        self.response.as_any_mut().downcast_mut::<T>().unwrap()
    }

    /// Returns `true` if this RPC has been requested to be cancelled.
    pub fn cancelled(&self) -> bool {
        self.cancel.as_ref().map(|b| b.load(Ordering::Relaxed)).unwrap_or(false)
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    pub fn timed_out(&self, now: Instant) -> bool {
        self.deadline <= now
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rpc {{ service: {}, method: {}, deadline: {:?} }}",
               self.service_name, self.method_name, self.deadline)
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;
    use std::sync::Arc;

    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use super::*;

    use env_logger;
    use kudu_pb;

    #[test]
    fn test_master_ping() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));
        let messenger = Messenger::new().unwrap();
        let rpc = master::ping(cluster.master_addrs()[0],
                               Instant::now() + Duration::from_secs(5),
                               kudu_pb::master::PingRequestPB::new());

        let (result, _rpc) = messenger.send_sync(rpc);
        result.unwrap();
    }

    #[test]
    fn test_list_masters() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(3)
                                                         .num_tservers(0));
        let messenger = Messenger::new().unwrap();

        let (send, recv) = sync_channel::<(RpcResult, Rpc)>(3);

        ::std::thread::sleep_ms(5000);

        for &master in cluster.master_addrs() {
            info!("master addr: {:?}", master);
            let mut rpc = master::list_masters(master, Instant::now() + Duration::from_secs(5),
                                               kudu_pb::master::ListMastersRequestPB::new());
            let send = send.clone();
            rpc.callback = Some(Box::new(move |result, rpc| {
                send.send((result, rpc)).unwrap();
            }));
            messenger.send(rpc);
        }

        drop(send);

        for (result, rpc) in recv {
            assert!(result.is_ok());
            info!("response: {:?}", rpc.response::<kudu_pb::master::ListMastersResponsePB>());
            info!("response size: {:?}", rpc.response::<kudu_pb::master::ListMastersResponsePB>().get_masters().len());
            info!("rpc addr: {:?}", &rpc.addr);
        }
    }
}
