use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rpc::{Request, Response, RpcError};
use rpc::connection::{Connection, ConnectionOptions};

use eventual::Future;
use mio::{
    EventLoop,
    EventSet,
    Handler,
    Sender,
    Token,
};
use protobuf::Message;
use slab::Slab;

pub type Loop = EventLoop<MessengerHandler>;

#[derive(Debug)]
pub enum Command {
    Shutdown,
    Request {
        addr: SocketAddr,
        request: Request,
    },
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

    pub fn send(&self,
                addr: SocketAddr,
                service_name: &'static str,
                method_name: &'static str,
                timeout: Duration,
                required_feature_flags: Vec<u32>,
                request: Box<Message>,
                response: Box<Message>) -> Future<Response, RpcError> {
        let (complete, future) = Future::pair();
        let now = Instant::now();
        let request = Command::Request {
            addr: addr,
            request: Request {
                service_name: service_name,
                method_name: method_name,
                start: now,
                deadline: now + timeout,
                required_feature_flags: required_feature_flags,
                request_message: request,
                response_message: response,
                complete: complete,
            },
        };

        self.channel.send(request).unwrap();
        future
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

    type Timeout = Token;
    type Message = Command;

    fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        self.slab[token].ready(event_loop, token, events)
    }

    fn notify(&mut self, event_loop: &mut Loop, command: Command) {
        match command {
            Command::Shutdown => event_loop.shutdown(),
            Command::Request { addr, request } => {
                let token = self.index.get(&addr).map(|t| *t).unwrap_or_else(|| {
                    // No open connection for the socket address, create a new one.
                    if !self.slab.has_remaining() {
                        let count = self.slab.count();
                        self.slab.grow(count);
                    }
                    let cxn_options = self.cxn_options.clone();
                    self.slab
                        .insert_with(|token| Connection::new(event_loop, token, addr, cxn_options))
                        .unwrap()
                });
                self.slab[token].send_request(event_loop, token, request);
            },
        }
    }

    fn timeout(&mut self, event_loop: &mut Loop, token: Token) {
        if let Some(cxn) = self.slab.get_mut(token) {
            cxn.timeout(event_loop, token);
        }
    }
}
