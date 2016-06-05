use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::fmt;

use rpc::{Rpc, RpcResult};
use rpc::connection::{Connection, ConnectionOptions};
use util::duration_to_ms;

use mio::{
    EventLoop,
    EventLoopConfig,
    EventSet,
    Handler,
    Sender,
    Token,
};
use slab::Slab;

pub type Loop = EventLoop<MessengerHandler>;

pub enum Command {
    Shutdown,
    Send(Rpc),
    Timer((Duration, Box<FnMut() + Send>)),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutKind {
    /// A state timeout.
    ///
    /// This timeout has different actions depending on the current state of the connection:
    ///
    /// * *Initializing* the timeout is a negotiation timeout. If the connection is unable to
    ///                  complete negotiation and succesfully connect to the server before the
    ///                  timeout exprires, then the connection is reset and initialization is
    ///                  retried after a backoff period.
    ///
    /// * *Connected* the timeout is an idle timeout. This should only be active while the
    ///               connection has no queued RPCs. After expiration, the connection is closed
    ///               by the `MessengerHandler`.
    ///
    /// * *Reset* The timeout is the reset timer. Upon expiration the connection will attempt to
    ///           reinitialize the connection with the server if the connection has queued RPCs.
    ///           Otherwise, the connection is closed by the `MessengerHandler`.
    ConnectionState,

    /// An RPC timeout.
    ///
    /// This timeout tracks the next RPC timeout deadline. When it expires, any timed out RPCs are
    /// failed.
    ConnectionRpc,

    /// An independent timer timeout.
    Timer,
}

pub struct Inner {
    channel: Sender<Command>,
    thread: JoinHandle<io::Result<()>>,
}

#[derive(Clone)]
pub struct Messenger {
    inner: Arc<Inner>,
}

impl Messenger {
    pub fn new() -> io::Result<Messenger> {
        let mut event_loop_config = EventLoopConfig::new();
        // Timer granularity of 10ms.
        event_loop_config.timer_tick_ms(10);
        let mut event_loop = try!(EventLoop::configured(event_loop_config));
        let channel = event_loop.channel();
        let thread = thread::spawn(move || {
            let mut connection_manager = MessengerHandler::new();
            event_loop.run(&mut connection_manager)
        });
        Ok(Messenger {
            inner: Arc::new(Inner {
                channel: channel,
                thread: thread,
            }),
        })
    }

    /// Sends a generic Kudu RPC, and executes the callback when the RPC is complete.
    pub fn send(&self, rpc: Rpc) {
        debug_assert!(rpc.deadline.is_some() || rpc.cancel.is_some());
        // TODO: is there a better way to handle queue failure?
        self.inner.channel.send(Command::Send(rpc)).unwrap();
    }

    pub fn send_sync(&self, mut rpc: Rpc) -> (RpcResult, Rpc) {
        let (send, recv) = sync_channel(0);
        assert!(rpc.callback.is_none());
        rpc.callback = Some(Box::new(move |result, rpc| send.send((result, rpc)).unwrap()));
        self.send(rpc);
        recv.recv().unwrap()
    }

    pub fn timer(&self, duration: Duration, callback: Box<FnMut() + Send>) {
        self.inner.channel.send(Command::Timer((duration, callback))).unwrap();
    }
}

#[derive(Debug)]
pub struct MessengerHandler {
    connection_slab: Slab<Connection, Token>,
    timer_slab: Slab<Box<FnMut()>, Token>,
    index: HashMap<SocketAddr, Token>,
    cxn_options: Rc<ConnectionOptions>,
}

impl MessengerHandler {
    fn new() -> MessengerHandler {
        MessengerHandler {
            connection_slab: Slab::new(128),
            timer_slab: Slab::new(128),
            index: HashMap::new(),
            cxn_options: Rc::new(ConnectionOptions::default()),
        }
    }
}

impl Handler for MessengerHandler {

    type Timeout = (TimeoutKind, Token);
    type Message = Command;

    fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        self.connection_slab[token].ready(event_loop, token, events)
    }

