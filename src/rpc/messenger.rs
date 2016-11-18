use std::collections::HashMap;
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::fmt;

use shared::Shared;
use Error;
use rpc::Rpc;
use rpc::connection::{Connection, ConnectionOptions};

use futures::{Async, Future, Poll, Sink, StartSend};
use tokio::reactor::{Handle, Timeout};

pub struct Messenger {
    handle: Handle,
    options: Rc<ConnectionOptions>,
    connections: HashMap<SocketAddr, Shared<Connection>>,
}

impl Messenger {
    fn new(handle: Handle, options: ConnectionOptions) -> Messenger {
        Messenger {
            handle: handle,
            options: Rc::new(options),
            connections: HashMap::new(),
        }
    }

    fn connection(&mut self, addr: SocketAddr) -> &mut Shared<Connection> {
        let Messenger { ref mut handle, ref options, ref mut connections } = *self;
        connections.entry(addr)
                   .or_insert_with(|| {
                       let connection = Shared::new(Connection::new(handle.clone(), addr, options.clone()));
                       handle.spawn(connection.clone());
                       connection
                   })
    }

    /*
    pub fn delayed_send(&mut self, delay: Duration, mut rpc: Rpc) {
        debug_assert!(rpc.oneshot.is_some());
        rpc.response.clear();

        let deadline = rpc.deadline.clone();
        if Instant::now() + delay > deadline {
            return rpc.fail(Error::TimedOut);
        }

        let connection = self.connection(rpc.addr).clone();
        let f = Timeout::new(delay, &self.handle)
                        .unwrap()
                        .map_err(|_| ())
                        .and_then(|_| connection.send(rpc).map(|_| ()));

        self.handle.spawn(f);
    }
    */
}

impl Sink for Messenger {
    type SinkItem = Rpc;
    type SinkError = ();

    fn start_send(&mut self, mut rpc: Rpc) -> StartSend<Rpc, ()> {
        unimplemented!()
        /*
        info!("{:?}: start_send, rpc: {:?}", self, rpc);
        debug_assert!(rpc.oneshot.is_some());
        rpc.response.clear();
        self.connection(rpc.addr).start_send(rpc)
        */
    }

    fn poll_complete(&mut self) -> Poll<(), ()> {
        unimplemented!()
        /*
        for connection in self.connections.values_mut() {
            try_ready!(connection.poll_complete());
        }
        Ok(Async::Ready(()))
        */
    }
}


impl fmt::Debug for Messenger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Messenger {{ connections: {} }}", self.connections.len())
    }
}

#[cfg(test)]
mod tests {

    use std::iter;
    use std::time::{Duration, Instant};

    use env_logger;
    use kudu_pb;

    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use rpc::connection::ConnectionOptions;
    use rpc::{master, RpcResult};
    use super::*;
    use tokio::reactor::Core;
    use futures::{self, Future, Sink};
    use futures::sync::oneshot;

    #[test]
    fn send() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));

        let mut core = Core::new().unwrap();
        let addr = cluster.master_addrs()[0];

        let mut messenger = Messenger::new(core.handle(), ConnectionOptions::default());
        let mut rpc = master::ping(addr,
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        let oneshot = rpc.oneshot();

        let f = futures::lazy(move || {
            assert!(messenger.start_send(rpc).unwrap().is_ready());
            oneshot
        });

        let result = core.run(f);
        result.unwrap().unwrap();
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
        options.rpc_queue_len = 1;
        let messenger = Messenger::new(core.handle(), options);

        let (oneshots, rpcs): (Vec<_>, Vec<_>) = iter::repeat(0u32).take(100).map(|_| {
            let mut rpc = master::ping(addr,
                                       Instant::now() + Duration::from_secs(5),
                                       kudu_pb::master::PingRequestPB::new());
            let oneshot: oneshot::Receiver<RpcResult> = rpc.oneshot();
            (oneshot, Ok(rpc))
        }).unzip();


        let send = futures::lazy(move || messenger.send_all(futures::stream::iter(rpcs)));
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

    /// Tests that a connection will fail an RPC after a failure to connect.
    #[test]
    fn test_connection_error() {
        let _ = env_logger::init();
        let messenger = Messenger::new().unwrap();

        let rpc = master::ping(mini_cluster::get_unbound_address(),
                               Instant::now() + Duration::from_millis(100),
                               kudu_pb::master::PingRequestPB::new());

        let (result, _) = messenger.send_sync(rpc);
        assert_eq!(Err(Error::ConnectionError), result);
    }

    /// Tests that a connection will fail an RPC after a failure to connect.
    #[test]
    fn connection_hangup() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_tservers(0)
                                                             .log_rpc_negotiation_trace(true)
                                                             .log_rpc_trace(true));
        let messenger = Messenger::new().unwrap();
        let mut rpc = master::ping(cluster.master_addrs()[0],
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());

        let (send, recv) = sync_channel::<(Result<()>, Rpc)>(0);
        rpc.callback = Some(retry_channel_callback(messenger.clone(), send));
        messenger.send(rpc);

        let (result, _) = recv.recv().unwrap();

        assert_eq!(Ok(()), result);

        let master = cluster.master_addrs()[0];
        cluster.stop_node(master);

        let rpc = master::ping(cluster.master_addrs()[0],
                               Instant::now() + Duration::from_secs(5),
                               kudu_pb::master::PingRequestPB::new());

        let (result, _) = messenger.send_sync(rpc);
        assert_eq!(Err(Error::ConnectionError), result);
    }
    */
}
