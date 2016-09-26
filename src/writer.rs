use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
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

    /// Determines which events are sent to the event_channel.
    event_set: EventSet,

    event_channel: Option<SyncSender<Event>>,
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
            event_set: EventSet::Flushes,
            event_channel: None,
        }
    }
}

impl WriterConfig {

    pub fn event_channel(&mut self) -> Receiver<Event> {
        self.event_channel_with_capacity(100)
    }

    pub fn event_channel_with_capacity(&mut self, capacity: usize) -> Receiver<Event> {
        let (send, recv) = sync_channel(capacity);
        self.event_channel = Some(send);
        recv
    }

    pub fn with_event_channel(&mut self, sender: SyncSender<Event>) {
        self.event_channel = Some(sender);
    }

    fn into_config(mut self) -> (Config, Option<SyncSender<Event>>) {
        (Config {
            flush_timeout: self.flush_timeout,
            background_flush_interval: self.background_flush_interval,
            max_buffered_data: self.max_buffered_data,
            max_data_per_batch: self.max_data_per_batch,
            max_batches_per_tablet: self.max_batches_per_tablet,
            early_flush_watermark: self.early_flush_watermark,
            event_set: self.event_set,
        },
        self.event_channel.take())
    }
}

/// WriterConfig without the event_channel.
struct Config {
    flush_timeout: Duration,
    background_flush_interval: Duration,
    max_buffered_data: usize,
    max_data_per_batch: usize,
    max_batches_per_tablet: u8,
    early_flush_watermark: u8,
    event_set: EventSet,
}

impl WriterConfig {
    fn early_flush_watermark(&self) -> usize {
        (self.max_buffered_data / 100) * self.early_flush_watermark as usize
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    Flush(FlushStats),
    SuccessfulOperation(Row, OperationType),
    FailedOperation(Row, OperationType, Error),
}

impl Event {
    pub fn is_flush(&self) -> bool {
        match *self {
            Event::Flush(_) => true,
            _ => false,
        }
    }
    pub fn is_successful_operation(&self) -> bool {
        match *self {
            Event::SuccessfulOperation(..) => true,
            _ => false,
        }
    }
    pub fn is_failed_operation(&self) -> bool {
        match *self {
            Event::FailedOperation(..) => true,
            _ => false,
        }
    }
}

/// The events that the writer should return in the event channel. Each event set builds on the
/// previous.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventSet {
    /// Flush events.
    Flushes,
    /// Flush events and failed operation events.
    FailedOperations,
    /// Flush events, failed operation events, and successful operation events.
    SuccessfulOperations,
}

impl EventSet {
    pub fn has_failed_operations(&self) -> bool {
        match *self {
            EventSet::Flushes => false,
            _ => true,
        }
    }

