use std::collections::HashMap;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use kudu_pb::tserver;

use Client;
use Error;
use Operation;
use Result;
use TabletId;
use backoff::Backoff;
use queue_map::QueueMap;
use row::OperationEncoder;
use rpc::{
    Callback,
    Rpc,
    tablet_server,
};
use util;

#[derive(Clone)]
pub struct SessionConfig {

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

    /// Maximum amount of row operation data to buffer in the Session. If an operation is applied
    /// to the `Session` which would push the amount of buffered data over this limit, the
    /// operation will fail with `Error::Backoff`. In-flight rows (rows which have already been
    /// dispatched via RPC to a tablet server) and rows which are not yet flushed count towards
    /// this limit.
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
    /// `max_buffered_data * early_flush_watermark / 100` the `Session` will automatically flush
    /// the largest batches. In order to preemptively make space for new operations to be applied.
    ///
    /// Defaults to 80. Must be between 0 (exclusive) and 100 (inclusive).
    early_flush_watermark: u8,
}

impl Default for SessionConfig {
    fn default() -> SessionConfig {
        SessionConfig {
            flush_timeout: Duration::from_secs(30),
            background_flush_interval: Duration::from_secs(1),
            max_buffered_data: 256 * 1024 * 1024,
            max_data_per_batch: 7 * 1024 * 1024,
            max_batches_per_tablet: 2,
            early_flush_watermark: 80,
        }
    }
}

pub struct Session<E> {
    client: Client,
    inner: Arc<Mutex<Inner<E>>>,
    config: SessionConfig,
}

/// `Inner` wraps all the mutable state of a session.
///
/// # Data Flow
///
/// When operations are first applied to a session, they are put into the
/// `Inner::operations_in_lookup` queue. This assigns operations a `Session`-unique `idx`.  The
/// `idx` is used to ensure that operations to the same tablet are sent in `apply` order[1].
///
/// When the operation location lookup is done and the `MetaCache` executes the callback, the
/// operation is moved from `operations_in_lookup` to the appropriate batch in `batches`.
///
/// # Flush Epoch
///
/// Each time the application calls `Session::flush`, `Inner` will increment its flush-epoch and
/// attempt to send all outstanding `Batch` instances to their respective tablet server. A `Batch`
/// may not be able to be sent if there are already `SessionConfig::max_batches_per_tablet`
/// in-flight to the tablet. In this case the `Batch` will stay in `batches`, and will be flushed
/// when an in-flight `Batch` to that tablet is complete.
///
/// [1] There are two known holes in the `apply`-order guarantee:
///     1. two operations can have their `MetaCache` lookup complete out of order, and subsequently
///        get added to two different batches due to a background flush.
///     2. two operations can be added to different batches which are sent in correct `idx` order,
///        but the batches may not be completed by the server in-order due to retries.
struct Inner<E> {
    /// Associative queue tracking operations currently in lookup.
    operations_in_lookup: QueueMap<OperationInLookup<E>>,

    /// Map of tablets to current batch and outstanding batch count.
    batches: HashMap<TabletId, (Option<Batch<E>>, u8)>,

    /// Associative queue tracking flushes currently ongoing.
    flushes: QueueMap<FlushState>,

    /// Counter tracking current amount of buffered data.
    buffered_data: usize,
}

