use std::error;
use std::fmt;
use std::io;

use prost;

use pb::rpc::ErrorStatusPb;

pub use pb::rpc::error_status_pb::{RpcErrorCodePb as RpcErrorCode};

/// An RPC error.
#[derive(Debug)]
pub enum Error {

    /// A Kudu RPC error.
    Rpc(RpcError),

    /// An I/O error.
    Io(io::Error),

    /// An error serializing, deserializing, encoding, or decoding data.
    Serialization(String),

    /// The RPC timed out.
    TimedOut,

    /// Negotiation with the remote server failed.
    NegotiationError(&'static str),
}

impl Clone for Error {
    fn clone(&self) -> Error {
        match *self {
            Error::Rpc(ref error) => Error::Rpc(error.clone()),
            Error::Io(ref error) => {
                match error.raw_os_error() {
                    Some(error) => Error::Io(io::Error::from_raw_os_error(error)),
                    // TODO: this is not a full copy in all cases.
                    None => Error::Io(io::Error::from(error.kind())),
                }
            },
            Error::Serialization(ref error) => Error::Serialization(error.clone()),
            Error::TimedOut => Error::TimedOut,
            Error::NegotiationError(error) => Error::NegotiationError(error),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Rpc(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::Serialization(ref error) => error,
            Error::TimedOut => "RPC timed out",
            Error::NegotiationError(error) => error,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => error.cause(),
            _ => None,
        }
    }
}

impl From<RpcError> for Error {
    fn from(error: RpcError) -> Error {
        Error::Rpc(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(error: prost::DecodeError) -> Error {
        Error::Serialization(error.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// An error returned by a remote server in response to an RPC.
#[derive(Debug, Clone, PartialEq)]
pub struct RpcError {
    /// The error kind.
    pub code: RpcErrorCode,
    /// The error message.
    pub message: String,
    /// The unsupported feature flags, if the error code is `ErrorInvalidRequest`.
    pub unsupported_feature_flags: Vec<u32>,
}

impl RpcError {

    /// Returns `true` if the error is fatal.
    ///
    /// Fatal errors cause the connection to the server to be reset.
    pub fn is_fatal(&self) -> bool {
        match self.code {
            RpcErrorCode::FatalDeserializingRequest |
            RpcErrorCode::FatalInvalidAuthenticationToken |
            RpcErrorCode::FatalInvalidRpcHeader |
            RpcErrorCode::FatalServerShuttingDown |
            RpcErrorCode::FatalUnauthorized |
            RpcErrorCode::FatalUnknown |
            RpcErrorCode::FatalVersionMismatch  => true,
            _ => false,
        }
    }

    /// Returns `true` if the request can be retried.
    pub fn is_retriable(&self) -> bool {
        self.code == RpcErrorCode::ErrorServerTooBusy
    }
}

impl error::Error for RpcError {
    fn description(&self) -> &str {
        match self.code {
            RpcErrorCode::ErrorApplication => "application error",
            RpcErrorCode::ErrorInvalidRequest => "invalid request",
            RpcErrorCode::ErrorNoSuchMethod => "no such method",
            RpcErrorCode::ErrorNoSuchService => "no such service",
            RpcErrorCode::ErrorRequestStale => "request stale",
            RpcErrorCode::ErrorServerTooBusy => "server too busy",
            RpcErrorCode::ErrorUnavailable => "unavailable",

            RpcErrorCode::FatalDeserializingRequest => "error deserializing request",
            RpcErrorCode::FatalInvalidAuthenticationToken => "invalid authentication token",
            RpcErrorCode::FatalInvalidRpcHeader => "invalid RPC header",
            RpcErrorCode::FatalServerShuttingDown => "server shutting down",
            RpcErrorCode::FatalUnauthorized => "unauthorized",
            RpcErrorCode::FatalVersionMismatch => "version mismatch",

            RpcErrorCode::FatalUnknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<ErrorStatusPb> for RpcError {
    fn from(error: ErrorStatusPb) -> RpcError {
        let code = error.code();
        let message = error.message;
        let unsupported_feature_flags = error.unsupported_feature_flags;

        RpcError {
            code: code,
            message: message,
            unsupported_feature_flags: unsupported_feature_flags,
        }
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
