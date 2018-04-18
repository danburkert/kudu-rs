#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    TabletServerService,
    WriteRequestPb,
    WriteResponsePb,
};
use pb::SchemaPb;
use retry::RetryFuture;
use PartitionKey;
use util::{
    RetryWithBackoff,
    RetryCause,
};
use replica::{
    ReplicaSet,
    ReplicaRpc,
    Speculation,
    Selection,
};
use row::Row;
use Schema;

pub(crate) struct Buffer {
    pub(crate) encoder: OperationEncoder,
    pub(crate) operations: usize,
}

struct BatchFuture {
    tablet_id: TabletId,
    call: Call<WriteRequestPb, WriteResponsePb>,
    rpc: ReplicaRpc<Arc<Tablet>, WriteRequestPb, WriteResponsePb>,
    schema: Schema,
    num_operations: usize,
    data: usize,
}


// TODO:
//
// 1) Add initial step to BatchFuture that locates the tablet.
// 2) Add API to Call to get mutable request.

impl BatchFuture {

    fn new(schema: Schema,
           tablet: Arc<Tablet>,
           operations: OperationEncoder,
           num_operations: usize,
           deadline: Instant) -> BatchFuture {
        let data = operations.len();
        let tablet_id = tablet.id();
        let mut request = WriteRequestPb::default();
        request.tablet_id = tablet.id().to_string().into_bytes();
        request.schema = Some(schema.as_pb());
        //request.propagated_timestamp = Some(self.client().latest_observed_timestamp());
        request.row_operations = Some(operations.into_pb());
        let call = TabletServerService::write(Arc::new(request), deadline);
        let rpc = ReplicaRpc::new(tablet,
                                  call.clone(),
                                  Speculation::Staggered(Duration::from_millis(100)),
                                  Selection::Leader,
                                  Backoff::default());

        BatchFuture {
            tablet_id,
            call,
            rpc,
            schema,
            num_operations,
            data,
        }
    }

    fn unwrap_operation_encoder(&mut self) -> OperationEncoder {
        OperationEncoder::from_pb(Arc::get_mut(&mut self.call.request)
                                      .unwrap()
                                      .row_operations
                                      .take()
                                      .unwrap())
    }
}

impl Future for BatchFuture {
    type Item = BatchStats;
    type Error = BatchError;
    fn poll(&mut self) -> Poll<BatchStats, BatchError> {
        match self.rpc.poll() {

            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready((_, mut response, _))) => {
                assert!(response.error.is_none());

                let failed_operations = response.per_row_errors.len();
                if failed_operations != 0 {
                    let operation_encoder = self.unwrap_operation_encoder();
                    let mut rows = operation_encoder.iter(self.schema.clone());
                    response.per_row_errors.sort_by_key(|error| error.row_index as usize);
                }

                Ok(Async::Ready(BatchStats {
                    tablet: self.tablet_id,
                    successful_operations: self.num_operations - failed_operations,
                    failed_operations,
                    data: self.data,
                }))
            },
            Err(error) => Err(unimplemented!()),
        }
    }
}

pub(crate) struct BatchStats {
    tablet: TabletId,
    successful_operations: usize,
    failed_operations: usize,
    data: usize,
}

pub(crate) struct BatchError {
    operations: OperationEncoder,
    num_operations: usize,
    error: Error,
}