impl <E> Session<E> where E: FnOnce(Operation, Error) + Send + 'static {

    pub fn new(client: Client) -> Session<E> {
        Session::with_config(client, SessionConfig::default())
    }

    pub fn with_config(client: Client, config: SessionConfig) -> Session<E> {
        Session {
            client: client,
            inner: Arc::new(Mutex::new(Inner {
                operations_in_lookup: QueueMap::new(),
                batches: HashMap::new(),
                flushes: QueueMap::new(),
                buffered_data: 0,
            })),
            config: config,
        }
    }

    pub fn apply(&self, operation: Operation, error_cb: E) {
        let partition_key = match operation.partition_key() {
            Ok(partition_key) => partition_key,
            Err(error) => {
                error_cb(operation, error);
                return;
            },
        };
        let encoded_len = OperationEncoder::encoded_len(&operation);

        // Sanity check: if the operation is bigger than the max batch data size,
        // then we must reject it.
        if encoded_len > self.config.max_data_per_batch {
            error_cb(operation, Error::InvalidArgument(
                     "encoded row operation is greater than the max batch size".to_owned()));
            return;
        }

        let meta_cache = operation.meta_cache().clone();
        let idx = {
            let mut inner = self.inner.lock();

            // Check that there is space for the operation. If buffering the operation would push
            // the `buffered_data` counter over the `max_buffered_data` limit, then we reject the
            // operation with `Error::Backoff`, If buffering the operation would push the `Session`
            // over the early-flush watermark, then we preemptively send large batches.
            let data = inner.buffered_data.saturating_add(encoded_len);
            if data > self.config.max_buffered_data {
                drop(inner);
                error_cb(operation, Error::Backoff);
                return;
            } else if data > (self.config.max_buffered_data / 100) *
                             self.config.early_flush_watermark as usize {
                // Do early flush.
            }

            // Add the operation to the operations_in_flight queue. This assigns a Session-unique
            // idx which uniquely identifies the operation and gives it a total ordering among
            // applied operations in the Session.
            inner.buffered_data = data;
            let flush_epoch = inner.flush_epoch();
            inner.operations_in_lookup.push(
                OperationInLookup::new(operation, error_cb, flush_epoch, encoded_len))
        };

        let session = self.clone();
        meta_cache.tablet_id(partition_key, Instant::now() + self.config.flush_timeout, move |result| {
            if let Some((operation, error_cb, error)) =
                session.operation_location_lookup_complete(idx, result) {
                error_cb(operation, error);
            }
        });
    }

    /// Flush the `Session`. The provided callback is called with statistics about the flush when
    /// it compeletes.
    pub fn flush<F>(&self, cb: F) where F: FnOnce(FlushStats) + Send + 'static {
        unimplemented!()
    }

    /// The per-operation `MetaCache` lookup callback.
    ///
    /// Moves an operation from `operations_in_lookup` to the batch assigned to the tablet.
    /// There are a few non-happy path cases to account for:
    ///
    /// * [1] The batch does not exist
    ///          If the batch doesn't exist a new one is created.
    /// * [2] The batch is full
    ///          If applying the operation to the batch would push it over `max_data_per_batch`
    ///          limit, then the batch is sent to the tablet server.
    /// * [3] The batch is full, and the tablet has reached the limit of batches in-flight
    ///          If sending the full batch would result in exceeding the `max_batches_per_tablet`
    ///          limit, then the operation is failed with Error::Backoff.
    ///
    /// If the location lookup or case [3] occurs, then this method returns the operation, the
    /// error callback, and the associated error. The caller should complete the error callback.
    fn operation_location_lookup_complete(&self,
                                          idx: usize,
                                          result: Result<Option<TabletId>>)
                                          -> Option<(Operation, E, Error)> {
        let mut inner = self.inner.lock();

        // Retrieve the operation in lookup, and decrement the epoch's
        // operations in lookup counter.
        let OperationInLookup { operation, error_cb, flush_epoch, encoded_len } =
            inner.operations_in_lookup.remove(idx).unwrap();
        debug_assert!(inner.flushes[flush_epoch].lookups_outstanding > 0);
        inner.flushes[flush_epoch].lookups_outstanding -= 1;

        // Temporarily decrease the buffered data amount by the operation's length so that early
        // exit points don't have to handle it.
        debug_assert!(inner.buffered_data > encoded_len);
        inner.buffered_data -= encoded_len;

        match result {
            Ok(Some(tablet_id)) => {
                let Inner { ref mut batches, ref mut flushes, ref mut buffered_data, .. } = *inner;
                let (ref mut batch, ref mut outstanding) = *batches.entry(tablet_id)
                                                                   .or_insert_with(|| (None, 0));
                match *batch {
                    None => {
                        // Case [1].
                        *batch = Some(Batch::new(tablet_id, self.clone(), flush_epoch));
                        *outstanding += 1;
                    },
                    Some(ref mut batch) if batch.buffered_data.saturating_add(encoded_len) >
                                        self.config.max_data_per_batch => {
                        if *outstanding >= self.config.max_batches_per_tablet {
                            // Case [3].
                            return Some((operation, error_cb, Error::Backoff));
                        }
                        // Case [2].
                        mem::replace(batch, Batch::new(tablet_id, self.clone(), flush_epoch)).send();
                        *outstanding += 1;
                    }
                    _ => (),
                };

                let batch = batch.as_mut().unwrap();

                // Ensure that the table schema associated with the batch matches the new operation
                // schema. We could potentially handle this by flushing the batch, but applications
                // really shouldn't be mixing schemas for a single Session, so we just fail the
                // operation instead.
                if let Some(&(_, ref batch_operation, _)) = batch.operations.get(0) {
                    if batch_operation.row().schema() != operation.row().schema() {
                        let error = Error::InvalidArgument(
                            "operation schema must match existing schemas in session batch".to_owned());
                        return Some((operation, error_cb, error));
                    }
                }

                // Add the operation to the batch.
                batch.operations.push((idx, operation, error_cb));
                batch.buffered_data += encoded_len;
                *buffered_data += encoded_len;

                // Check if the operation's epoch falls before the batch's epoch. If so, we need to
                // back-date the batch to the older epoch so that the new operation gets flushed at
                // the appropriate time.
                if flush_epoch < batch.flush_epoch {
                    debug_assert!(flushes[batch.flush_epoch].batches_outstanding > 0);
                    flushes[batch.flush_epoch].batches_outstanding -= 1;
                    flushes[flush_epoch].batches_outstanding += 1;
                    batch.flush_epoch = flush_epoch;
                }
                None
            },
            Ok(None) => Some((operation, error_cb, Error::NoRangePartition)),
            Err(error) => Some((operation, error_cb, error)),
        }
    }
}

impl <E> Inner<E> {

    /// Returns the current flush epoch.
    fn flush_epoch(&self) -> usize {
        self.flushes.back_key().unwrap()
    }
}

