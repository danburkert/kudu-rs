#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::mem;
use std::sync::Arc;
use std::time::Instant;

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
use TabletServerId;
use backoff::Backoff;
use meta_cache::{
    TableLocations,
    Tablet,
    TabletReplica,
};
use pb::tserver::{
    WriteRequestPb,
    WriteResponsePb,
};
use retry::RetryFuture;
use timer;
use PartitionKey;
use util::{
    RetryWithBackoff,
    RetryCause,
};

pub(crate) struct Batch {

    table_locations: TableLocations,
    tablet: Arc<Tablet>,

    tablet_future: Option<Box<Future<Item=Option<Arc<Tablet>>, Error=Error>>>,

    /// The batch being accumulated. Buffers operations until it is flushed.
    buffer: Buffer,
}

impl Batch {
    fn new(table_locations: TableLocations, tablet: Arc<Tablet>) -> Batch {
        Batch {
            table_locations,
            tablet,
            tablet_future: None,
            buffer: Buffer::new(),
        }
    }

    /*
    fn poll_ready(&mut self) -> Poll<(), Error> {
        loop {
            while self.tablet.is_stale() {
                // TODO: do some kind of backoff here so we aren't spamming the metastore.

                // NLL hack
                let Batch { ref table_locations, ref mut tablet, ref mut tablet_future, .. } = *self;
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

            let leader = match self.tablet.leader_replica() {
                Some(leader) => leader.proxy(),
                None => {
                    // Refresh the tablet.
                    //self.tablet.mark_stale();
                    continue;
                },
            };

            //let mut message = WriteRequestPb::default();
            //let ops = self.buffer.buffer.into_pb();
            //message.row_operations = Some(ops);

            return Ok(Async::Ready(()))
        }
    }
    */
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

enum BatchState {

    Initial,

    /// The tablet metadata is being refreshed.
    TabletRefresh {
        tablet: Box<Future<Item=Option<Arc<Tablet>>, Error=Error>>,
    },

    /// The request is in-flight.
    InFlight {
        tserver: TabletServerId,
    },

    /// The batch failed to write due to a retriable error, another attempt will be made after a
    /// waiting period.
    Waiting {
        sleep: timer::Sleep,
        request: Box<WriteRequestPb>,
    },
}

struct BatchFuture {
    table_locations: TableLocations,
    tablet: Arc<Tablet>,
    timer: timer::Timer,
    blacklist: HashSet<TabletServerId>,
    backoff: Backoff,
    state: BatchState,
    request: Arc<WriteRequestPb>,
}

impl BatchFuture {
    fn new(table_locations: TableLocations,
           tablet: Arc<Tablet>,
           timer: timer::Timer,
           buffer: Buffer) {
    }

    fn select_replica(&self) -> Option<&TabletReplica> {
        let mut follower = None;
        for replica in self.tablet.replicas() {
            if self.blacklist.contains(&replica.id()) {
                continue;
            } else if replica.is_leader() {
                return Some(replica);
            } else {
                follower.get_or_insert(replica);
            }
        }

        follower
    }
}

impl Future for BatchFuture {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        let state = mem::replace(&mut self.state, BatchState::Initial);
        match state {
            BatchState::Initial => {

            },
            BatchState::InFlight { ..  } => {
            },
            BatchState::TabletRefresh { ..  } => {
            },
            BatchState::Waiting { ..  } => {
            },
        }

        unimplemented!()
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
