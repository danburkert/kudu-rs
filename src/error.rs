#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StatusCode {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Status {
    code: StatusCode,
    message: Option<String>,
    posix_code: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TabletServerErrorCode {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TabletServerError {
    code: TabletServerErrorCode,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MasterErrorCode {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MasterError {
    code: MasterErrorCode,
    status: Status,
}
