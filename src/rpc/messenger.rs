use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::{Duration, Instant};
use std::fmt;

use rpc::Rpc;
use rpc::connection::{Connection, ConnectionOptions};
use Result;
use Error;

use mio::{
    Ready,
    Token,
};
use mio::deprecated::{
    EventLoop,
    EventLoopBuilder,
    Handler,
    Sender,
};
use slab::Slab;

pub type Loop = EventLoop<MessengerHandler>;

pub enum Command {
    Shutdown,
    Send(Rpc),
    Timer((Duration, Box<TimerCallback>)),
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Shutdown => write!(f, "Command::Shutdown"),
            Command::Send(ref rpc) => write!(f, "Command::Send({:?})", rpc),
            Command::Timer((ref duration, _)) => write!(f, "Command::Timer({:?})", duration),
        }
    }
}

/// Essentially FnBox...
pub trait TimerCallback: Send {
    fn callback(self: Box<Self>);
}

impl <F> TimerCallback for F where F: FnOnce() + Send {
    fn callback(self: Box<F>) {
        (*self)()
    }
}

pub enum TimeoutKind {

    /// Active while a connection is reset. After expiration, the connection creates a new socket
    /// and attempts negotiation.
    ConnectionReset(Token),

    /// An RPC timeout.
    ///
    /// This timeout tracks an RPC timeout deadline. When it expires, the RPC should be timed out.
    Rpc(Token, usize),

    /// A general timer timeout.
    Timer(Box<TimerCallback>),
}

#[derive(Clone)]
pub struct Messenger {
    channel: Sender<Command>,
}

impl Messenger {
    pub fn new() -> io::Result<Messenger> {
        let mut event_loop_builder = EventLoopBuilder::new();
        // Timer granularity of 10ms.
        event_loop_builder.timer_tick(Duration::from_millis(10));
        let mut event_loop = try!(event_loop_builder.build());
        let channel = event_loop.channel();
        thread::spawn(move || {
            let mut connection_manager = MessengerHandler::new();
            event_loop.run(&mut connection_manager)
        });
        Ok(Messenger { channel: channel })
    }


    /// Sends a generic Kudu RPC, and executes the callback when the RPC is complete.
    pub fn send(&self, mut rpc: Rpc) {
        // TODO: is there a better way to handle queue failure?
        debug_assert!(rpc.callback.is_some());
        rpc.response.clear();
        self.channel.send(Command::Send(rpc)).unwrap();
    }

    pub fn delayed_send(&self, delay: Duration, rpc: Rpc) {
        let deadline = rpc.deadline.clone();
        if Instant::now() + delay > deadline {
            rpc.fail(Error::TimedOut);
            return;
        }

        let messenger = self.clone();
        let rpc: Rpc = rpc;
        self.timer(delay, Box::new(move || { messenger.send(rpc) }));
    }

    pub fn send_sync(&self, mut rpc: Rpc) -> (Result<()>, Rpc) {
        let (send, recv) = sync_channel(0);
        assert!(rpc.callback.is_none());
        rpc.callback = Some(Box::new(move |result, rpc| send.send((result, rpc)).unwrap()));
        self.send(rpc);
        recv.recv().unwrap()
    }

    pub fn timer(&self, duration: Duration, callback: Box<TimerCallback>) {
        self.channel.send(Command::Timer((duration, callback))).unwrap();
    }
}

pub struct MessengerHandler {
    connection_slab: Slab<Connection, Token>,
    index: HashMap<SocketAddr, Token>,
    cxn_options: Rc<ConnectionOptions>,
}

impl MessengerHandler {
    fn new() -> MessengerHandler {
        MessengerHandler {
            connection_slab: Slab::with_capacity(128),
            index: HashMap::new(),
            cxn_options: Rc::new(ConnectionOptions::default()),
        }
    }
}

impl fmt::Debug for MessengerHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessengerHandler {{ connections: {} }}", self.connection_slab.len())
    }
}

impl Handler for MessengerHandler {

    type Timeout = TimeoutKind;
    type Message = Command;

    fn ready(&mut self, event_loop: &mut Loop, token: Token, events: Ready) {
        self.connection_slab[token].ready(event_loop, token, events)
    }

