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
use meta_cache::{
    TableLocations,
    Tablet,
};
use pb::tserver::{
    WriteRequestPb,
    WriteResponsePb,
};
use retry::RetryFuture;
use PartitionKey;

/// `Batcher` accumulates write operations for a tablet into batches, and keeps stats on buffered
/// and in-flight batches to to the tablet.
pub(crate) struct Batcher {

    table_locations: TableLocations,

    tablet: Arc<Tablet>,

    tablet_future: Option<Box<Future<Item=Option<Arc<Tablet>>, Error=Error>>>,

    /// The batch being accumulated. Buffers operations until it is flushed.
    buffer: Buffer,
}

impl Batcher {
    fn new(table_locations: TableLocations, tablet: Arc<Tablet>) -> Batcher {
        Batcher {
            table_locations,
            tablet,
            tablet_future: None,
            buffer: Buffer::new(),
        }
    }

    fn poll_ready(&mut self) -> Poll<(), Error> {
        while self.tablet.is_stale() {
            // TODO: do some kind of backoff here so we aren't spamming the metastore.

            // NLL hack
            let Batcher { ref table_locations, ref mut tablet, ref mut tablet_future, .. } = *self;
            let tablet_future = tablet_future.get_or_insert_with(|| {
                Box::new(table_locations.tablet(tablet.lower_bound()))
            });

            if let Some(new_tablet) = try_ready!(tablet_future.poll()) {
                if tablet.id() != new_tablet.id() {
                    // TODO
                    unimplemented!("partitioning has changed");
                }
                *tablet = new_tablet;
            } else {
                unimplemented!("partitioning has changed to non-covered range");
            }
        }

        Ok(Async::Ready(()))
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

/*
struct BatchError {
    tablet: Tablet,
    buffer: Buffer,
    error: Error,
}

struct BatchFuture {
    tablet: Box<Future<Item=BatchResult, Error=BatchError>>,
    lower_bound: PartitionKey,
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
*/