    fn notify(&mut self, event_loop: &mut Loop, command: Command) {
        match command {
            Command::Shutdown => event_loop.shutdown(),
            Command::Send(rpc) => {
                let token = self.index.get(&rpc.addr).map(|t| *t).unwrap_or_else(|| {
                    // No open connection for the socket address; create a new one.
                    if !self.connection_slab.has_remaining() {
                        let count = self.connection_slab.count();
                        self.connection_slab.grow(count / 2);
                    }
                    let cxn_options = self.cxn_options.clone();
                    let token = self.connection_slab
                                    .insert_with(|token| Connection::new(event_loop, token,
                                                                         rpc.addr.clone(),
                                                                         cxn_options))
                                    .unwrap();
                    self.index.insert(rpc.addr, token);
                    token
                });
                self.connection_slab[token].send_rpc(event_loop, token, rpc);
            },
            Command::Timer((duration, callback)) => {
                if !self.timer_slab.has_remaining() {
                    let count = self.timer_slab.count();
                    self.timer_slab.grow(count / 2);
                }
                let token = match self.timer_slab.insert(callback) {
                    Ok(token) => token,
                    Err(_) => unreachable!()
                };
                event_loop.timeout_ms((TimeoutKind::Timer, token), duration_to_ms(&duration)).unwrap();
            }
        }
    }

    fn timeout(&mut self, event_loop: &mut Loop, timeout_token: (TimeoutKind, Token)) {
        let (timeout, token) = timeout_token;

        match timeout {
            TimeoutKind::Timer => {
                self.timer_slab.remove(token).unwrap()();
            },
            _ => {
                let mut drop_cxn = false;
                if let Some(cxn) = self.connection_slab.get_mut(token) {
                    drop_cxn = cxn.timeout(event_loop, token, timeout);
                };
                if drop_cxn {
                    let cxn = self.connection_slab.remove(token).unwrap();
                    let removed_token = self.index.remove(cxn.addr()).unwrap();
                    assert!(removed_token == token);
                    debug!("{:?}: closing", cxn);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::sync_channel;
    use std::time::{Duration, Instant};

    use mini_cluster::{get_unbound_address};
    use rpc::{master, Rpc, RpcError, RpcResult};
    use super::*;

    use env_logger;
    use kudu_pb;

    #[test]
    fn test_timeout() {
        let _ = env_logger::init();
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let mut rpc = master::ping(get_unbound_address(), kudu_pb::master::PingRequestPB::new());
        rpc.set_deadline(now + Duration::from_millis(300));

        let (result, _) = messenger.send_sync(rpc);

        match result {
            Ok(()) => panic!("expected failure"),
            Err(RpcError::TimedOut) => (),
            Err(other) => panic!("unexpected error: {}", other),
        }

        let elapsed = Instant::now().duration_since(now);
        info!("elapsed: {:?}", elapsed);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(275));
        assert!(elapsed < Duration::from_millis(325));
    }

    #[test]
    fn test_cancel() {
        let _ = env_logger::init();
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let mut rpc = master::ping(get_unbound_address(), kudu_pb::master::PingRequestPB::new());

        let (send, recv) = sync_channel::<(RpcResult, Rpc)>(0);
        assert!(rpc.callback.is_none());
        let cancel = Arc::new(AtomicBool::new(false));
        rpc.cancel = Some(cancel.clone());
        rpc.callback = Some(Box::new(move |result, rpc| send.send((result, rpc)).unwrap()));
        messenger.send(rpc);

        cancel.store(true, Ordering::Relaxed);
        let (result, _) = recv.recv().unwrap();

        match result {
            Ok(()) => panic!("expected failure"),
            Err(RpcError::Cancelled) => (),
            Err(other) => panic!("unexpected error: {}", other),
        }

        let elapsed = Instant::now().duration_since(now);
        info!("elapsed: {:?}", elapsed);

        assert!(elapsed < Duration::from_millis(20));
    }

    #[test]
    fn test_timer() {
        let _ = env_logger::init();
        let messenger = Messenger::new().unwrap();

        let now = Instant::now();
        let (send, recv) = sync_channel::<()>(0);

        messenger.timer(Duration::from_millis(100), Box::new(move || send.send(()).unwrap()));

        recv.recv().unwrap();

        let elapsed = Instant::now().duration_since(now);
        info!("elapsed: {:?}", elapsed);

        // If this gets flaky, figure out how to get tighter times out of mio.
        assert!(elapsed > Duration::from_millis(75));
        assert!(elapsed < Duration::from_millis(125));
    }
}
