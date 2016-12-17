use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;
use futures::sync::mpsc;
use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use parking_lot::Mutex;
use tokio::reactor::Remote;

use Error;
use rpc::Rpc;
use rpc::connection::{
    ConnectionOptions,
    RpcReceiver,
    connect,
    forward,
};
use util;

#[derive(Clone)]
pub struct Messenger {
    inner: Arc<Inner>,
}

struct Inner {
    options: ConnectionOptions,
    remotes: Box<[Remote]>,
    connections: Mutex<HashMap<SocketAddr, mpsc::Sender<Rpc>>>,
}

impl Messenger {

    pub fn new(remotes: &[Remote], options: ConnectionOptions) -> Messenger {
        Messenger {
            inner: Arc::new(Inner {
                options: options,
                remotes: remotes.to_owned().into_boxed_slice(),
                connections: Mutex::new(HashMap::new()),
            }),
        }
    }
}

impl Sink for Messenger {
    type SinkItem = Rpc;
    type SinkError = !;

    fn start_send(&mut self, mut rpc: Rpc) -> StartSend<Rpc, !> {
        rpc.response.clear();
        debug_assert!(rpc.oneshot.is_some());
        trace!("{:?}: start_send, rpc: {:?}", self, rpc);

        let addr = rpc.addr;
        let Inner { ref options, ref remotes, ref connections } = *self.inner;
        let mut connections = connections.lock();
        let start_send = connections.entry(addr).or_insert_with(move || {
            let idx = if remotes.len() == 1 {
                0
            } else {
                let mut hasher = FnvHasher::default();
                addr.hash(&mut hasher);
                hasher.finish() % remotes.len() as u64
            } as usize;

            let options = options.clone();
            let (send, recv) = mpsc::channel(options.max_rpcs_in_flight as usize);

            remotes[idx].spawn(move |handle| {
                let receiver = RpcReceiver { receiver: recv };
                connect(&addr, handle, options)
                    .map_err(move |error| warn!("unable to connect to {}: {}", addr, error))
                    .and_then(move |connection| forward(receiver, connection))
            });
            send
        }).start_send(rpc);
        match start_send {
            Ok(async_sink) => Ok(async_sink),
            Err(..) => {
                connections.remove(&addr);
                Ok(AsyncSink::Ready)
            },
        }
    }

    fn poll_complete(&mut self) -> Poll<(), !> {
        Ok(Async::Ready(()))
    }
}


