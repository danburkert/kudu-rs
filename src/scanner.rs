use std::fmt;
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Bytes;
use krpc::Proxy;
use futures::{
    Async,
    Future,
    Stream,
    Poll,
};

use meta_cache::{
    Entry,
    TableLocations,
    Tablet,
};

use Error;
use ScannerId;
use pb::{
    ExpectField,
    RowwiseRowBlockPb,
};
use pb::tserver::{
    NewScanRequestPb,
    ScanRequestPb,
    ScanResponsePb,
    TabletServerService,
};
use replica::{
    ReplicaRpc,
    Selection,
    Speculation,
};
use backoff::Backoff;

pub struct Scanner {
    table_locations: TableLocations,
    state: ScannerState,
}

enum ScannerState {
    Lookup {
        lookup: Box<Future<Item=Entry, Error=Error> + Send + 'static>,
    },
    Scan {
        tablet: Arc<Tablet>,
        scanner: TabletScanner,
    },
    Finished,
}

impl Scanner {
    pub(crate) fn new(table_locations: TableLocations) -> Scanner {
        let lookup = Box::new(table_locations.entry(&[]));
        let state = ScannerState::Lookup { lookup };
        Scanner { table_locations, state }
    }
}

impl Stream for Scanner {
    type Item = RowBatch;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<RowBatch>, Error> {
        trace!("Scanner::poll");
        loop {
            match mem::replace(&mut self.state, ScannerState::Finished) {
                ScannerState::Lookup { mut lookup } => {
                    match lookup.poll()? {
                        Async::Ready(Entry::Tablet(tablet)) => {
                            let scanner = TabletScanner::new(tablet.clone());
                            self.state = ScannerState::Scan { tablet, scanner };
                        },
                        Async::Ready(Entry::NonCoveredRange { upper_bound, .. }) => if !upper_bound.is_empty() {
                            let lookup = Box::new(self.table_locations.entry(&upper_bound));
                            self.state = ScannerState::Lookup { lookup };
                        },
                        Async::NotReady => {
                            self.state = ScannerState::Lookup { lookup };
                            return Ok(Async::NotReady);
                        }
                    }
                },
                ScannerState::Scan { tablet, mut scanner } => {
                    match scanner.poll()? {
                        Async::Ready(Some(batch)) => {
                            self.state = ScannerState::Scan { tablet, scanner };
                            return Ok(Async::Ready(Some(batch)))
                        },
                        Async::Ready(None) => if !tablet.upper_bound().is_empty() {
                            let lookup = Box::new(self.table_locations.entry(tablet.upper_bound()));
                            self.state = ScannerState::Lookup { lookup };
                        },
                        Async::NotReady => {
                            self.state = ScannerState::Scan { tablet, scanner };
                            return Ok(Async::NotReady);
                        },
                    }
                },
                ScannerState::Finished => return Ok(Async::Ready(None)),
            }
        }
    }
}

impl fmt::Debug for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Scanner")
            .finish()
    }
}

pub struct RowBatch {
    len: usize,
}

impl RowBatch {
    fn new(block: RowwiseRowBlockPb, sidecars: Vec<Bytes>) -> RowBatch {
        trace!("RowBatch::new; block: {:?}", block);
        RowBatch {
            len: block.num_rows() as usize,
        }
    }
}

enum TabletScanner {
    New {
        rpc: ReplicaRpc<Arc<Tablet>, ScanRequestPb, ScanResponsePb>,
    },
    Continue {
        scanner_id: ScannerId,
        call_seq_id: u32,
        rpc: ReplicaRpc<Proxy, ScanRequestPb, ScanResponsePb>,
    },
    Finished,
}