impl <E> Clone for Session<E> {
    // Derive isn't sufficient, since E doesn't need to be Clone.
    fn clone(&self) -> Session<E> {
        Session {
            client: self.client.clone(),
            inner: self.inner.clone(),
            config: self.config.clone(),
        }
    }
}

#[must_use]
struct Batch<E> {
    tablet: TabletId,
    leader_addrs: Vec<SocketAddr>,
    operations: Vec<(usize, Operation, E)>,
    session: Session<E>,
    backoff: Backoff,
    buffered_data: usize,
    flush_epoch: usize,
}

impl <E> Batch<E> where E: Send + 'static {

    /// Creates a new `Batch`.
    pub fn new(tablet: TabletId, session: Session<E>, flush_epoch: usize) -> Batch<E> {
        let backoff = Backoff::with_duration_range(10, util::duration_to_ms(&session.config.flush_timeout) as u32 / 2);
        Batch {
            tablet: tablet,
            leader_addrs: Vec::new(),
            operations: Vec::new(),
            session: session,
            backoff: backoff,
            buffered_data: 0,
            flush_epoch: flush_epoch,
        }
    }

    /// Dispatches the `Batch` to the tablet server.
    fn send(mut self) {
        // Sort the operations so that they are written in apply order.
        // TODO: the operations should already be in order, look into bubble or insertion sort.
        self.operations.sort_by(|a, b| a.0.cmp(&b.0));

        let data_len = self.operations[0].1.row().schema().row_size() * self.operations.len();
        let indirect_data_len = self.buffered_data - data_len;
        let mut encoder = OperationEncoder::with_capacity(data_len, indirect_data_len);

        for &(_, ref operation, _) in &self.operations {
            encoder.encode(operation);
        }

        let (data, indirect_data) = encoder.unwrap();
        debug_assert_eq!(data.len(), data_len);
        debug_assert_eq!(indirect_data.len(), indirect_data_len);

        let mut message = tserver::WriteRequestPB::new();
        message.mut_row_operations().set_rows(data);
        message.mut_row_operations().set_indirect_data(indirect_data);
        message.set_schema(self.operations[0].1.row().schema().as_pb());
        message.set_propagated_timestamp(self.session.client.latest_observed_timestamp());


        let rpc = tablet_server::write(util::dummy_addr(),
                                       Instant::now() + self.session.config.flush_timeout,
                                       message);

        self.lookup_locations(rpc);
    }

    fn handle_result(mut self: Box<Self>, result: Result<()>, rpc: Rpc) {
        unimplemented!()
    }

    fn lookup_locations(mut self, rpc: Rpc) {
        let partition_key = self.operations[0].1.partition_key().unwrap();

        self.operations[0].1.meta_cache().clone().tablet_leader(partition_key, rpc.deadline, move |tablet| {
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
                    warn!("unable to look up leader address for tablet {}", self.tablet);
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
                let messenger = self.session.client.messenger().clone();
                rpc.callback = Some(Box::new(self));
                messenger.send(rpc);
            },
            None => {
                self.retry(rpc);
            }
        }
    }

    fn retry(mut self, rpc: Rpc) {
        let duration = Duration::from_millis(self.backoff.next_backoff_ms());
        self.session.client.messenger().clone().timer(duration, Box::new(move || {
            self.lookup_locations(rpc);
        }));
    }

    /// Reapply each operation in the batch to the session with the same epoch.
    fn reapply(self) {
        unimplemented!()
    }
}

impl <E> Callback for Batch<E> where E: Send + 'static {
    fn callback(mut self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
        self.handle_result(result, rpc);
    }
}

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

#[must_use]
struct OperationInLookup<E> {
    operation: Operation,
    error_cb: E,
    flush_epoch: usize,
    encoded_len: usize,
}

impl <E> OperationInLookup<E> {
    pub fn new(operation: Operation,
               error_cb: E,
               flush_epoch: usize,
               encoded_len: usize)
               -> OperationInLookup<E> {
        OperationInLookup {
            operation: operation,
            error_cb: error_cb,
            flush_epoch: flush_epoch,
            encoded_len: encoded_len,
        }
    }
}

struct FlushState {
    stats: FlushStats,
    lookups_outstanding: usize,
    batches_outstanding: usize,
    callback: Option<Box<FnOnce(FlushStats) + Send + 'static>>,
}

#[cfg(test)]
mod test {

    use std::time::{Duration, Instant};
    use std::sync::mpsc::sync_channel;

    use AlterTableBuilder;
    use ClientConfig;
    use Client;
    use Column;
    use DataType;
    use RangePartitionBound;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use schema::tests::simple_schema;
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

        let mut table_builder = TableBuilder::new("create_and_delete_table", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        let session = client.new_session();
        let (send, recv) = sync_channel(2);

        {
            let mut insert = table.insert();
            insert.mut_row().set_by_name("key", 123i32);
            insert.mut_row().set_by_name("val", 123i32);
            let send = send.clone();

            session.apply(insert, |operation, error| send.send((operation, error)).unwrap());
        }
    }
}
