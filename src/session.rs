use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use parking_lot::Mutex;

use Client;
use Error;
use Operation;
use OperationError;
use OperationResult;
use queue_map::QueueMap;
use row::OperationEncoder;
use TableId;
use TabletId;
use Tablet;

pub struct SessionConfig {
    flush_timeout: Duration,
}

pub struct Session<F> where F: FnOnce(OperationResult) {
    client: Client,
    inner: Arc<Mutex<Inner<F>>>,
    stats: Arc<FlushStats>,
}

struct Inner<F> where F: FnOnce(OperationResult) {
    operations_in_lookup: QueueMap<(Operation, F)>,
    batches: HashMap<TabletId, Batch<F>>,
    flushing_batches: QueueMap<Batch<F>>,
    flush_callback: Option<Box<FnOnce(FlushStats)>>,
}

impl <F> Session<F> where F: FnOnce(OperationResult) {

    pub fn apply(&self, operation: Operation, callback: F) {
        let table = operation.table();
        let idx = {
            let mut inner = self.inner.lock();
            inner.operations_in_lookup.push((operation, callback))
        };

        let meta_cache = self.client.meta_cache(&table);



        //meta_cache.get(
    }

    /// This function takes the session as a mutable reference, because it makes the implementation
    /// easier, and flushing is extremely difficult to use correctly with a shared session.
    /// Consider that the flush is completed only after waiting for the most recently inserted
    /// operations; it doesn't wait for prior flushes to finish. If multiple threads are sharing a
    /// single session and flushing, it's very difficult to know whether the flush actually
    /// contained the data any individual thread thought was in it.
    pub fn flush<FF>(&mut self, cb: FF) where FF: FnOnce(FlushStats) {
    }
}

struct Batch<F> where F: FnOnce(OperationResult) {
    tablet: Tablet,
    operations: Vec<(usize, Operation, F)>,
    session: Session<F>,
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
