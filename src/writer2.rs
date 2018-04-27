#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::{
    HashMap,
    VecDeque,
};
use std::collections::hash_map::Entry;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::mem;

use futures::{
    Async,
    Future,
    Poll,
    Sink,
    Stream,
};
use futures::stream::{
    FuturesOrdered,
    FuturesUnordered,
};
use futures::sync::mpsc::{
    self,
    UnboundedReceiver,
    UnboundedSender,
};
use krpc::Call;

use Client;
use Error;
use PartitionSchema;
use Schema;
use Table;
use TabletId;
use key;
use meta_cache::Tablet;
use pb::tserver::{
    TabletServerService,
    WriteRequestPb,
    WriteResponsePb,
};
use operation::{
    Operation,
    OperationDecoder,
    OperationEncoder,
    OperationError,
    OperationKind,
};
use replica::{
    ReplicaSet,
    ReplicaRpc,
    Speculation,
    Selection,
};
use backoff::Backoff;
use tokio_timer::Delay;
use Row;
use partition::PartitionKey;
use replica::Replica;

#[derive(Debug, Clone)]
pub struct WriterConfig {

    /// Maximum amount of time to wait for a batch of write operations to be sent to a tablet
    /// server. If the timeout expires before the batch completes, the operations will fail with
    /// `Error::TimedOut`.
    ///
    /// Defaults to 120 seconds.
    flush_timeout: Duration,

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

    /// Maximum number of concurrent in-flight batches per tablet.
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
            flush_timeout: Duration::from_secs(120),
            max_buffered_data: 256 * 1024 * 1024,
            max_data_per_batch: 7 * 1024 * 1024,
            max_batches_per_tablet: 2,
            early_flush_watermark: 80,
        }
    }
}

