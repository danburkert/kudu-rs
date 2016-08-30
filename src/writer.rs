use std::collections::HashMap;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use kudu_pb::tserver;
use parking_lot::{Mutex, MutexGuard};

use Client;
use Error;
use PartitionSchema;
use Result;
use Row;
use Schema;
use Table;
use TabletId;
use backoff::Backoff;
use error;
use key;
use kudu_pb::wire_protocol::{RowOperationsPB_Type as OperationTypePB};
use meta_cache::MetaCache;
use queue_map::QueueMap;
use row::OperationEncoder;
use rpc::{Callback, Messenger, Rpc, tablet_server};
use util;

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


pub struct Writer<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {
    inner: Arc<Inner<E>>,
}

struct Inner<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {
    table: Table,
    error_callback: E,
    config: WriterConfig,
    state: Mutex<State>,
}

struct State {
    /// Operations currently in lookup.
    operations_in_lookup: QueueMap<OperationInLookup>,

    /// Map of Tablet Id to an optional buffer, and a count of batches in flight.
    tablets: HashMap<TabletId, (Option<Buffer>, u8)>,

    /// Flushes currently in progress.
    flushes: QueueMap<FlushState>,

    /// Current amount of unflushed data (in-lookup + buffered + batches in flight).
    buffered_data: usize,
}

impl State {
    /// Returns the current flush epoch.
    fn flush_epoch(&self) -> usize {
        self.flushes.back_key().unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationType {
    Insert,
    Update,
    Upsert,
    Delete,
}

impl OperationType {
    fn as_pb(self) -> OperationTypePB {
        match self {
            OperationType::Insert => OperationTypePB::INSERT,
            OperationType::Update => OperationTypePB::UPDATE,
            OperationType::Upsert => OperationTypePB::UPSERT,
            OperationType::Delete => OperationTypePB::DELETE,
        }
    }
}

#[must_use]
struct OperationInLookup {
    row: Row,
    flush_epoch: usize,
    encoded_len: usize,
    op_type: OperationType,
}

#[must_use]
struct Buffer {
    operations: Vec<(Row, usize, OperationType)>,
    /// Current amount of buffered data.
    buffered_data: usize,
    flush_epoch: usize,
}

impl Buffer {
    fn new(flush_epoch: usize) -> Buffer {
        Buffer { operations: Vec::new(),
                 buffered_data: 0,
                 flush_epoch: flush_epoch,
        }
    }
}

/// Holds state associated with a flush epoch.
struct FlushState {
    stats: FlushStats,
    lookups_outstanding: usize,
    batches_outstanding: usize,
    callback: Option<Box<FnOnce(FlushStats) + Send + 'static>>,
}

/// Carries information about the batches and row operations in a flush.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FlushStats {
    successful_batches: usize,
    failed_batches: usize,
    successful_operations: usize,
    failed_operations: usize,
    data: usize,
}

impl FlushStats {
    fn successful_batches(&self) -> usize {
        self.successful_batches
    }
    fn failed_batches(&self) -> usize {
        self.successful_batches
    }
    fn successful_operations(&self) -> usize {
        self.successful_batches
    }
    fn failed_operations(&self) -> usize {
        self.successful_batches
    }
    fn data(&self) -> usize {
        self.successful_batches
    }
}

impl <E> Writer<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {

    fn new(table: Table, error_callback: E, config: WriterConfig) -> Writer<E> {
        Writer {
            inner: Arc::new(Inner {
                table: table,
                error_callback: error_callback,
                config: config,
                state: Mutex::new(State {
                    operations_in_lookup: QueueMap::new(),
                    tablets: HashMap::new(),
                    flushes: QueueMap::new(),
                    buffered_data: 0,
                }),
            }),
        }
    }

