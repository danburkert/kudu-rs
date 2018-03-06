#![allow(unused_imports)]
#![allow(unused_variables)]

use std::sync::Arc;

use futures::{
    Async,
    Future,
    Poll,
};
use krpc::Call;

use Error;
use HostPort;
use OperationEncoder;
use TabletId;
use meta_cache::Tablet;
use pb::tserver::{
    WriteRequestPb,
    WriteResponsePb,
};
use retry::RetryFuture;

/// `Batcher` accumulates write operations for a tablet into batches, and keeps stats on buffered
/// and in-flight batches to to the tablet.
pub(crate) struct Batcher {
    /// The inclusive lower bound partition key of the tablet.
    pub lower_bound: PartitionKey,

    /// The exclusive upper bound partition key of the tablet.
    pub upper_bound: PartitionKey,

    /// The batch being accumulated. Buffers operations until it is flushed.
    pub buffer: Buffer,

    /// The amount of data currently in-flight to the tablet.
    pub data_in_flight: usize,

    /// The number of batches currently in-flight to the tablet.
    pub batches_in_flight: u8,
}

impl Batcher {
    fn new(lower_bound: PartitionKey,
           upper_bound: PartitionKey) -> Batcher {
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

struct BatchFuture {
    tablet: Arc<Tablet>,
    partition_key: Vec<u8>,
    operations: usize,
    state: State,
    call: Call<WriteRequestPb, WriteResponsePb>,
}

enum State {
    LeaderLookup(Box<Future<Item=Option<Box<[HostPort]>>, Error=Error>>),
    InFlight(RetryFuture<WriteRequestPb, WriteResponsePb>),
}

impl Future for BatchFuture {
    type Item=BatchResult;
    type Error=BatchResult;
    fn poll(&mut self) -> Result<Async<BatchResult>, BatchResult> {

        let BatchFuture { ref meta_cache,
                          ref tserver_proxies,
                          ref partition_key,
                          operations,
                          ref mut state,
                          ref call } = *self;

        let leaders = match *state {
            State::LeaderLookup(ref mut leader_lookup) => {
                try_ready!(leader_lookup.poll().map_err(|err| {
                    BatchResult {
                        call: call.clone(),
                        response: Err(err),
                    }
                }))
            },
            State::InFlight(ref mut in_flight) => {
                unimplemented!()
            },
        };

        /*
        match leaders {
            Some(leaders) => {
            },
            None => {
            },
        }
        */

        unimplemented!()
    }
}

pub(crate) struct BatchResult {
    call: Call<WriteRequestPb, WriteResponsePb>,
    response: Result<WriteResponsePb, Error>,
}