pub struct Writer {
    operations_in_lookup: FuturesOrdered<Box<Future<Item=(Option<Arc<Tablet>>, Operation<'static>, usize),
                                                    Error=(Operation<'static>, Error)>>>,

    /// Batchers; one per tablet server.
    batchers: HashMap<TabletId, TabletBatcher>,

    /// Current amount of unflushed data (in-lookup + queued batches + batches in flight).
    buffered_data: usize,

    common: Common,
}

struct Common {
    config: WriterConfig,
    table: Table,

    batches_in_flight: FuturesUnordered<Box<Future<Item=BatchStats, Error=BatchError>>>,

    error_sender: UnboundedSender<OperationError>,
    error_receiver: UnboundedReceiver<OperationError>,
}

impl Writer {

    pub(crate) fn new(table: Table, config: WriterConfig) -> Writer {
        let (error_sender, error_receiver) = mpsc::unbounded();
        Writer {
            operations_in_lookup: FuturesOrdered::new(),
            batchers: HashMap::new(),
            buffered_data: 0,
            common: Common {
                config,
                table,
                batches_in_flight: FuturesUnordered::new(),
                error_sender,
                error_receiver,
            },
        }
    }

    pub fn poll_ready(&mut self) -> Poll<(), Error> {
        self.poll_operations_in_lookup()?;

        self.poll_batches_in_flight()?;

        // TODO: figure out if the amount of data in batches is over the early flush watermark.

        if self.buffered_data >= self.common.config.max_buffered_data {
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(()))
        }
    }

    pub fn poll_flush(&mut self) -> Poll<FlushStats, Error> {
        self.poll_operations_in_lookup()?;

        self.poll_batches_in_flight()?;

        if !self.operations_in_lookup.is_empty() {
            return Ok(Async::NotReady);
        }

        // Flush all tablets which have not hit their max batches in flight limit.
        for batcher in self.batchers.values_mut() {
            if !batcher.batch.is_empty() &&
               batcher.batches_in_flight < self.common.config.max_batches_per_tablet {
                let batch = mem::replace(&mut batcher.batch, Batch::new());
                batch.send(batcher.tablet.clone(), &mut self.common);
            }
        }

        Ok(Async::NotReady)
    }

    pub fn apply(&mut self, op: Operation) {
        if op.row.schema() != self.common.table.schema() {
            self.fail_operation(op, Error::InvalidArgument(
                    "row operation schema does not match the writer schema".to_owned()));
            return;
        }

        let encoded_len = OperationEncoder::encoded_len(&op.row);

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.common.config.max_data_per_batch {
            self.fail_operation(op, Error::InvalidArgument(
                    "row operation size is greater than the max batch size".to_owned()));
            return;
        }

        self.buffered_data += encoded_len;

        let partition_key = match key::encode_partition_key(self.common.table.partition_schema(), &op.row) {
            Ok(partition_key) => partition_key,
            Err(error) => {
                self.fail_operation(op, error);
                return;
            },
        };

        let mut tablet = self.common.table.table_locations().tablet(&*partition_key);
        let poll = if self.operations_in_lookup.is_empty() {
            tablet.poll()
        } else {
            Ok(Async::NotReady)
        };

        match poll {
            Ok(Async::Ready(Some(tablet))) => self.buffer_operation(tablet, op, encoded_len),
            Ok(Async::Ready(None)) => self.fail_operation(op, Error::NoRangePartition),
            Ok(Async::NotReady) => {
                let op = op.into_owned();
                let operation_in_lookup = Box::new(tablet.then(move |result| match result {
                    Ok(tablet) => Ok((tablet, op, encoded_len)),
                    Err(error) => Err((op, error)),
                }));
                self.operations_in_lookup.push(operation_in_lookup);
            },
            Err(error) => self.fail_operation(op, error),
        }
    }

    pub fn insert(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Insert })
    }

    pub fn update(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Update })
    }

    pub fn delete(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Delete })
    }

    fn poll_operations_in_lookup(&mut self) -> Poll<(), Error> {
        loop {
            match self.operations_in_lookup.poll() {
                Ok(Async::Ready(Some((Some(tablet), op, encoded_len)))) => self.buffer_operation(tablet, op, encoded_len),
                Ok(Async::Ready(Some((None, op, _)))) => self.fail_operation(op, Error::NoRangePartition),
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(None)) => return Ok(Async::Ready(())),
                Err((op, error)) => self.fail_operation(op, error),
            }
        }
    }

    fn poll_batches_in_flight(&mut self) -> Poll<(), Error> {
        loop {
            match self.common.batches_in_flight.poll() {
                Ok(Async::Ready(Some(stats))) => {
                    self.buffered_data -= stats.data;
                    match self.batchers.entry(stats.tablet) {
                        Entry::Occupied(ref mut entry) => {
                            if !entry.get().batch_queue.is_empty() &&
                                entry.get().batches_in_flight < self.common.config.max_batches_per_tablet {
                                entry.get_mut().batch_queue.pop_front().unwrap().send(entry.get().tablet.clone(), &mut self.common);
                            } else {
                                entry.get_mut().batches_in_flight -= 1;
                            }
                        },
                        Entry::Vacant(..) => unreachable!("unknown batch tablet"),
                    }
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(None)) => return Ok(Async::Ready(())),
                Err(BatchError { call, stats, error }) => {
                    // TODO: handle recoverable errors here.
                    self.buffered_data -= stats.data;
                    return Err(error);
                },
            };
        }
    }

    /// Applies an operation to the appropriate tablet batch.
    fn buffer_operation(&mut self, tablet: Arc<Tablet>, op: Operation, encoded_len: usize) {
        let batcher = self.batchers.entry(tablet.id()).or_insert_with(|| TabletBatcher::new(tablet.clone()));

        // Overwrite the tablet in case it's been updated.
        batcher.tablet = tablet;

        if batcher.batch.encoder.len() + encoded_len > self.common.config.max_data_per_batch {
            batcher.flush(&mut self.common);
        }
        batcher.batch.encoder.encode_row(op.kind.as_pb(), &op.row);
    }

    fn fail_operation(&self, operation: Operation, error: Error) {
        let _ = self.common.error_sender.unbounded_send(OperationError {
            row: operation.row.into_owned(),
            kind: operation.kind,
            error,
        });
    }
}

pub(crate) struct ErrorCollector {
    error_sender: UnboundedSender<OperationError>,
    error_receiver: UnboundedReceiver<OperationError>,
}

struct TabletBatcher {

    /// The lower-bound partition key of the tablet.
    tablet: Arc<Tablet>,

    /// The active batch being applied to.
    batch: Batch,

    /// The number of batches which are currently being sent. Must not exceed the
    /// `max_batches_per_tablet` configuration.
    batches_in_flight: u8,