    pub fn apply(&self, row: Row, op_type: OperationType) {
        if row.schema() != self.schema() {
            self.fail_operation(row, op_type, Error::InvalidArgument(
                    "row operation schema must match the writer table schema".to_owned()));
            return;
        }

        let partition_key = match key::encode_partition_key(self.partition_schema(), &row) {
            Ok(partition_key) => partition_key,
            Err(error) => {
                self.fail_operation(row, op_type, error);
                return;
            },
        };
        let encoded_len = OperationEncoder::encoded_len(&row);

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.config().max_data_per_batch {
            self.fail_operation(row, op_type, Error::InvalidArgument(
                    "row operation size is greater than the max batch size".to_owned()));
            return;
        }

        let idx = {
            let mut state = self.state();

            // Check that there is space for the operation. If buffering the operation would push
            // the `buffered_data` counter over the `max_buffered_data` limit, then we reject the
            // operation with `Error::Backoff`, If buffering the operation would push the writer
            // over the early-flush watermark, then we preemptively send large batches.
            let data = state.buffered_data.saturating_add(encoded_len);
            if data > self.config().max_buffered_data {
                drop(state);
                self.fail_operation(row, op_type, Error::Backoff);
                return;
            } else if data > (self.config().max_buffered_data / 100) *
                             self.config().early_flush_watermark as usize {
                // Do early flush.
                unimplemented!()
            }

            // Add the operation to the operations_in_flight queue. This assigns an idx which
            // uniquely identifies the operation and gives it a total ordering among applied
            // operations in the writer.
            state.buffered_data = data;
            let flush_epoch = state.flush_epoch();
            state.operations_in_lookup.push(OperationInLookup {
                row: row,
                flush_epoch: flush_epoch,
                encoded_len: encoded_len,
                op_type: op_type,
            })
        };

        let writer: Writer<E> = self.clone();
        let deadline = Instant::now() + self.config().flush_timeout;
        self.meta_cache().tablet_id(partition_key, deadline, move |result| {
            if let Some((row, op_type, error)) = writer.op_lookup_complete(idx, result) {
                writer.fail_operation(row, op_type, error);
            }
        });
    }

    fn op_lookup_complete(&self, idx: usize, result: Result<Option<TabletId>>) -> Option<(Row, OperationType, Error)> {
        let mut state = self.state();

        // Retrieve the operation in lookup, and decrement the epoch's operations in lookup
        // counter.
        let OperationInLookup { row, flush_epoch, encoded_len, op_type } =
            state.operations_in_lookup.remove(idx).unwrap();
        state.flushes[flush_epoch].lookups_outstanding -= 1;

        // Temporarily decrease the buffered data amount by the operation's size so that early
        // exit points don't have to handle it.
        state.buffered_data -= encoded_len;

        match result {
            Ok(Some(tablet_id)) => {
                let State { ref mut tablets, ref mut flushes, ref mut buffered_data, .. } = *state;
                let (ref mut buffer, ref mut batches_in_flight) = *tablets.entry(tablet_id)
                                                                          .or_insert_with(|| (None, 0));
                match *buffer {
                    None => {
                        // Case [1].
                        *buffer = Some(Buffer::new(flush_epoch));
                        *batches_in_flight += 1;
                    },
                    Some(ref mut buffer) if buffer.buffered_data.saturating_add(encoded_len) >
                                            self.config().max_data_per_batch => {
                        if *batches_in_flight >= self.config().max_batches_per_tablet {
                            // Case [3].
                            return Some((row, op_type, Error::Backoff));
                        }
                        // Case [2].
                        let buffer = mem::replace(buffer, Buffer::new(flush_epoch));
                        *batches_in_flight += 1;
                        let writer = self.clone();
                        // Translating the buffer into a WriteRequestPB is pretty heavy, so do it
                        // on another thread so that it's not in the critical section.
                        // TODO: can this be done outside the lock without spawning a new thread?
                        thread::spawn(move || {
                            Batch::send(writer, tablet_id, buffer);
                        });
                    }
                    _ => (),
                };

                let buffer = buffer.as_mut().unwrap();

                // Add the operation to the buffer.
                buffer.operations.push((row, idx, op_type));
                buffer.buffered_data += encoded_len;
                *buffered_data += encoded_len;

                // Check if the operation's epoch falls before the buffer's epoch. If so, we need to
                // back-date the buffer to the older epoch so that the new operation gets flushed at
                // the appropriate time.
                if flush_epoch < buffer.flush_epoch {
                    flushes[buffer.flush_epoch].batches_outstanding -= 1;
                    flushes[flush_epoch].batches_outstanding += 1;
                    buffer.flush_epoch = flush_epoch;
                }
                None
            },
            Ok(None) => Some((row, op_type, Error::NoRangePartition)),
            Err(error) => Some((row, op_type, error)),
        }
    }