    fn notify(&mut self, event_loop: &mut Loop, command: Command) {
        match command {
            Command::Shutdown => event_loop.shutdown(),
            Command::Send(rpc) => {
                let token = self.index.get(&rpc.addr).cloned().unwrap_or_else(|| {
                    // No open connection for the socket address; create a new one.
                    if !self.connection_slab.has_available() {
                        let len = self.connection_slab.len();
                        self.connection_slab.reserve_exact(len / 2);
                    }
                    let cxn_options = self.cxn_options.clone();
                    let token = {
                        let entry = self.connection_slab.vacant_entry().unwrap();
                        let token = entry.index();
                        let connection = Connection::new(event_loop, token, rpc.addr, cxn_options);
                        entry.insert(connection);
                        token
                    };
                    self.index.insert(rpc.addr, token);
                    token
                });
                self.connection_slab[token].send_rpc(event_loop, token, rpc);
            },
            Command::Timer((duration, callback)) => {
                event_loop.timeout(TimeoutKind::Timer(callback), duration).unwrap();
            }
        }
    }

    fn timeout(&mut self, event_loop: &mut Loop, timeout: TimeoutKind) {

        match timeout {
            TimeoutKind::ConnectionReset(token) => {
                let drop_cxn = self.connection_slab
                                   .get_mut(token)
                                   .unwrap()
                                   .reset_timeout(event_loop, token);
                if drop_cxn {
                    let cxn = self.connection_slab.remove(token).unwrap();
                    let removed_token = self.index.remove(cxn.addr()).unwrap();
                    assert!(removed_token == token);
                    debug!("{:?}: closing", cxn);
                }
            },
            TimeoutKind::Rpc(token, call_id) => {
                // Connection RPC timeout. A note on the unwrap calls below: the connection
                // carefully cancels all outstanding timers before asking to be torn down in
                // Connection#timeout, so if a connection timer fires, the connection must still
                // exist.
                self.connection_slab
                    .get_mut(token)
                    .unwrap()
                    .rpc_timeout(call_id);
            },
            TimeoutKind::Timer(callback) => callback.callback(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::{sync_channel, SyncSender};
    use std::time::{Duration, Instant};

    use env_logger;
    use kudu_pb;

    use mini_cluster::{self, MiniCluster, MiniClusterConfig};
    use rpc::{channel_callback, retry_channel_callback, master, Callback, Rpc};
    use super::*;
    use Error;
    use Result;

    #[test]
    fn send() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));

        let messenger = Messenger::new().unwrap();
        let (send, recv) = sync_channel::<(Result<()>, Rpc)>(0);
        let mut rpc = master::ping(cluster.master_addrs()[0],
                                   Instant::now() + Duration::from_secs(5),
                                   kudu_pb::master::PingRequestPB::new());
        rpc.callback = Some(retry_channel_callback(messenger.clone(), send));

        messenger.send(rpc);
        let (result, _) = recv.recv().unwrap();
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn send_concurrent() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));
        let messenger = Messenger::new().unwrap();

        let (send, recv) = sync_channel::<Result<()>>(100);

        struct PingCB(u8, Messenger, SyncSender<Result<()>>);
        impl Callback for PingCB {
            fn callback(mut self: Box<Self>, result: Result<()>, mut rpc: Rpc) {
                if result.as_ref().err().map_or(false, |error| error.is_network_error()) {
                    trace!("retrying RPC {:?} after network error: {}", rpc, result.unwrap_err());
                    let messenger = self.1.clone();
                    rpc.response.clear();
                    rpc.callback = Some(self);
                    return messenger.send(rpc);
                }

                let is_ok = result.is_ok();
                self.2.send(result).unwrap();
                self.0 -= 1;
                if is_ok && self.0 > 0 {
                    let messenger = self.1.clone();
                    rpc.response.clear();
                    rpc.callback = Some(self);
                    messenger.send(rpc);
                }
            }
        }

        // Send 100 pings, with 10 outstanding at a time.
        for _ in 0..10 {
            let mut rpc = master::ping(cluster.master_addrs()[0],
                                       Instant::now() + Duration::from_secs(5),
                                       kudu_pb::master::PingRequestPB::new());


            rpc.callback = Some(Box::new(PingCB(10, messenger.clone(), send.clone())));
            messenger.send(rpc);
        }
        drop(send);

        let mut count = 0;
        for result in recv {
            result.unwrap();
            count += 1;
        }
        assert_eq!(100, count);
    }

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
}
