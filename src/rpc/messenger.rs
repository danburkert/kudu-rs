use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::sync_channel;

use rpc::{Rpc, RpcResult};
use rpc::connection::{Connection, ConnectionOptions, TimeoutKind};

use mio::{
    EventLoop,
    EventSet,
    Handler,
    Sender,
    Token,
};
use slab::Slab;

pub type Loop = EventLoop<MessengerHandler>;

#[derive(Debug)]
pub enum Command {
    Shutdown,
    Send(Rpc)
}

pub struct Messenger {
    channel: Sender<Command>,
    thread: JoinHandle<io::Result<()>>,
}

impl Messenger {
    pub fn new() -> io::Result<Messenger> {
        let mut event_loop = try!(EventLoop::new());
        let channel = event_loop.channel();
        let thread = thread::spawn(move || {
            let mut connection_manager = MessengerHandler::new();
            event_loop.run(&mut connection_manager)
        });
        Ok(Messenger {
            channel: channel,
            thread: thread,
        })
    }

    /// Sends a generic Kudu RPC, and executes the callback when the RPC is complete.
    pub fn send(&self, rpc: Rpc) {
        // TODO: is there a better way to handle queue failure?
        self.channel.send(Command::Send(rpc)).unwrap();
    }

    pub fn send_sync(&self, mut rpc: Rpc) -> (RpcResult, Rpc) {
        let (send, recv) = sync_channel(0);
        assert!(rpc.callback.is_none());
        rpc.callback = Some(Box::new(move |result, rpc| send.send((result, rpc)).unwrap()));
        self.send(rpc);
        recv.recv().unwrap()
    }
}

#[derive(Debug)]
pub struct MessengerHandler {
    slab: Slab<Connection, Token>,
    index: HashMap<SocketAddr, Token>,
    cxn_options: Rc<ConnectionOptions>,
}

impl MessengerHandler {
    fn new() -> MessengerHandler {
        MessengerHandler {
            slab: Slab::new(128),
            index: HashMap::new(),
            cxn_options: Rc::new(ConnectionOptions::default()),
        }
    }
}

impl Handler for MessengerHandler {

    type Timeout = (Token, TimeoutKind);
    type Message = Command;

    fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        self.slab[token].ready(event_loop, token, events)
    }

    fn notify(&mut self, event_loop: &mut Loop, command: Command) {
        match command {
            Command::Shutdown => event_loop.shutdown(),
            Command::Send(rpc) => {
                let token = self.index.get(&rpc.addr).map(|t| *t).unwrap_or_else(|| {
                    // No open connection for the socket address; create a new one.
                    if !self.slab.has_remaining() {
                        let count = self.slab.count();
                        self.slab.grow(count);
                    }
                    let cxn_options = self.cxn_options.clone();
                    self.slab
                        .insert_with(|token| Connection::new(event_loop, token,
                                                             rpc.addr.clone(), cxn_options))
                        .unwrap()
                });
                self.slab[token].send_rpc(event_loop, token, rpc);
            },
        }
    }

    fn timeout(&mut self, event_loop: &mut Loop, timeout: (Token, TimeoutKind)) {
        let mut drop_cxn = false;
        if let Some(cxn) = self.slab.get_mut(timeout.0) {
            drop_cxn = cxn.timeout(event_loop, timeout.0, timeout.1);
        };
        if drop_cxn {
            let cxn = self.slab.remove(timeout.0);
            debug!("{:?}: closing", cxn.unwrap())
        }
    }
}
