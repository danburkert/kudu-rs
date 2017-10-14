use std::error;
use std::fmt;
use std::io;
use std::result;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use pb::master::MasterErrorPb;
use pb::tserver::TabletServerErrorPb;
use pb::{AppStatusPb as StatusPb};

pub use pb::master::master_error_pb::{Code as MasterErrorCode};
pub use pb::tserver::tablet_server_error_pb::{Code as TabletServerErrorCode};
pub use pb::app_status_pb::{ErrorCode as StatusCode};

use krpc;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {

    /// The operation failed because of an invalid argument.
    InvalidArgument(String),

    /// A Kudu RPC error.
    Rpc(krpc::RpcError),

    /// A Kudu Master error.
    Master(MasterError),

    /// A Kudu tablet server error.
    TabletServer(TabletServerError),

    /// An I/O error.
    Io(io::Error),

    /// An error serializing, deserializing, encoding, or decoding data.
    Serialization(String),

    /// The operation timed out.
    TimedOut,

    Negotiation(String),

    /// An operation failed because the range partition did not exist.
    NoRangePartition,

    /// A compound error.
    Compound(String, Vec<Error>),
}

impl Error {
    pub(crate) fn is_retriable(&self) -> bool {
        match *self {
            Error::Rpc(ref error) => error.is_retriable(),
            Error::Master(ref error) => error.is_retriable(),
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
            Error::Io(ref error) => Error::Io(io::Error::from(error.kind())),
            Error::Serialization(ref error) => Error::Serialization(error.clone()),
            Error::TimedOut => Error::TimedOut,
            Error::Negotiation(ref error) => Error::Negotiation(error.clone()),
            Error::NoRangePartition => Error::NoRangePartition,
            Error::Compound(ref description, ref errors) => Error::Compound(description.clone(), errors.clone()),
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
            Error::Serialization(ref description) => description,
            Error::TimedOut => "operation timed out",
            Error::Negotiation(ref error) => error,
            Error::NoRangePartition => "no range partition",
            Error::Compound(ref description, _) => description,
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
            Error::TimedOut => None,
            Error::Negotiation(_) => None,
            Error::NoRangePartition => None,
            Error::Compound(_, ref errors) => errors.iter().next().map(|error| error as _),
        }
    }
}

impl From<krpc::Error> for Error {
    fn from(error: krpc::Error) -> Error {
        match error {
            krpc::Error::Rpc(error) => Error::Rpc(error),
            krpc::Error::Io(error) => Error::Io(error),
            krpc::Error::Serialization(msg) => Error::Serialization(msg),
            krpc::Error::TimedOut => Error::TimedOut,
            krpc::Error::Negotiation(msg) => Error::Negotiation(msg),
        }
    }
}

impl From<krpc::RpcError> for Error {
    fn from(error: krpc::RpcError) -> Error {
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

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Error {
        Error::Serialization(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::Serialization(error.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub struct Status {
    code: StatusCode,
    message: Option<String>,
    posix_code: Option<i32>,
}

impl Status {
    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_ref().map(String::as_str)
    }

    pub fn posix_code(&self) -> Option<i32> {
        self.posix_code
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?}", self.code));
        if let Some(code) = self.posix_code {
            try!(write!(f, "({})", code));
        }
        if let Some(ref message) = self.message {
            try!(write!(f, ": {}", message));
        }
        Ok(())
    }
}

impl From<StatusPb> for Status {
    fn from(status: StatusPb) -> Status {
        Status {
            code: status.code(),
            message: status.message,
            posix_code: status.posix_code,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TabletServerError {
    pub code: TabletServerErrorCode,
    pub status: Status,
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
            TabletServerErrorCode::AlreadyInprogress => "already in progress",
            TabletServerErrorCode::Throttled => "throttled",
            TabletServerErrorCode::TabletFailed => "tablet failed",
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

impl From<TabletServerErrorPb> for TabletServerError {
    fn from(error: TabletServerErrorPb) -> TabletServerError {
        TabletServerError {
            code: TabletServerErrorCode::from(error.code()),
            status: Status::from(error.status),
        }
    }
}

impl From<StatusPb> for TabletServerError {
    fn from(error: StatusPb) -> TabletServerError {
        TabletServerError {
            code: TabletServerErrorCode::UnknownError,
            status: Status::from(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MasterError {
    pub code: MasterErrorCode,
    pub status: Status,
}

impl MasterError {
    pub fn is_retriable(&self) -> bool {
        match self.code {
            MasterErrorCode::CatalogManagerNotInitialized => true,
            MasterErrorCode::TabletNotRunning => true,
            _ => false,
        }
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
            MasterErrorCode::EvenReplicationFactor => "even replication factor",
            MasterErrorCode::IllegalReplicationFactor => "illegal replication factor",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for MasterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}: {:?}", self.description(), self.status)
    }
}

impl From<MasterErrorPb> for MasterError {
    fn from(error: MasterErrorPb) -> MasterError {
        MasterError {
            code: error.code(),
            status: error.status.into(),
        }
    }
}
