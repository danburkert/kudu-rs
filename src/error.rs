use std::error;
use std::fmt;
use std::io;
use std::mem;
use std::result;
use std::str::Utf8Error;

use kudu_pb::master::{
    MasterErrorPB,
    MasterErrorPB_Code as MasterErrorCodePB,
};
use kudu_pb::rpc_header::{
    ErrorStatusPB as RpcErrorPB,
    ErrorStatusPB_RpcErrorCodePB as RpcErrorCodePB
};
use kudu_pb::tserver::{
    TabletServerErrorPB,
    TabletServerErrorPB_Code as TabletServerErrorCodePB,
};
use kudu_pb::wire_protocol::{
    AppStatusPB as StatusPB,
    AppStatusPB_ErrorCode as StatusCodePB,
};
use protobuf::ProtobufError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {

    /// The operation failed because of an invalid argument.
    InvalidArgument(String),

    /// A Kudu RPC error.
    Rpc(RpcError),

    /// A Kudu Master error.
    Master(MasterError),

    /// A Kudu tablet server error.
    TabletServer(TabletServerError),

    /// An I/O error.
    Io(io::Error),

    /// An error serializing, deserializing, encoding, or decoding data.
    Serialization(String),

    /// An error due to a version mismatch between the client and server.
    VersionMismatch(String),

    /// The send queue is full.
    Backoff,

    /// The operation timed out. Includes zero or more errors which resulted in retries.
    TimedOut,

    /// The operation was cancelled.
    Cancelled,

    /// The server closed the connection.
    ConnectionHangup,

    /// The connection errored.
    ConnectionError,

    NegotiationError(&'static str),
}

impl Error {
    pub fn is_network_error(&self) -> bool {
        match *self {
            Error::Io(_) | Error::ConnectionHangup | Error::ConnectionError => true,
            _ => false,
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        match *self {
            Error::InvalidArgument(ref error) => Error::InvalidArgument(error.clone()),
            Error::Rpc(ref error) => Error::Rpc(error.clone()),
            Error::Master(ref error) => Error::Master(error.clone()),
            Error::TabletServer(ref error) => Error::TabletServer(error.clone()),
            Error::Io(ref error) => {
                // We (and our dependencies) never create an IO error with a boxed error object, so
                // this unwrap is ok.
                Error::Io(io::Error::from_raw_os_error(error.raw_os_error().unwrap()))
            },
            Error::Serialization(ref error) => Error::Serialization(error.clone()),
            Error::VersionMismatch(ref error) => Error::VersionMismatch(error.clone()),
            Error::Backoff => Error::Backoff,
            Error::TimedOut => Error::TimedOut,
            Error::Cancelled => Error::Cancelled,
            Error::ConnectionHangup => Error::ConnectionHangup,
            Error::ConnectionError => Error::ConnectionError,
            Error::NegotiationError(error) => Error::NegotiationError(error),
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::InvalidArgument(ref a), &Error::InvalidArgument(ref b)) => a == b,
            (&Error::Rpc(ref a), &Error::Rpc(ref b)) => a == b,
            (&Error::Master(ref a), &Error::Master(ref b)) => a == b,
            (&Error::TabletServer(ref a), &Error::TabletServer(ref b)) => a == b,
            (&Error::Io(ref a), &Error::Io(ref b)) => a.raw_os_error().unwrap() == b.raw_os_error().unwrap(),
            (&Error::Serialization(ref a), &Error::Serialization(ref b)) => a == b,
            (&Error::VersionMismatch(ref a), &Error::VersionMismatch(ref b)) => a == b,
            (&Error::Backoff, &Error::Backoff) => true,
            (&Error::TimedOut, &Error::TimedOut) => true,
            (&Error::Cancelled, &Error::Cancelled) => true,
            (&Error::ConnectionHangup, &Error::ConnectionHangup) => true,
            (&Error::ConnectionError, &Error::ConnectionError) => true,
            (&Error::NegotiationError(ref a), &Error::NegotiationError(ref b)) => a == b,
            _ => false,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidArgument(_) => "illegal argument",
            Error::Rpc(ref error) => error.description(),
            Error::Master(ref error) => error.description(),
            Error::TabletServer(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::Serialization(ref description) => &description,
            Error::VersionMismatch(ref description) => &description,
            Error::Backoff => "backoff",
            Error::TimedOut => "operation timed out",
            Error::Cancelled => "operation cancelled",
            Error::ConnectionHangup => "connection hangup",
            Error::ConnectionError => "connection error",
            Error::NegotiationError(error) => error,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidArgument(_) => None,
            Error::Rpc(ref error) => error.cause(),
            Error::Master(ref error) => error.cause(),
            Error::TabletServer(ref error) => error.cause(),
            Error::Io(ref error) => error.cause(),
            Error::Serialization(_) => None,
            Error::VersionMismatch(_) => None,
            Error::Backoff => None,
            Error::TimedOut => None,
            Error::Cancelled => None,
            Error::ConnectionHangup => None,
            Error::ConnectionError => None,
            Error::NegotiationError(_) => None,
        }
    }
}

impl From<RpcError> for Error {
    fn from(error: RpcError) -> Error {
        Error::Rpc(error)
    }
}

impl From<MasterError> for Error {
    fn from(error: MasterError) -> Error {
        Error::Master(error)
    }
}

impl From<TabletServerError> for Error {
    fn from(error: TabletServerError) -> Error {
        Error::TabletServer(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<ProtobufError> for Error {
    fn from(error: ProtobufError) -> Error {
        match error {
            ProtobufError::IoError(error) => Error::Io(error),
            ProtobufError::WireError(msg) => Error::Serialization(msg),
            ProtobufError::MessageNotInitialized { message } =>
                // This should never happen, all Protobuf messages are initialized internally.
                panic!("Protobuf message not initialized: {}", message),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Error {
        Error::Serialization(error.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RpcError {
    code: RpcErrorCode,
    message: String,
    unsupported_feature_flags: Vec<u32>,
}

impl RpcError {
    pub fn invalid_rpc_header(message: String) -> RpcError {
        RpcError {
            code: RpcErrorCode::FatalInvalidRpcHeader,
            message: message,
            unsupported_feature_flags: Vec::new(),
        }
    }
    pub fn is_fatal(&self) -> bool {
        match self.code {
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

impl error::Error for RpcError {
    fn description(&self) -> &str {
        match self.code {
            RpcErrorCode::ApplicationError => "application error",
            RpcErrorCode::NoSuchMethod => "no such method",
            RpcErrorCode::NoSuchService => "no such service",
            RpcErrorCode::ServerTooBusy => "server too busy",
            RpcErrorCode::InvalidRequest => "invalid request",
            RpcErrorCode::FatalUnknown => "unknown error",
            RpcErrorCode::FatalServerShuttingDown => "server shutting down",
            RpcErrorCode::FatalInvalidRpcHeader => "invalid RPC header",
            RpcErrorCode::FatalDeserializingRequest => "error deserializing request",
            RpcErrorCode::FatalVersionMismatch => "version mismatch",
            RpcErrorCode::FatalUnauthorized => "unauthorized",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<RpcErrorPB> for RpcError {
    fn from(mut error: RpcErrorPB) -> RpcError {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusCode {
    UnknownError,
    NotFound,
    Corruption,
    NotSupported,
    InvalidArgument,
    IoError,
    AlreadyPresent,
    RuntimeError,
    NetworkError,
    IllegalState,
    NotAuthorized,
    Aborted,
    RemoteError,
    ServiceUnavailable,
    TimedOut,
    Uninitialized,
    ConfigurationError,
    Incomplete,
    EndOfFile,
}

impl From<StatusCodePB> for StatusCode {
    fn from(code: StatusCodePB) -> StatusCode {
        match code {
            StatusCodePB::UNKNOWN_ERROR => StatusCode::UnknownError,
            StatusCodePB::OK => unreachable!("shouldn't be accessing an OK status"),
            StatusCodePB::NOT_FOUND => StatusCode::NotFound,
            StatusCodePB::CORRUPTION => StatusCode::Corruption,
            StatusCodePB::NOT_SUPPORTED => StatusCode::NotSupported,
            StatusCodePB::INVALID_ARGUMENT => StatusCode::InvalidArgument,
            StatusCodePB::IO_ERROR => StatusCode::IoError,
            StatusCodePB::ALREADY_PRESENT => StatusCode::AlreadyPresent,
            StatusCodePB::RUNTIME_ERROR => StatusCode::RuntimeError,
            StatusCodePB::NETWORK_ERROR => StatusCode::NetworkError,
            StatusCodePB::ILLEGAL_STATE => StatusCode::IllegalState,
            StatusCodePB::NOT_AUTHORIZED => StatusCode::NotAuthorized,
            StatusCodePB::ABORTED => StatusCode::Aborted,
            StatusCodePB::REMOTE_ERROR => StatusCode::RemoteError,
            StatusCodePB::SERVICE_UNAVAILABLE => StatusCode::ServiceUnavailable,
            StatusCodePB::TIMED_OUT => StatusCode::TimedOut,
            StatusCodePB::UNINITIALIZED => StatusCode::Uninitialized,
            StatusCodePB::CONFIGURATION_ERROR => StatusCode::ConfigurationError,
            StatusCodePB::INCOMPLETE => StatusCode::Incomplete,
            StatusCodePB::END_OF_FILE => StatusCode::EndOfFile,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    code: StatusCode,
    message: Option<String>,
    posix_code: Option<i32>,
}

impl From<StatusPB> for Status {
    fn from(mut status: StatusPB) -> Status {
        Status {
            code: StatusCode::from(status.get_code()),
            message: if status.has_message() { Some(status.take_message()) } else { None },
            posix_code: if status.has_posix_code() { Some(status.get_posix_code()) } else { None },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TabletServerErrorCode {
    /// An error which has no more specific error code. The code and message in 'status' may reveal
    /// more details.
    UnknownError,
    /// The schema provided for a request was not well-formed.
    InvalidSchema,
    /// The row data provided for a request was not well-formed.
    InvalidRowBlock,
    /// The mutations or mutation keys provided for a request were not well formed.
    InvalidMutation,
    /// The schema provided for a request didn't match the actual schema of the tablet.
    MismatchedSchema,
    /// The requested tablet ID is not currently hosted on the server.
    TabletNotFound,
    /// A request was made against a scanner ID that was either never created or has expired.
    ScannerExpired,
    /// An invalid scan was specified -- e.g the values passed for predicates were incorrect sizes.
    InvalidScanSpec,
    /// The provided configuration was not well-formed and/or had a sequence number that was below
    /// the current config.
    InvalidConfig,
    /// On a create tablet request, signals that the tablet already exists.
    TabletAlreadyExists,
    /// If the tablet has a newer schema than the requested one the "alter" request will be
    /// rejected with this error.
    TabletHasANewerSchema,
    /// The tablet is hosted on this server, but not in `RUNNING` state.
    TabletNotRunning,
    /// Client requested a snapshot read but the snapshot was invalid.
    InvalidSnapshot,
    /// An invalid scan call sequence ID was specified.
    InvalidScanCallSeqId,
    /// This tserver is not the leader of the consensus configuration.
    NotTheLeader,
    /// The destination UUID in the request does not match this server.
    WrongServerUuid,
    /// The compare-and-swap specified by an atomic RPC operation failed.
    CasFailed,
    /// The requested operation is already inprogress, e.g. RemoteBootstrap.
    AlreadyInProgress,
    /// The request is throttled.
    Throttled,
}

impl From<TabletServerErrorCodePB> for TabletServerErrorCode {
    fn from(error: TabletServerErrorCodePB) -> TabletServerErrorCode {
        match error {
            TabletServerErrorCodePB::UNKNOWN_ERROR => TabletServerErrorCode::UnknownError,
            TabletServerErrorCodePB::INVALID_SCHEMA => TabletServerErrorCode::InvalidSchema,
            TabletServerErrorCodePB::INVALID_ROW_BLOCK => TabletServerErrorCode::InvalidRowBlock,
            TabletServerErrorCodePB::INVALID_MUTATION => TabletServerErrorCode::InvalidMutation,
            TabletServerErrorCodePB::MISMATCHED_SCHEMA => TabletServerErrorCode::MismatchedSchema,
            TabletServerErrorCodePB::TABLET_NOT_FOUND => TabletServerErrorCode::TabletNotFound,
            TabletServerErrorCodePB::SCANNER_EXPIRED => TabletServerErrorCode::ScannerExpired,
            TabletServerErrorCodePB::INVALID_SCAN_SPEC => TabletServerErrorCode::InvalidScanSpec,
            TabletServerErrorCodePB::INVALID_CONFIG => TabletServerErrorCode::InvalidConfig,
            TabletServerErrorCodePB::TABLET_ALREADY_EXISTS => TabletServerErrorCode::TabletAlreadyExists,
            TabletServerErrorCodePB::TABLET_HAS_A_NEWER_SCHEMA => TabletServerErrorCode::TabletHasANewerSchema,
            TabletServerErrorCodePB::TABLET_NOT_RUNNING => TabletServerErrorCode::TabletNotRunning,
            TabletServerErrorCodePB::INVALID_SNAPSHOT => TabletServerErrorCode::InvalidSnapshot,
            TabletServerErrorCodePB::INVALID_SCAN_CALL_SEQ_ID => TabletServerErrorCode::InvalidScanCallSeqId,
            TabletServerErrorCodePB::NOT_THE_LEADER => TabletServerErrorCode::NotTheLeader,
            TabletServerErrorCodePB::WRONG_SERVER_UUID => TabletServerErrorCode::WrongServerUuid,
            TabletServerErrorCodePB::CAS_FAILED => TabletServerErrorCode::CasFailed,
            TabletServerErrorCodePB::ALREADY_INPROGRESS => TabletServerErrorCode::AlreadyInProgress,
            TabletServerErrorCodePB::THROTTLED => TabletServerErrorCode::Throttled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TabletServerError {
    code: TabletServerErrorCode,
    status: Status,
}

impl error::Error for TabletServerError {
    fn description(&self) -> &str {
        match self.code {
            TabletServerErrorCode::UnknownError => "unknown error",
            TabletServerErrorCode::InvalidSchema => "invalid schema",
            TabletServerErrorCode::InvalidRowBlock => "invalid row block",
            TabletServerErrorCode::InvalidMutation => "invalid mutation",
            TabletServerErrorCode::MismatchedSchema => "mismatched schema",
            TabletServerErrorCode::TabletNotFound => "tablet not found",
            TabletServerErrorCode::ScannerExpired => "scanner expired",
            TabletServerErrorCode::InvalidScanSpec => "invalid scan spec",
            TabletServerErrorCode::InvalidConfig => "invalid config",
            TabletServerErrorCode::TabletAlreadyExists => "tablet already exists",
            TabletServerErrorCode::TabletHasANewerSchema => "tablet has a newer schema",
            TabletServerErrorCode::TabletNotRunning => "tablet not running",
            TabletServerErrorCode::InvalidSnapshot => "invalid snapshot",
            TabletServerErrorCode::InvalidScanCallSeqId => "invalid scan call sequence id",
            TabletServerErrorCode::NotTheLeader => "not the leader",
            TabletServerErrorCode::WrongServerUuid => "wrong server UUID",
            TabletServerErrorCode::CasFailed => "CAS failed",
            TabletServerErrorCode::AlreadyInProgress => "already in progress",
            TabletServerErrorCode::Throttled => "throttled",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for TabletServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<TabletServerErrorPB> for TabletServerError {
    fn from(mut error: TabletServerErrorPB) -> TabletServerError {
        TabletServerError {
            code: TabletServerErrorCode::from(error.get_code()),
            status: Status::from(error.take_status()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MasterErrorCode {
    /// An error which has no more specific error code. The `Status` code and message may reveal
    /// more details.
    UnknownError,
    /// The schema provided for a request was not well-formed.
    InvalidSchema,
    /// The requested table does not exist
    TableNotFound,
    /// The name requested for the table is already in use
    TableAlreadyPresent,
    /// The number of tablets requested for a new table is over the per TS limit.
    TooManyTablets,
    /// Catalog manager is not yet initialized.
    CatalogManagerNotInitialized,
    /// The operation attempted can only be invoked against either the leader or a single
    /// non-distributed master, which this node isn't.
    NotTheLeader,
    /// The number of replicas requested is greater than the number of live servers in the cluster.
    ReplicationFactorTooHigh,
    /// A tablet involved in the operation is not running.
    TabletNotRunning,
}

impl From<MasterErrorCodePB> for MasterErrorCode {
    fn from(error: MasterErrorCodePB) -> MasterErrorCode {
        match error {
            MasterErrorCodePB::UNKNOWN_ERROR => MasterErrorCode::UnknownError,
            MasterErrorCodePB::INVALID_SCHEMA => MasterErrorCode::InvalidSchema,
            MasterErrorCodePB::TABLE_NOT_FOUND => MasterErrorCode::TableNotFound,
            MasterErrorCodePB::TABLE_ALREADY_PRESENT => MasterErrorCode::TableAlreadyPresent,
            MasterErrorCodePB::TOO_MANY_TABLETS => MasterErrorCode::TooManyTablets,
            MasterErrorCodePB::CATALOG_MANAGER_NOT_INITIALIZED => MasterErrorCode::CatalogManagerNotInitialized,
            MasterErrorCodePB::NOT_THE_LEADER => MasterErrorCode::NotTheLeader,
            MasterErrorCodePB::REPLICATION_FACTOR_TOO_HIGH => MasterErrorCode::ReplicationFactorTooHigh,
            MasterErrorCodePB::TABLET_NOT_RUNNING => MasterErrorCode::TabletNotRunning,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MasterError {
    code: MasterErrorCode,
    status: Status,
}

impl MasterError {
    pub fn new(code: MasterErrorCode, status: Status) -> MasterError {
        MasterError {
            code: code,
            status: status,
        }
    }
    pub fn code(&self) -> MasterErrorCode {
        self.code
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
}

impl error::Error for MasterError {
    fn description(&self) -> &str {
        match self.code {
            MasterErrorCode::UnknownError => "unknown error",
            MasterErrorCode::InvalidSchema => "invalid schema",
            MasterErrorCode::TableNotFound => "table not found",
            MasterErrorCode::TableAlreadyPresent => "table already exists",
            MasterErrorCode::TooManyTablets => "too many tablets",
            MasterErrorCode::CatalogManagerNotInitialized => "catalog manager not initialized",
            MasterErrorCode::NotTheLeader => "not the leader",
            MasterErrorCode::ReplicationFactorTooHigh => "replication factor too high",
            MasterErrorCode::TabletNotRunning => "tablet not running",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for MasterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<MasterErrorPB> for MasterError {
    fn from(mut error: MasterErrorPB) -> MasterError {
        MasterError {
            code: MasterErrorCode::from(error.get_code()),
            status: Status::from(error.take_status()),
        }
    }
}
