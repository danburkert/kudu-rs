use futures::{
    Async,
    Future,
    Poll,
};
use krpc;

use pb::tserver::{
    //TabletServerService,
    WriteRequestPb,
    WriteResponsePb,
};
use OperationEncoder;
use tserver;
use Error;
use HostPort;
use meta_cache::MetaCache;
use TabletId;

/// `Batcher` accumulates write operations for a tablet into batches, and keeps stats on buffered
/// and in-flight batches to to the tablet.
pub(crate) struct Batcher {
    /// The batch being accumulated. Buffers operations until it is flushed.
    pub buffer: Buffer,

    /// A partition key belonging to the tablet. Used for meta cache lookups.
    pub partition_key: Vec<u8>,

    /// The amount of data currently in-flight to the tablet.
    pub data_in_flight: usize,

    /// The number of batches currently in-flight to the tablet.
    pub batches_in_flight: u8,
}

impl Batcher {
    fn new(partition_key: Vec<u8>) -> Batcher {
        Batcher {
            buffer: Buffer::new(),
            partition_key,
            data_in_flight: 0,
            batches_in_flight: 0,
        }
    }
}

/// Holds buffered operations for a tablet until they are flushed.
pub(crate) struct Buffer {
    operations: usize,
    buffer: OperationEncoder,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            operations: 0,
            buffer: OperationEncoder::new(),
        }
    }
}

struct InFlight {
    meta_cache: MetaCache,
    tserver_proxies: tserver::ProxyCache,
    tablet: TabletId,
    partition_key: Vec<u8>,
    operations: usize,
    state: InFlightState,
}

enum InFlightState {
    LeaderLookup {
        call: krpc::Call<WriteRequestPb, WriteResponsePb>,
        lookup: Box<Future<Item=Option<Box<[HostPort]>>, Error=Error>>,
    },
    InFlight {
        //response: krpc::Response<WriteResponsePb>,
    },
}

impl Future for InFlight {
    type Item=BatchResult;
    type Error=BatchResult;
    fn poll(&mut self) -> Result<Async<BatchResult>, BatchResult> {
        unimplemented!()
    }
}

pub(crate) struct BatchResult {
    request: Box<WriteRequestPb>,
    response: Result<WriteResponsePb, Error>,
}