    fn batch_complete(&self,
                      success: bool,
                      tablet: TabletId,
                      flush_epoch: usize,
                      successful_operations: usize,
                      failed_operations: usize,
                      data: usize) {
        let mut complete_flushes = Vec::new();
        {
            let mut state = self.state();
            state.buffered_data -= data;
            {
                let flush = &mut state.flushes[flush_epoch];
                if success {
                    flush.stats.successful_batches += 1;
                } else {
                    flush.stats.failed_batches += 1;
                }

                flush.stats.successful_operations += successful_operations;
                flush.stats.failed_operations += failed_operations;
                flush.stats.data += data;

                flush.batches_outstanding -= 1;
            }

            // Check if this is the oldest flush, if not early return.
            let mut oldest_epoch = state.flushes.front_key().unwrap();
            if flush_epoch != oldest_epoch { return; }

            // Remove each complete flush from the writer and save it so that it can be completed
            // outside the lock.
            while {
                let flush = &state.flushes[oldest_epoch];
                flush.callback.is_some() &&
                    flush.lookups_outstanding == 0 &&
                    flush.batches_outstanding == 0
            } {
                complete_flushes.push(state.flushes.pop().unwrap().1);
                match state.flushes.front_key() {
                    Some(key) => oldest_epoch = key,
                    None => break,
                }
            }
        }

        for flush in complete_flushes {
            let FlushState { stats, callback, .. } = flush;
            callback.unwrap().call_once(stats);
        }
    }

    fn schema(&self) -> &Schema {
        self.inner.table.schema()
    }

    fn partition_schema(&self) -> &PartitionSchema {
        self.inner.table.partition_schema()
    }

    fn fail_operation(&self, row: Row, op_type: OperationType, error: Error) {
        (self.inner.error_callback)(row, op_type, error)
    }

    pub fn config(&self) -> &WriterConfig {
        &self.inner.config
    }

    fn state(&self) -> MutexGuard<State> {
        self.inner.state.lock()
    }

    fn meta_cache(&self) -> &MetaCache {
        self.inner.table.meta_cache()
    }

    fn client(&self) -> &Client {
        self.inner.table.client()
    }

    fn messenger(&self) -> &Messenger {
        self.inner.table.client().messenger()
    }
}

impl <E> Clone for Writer<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {
    fn clone(&self) -> Writer<E> {
        Writer { inner: self.inner.clone() }
    }
}

#[must_use]
struct Batch<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {
    tablet: TabletId,
    leader_addrs: Vec<SocketAddr>,
    operations: Vec<(Row, usize, OperationType)>,
    writer: Writer<E>,
    backoff: Backoff,
    buffered_data: usize,
    flush_epoch: usize,
}

impl <E> Batch<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {

