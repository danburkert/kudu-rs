#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::{
    HashMap,
    VecDeque,
};
use std::collections::hash_map::Entry;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{
    Async,
    Future,
    Poll,
    Sink,
    Stream,
    future,
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
use Row;
use backoff::Backoff;
use key;
use operation::{
    Operation,
    OperationDecoder,
    OperationEncoder,
    OperationError,
    OperationKind,
};
use partition::PartitionKey;
use pb::tserver::{
    TabletServerService,
    WriteRequestPb,
    WriteResponsePb,
};
use replica::{
    Replica,
    ReplicaRpc,
    ReplicaSet,
    Selection,
    Speculation,
};
use tablet::Tablet;
use tokio_timer::Delay;

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
                                                    Error=(Operation<'static>, Error)> + Send>>,

    /// Batchers; one per tablet server.
    batchers: HashMap<TabletId, TabletBatcher>,

    /// Current amount of unflushed data (in-lookup + queued batches + batches in flight).
    buffered_data: usize,

    /// Stats for current flush.
    flush_stats: FlushStats,

    common: Common,
}

struct Common {
    config: WriterConfig,
    table: Table,

    batches_in_flight: FuturesUnordered<Box<Future<Item=BatchStats, Error=BatchError> + Send>>,

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
            flush_stats: FlushStats::new(),
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

        self.poll_batches_in_flight(false)?;

        // TODO: figure out if the amount of data in batches is over the early flush watermark.

        if self.buffered_data >= self.common.config.max_buffered_data {
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(()))
        }
    }

    pub fn poll_flush(&mut self) -> Poll<FlushStats, Error> {
        debug!("{:?}: poll_flush", self);
        self.poll_operations_in_lookup()?;

        // Flush all tablets which have not hit their max batches in flight limit.
        for batcher in self.batchers.values_mut() {
            batcher.flush(&mut self.common, false);
        }

        self.poll_batches_in_flight(true)?;

        if self.buffered_data == 0 {
            Ok(Async::Ready(mem::replace(&mut self.flush_stats, FlushStats::new())))
        } else {
            Ok(Async::NotReady)
        }
    }

    pub fn flush(self) -> Flush {
        Flush { writer: Some(self) }
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

        // TODO: encode the partition key into a cached buffer in order to avoid allocating for
        // every insert.
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

    pub fn apply_all<'data, I>(self, rows: I) -> WriteAll<I::IntoIter>
        where I: IntoIterator<Item=Operation<'data>>
    {
        WriteAll {
            items: rows.into_iter(),
            f: Writer::apply,
            writer: Some(self)
        }
    }

    pub fn insert(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Insert })
    }

    pub fn insert_all<'data, I>(self, rows: I) -> WriteAll<I::IntoIter>
        where I: IntoIterator<Item=Row<'data>>
    {
        WriteAll {
            items: rows.into_iter(),
            f: Writer::insert,
            writer: Some(self)
        }
    }

    pub fn update(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Update })
    }

    pub fn update_all<'data, I>(self, rows: I) -> WriteAll<I::IntoIter>
        where I: IntoIterator<Item=Row<'data>>
    {
        WriteAll {
            items: rows.into_iter(),
            f: Writer::update,
            writer: Some(self)
        }
    }

    pub fn delete(&mut self, row: Row) {
        self.apply(Operation { row, kind: OperationKind::Delete })
    }

    pub fn delete_all<'data, I>(self, rows: I) -> WriteAll<I::IntoIter>
        where I: IntoIterator<Item=Row<'data>>
    {
        WriteAll {
            items: rows.into_iter(),
            f: Writer::delete,
            writer: Some(self)
        }
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

    fn poll_batches_in_flight(&mut self, flush_batches: bool) -> Poll<(), Error> {
        trace!("{:?}: poll_batches_in_flight; flush_batches: {}", self, flush_batches);
        loop {
            match self.common.batches_in_flight.poll() {
                Ok(Async::Ready(Some(stats))) => {
                    self.buffered_data -= stats.data;
                    self.flush_stats.add_succesful_batch(&stats);

                    match self.batchers.entry(stats.tablet) {
                        Entry::Occupied(ref mut entry) => {
                            entry.get_mut().batches_in_flight -= 1;
                            if flush_batches {
                                entry.get_mut().flush(&mut self.common, false);
                            } else {
                                entry.get_mut().send_batches(&mut self.common);
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
                    self.flush_stats.failed_batches += 1;
                    return Err(error);
                },
            };
        }
    }

    /// Applies an operation to the appropriate tablet batch.
    fn buffer_operation(&mut self, tablet: Arc<Tablet>, op: Operation, encoded_len: usize) {
        trace!("{:?}: buffer_operation; tablet: {:?}, op: {:?}, len: {:?}", self, tablet, op, encoded_len);
        let batcher = self.batchers.entry(tablet.id()).or_insert_with(|| TabletBatcher::new(tablet.clone()));

        // Overwrite the tablet in case it's been updated.
        batcher.tablet = tablet;

        if batcher.batch.encoder.len() + encoded_len > self.common.config.max_data_per_batch {
            batcher.flush(&mut self.common, true);
        }
        batcher.batch.encoder.encode_row(op.kind.as_pb(), &op.row);
        batcher.batch.operations += 1;
    }

    fn fail_operation(&self, operation: Operation, error: Error) {
        let _ = self.common.error_sender.unbounded_send(OperationError {
            row: operation.row.into_owned(),
            kind: operation.kind,
            error,
        });
    }
}

impl fmt::Debug for Writer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer")
            .field("table", &self.common.table.id())
            .field("table_name", &self.common.table.name())
            .field("buffered_data", &self.buffered_data)
            .field("operations_in_lookup", &self.operations_in_lookup.len())
            .field("batches_in_flight", &self.common.batches_in_flight.len())
            .finish()
    }
}

#[derive(Debug)]
pub struct Flush {
    writer: Option<Writer>,
}

impl Future for Flush {
    type Item = (Writer, FlushStats);
    type Error = Error;
    fn poll(&mut self) -> Poll<(Writer, FlushStats), Error> {
        let flush_stats = try_ready!(self.writer.as_mut().unwrap().poll_flush());
        Ok(Async::Ready((self.writer.take().unwrap(), flush_stats)))
    }
}

pub struct WriteAll<Iter>
where Iter: Iterator {
    items: Iter,
    f: fn(&mut Writer, Iter::Item),
    writer: Option<Writer>,
}

impl <Iter> Future for WriteAll<Iter>
where Iter: Iterator {
    type Item = Writer;
    type Error = Error;
    fn poll(&mut self) -> Poll<Writer, Error> {
        loop {
            try_ready!(self.writer.as_mut().unwrap().poll_ready());
            match self.items.next() {
                Some(item) => (self.f)(self.writer.as_mut().unwrap(), item),
                None => return Ok(Async::Ready(self.writer.take().unwrap())),
            }
        }
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
    fn flush(&mut self, common: &mut Common, force: bool) {
        trace!("{:?}: flush; force: {}", self, force);
        if !self.batch.is_empty() &&
           (force || self.batches_in_flight < common.config.max_batches_per_tablet) {
            let batch = mem::replace(&mut self.batch, Batch::new());
            self.batch_queue.push_back(batch);
        }
        self.send_batches(common);
    }

    /// Sends batches waiting in the queue to the remote tablet server, until the maximum number of
    /// batches in-flight limit is reached.
    fn send_batches(&mut self, common: &mut Common) {
        trace!("{:?}: send_batches", self);
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

impl fmt::Debug for TabletBatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer")
            .field("tablet", &self.tablet.id())
            .field("batch-size", &self.batch.encoder.len())
            .field("batches_queue", &self.batch_queue.len())
            .field("batches_in_flight", &self.batches_in_flight)
            .finish()
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
                    debug!("row_errors: {:?}", response.per_row_errors);
                    let row_operations = call2.request.row_operations.as_ref().unwrap();
                    let mut decoder = OperationDecoder::new(&schema,
                                                            row_operations.rows(),
                                                            row_operations.indirect_data());
                    let mut decoder_idx = 0;

                    for error in response.per_row_errors {
                        if error.row_index < 0 || error.row_index as usize >= stats.operations {
                            return Err(Error::Serialization(
                                    format!("row error contains invalid index: {:?}", error)));
                        }
                        let error_idx = error.row_index as usize;
                        if error_idx < decoder_idx {
                            return Err(Error::Serialization(format!("out-of-order row error")));
                        }

                        let operation = decoder.nth(error_idx - decoder_idx).unwrap();
                        decoder_idx = error_idx + 1;

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
    successful_batches: usize,
    failed_batches: usize,
    operations: usize,
    row_errors: usize,
    data: usize,
}

impl FlushStats {
    pub(crate) fn new() -> FlushStats {
        FlushStats {
            successful_batches: 0,
            failed_batches: 0,
            operations: 0,
            row_errors: 0,
            data: 0,
        }
    }

    pub fn batches(&self) -> usize {
        self.successful_batches + self.failed_batches
    }

    pub fn successful_batches(&self) -> usize {
        self.successful_batches
    }

    pub fn failed_batches(&self) -> usize {
        self.failed_batches
    }

    pub fn operations(&self) -> usize {
        self.operations
    }

    pub fn successful_operations(&self) -> usize {
        self.operations - self.row_errors
    }

    pub fn failed_operations(&self) -> usize {
        self.row_errors
    }

    pub fn data(&self) -> usize {
        self.data
    }

    fn add_succesful_batch(&mut self, batch: &BatchStats) {
        self.successful_batches += 1;
        self.operations += batch.operations;
        self.row_errors += batch.row_errors;
        self.data += batch.data;
    }
}

#[cfg(test)]
mod test {

    use std::time::{Duration, Instant};

    use Client;
    use Column;
    use DataType;
    use Options;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use super::*;

    use env_logger;
    use futures::future;
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn insert() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime.block_on(Client::new(cluster.master_addrs(), Options::default()))
                                .expect("client");

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("insert", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        // TODO: don't wait for table creation in order to test retry logic.
        let table_id = runtime.block_on(client.create_table(table_builder)).unwrap();

        let table = runtime.block_on(client.open_table_by_id(table_id)).unwrap();

        let mut writer = table.new_writer(WriterConfig::default());

        // TODO: remove lazy once apply no longer polls.
        runtime.block_on(future::lazy::<_, Result<(), ()>>(|| {
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
            Ok(())
        })).unwrap();

        let stats = runtime.block_on(future::poll_fn(|| writer.poll_flush())).unwrap();
        assert_eq!(stats.successful_batches, 4);
        assert_eq!(stats.failed_batches, 0);
        assert_eq!(stats.operations, 12);
        assert_eq!(stats.row_errors, 1);
        assert_eq!(stats.data, 128);
    }
}
