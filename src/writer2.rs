use std::collections::{
    HashMap,
    VecDeque,
};
use std::time::Duration;
use std::mem;

use futures::{
    Async,
    Future,
    Stream,
};
use futures::stream::{
    FuturesOrdered,
    FuturesUnordered,
};

use Error;
use PartitionSchema;
use Row;
use Schema;
use Table;
use TabletId;
use key;
use meta_cache::MetaCache;
use pb::tserver::{
    WriteRequestPb,
    WriteResponsePb,
};
use pb::row_operations_pb::{Type as OperationTypePb};
use row::OperationEncoder;
use tserver;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationKind {
    Insert,
    Update,
    Upsert,
    Delete,
}

struct Operation {
    row: Row,
    kind: OperationKind,
}

impl OperationKind {
    fn as_pb(self) -> OperationTypePb {
        match self {
            OperationKind::Insert => OperationTypePb::Insert,
            OperationKind::Update => OperationTypePb::Update,
            OperationKind::Upsert => OperationTypePb::Upsert,
            OperationKind::Delete => OperationTypePb::Delete,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WriterConfig {

    /// Maximum amount of time to wait for a batch of write operations to be sent to a tablet
    /// server. If the timeout expires before the batch completes, the operations will fail with
    /// `Error::TimedOut`.
    ///
    /// Defaults to 30 second.
    flush_timeout: Duration,

    /// Maximum amount of time to batch write operations before flushing.
    ///
    /// Defaults to 1 second.
    background_flush_interval: Duration,

    /// Maximum amount of row operation data to buffer in the writer. If an operation is applied to
    /// the writer which would push the amount of buffered data over this limit, the operation will
    /// fail with `Error::Backoff`. In-flight rows (rows which have already been dispatched via RPC
    /// to a tablet server) and rows which are not yet flushed count towards this limit.
    ///
    /// Defaults to 256MiB.
    max_buffered_data: usize,

    /// Maximum amount of row operation data to batch per write RPC to a tablet. When a batch
    /// becomes full it is automatically dispatched to the tablet server.
    ///
    /// Defaults to 7MiB.
    max_data_per_batch: usize,

    /// Maximum number of concurrent in-flight batches per tablet. Once this limit is reached,
    /// attempts to apply operations to the tablet will fail with `Error::Backoff` until the
    /// one of batches are completed.
    ///
    /// Defaults to 2. Must be at least 1.
    max_batches_per_tablet: u8,

    /// When the amount of buffered row operation data surpasses
    /// `max_buffered_data * early_flush_watermark / 100` the writer will automatically flush
    /// the largest batches. In order to preemptively make space for new operations to be applied.
    ///
    /// Defaults to 80. Must be between 0 (exclusive) and 100 (inclusive).
    early_flush_watermark: u8,
}

impl Default for WriterConfig {
    fn default() -> WriterConfig {
        WriterConfig {
            flush_timeout: Duration::from_secs(30),
            background_flush_interval: Duration::from_secs(1),
            max_buffered_data: 256 * 1024 * 1024,
            max_data_per_batch: 7 * 1024 * 1024,
            max_batches_per_tablet: 2,
            early_flush_watermark: 80,
        }
    }
}

struct Writer {
    config: WriterConfig,
    meta_cache: MetaCache,
    tserver_proxies: tserver::ProxyCache,
    table: Table,

    operations_in_lookup: OperationsInLookup,
    batches_in_flight: FuturesUnordered<Box<Future<Item=BatchStats, Error=(Error, WriteRequestPb)>>>,

    /// Batchers; one per tablet server.
    batchers: HashMap<TabletId, Batcher>,

    errors: ErrorCollector,
}

impl Writer {

    pub fn apply(&mut self, op: Operation) {
        if op.row.schema() != self.schema() {
            self.fail_operation(op, Error::InvalidArgument(
                    "row operation schema must match the writer table schema".to_owned()));
            return;
        }

        let partition_key = match key::encode_partition_key(self.partition_schema(), &op.row) {
            Ok(partition_key) => partition_key,
            Err(error) => {
                self.fail_operation(op, error);
                return;
            },
        };

        let (direct_len, indirect_len) = OperationEncoder::encoded_len(&op.row);
        let encoded_len = direct_len + indirect_len;

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.config.max_data_per_batch {
            self.fail_operation(op, Error::InvalidArgument(
                    "row operation size is greater than the max batch size".to_owned()));
            return;
        }
    }

    fn poll_operations_in_lookup(&mut self) {
        match self.operations_in_lookup.poll() {
            Some((op, Ok(Some(tablet_id)))) => self.buffer_operation(tablet_id, op),
            Some((op, Ok(None))) => self.fail_operation(op, Error::NoRangePartition),
            Some((op, Err(error))) => self.fail_operation(op, error),
            None => (),
        }
    }

    fn poll_batches_in_flight(&mut self) {
        unimplemented!()
    }

    fn buffer_operation(&mut self, tablet_id: TabletId, op: Operation) {
        let (direct_len, indirect_len) = OperationEncoder::encoded_len(&op.row);
        let encoded_len = direct_len + indirect_len;

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.config.max_data_per_batch {
            self.fail_operation(op, Error::InvalidArgument(
                    "row operation size is greater than the max batch size".to_owned()));
            return;
        }

        let batch_to_send: Option<OperationEncoder> = None;

        { // NLL hack.
            let batcher = self.batchers.entry(tablet_id).or_insert_with(Batcher::new);
            if batcher.buffer.len() + encoded_len > self.config.max_data_per_batch {
                if batcher.batches_in_flight == self.config.max_batches_per_tablet {
                    unimplemented!("max # of in-flight batches for tablet reached");
                } else {
                    batcher.batches_in_flight += 1;
                    let batch_to_send = Some(mem::replace(&mut batcher.buffer, OperationEncoder::new()));
                }
            }
            batcher.buffer.encode_row(op.kind.as_pb(), &op.row);
        }

        if let Some(operations) = batch_to_send {
            self.send_batch(tablet_id, operations);
        }
    }

    fn send_batch(&mut self, tablet_id: TabletId, operations: OperationEncoder) {

    }

    pub fn flush(self) -> Flush {
        unimplemented!()
    }

    fn schema(&self) -> &Schema {
        self.table.schema()
    }

    fn partition_schema(&self) -> &PartitionSchema {
        self.table.partition_schema()
    }

    fn fail_operation(&self, op: Operation, error: Error) {
        unimplemented!()
    }
}

struct Flush {
    successful_batches: usize,
    failed_batches: usize,
    successful_operations: usize,
    failed_operations: usize,
    data: usize,
    errors: ErrorCollector,
}

struct ErrorCollector {
}

struct Batcher {
    buffer: OperationEncoder,
    batches_in_flight: u8,
    data_in_flight: usize,
    data_buffered: usize,
}

impl Batcher {
    fn new() -> Batcher {
        Batcher {
            buffer: OperationEncoder::new(),
            batches_in_flight: 0,
            data_in_flight: 0,
            data_buffered: 0,
        }
    }

    fn batch_complete(&mut self, data: usize) {
        self.batches_in_flight -= 1;
        self.data_in_flight -= data;
    }
}

struct BatchStats {
    successful_operations: usize,
    failed_operations: usize,
    data: usize,
}

struct OperationsInLookup {
    rows: VecDeque<Operation>,
    futures: FuturesOrdered<Box<Future<Item=Option<TabletId>, Error=Error>>>,
}

impl OperationsInLookup {
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn push(&mut self,
                op: Operation,
                future: Box<Future<Item=Option<TabletId>, Error=Error>>) {
        self.rows.push_back(op);
        self.futures.push(future);
    }

    pub fn poll(&mut self) -> Option<(Operation, Result<Option<TabletId>, Error>)> {
        match self.futures.poll() {
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => None,
            Ok(Async::Ready(Some(tablet_id))) => Some((self.rows.pop_front().unwrap(), Ok(tablet_id))),
            Err(error) => Some((self.rows.pop_front().unwrap(), Err(error))),

        }
    }
}