impl fmt::Debug for Messenger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Messenger {{ reactors: {}, connections: {} }}",
               self.inner.remotes.len(), self.inner.connections.lock().len())
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};
    use std::iter;

    use env_logger;
    use futures::{self, Future, Sink};
    use kudu_pb;
    use tokio::reactor::Core;

    use mini_cluster::{self, MiniCluster, MiniClusterConfig};
    use rpc::RpcFuture;
    use rpc::connection::ConnectionOptions;
    use rpc::master;
    use super::*;
    use util;
    use Error;

    #[test]
    fn send_single() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));

        let mut core = Core::new().unwrap();
        let addr = cluster.master_addrs()[0];

        let mut messenger = Messenger::new(&[core.remote()], ConnectionOptions::default());
        let mut rpc = master::ping(addr,
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        let oneshot = rpc.future();

        let f = futures::lazy(move || {
            assert!(messenger.start_send(rpc).unwrap().is_ready());
            oneshot
        });

        let result = core.run(f);
        result.unwrap();
    }

    #[test]
    fn send_concurrent() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));
        let mut core = Core::new().unwrap();
        let addr = cluster.master_addrs()[0];

        let mut options = ConnectionOptions::default();
        options.max_rpcs_in_flight = 10;
        let messenger = Messenger::new(&[core.remote()], options);

        let mut rpcs: Vec<Rpc> = iter::repeat(()).take(100).map(|_| {
            master::ping(addr,
                         Instant::now() + Duration::from_secs(5),
                         kudu_pb::master::PingRequestPB::new())
        }).collect();
        let oneshots: Vec<RpcFuture> = rpcs.iter_mut().map(|rpc| rpc.future()).collect();

        let send = futures::lazy(move || messenger.send_all(futures::stream::iter::<_, Rpc, !>(rpcs.into_iter().map(|rpc| Ok(rpc)))));
        let recv = futures::future::join_all(oneshots)
                                   .map_err(|error| panic!("error: {:?}", error));

        let (_, results) = core.run(send.join(recv)).unwrap();

        assert_eq!(100, results.len());
    }

    /*
    #[test]
    fn timeout() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .rpc_negotiation_delay(1000));
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let mut rpc = master::ping(cluster.master_addrs()[0], now + Duration::from_millis(100),
                                   kudu_pb::master::PingRequestPB::new());

        let (send, recv) = sync_channel::<(Result<()>, Rpc)>(0);
        rpc.callback = Some(retry_channel_callback(messenger.clone(), send));
        messenger.send(rpc);

        let (result, _) = recv.recv().unwrap();

        match result {
            Ok(()) => panic!("expected failure"),
            Err(Error::TimedOut) => (),
            Err(other) => panic!("unexpected error: {}", other),
        }

        let elapsed = Instant::now().duration_since(now);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(90), "expected: 100ms, elapsed: {:?}", elapsed);
        assert!(elapsed < Duration::from_millis(150), "expected: 100ms, elapsed: {:?}", elapsed);
    }

    #[test]
    fn cancel() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .rpc_negotiation_delay(1000));
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let mut rpc = master::ping(cluster.master_addrs()[0], now + Duration::from_millis(500),
                                   kudu_pb::master::PingRequestPB::new());

        let (send, recv) = sync_channel::<(Result<()>, Rpc)>(0);
        let cancel = Arc::new(AtomicBool::new(false));
        rpc.cancel = Some(cancel.clone());
        rpc.callback = Some(channel_callback(send));
        messenger.send(rpc);

        cancel.store(true, Ordering::Relaxed);
        let (result, _) = recv.recv().unwrap();

        match result {
            Ok(()) => panic!("expected failure"),
            Err(Error::Cancelled) => (),
            Err(other) => panic!("unexpected error: {}", other),
        }

        let elapsed = Instant::now().duration_since(now);
        assert!(elapsed < Duration::from_millis(25), "expected: 0ms, elapsed: {:?}", elapsed);
    }

    #[test]
    fn timer() {
        let _ = env_logger::init();
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let (send, recv) = sync_channel::<()>(0);

        messenger.timer(Duration::from_millis(100), Box::new(move || send.send(()).unwrap()));

        recv.recv().unwrap();

        let elapsed = Instant::now().duration_since(now);
        info!("elapsed: {:?}", elapsed);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(75), "expected: 100ms, elapsed: {:?}", elapsed);
        assert!(elapsed < Duration::from_millis(125), "expected: 100ms, elapsed: {:?}", elapsed);
    }
    */

    /// Tests that a connection will fail an RPC after a failure to connect.
    #[test]
    fn test_connection_error() {
        let _ = env_logger::init();
        let mut core = Core::new().unwrap();
        let mut messenger = Messenger::new(&[core.remote()], ConnectionOptions::default());
        let mut rpc = master::ping(mini_cluster::get_unbound_address(),
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        let oneshot = rpc.future();

        let f = futures::lazy(move || {
            assert!(messenger.start_send(rpc).unwrap().is_ready());
            oneshot
        });

        let result = core.run(f);
        assert!(result.is_err());
    }

    /// Tests that a connection will fail an RPC after connecting to the server succesfully, and
    /// the server becomes unreachable.
    #[test]
    fn connection_hangup() {
        let _ = env_logger::init();
        let mut test_reactor = util::TestReactor::default();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_tservers(0)
                                                             .log_rpc_negotiation_trace(true)
                                                             .log_rpc_trace(true));

        let messenger = test_reactor.io.messenger();
        let mut rpc = master::ping(cluster.master_addrs()[0],
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        let oneshot = rpc.future();
        messenger.send(rpc).wait();
        oneshot.wait().unwrap();

        let master = cluster.master_addrs()[0];
        info!("stopping master {}", master);
        cluster.stop_node(master);

        let mut rpc = master::ping(cluster.master_addrs()[0],
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        let oneshot = rpc.future();
        messenger.send(rpc).wait();
        let error = oneshot.wait().unwrap_err();

        assert_eq!(error.error, Error::ConnectionError);
    }
}