    /// Batches which have not yet been sent because the maximum number of batches is already
    /// in-flight.
    batch_queue: VecDeque<Batch>,
}

impl TabletBatcher {

    /// Creates a new empty tablet batcher.
    fn new(tablet: Arc<Tablet>) -> TabletBatcher {
        TabletBatcher {
            tablet,
            batch: Batch::new(),
            batches_in_flight: 0,
            batch_queue: VecDeque::new(),
        }
    }

    /// Rolls the currently active batch, and sends it to the remote tablet server if the max
    /// batches in-flight limit has not been reached for the tablet.
    fn flush(&mut self, common: &mut Common) {
        debug_assert!(!self.batch.is_empty());
        let batch = mem::replace(&mut self.batch, Batch::new());
        self.batch_queue.push_back(batch);
        self.send_batches(common);
    }

    /// Sends batches waiting in the queue to the remote tablet server, until the maximum number of
    /// batches in-flight limit is reached.
    fn send_batches(&mut self, common: &mut Common) {
        while self.batches_in_flight < common.config.max_batches_per_tablet {
            if let Some(batch) = self.batch_queue.pop_front() {
                self.batches_in_flight += 1;
                batch.send(self.tablet.clone(), common);
            } else {
                break;
            }
        }
    }
}

struct Batch {
    encoder: OperationEncoder,
    operations: usize,
}

impl Batch {
    fn new() -> Batch {
        Batch {
            encoder: OperationEncoder::new(),
            operations: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.operations == 0
    }

    fn send(self, tablet: Arc<Tablet>, common: &mut Common) {
        let mut stats = BatchStats {
            tablet: tablet.id(),
            operations: self.operations,
            row_errors: 0,
            data: self.encoder.len(),
        };

        let mut request = WriteRequestPb::default();
        request.tablet_id = tablet.id().to_string().into_bytes();
        request.schema = Some(common.table.schema().as_pb());
        //request.propagated_timestamp = Some(self.client().latest_observed_timestamp());
        request.row_operations = Some(self.encoder.into_pb());
        let call1 = TabletServerService::write(Arc::new(request),
                                               Instant::now() + common.config.flush_timeout);
        let call2 = call1.clone();
        let call3 = call1.clone();

        let schema = common.table.schema().clone();
        let error_sender = common.error_sender.clone();

        common.batches_in_flight.push(
            Box::new(ReplicaRpc::new(tablet,
                        call1,
                        Speculation::Staggered(Duration::from_millis(100)),
                        Selection::Leader,
                        Backoff::default())
            .and_then(move |(_, response, _)| {
                assert!(response.error.is_none());
                let row_errors = response.per_row_errors.len();
                if row_errors != 0 {
                    let row_operations = call2.request.row_operations.as_ref().unwrap();
                    let decoder = OperationDecoder::new(&schema,
                                                        row_operations.rows(),
                                                        row_operations.indirect_data());

                    for error in response.per_row_errors {
                        if error.row_index < 0 || error.row_index as usize >= stats.operations {
                            return Err(Error::Serialization(
                                    format!("row error contains invalid index: {:?}", error)));
                        }

                        let operation = decoder.decode_operation(error.row_index as usize);

                        let error = OperationError {
                            row: operation.row.into_owned(),
                            kind: operation.kind,
                            error: Error::RowError(error.error.into()),
                        };

                        if let Err(_) = error_sender.unbounded_send(error) {
                            // The receiver has been dropped, so no point in sending additional errors.
                            break;
                        }
                    }
                }

                stats.row_errors = row_errors;
                Ok(stats)
            }).map_err(move |error| {
                BatchError {
                    call: call3,
                    stats,
                    error: error.into(),
                }
            })));
    }
}

#[derive(Clone, Copy)]
struct BatchStats {
    tablet: TabletId,
    operations: usize,
    row_errors: usize,
    data: usize,
}

struct BatchError {
    call: Call<WriteRequestPb, WriteResponsePb>,
    stats: BatchStats,
    error: Error,
}

/// Carries information about the batches and row operations in a flush.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlushStats {
    epoch: usize,
    successful_batches: usize,
    failed_batches: usize,
    successful_operations: usize,
    failed_operations: usize,
    data: usize,
}
