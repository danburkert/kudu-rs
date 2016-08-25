use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;

use Client;
use Error;
use Operation;
use Tablet;
use TabletId;
use meta_cache;
use queue_map::QueueMap;

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
    stats: Arc<FlushStats>,
    config: SessionConfig,
}

struct Inner<E> {
    operations_in_lookup: QueueMap<(Operation, E)>,
    batches: HashMap<TabletId, Batch<E>>,
    flushing_batches: QueueMap<Batch<E>>,
    flush_callback: Option<Box<FnOnce(FlushStats) + Send + 'static>>,
    buffered_data: usize,
}

impl <E> Session<E> where E: FnOnce(Operation, Error) + Send + 'static {


    /*
    fn do_apply(&self, operation: Operation, error_cb: E) -> Option<(Operation, Error)> {
        let partition_key = match operation.partition_key() {
            Ok(partition_key) => partition_key,
            Err(error) => return Some((operation, error)),
        };
        let data_len = operation.data_len()
        let meta_cache = operation.meta_cache().clone();
        let idx = {
            let mut inner = self.inner.lock();
            inner.operations_in_lookup.push((operation, error_cb))
        };

        let session = self.clone();
        meta_cache.get(partition_key, Instant::now() + self.config.flush_timeout, move |result| {
            match result {
                Ok(meta_cache::Entry::Tablet(tablet)) => {
                    let mut inner = session.inner.lock();
                    let (operation, error_cb) = inner.operations_in_lookup.remove(idx).unwrap();
                    inner.batches
                         .entry(*tablet.id())
                         .or_insert_with(|| Batch::new(tablet, session.clone()))
                         .operations
                         .push((idx, operation, error_cb));

                    // TODO: check if the batch is full.
                    // TODO: schedule a background flush.
                },
                Ok(meta_cache::Entry::NonCoveredRange { .. }) => {
                    let (operation, error_cb) =
                        session.inner.lock().operations_in_lookup.remove(idx).unwrap();
                    error_cb(operation, Error::NoRangePartition);
                },
                Err(error) => {
                    let (operation, error_cb) =
                        session.inner.lock().operations_in_lookup.remove(idx).unwrap();
                    error_cb(operation, error);
                },
            }
        });

    }
    */

    pub fn apply(&self, operation: Operation, error_cb: E) {
        let partition_key = match operation.partition_key() {
            Ok(partition_key) => partition_key,
            Err(error) => {
                error_cb(operation, error);
                return;
            },
        };
        let meta_cache = operation.meta_cache().clone();
        let idx = {
            let mut inner = self.inner.lock();
            inner.operations_in_lookup.push((operation, error_cb))
        };

        let session = self.clone();
        meta_cache.get(partition_key, Instant::now() + self.config.flush_timeout, move |result| {
            match result {
                Ok(meta_cache::Entry::Tablet(tablet)) => {
                    let mut inner = session.inner.lock();
                    let (operation, error_cb) = inner.operations_in_lookup.remove(idx).unwrap();
                    inner.batches
                         .entry(*tablet.id())
                         .or_insert_with(|| Batch::new(tablet, session.clone()))
                         .operations
                         .push((idx, operation, error_cb));

                    // TODO: check if the batch is full.
                    // TODO: schedule a background flush.
                },
                Ok(meta_cache::Entry::NonCoveredRange { .. }) => {
                    let (operation, error_cb) =
                        session.inner.lock().operations_in_lookup.remove(idx).unwrap();
                    error_cb(operation, Error::NoRangePartition);
                },
                Err(error) => {
                    let (operation, error_cb) =
                        session.inner.lock().operations_in_lookup.remove(idx).unwrap();
                    error_cb(operation, error);
                },
            }
        });
    }

    /// This function takes the session as a mutable reference, because it makes the implementation
    /// easier, and flushing is extremely difficult to use correctly with a shared session.
    /// Consider that the flush is completed only after waiting for the most recently inserted
    /// operations; it doesn't wait for prior flushes to finish. If multiple threads are sharing a
    /// single session and flushing, it's very difficult to know whether the flush actually
    /// contained the data any individual thread thought was in it.
    pub fn flush<F>(&mut self, cb: F) where F: FnOnce(FlushStats) + Send + 'static {

    }
}

impl <E> Clone for Session<E> {
    // Derive isn't sufficient, since E doesn't need to be Clone.
    fn clone(&self) -> Session<E> {
        Session {
            client: self.client.clone(),
            inner: self.inner.clone(),
            stats: self.stats.clone(),
            config: self.config.clone(),
        }
    }
}

struct Batch<E> {
    tablet: Tablet,
    operations: Vec<(usize, Operation, E)>,
    session: Session<E>,
}

impl <E> Batch<E> {
    fn new(tablet: Tablet, session: Session<E>) -> Batch<E> {
        Batch {
            tablet: tablet,
            operations: Vec::new(),
            session: session,
        }
    }
}

pub struct FlushStats {
    successful_batches: AtomicUsize,
    failed_batches: AtomicUsize,
    successful_operations: AtomicUsize,
    failed_operations: AtomicUsize,
    data: AtomicUsize,
    outstanding_batches: AtomicUsize,
}

impl FlushStats {
    fn successful_batches(&self) -> usize {
        self.successful_batches.load(Ordering::SeqCst)
    }
    fn failed_batches(&self) -> usize {
        self.successful_batches.load(Ordering::SeqCst)
    }
    fn successful_operations(&self) -> usize {
        self.successful_batches.load(Ordering::SeqCst)
    }
    fn failed_operations(&self) -> usize {
        self.successful_batches.load(Ordering::SeqCst)
    }
    fn data(&self) -> usize {
        self.successful_batches.load(Ordering::SeqCst)
    }
}