impl TabletScanner {
    fn new(tablet: Arc<Tablet>) -> TabletScanner {
        debug!("TabletScanner::new; tablet: {:?}", &*tablet);
        let mut request = ScanRequestPb::default();
        request.new_scan_request
               .get_or_insert_with(NewScanRequestPb::default)
               .tablet_id = tablet.id().to_string().into_bytes();
        let call = TabletServerService::scan(Arc::new(request),
                                             Instant::now() + Duration::from_secs(60));
        let rpc = ReplicaRpc::new(tablet,
                                  call,
                                  Speculation::Staggered(Duration::from_millis(100)),
                                  Selection::Closest,
                                  Backoff::default());
        TabletScanner::New { rpc }
    }

    fn cont(scanner_id: ScannerId, call_seq_id: u32, proxy: Proxy) -> TabletScanner {
        let mut request = ScanRequestPb::default();
        request.scanner_id = Some(scanner_id.to_string().into_bytes());
        request.call_seq_id = Some(call_seq_id);

        let call = TabletServerService::scan(Arc::new(request),
                                                Instant::now() + Duration::from_secs(60));

        let rpc = ReplicaRpc::new(proxy,
                                  call,
                                  Speculation::Full,
                                  Selection::Closest,
                                  Backoff::default());
        TabletScanner::Continue { scanner_id, call_seq_id, rpc }
    }
}

impl Stream for TabletScanner {
    type Item = RowBatch;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<RowBatch>, Error> {
        trace!("TabletScanner::poll");
        match self {
            TabletScanner::New { ref mut rpc } => {
                let (proxy, mut response, sidecars) = try_ready!(rpc.poll());
                let batch = RowBatch::new(response.data.take().unwrap_or_default(), sidecars);
                *self = if response.has_more_results() {
                    let scanner_id = ScannerId::parse_bytes(&response.scanner_id
                                                                     .expect_field("ScanResponsePb",
                                                                                   "scanner_id")?)?;
                    TabletScanner::cont(scanner_id, 1, proxy)
                } else {
                    TabletScanner::Finished
                };

                Ok(Async::Ready(Some(batch)))
            },
            TabletScanner::Continue { scanner_id, call_seq_id, ref mut rpc } => {
                let (proxy, mut response, sidecars) = try_ready!(rpc.poll());
                let batch = RowBatch::new(response.data.take().unwrap_or_default(), sidecars);

                *self = if response.has_more_results() {
                    TabletScanner::cont(*scanner_id, *call_seq_id + 1, proxy)
                } else {
                    TabletScanner::Finished
                };

                Ok(Async::Ready(Some(batch)))
            },
            TabletScanner::Finished => Ok(Async::Ready(None)),
        }
    }
}

#[cfg(test)]
mod test {

    use Client;
    use Column;
    use DataType;
    use Options;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::MiniCluster;
    use super::*;
    use WriterConfig;

    use env_logger;
    use futures::future;
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn count() {
        let _ = env_logger::init();
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

        let mut table_builder = TableBuilder::new("count", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        // TODO: don't wait for table creation in order to test retry logic.
        let table_id = runtime.block_on(client.create_table(table_builder)).unwrap();

        let table = runtime.block_on(client.open_table_by_id(table_id)).unwrap();

        let mut writer = table.new_writer(WriterConfig::default());

        let num_rows = 100;

        // TODO: remove lazy once apply no longer polls.
        runtime.block_on(future::lazy::<_, Result<(), ()>>(|| {
            // Insert a bunch of values
            for i in 0..num_rows {
                let mut insert = table.schema().new_row();
                insert.set_by_name::<i32>("key", i).unwrap();
                insert.set_by_name::<i32>("val", i).unwrap();
                writer.insert(insert);
            }
            Ok(())
        })).unwrap();

        runtime.block_on(future::poll_fn(|| writer.poll_flush())).unwrap();

        let scanner: Scanner = runtime.block_on(::futures::future::lazy::<_, Result<Scanner, ()>>(|| Ok(table.new_scanner()))).unwrap();

        let batches: Vec<RowBatch> = runtime.block_on(::futures::future::lazy(|| scanner.collect())).unwrap();

        assert_eq!(num_rows as usize, batches.into_iter().map(|batch| batch.len).sum());
    }
}
