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
use futures::sync::mpsc::UnboundedSender;
use krpc::Call;

use Error;
use HostPort;
use OperationEncoder;
use operation::OperationDecoder;
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
use pb::tserver::write_response_pb::PerRowErrorPb;
use pb::SchemaPb;
use retry::RetryFuture;
use PartitionKey;
use util::{
    RetryWithBackoff,
    RetryCause,
};
use operation::OperationError;
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

// TODO:
//
// 1) Add initial step to BatchFuture that locates the tablet.
//      OR keep it as is, and add compose this on top later
//
// 2) Add API to Call to get mutable request.

struct BatchFuture {
    tablet_id: TabletId,
    call: Call<WriteRequestPb, WriteResponsePb>,
    rpc: ReplicaRpc<Arc<Tablet>, WriteRequestPb, WriteResponsePb>,
    schema: Schema,
    num_operations: usize,
    data: usize,
    errors: UnboundedSender<OperationError>,
}

impl BatchFuture {

    fn new(schema: Schema,
           tablet: Arc<Tablet>,
           operations: OperationEncoder,
           num_operations: usize,
           deadline: Instant,
           errors: UnboundedSender<OperationError>) -> BatchFuture {
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
            errors,
        }
    }

    fn handle_row_errors(&mut self, errors: Vec<PerRowErrorPb>) -> Result<(), Error> {
        let row_operations = self.call.request.row_operations.as_ref().unwrap();

        let decoder = OperationDecoder::new(&self.schema,
                                            row_operations.rows(),
                                            row_operations.indirect_data());

        for error in errors {
            if error.row_index < 0 || error.row_index as usize >= self.num_operations {
                return Err(Error::Serialization(format!("row error contains invalid index: {:?}", error)));
            }

            let operation = decoder.decode_operation(error.row_index as usize);

            let error = OperationError {
                row: operation.row.into_owned(),
                kind: operation.kind,
                status: error.error.into(),
            };

            if let Err(_) = self.errors.unbounded_send(error) {
                // The receiver has been dropped, so no point in sending additional errors.
                break;
            }
        }

        Ok(())
    }
}

impl Future for BatchFuture {
    type Item = BatchStats;
    type Error = BatchError;
    fn poll(&mut self) -> Poll<BatchStats, BatchError> {
        match self.rpc.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready((_, response, _))) => {
                assert!(response.error.is_none());

                let failed_operations = response.per_row_errors.len();
                if failed_operations != 0 {
                    if let Err(error) = self.handle_row_errors(response.per_row_errors) {
                        return Err(BatchError {
                            call: self.call.clone(),
                            num_operations: self.num_operations,
                            error,
                        });
                    }
                }

                Ok(Async::Ready(BatchStats {
                    tablet: self.tablet_id,
                    successful_operations: self.num_operations - failed_operations,
                    failed_operations,
                    data: self.data,
                }))
            },

            Err(error) => Err(BatchError {
                call: self.call.clone(),
                num_operations: self.num_operations,
                error: error.into(),
            }),
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
    call: Call<WriteRequestPb, WriteResponsePb>,
    num_operations: usize,
    error: Error,
}
