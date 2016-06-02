use std::error;
use std::fmt;
use std::io;
use std::mem;
use std::result;
use std::time::Instant;

use kudu_pb::rpc_header;
use kudu_pb::rpc_header::{ErrorStatusPB_RpcErrorCodePB as RpcErrorCodePB};

use eventual::Complete;
use protobuf::{Message, ProtobufError};

mod backoff;
mod connection;
mod master;
mod messenger;

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
    /// The `Connection` send queue is full.
    ConnectionQueueFull,
    /// The RPC timed out.
    TimedOut,
}


impl RpcError {
    pub fn is_fatal(&self) -> bool {
        match *self {
            RpcError::Io(..) => true,
            RpcError::Pb(..) => true,
            RpcError::Rpc { code, .. } => code.is_fatal(),
            RpcError::ConnectionQueueFull => false,
            RpcError::TimedOut => false,
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
            RpcError::ConnectionQueueFull => RpcError::ConnectionQueueFull,
            RpcError::TimedOut => RpcError::TimedOut,
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
            RpcError::ConnectionQueueFull => "connection queue full",
            RpcError::TimedOut => "RPC timed out",
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

pub type RpcResult<T> = result::Result<T, RpcError>;

#[derive(Debug)]
pub struct Request {
    service_name: &'static str,
    method_name: &'static str,
    start: Instant,
    deadline: Instant,
    required_feature_flags: Vec<u32>,
    request_message: Box<Message>,
    response_message: Box<Message>,
    complete: Complete<Response, RpcError>,
}

/// The response to an RPC.
#[derive(Debug)]
pub struct Response {
    request_message: Box<Message>,
    response_message: Box<Message>,
    sidecars: Vec<Vec<u8>>,
}

#[cfg(test)]
mod test {

    use super::*;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use rpc::messenger;
    use std::time::{Duration, Instant};
    use env_logger;
    use kudu_pb;
    use eventual::Async;
    use std::thread;
    use std::net::SocketAddr;
    use std::str::FromStr;

    fn threadsafe<T>(_: T) where T: Sync + Send { }

    #[test]
    fn test_ping_master() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let messenger = messenger::Messenger::new().unwrap();
        let master_addr = cluster.master_addrs()[0];
        let future = messenger.send(master_addr,
                                    "kudu.master.MasterService",
                                    "Ping",
                                    Duration::from_millis(10000),
                                    vec![],
                                    Box::new(kudu_pb::master::PingRequestPB::new()),
                                    Box::new(kudu_pb::master::PingResponsePB::new()));

        let response = future.await().unwrap();
    }

    #[test]
    fn test_get_tablet_servers() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let messenger = messenger::Messenger::new().unwrap();
        let master_addr = cluster.master_addrs()[0];
        thread::sleep_ms(1000);
        let future = messenger.send(master_addr,
                                    "kudu.master.MasterService",
                                    "ListTabletServers",
                                    Duration::from_millis(100),
                                    vec![],
                                    Box::new(kudu_pb::master::ListTabletServersRequestPB::new()),
                                    Box::new(kudu_pb::master::ListTabletServersResponsePB::new()));

        let response = future.await().unwrap();
    }

    #[test]
    fn test_rpc_timeout() {
        let _ = env_logger::init();
        let messenger = messenger::Messenger::new().unwrap();
        let addr = SocketAddr::from_str("127.0.0.1:7051").unwrap();
        let future = messenger.send(addr,
                                    "kudu.master.MasterService",
                                    "ListTabletServers",
                                    Duration::from_millis(1000),
                                    vec![],
                                    Box::new(kudu_pb::master::ListTabletServersRequestPB::new()),
                                    Box::new(kudu_pb::master::ListTabletServersResponsePB::new()));

        assert!(future.await().is_err());
    }
}