    pub fn has_successful_operation(&self) -> bool {
        match *self {
            EventSet::SuccessfulOperations => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Writer {
    inner: Arc<Inner>,
    event_channel: Option<SyncSender<Event>>,
}

struct Inner {
    table: Table,
    config: Config,
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

    fn flush(&mut self, flush_epoch: usize, config: &Config, buffers: &mut Vec<(TabletId, Buffer)>) {
        let flush = self.flushes
                        .iter()
                        .take_while(|flush| flush.0 <= flush_epoch)
                        .all(|flush| flush.1.lookups_outstanding == 0);

        if flush {
            let max_batches_per_tablet = config.max_batches_per_tablet;
            for (&tablet_id, &mut (ref mut buffer_opt, ref mut outstanding)) in self.tablets.iter_mut() {
                if let Some(buffer) = buffer_opt.take() {
                    // Check that the buffer belongs to the flushing epoch, or that it belongs to a
                    // previous epoch and it hasn't yet been flushed because the tablet already has
                    // the maximum number of batches in flight.
                    debug_assert!(buffer.flush_epoch == flush_epoch ||
                                  (buffer.flush_epoch < flush_epoch &&
                                   *outstanding == max_batches_per_tablet));

                    if buffer.flush_epoch == flush_epoch && *outstanding < max_batches_per_tablet {
                        // Flush the buffer!
                        buffers.push((tablet_id, buffer));
                        // Increment the per-tablet batches outstanding and per-flush epoch batches
                        // outstanding counters.
                        *outstanding += 1;
                        self.flushes[flush_epoch].batches_outstanding += 1;
                    } else {
                        *buffer_opt = Some(buffer);
                    }
                }
            }
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut buffers: usize = 0;
        let mut batches_in_flight: usize = 0;

        for &(ref buffer, batches) in self.tablets.values() {
            if buffer.is_some() { buffers += 1; }
            batches_in_flight += batches as usize;
        }

        write!(f, "State {{ operations_in_lookup: {}, buffers: {}, batches_in_flight: {}, \
                   flushes: {}, buffered_data: {} }}",
               self.operations_in_lookup.len(), buffers, batches_in_flight, self.flushes.len(),
               self.buffered_data)
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
    direct_len: usize,
    indirect_len: usize,
    op_type: OperationType,
}

#[must_use]
struct Buffer {
    operations: Vec<(Row, usize, OperationType)>,
    /// Current amount of buffered data.
    direct_buffered_data: usize,
    indirect_buffered_data: usize,
    flush_epoch: usize,
}

impl Buffer {
    fn new(flush_epoch: usize) -> Buffer {
        Buffer { operations: Vec::new(),
                 direct_buffered_data: 0,
                 indirect_buffered_data: 0,
                 flush_epoch: flush_epoch,
        }
    }

    fn buffered_data(&self) -> usize {
        self.direct_buffered_data + self.indirect_buffered_data
    }
}

/// Holds state associated with a flush epoch.
struct FlushState {
    stats: FlushStats,
    lookups_outstanding: usize,
    batches_outstanding: usize,
    callback: Option<Box<FlushCallback>>,
}

impl fmt::Debug for FlushState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlushState {{ stats: {:?}, lookups_outstanding: {:?}, \
                   batches_outstanding: {:?}, has_callback: {} }}",
               self.stats, self.lookups_outstanding,
               self.batches_outstanding, self.callback.is_some())
    }
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

impl FlushStats {
    fn new(epoch: usize) -> FlushStats {
        FlushStats {
            epoch: epoch,
            successful_batches: 0,
            failed_batches: 0,
            successful_operations: 0,
            failed_operations: 0,
            data: 0,
        }
    }
}

impl FlushStats {
    pub fn epoch(&self) -> usize {
        self.epoch
    }
    pub fn successful_batches(&self) -> usize {
        self.successful_batches
    }
    pub fn failed_batches(&self) -> usize {
        self.failed_batches
    }
    pub fn successful_operations(&self) -> usize {
        self.successful_operations
    }
    pub fn failed_operations(&self) -> usize {
        self.failed_operations
    }
    pub fn data(&self) -> usize {
        self.data
    }
}

impl Writer {

    #[doc(hidden)]
    pub fn new(table: Table, config: WriterConfig) -> Writer {
        let flush = FlushState {
            stats: FlushStats::new(0),
            lookups_outstanding: 0,
            batches_outstanding: 0,
            callback: None,
        };
        let mut flushes = QueueMap::new();
        debug_assert_eq!(0, flushes.push(flush));

        let (config, event_channel) = config.into_config();
        Writer {
            inner: Arc::new(Inner {
                table: table,
                config: config,
                state: Mutex::new(State {
                    operations_in_lookup: QueueMap::new(),
                    tablets: HashMap::new(),
                    flushes: flushes,
                    buffered_data: 0,
                }),
            }),
            event_channel: event_channel,
        }
    }

    pub fn insert(&self, row: Row) {
        self.apply(row, OperationType::Insert);
    }

    pub fn update(&self, row: Row) {
        self.apply(row, OperationType::Update);
    }

    pub fn upsert(&self, row: Row) {
        self.apply(row, OperationType::Upsert);
    }

    pub fn delete(&self, row: Row) {
        self.apply(row, OperationType::Delete);
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
        let (direct_len, indirect_len) = OperationEncoder::encoded_len(&row);
        let encoded_len = direct_len + indirect_len;

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.config().max_data_per_batch {
            self.fail_operation(row, op_type, Error::InvalidArgument(
                    "row operation size is greater than the max batch size".to_owned()));
            return;
        }

        let idx = {
            let mut state = self.lock_state();

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
            state.buffered_data += data;
            let flush_epoch = state.flush_epoch();
            state.flushes[flush_epoch].lookups_outstanding += 1;
            state.operations_in_lookup.push(OperationInLookup {
                row: row,
                flush_epoch: flush_epoch,
                direct_len: direct_len,
                indirect_len: indirect_len,
                op_type: op_type,
            })
        };

        let writer: Writer = self.clone();
        let deadline = Instant::now() + self.config().flush_timeout;
        self.meta_cache().tablet_id(partition_key, deadline, move |result| {
            writer.op_lookup_complete(idx, result);
        });
    }

    /// Flush the `Writer`. The provided callback is called with statistics about the flush when
    /// it compeletes.
    pub fn flush<F>(&self, cb: F) where F: FnOnce(FlushStats) + Send + 'static {
        let cb = Box::new(cb);
        let mut buffers = Vec::new();
        let mut state = self.lock_state();

        let new_flush_epoch = state.flushes.push(FlushState {
            stats: FlushStats::new(0),
            lookups_outstanding: 0,
            batches_outstanding: 0,
            callback: None,
        });
        let flush_epoch = new_flush_epoch - 1;

        state.flushes[flush_epoch].callback = Some(cb);

        // If all flush epochs before the new one have 0 outstanding lookups, then we can flush all
        // batches associated with the flushed epoch.
        state.flush(flush_epoch, self.config(), &mut buffers);
        drop(state);
        for (tablet_id, buffer) in buffers {
            Batch::send(self.clone(), tablet_id, buffer);
        }
    }

    fn op_lookup_complete(&self, idx: usize, result: Result<Option<TabletId>>) {
        let mut buffers = Vec::new();
        let mut state = self.lock_state();

        // Retrieve the operation in lookup, and decrement the epoch's operations in lookup
        // counter.
        let OperationInLookup { row, flush_epoch, direct_len, indirect_len, op_type } =
            state.operations_in_lookup.remove(idx).unwrap();
        let encoded_len = direct_len + indirect_len;
        state.flushes[flush_epoch].lookups_outstanding -= 1;

        // Temporarily decrease the buffered data amount by the operation's size so that early
        // exit points don't have to handle it.
        state.buffered_data -= encoded_len;

        let failed_op = match result {
            Ok(Some(tablet_id)) => {
                let State { ref mut tablets, ref mut flushes, ref mut buffered_data, .. } = *state;
                let (ref mut buffer, ref mut batches_in_flight) = *tablets.entry(tablet_id)
                                                                          .or_insert_with(|| (None, 0));
                let failed_op = match *buffer {
                    None => {
                        // Case [1].
                        trace!("creating new buffer for tablet {:?}", tablet_id);
                        *buffer = Some(Buffer::new(flush_epoch));
                        None
                    },
                    Some(ref mut buffer) if buffer.buffered_data().saturating_add(encoded_len) >
                                            self.config().max_data_per_batch => {
                        if *batches_in_flight >= self.config().max_batches_per_tablet {
                            // Case [3].
                            // TODO: this clone is unfortunate, as is the entire control flow of
                            // this funtion.
                            Some((row.clone(), op_type, Error::Backoff))
                        } else {
                            // Case [2].
                            let buffer = mem::replace(buffer, Buffer::new(flush_epoch));
                            buffers.push((tablet_id, buffer));
                            None
                        }
                    }
                    _ => None,
                };

                if failed_op.is_none() {
                    let buffer = buffer.as_mut().unwrap();

                    // Add the operation to the buffer.
                    buffer.operations.push((row, idx, op_type));
                    buffer.direct_buffered_data += direct_len;
                    buffer.indirect_buffered_data += indirect_len;
                    *buffered_data += encoded_len;

                    // Check if the operation's epoch falls before the buffer's epoch. If so, we
                    // need to back-date the buffer to the older epoch so that the new operation
                    // gets flushed at the appropriate time.
                    if flush_epoch < buffer.flush_epoch {
                        flushes[buffer.flush_epoch].batches_outstanding -= 1;
                        flushes[flush_epoch].batches_outstanding += 1;
                        buffer.flush_epoch = flush_epoch;
                    }
                }
                failed_op
            },
            Ok(None) => Some((row, op_type, Error::NoRangePartition)),
            Err(error) => Some((row, op_type, error)),
        };

        // If all flush epochs before the new one have 0 outstanding lookups, then we can flush all
        // batches associated with the flushed epoch.
        if state.flush_epoch() > flush_epoch && state.flushes[flush_epoch].lookups_outstanding == 0 {
            state.flush(flush_epoch, self.config(), &mut buffers);
        }

        drop(state);

        for (tablet_id, buffer) in buffers {
            Batch::send(self.clone(), tablet_id, buffer);
        }

        if let Some((row, op_type, error)) = failed_op {
            self.fail_operation(row, op_type, error);
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
            let mut state = self.lock_state();
            state.tablets.get_mut(&tablet).unwrap().1 -= 1;
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
                state.flushes.len() > 1 && flush.lookups_outstanding == 0 && flush.batches_outstanding == 0
            } {
                complete_flushes.push(state.flushes.pop().unwrap().1);
                oldest_epoch = state.flushes.front_key().unwrap();
            }
        }

        for flush in complete_flushes {
            let FlushState { stats, callback, .. } = flush;
            if let Some(ref channel) = self.event_channel {
                if channel.try_send(Event::Flush(stats.clone())).is_err() {
                    debug!("failed to send flush to event channel");
                }
            }
            callback.unwrap().call(stats);
        }
    }

    fn schema(&self) -> &Schema {
        self.inner.table.schema()
    }

    fn partition_schema(&self) -> &PartitionSchema {
        self.inner.table.partition_schema()
    }

    fn fail_operation(&self, row: Row, op_type: OperationType, error: Error) {
        if self.config().event_set.has_failed_operations() {
            if let Some(ref channel) = self.event_channel {
                if channel.try_send(Event::FailedOperation(row, op_type, error)).is_err() {
                    debug!("failed to send operation failure to event channel");
                }
            }
        }
    }

    fn config(&self) -> &Config {
        &self.inner.config
    }

    fn lock_state(&self) -> MutexGuard<State> {
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

#[must_use]
struct Batch {
    tablet: TabletId,
    leader_addrs: Vec<SocketAddr>,
    operations: Vec<(Row, usize, OperationType)>,
    writer: Writer,
    backoff: Backoff,
    buffered_data: usize,
    flush_epoch: usize,
}

impl Batch {

    /// Transforms the buffer into a batch, and sends it to the tablet server.
    fn send(writer: Writer, tablet_id: TabletId, buffer: Buffer) {
        let Buffer { mut operations, direct_buffered_data,
                     indirect_buffered_data, flush_epoch } = buffer;
        trace!("Flushing buffer; tablet {}, flush_epoch: {}, operations: {:?}",
               tablet_id, flush_epoch, operations);

        // Sort the operations so that they are written in apply order.
        // TODO: the operations should already be in order, look into bubble or insertion sort.
        operations.sort_by(|a, b| a.1.cmp(&b.1));

        let mut encoder = OperationEncoder::with_capacity(direct_buffered_data,
                                                          indirect_buffered_data);

        for &(ref row, _, op_type) in &operations {
            encoder.encode_row(op_type.as_pb(), row);
        }

        let (data, indirect_data) = encoder.unwrap();
        debug_assert_eq!(data.len(), direct_buffered_data, "direct data length");
        debug_assert_eq!(indirect_data.len(), indirect_buffered_data, "indirect data length");

        let mut message = tserver::WriteRequestPB::new();
        message.mut_row_operations().set_rows(data);
        message.mut_row_operations().set_indirect_data(indirect_data);
        message.set_schema(writer.schema().as_pb());
        message.set_propagated_timestamp(writer.client().latest_observed_timestamp());
        message.set_tablet_id(tablet_id.to_string().into_bytes());

        let rpc = tablet_server::write(util::dummy_addr(),
                                       Instant::now() + writer.config().flush_timeout,
                                       message);

        let backoff = Backoff::with_duration_range(10, util::duration_to_ms(&writer.config().flush_timeout) as u32 / 2);
        let batch = Batch {
            tablet: tablet_id,
            leader_addrs: Vec::new(),
            operations: operations,
            writer: writer,
            backoff: backoff,
            buffered_data: direct_buffered_data + indirect_buffered_data,
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

                trace!("batch response: {:?}", response);

                if response.has_error() {
                    // TODO
                    unimplemented!();
                }

                let failed_ops = response.get_per_row_errors().len();
                let successful_ops = self.operations.len() - failed_ops;

                debug!("batch complete; successful ops: {}, failed ops: {}",
                       successful_ops, failed_ops);

                if self.writer.config().event_set.has_failed_operations() {
                    let &mut Batch { ref mut operations, ref writer, .. } = &mut *self;
                    if let Some(ref channel) = writer.event_channel {
                        let mut errors = response.take_per_row_errors();
                        errors.sort_by_key(|error| error.get_row_index());

                        for mut error in errors.into_iter().rev() {
                            {
                                let ops = operations.drain(error.get_row_index() as usize + 1..);
                                if writer.config().event_set.has_successful_operation() {
                                    for (row, _, op_type) in ops {
                                        if channel.try_send(Event::SuccessfulOperation(row, op_type)).is_err() {
                                            debug!("failed to send successful operation to event channel");
                                        }
                                    }
                                } else {
                                    for _ in ops {}
                                }
                            }
                            let (row, _, op_type) = operations.pop().unwrap();
                            let error = error::TabletServerError::from(error.take_error());
                            if let Err(error) = channel.try_send(Event::FailedOperation(row, op_type, Error::TabletServer(error))) {
                                debug!("failed to send failed operation to event channel: {}", error);
                            }
                        }

                        if writer.config().event_set.has_successful_operation() {
                            for (row, _, op_type) in operations.drain(..) {
                                if channel.try_send(Event::SuccessfulOperation(row, op_type)).is_err() {
                                    debug!("failed to send successful operation to event channel");
                                }
                            }
                        }
                    }
                }

                self.writer.batch_complete(true,
                                           self.tablet,
                                           self.flush_epoch,
                                           successful_ops,
                                           failed_ops,
                                           self.buffered_data);
            },
            Err(error) => {
                panic!("error handling unimplemented. error: {}", error);
            }
        }
    }

    fn retry(mut self, rpc: Rpc) {
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

impl Callback for Batch {
    fn callback(self: Box<Self>, result: Result<()>, rpc: Rpc) {
        self.handle_response(result, rpc);
    }
}

trait FlushCallback: Send {
    fn call(self: Box<Self>, stats: FlushStats);
}
impl <F> FlushCallback for F where F: FnOnce(FlushStats) + Send {
    fn call(self: Box<F>, stats: FlushStats) {
        self(stats)
    }
}

#[cfg(test)]
mod test {

    use std::sync::mpsc::sync_channel;
    use std::time::{Duration, Instant};

    use ClientConfig;
    use Client;
    use Column;
    use DataType;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use super::*;

    use env_logger;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn insert() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(&MiniClusterConfig::default());

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("insert", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        // Don't wait for table creation in order to test retry logic.

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        let mut config = WriterConfig::default();
        let recv = config.event_channel();
        config.event_set = EventSet::FailedOperations;

        let writer = table.new_writer(config);

        // Insert a bunch of values
        for i in 0..10 {
            let mut insert = table.schema().new_row();
            insert.set_by_name::<i32>("key", i).unwrap();
            insert.set_by_name::<i32>("val", i).unwrap();
            writer.insert(insert);
        }

        // Insert a duplicate value
        {
            let mut insert = table.schema().new_row();
            insert.set_by_name::<i32>("key", 1).unwrap();
            insert.set_by_name::<i32>("val", 1).unwrap();

            writer.insert(insert);
        }

        // Insert a null value
        {
            let mut insert = table.schema().new_row();
            insert.set_by_name::<i32>("key", 11).unwrap();
            insert.set_by_name::<Option<i32>>("val", None).unwrap();
            writer.insert(insert);
        }

        let (send, flush_recv) = sync_channel(100);
        writer.flush(move |stats| send.send(stats).unwrap());

        drop(writer);

        let events = recv.into_iter().collect::<Vec<_>>();
        assert_eq!(2, events.len());
        assert!(events[0].is_failed_operation(), "expected FailedOperation, got: {:?}", events[0]);
        assert!(events[1].is_flush(), "expected Flush, got: {:?}", events[1]);

        let flush = flush_recv.recv().unwrap();
        assert_eq!(flush.epoch(), 0);
        assert_eq!(flush.successful_batches(), 4);
        assert_eq!(flush.failed_batches(), 0);
        assert_eq!(flush.successful_operations(), 11);
        assert_eq!(flush.failed_operations(), 1);
        assert_eq!(flush.data(), 128);
    }
}