    /// Transforms the buffer into a batch, and sends it to the tablet server.
    fn send(writer: Writer<E>, tablet: TabletId, buffer: Buffer) {
        let Buffer { mut operations, buffered_data, flush_epoch } = buffer;

        // Sort the operations so that they are written in apply order.
        // TODO: the operations should already be in order, look into bubble or insertion sort.
        operations.sort_by(|a, b| a.1.cmp(&b.1));

        let data_len = writer.schema().row_size() * operations.len();
        let indirect_data_len = buffered_data - data_len;
        let mut encoder = OperationEncoder::with_capacity(data_len, indirect_data_len);

        for &(ref row, _, op_type) in &operations {
            encoder.encode_row(op_type.as_pb(), row);
        }

        let (data, indirect_data) = encoder.unwrap();
        debug_assert_eq!(data.len(), data_len);
        debug_assert_eq!(indirect_data.len(), indirect_data_len);

        let mut message = tserver::WriteRequestPB::new();
        message.mut_row_operations().set_rows(data);
        message.mut_row_operations().set_indirect_data(indirect_data);
        message.set_schema(writer.schema().as_pb());
        message.set_propagated_timestamp(writer.client().latest_observed_timestamp());

        let rpc = tablet_server::write(util::dummy_addr(),
                                       Instant::now() + writer.config().flush_timeout,
                                       message);

        let backoff = Backoff::with_duration_range(10, util::duration_to_ms(&writer.config().flush_timeout) as u32 / 2);
        let batch = Batch {
            tablet: tablet,
            leader_addrs: Vec::new(),
            operations: operations,
            writer: writer,
            backoff: backoff,
            buffered_data: buffered_data,
            flush_epoch: flush_epoch,
        };

        batch.lookup_locations(rpc);
    }

    fn lookup_locations(mut self, rpc: Rpc) {
        let partition_key = key::encode_partition_key(self.writer.partition_schema(),
                                                      &self.operations[0].0).unwrap();

        self.writer.meta_cache().clone().tablet_leader(partition_key, rpc.deadline, move |tablet| {
            match tablet {
                Ok(Some((tablet_id, mut leader_addrs))) => {
                    // Check if the tablet matches.
                    if tablet_id != self.tablet {
                        return self.reapply();
                    }

                    // Reverse the leader addrs to use it like a stack.
                    leader_addrs.reverse();
                    self.leader_addrs = leader_addrs;

                    self.dispatch_next(rpc);
                },
                Ok(None) => self.reapply(),
                Err(error) => {
                    warn!("unable to look up leader address for tablet {}: {}", self.tablet, error);
                    self.retry(rpc);
                }
            }
        });
    }

    /// Sends the batch to the next leader address.
    fn dispatch_next(mut self, mut rpc: Rpc) {
        match self.leader_addrs.pop() {
            Some(addr) => {
                rpc.addr = addr;
                let messenger = self.writer.messenger().clone();
                rpc.callback = Some(Box::new(self));
                messenger.send(rpc);
            },
            None => {
                self.retry(rpc);
            }
        }
    }

    fn handle_response(mut self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
        match result {
            Ok(_) => {
                let response = rpc.mut_response::<tserver::WriteResponsePB>();

                if response.has_error() {
                    // TODO
                    unimplemented!();
                }

                for error in response.mut_per_row_errors().iter_mut() {
                    let offset = error.get_row_index();
                    let error = error::TabletServerError::from(error.take_error());

                    // TODO: is there a cleaner or more efficient way to remove the row from the
                    // operations vector?
                    let dummy_row = self.writer.schema().new_row();
                    let (row, _, op_type) = mem::replace(&mut self.operations[offset as usize],
                                                         (dummy_row, 0, OperationType::Insert));

                    self.writer.fail_operation(row, op_type, Error::TabletServer(error));
                }

                let failed_ops = response.get_per_row_errors().len();

                self.writer.batch_complete(true,
                                           self.tablet,
                                           self.flush_epoch,
                                           self.operations.len() - failed_ops,
                                           failed_ops,
                                           self.buffered_data);
            },
            Err(_) => {
                // TODO
                unimplemented!();
            }
        }
    }

    fn retry(mut self, mut rpc: Rpc) {
        let duration = Duration::from_millis(self.backoff.next_backoff_ms());
        self.writer.messenger().clone().timer(duration, Box::new(move || {
            self.lookup_locations(rpc);
        }));
    }

    /// Reapply each operation in the batch to the session with the same epoch and idx.
    ///
    /// This is executed when the batch determines that its tablet has been dropped.
    fn reapply(self) {
        unimplemented!();
    }
}

impl <E> Callback for Batch<E> where E: Fn(Row, OperationType, Error) + Send + Sync + 'static {
    fn callback(self: Box<Self>, result: Result<()>, rpc: Rpc) {
        self.handle_response(result, rpc);
    }
}
