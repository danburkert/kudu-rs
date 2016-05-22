use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::io::{self, ErrorKind, Write};
use std::thread::{self, JoinHandle};
use std::error;
use std::fmt;
use std::time::Instant;

use kudu_pb::rpc_header;
use rpc::{Request, Response, RpcError};
use rpc::connection::Connection;

use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use eventual::{Future, Complete};
use mio::{
    EventLoop,
    EventSet,
    Handler,
    PollOpt,
    Sender,
    Token,
};
use mio::tcp::TcpStream;
use protobuf::{parse_length_delimited_from, CodedInputStream, Message, ProtobufError};
use protobuf::rt::ProtobufVarint;
use slab::Slab;
use netbuf::Buf;

pub type Loop = EventLoop<MessengerHandler>;

#[derive(Debug)]
enum Command {
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
                timeout: Instant,
                required_feature_flags: Vec<u32>,
                request: Box<Message>,
                response: Box<Message>) -> Future<Response, RpcError> {
        let (complete, future) = Future::pair();
        let request = Command::Request {
            addr: addr,
            request: Request {
                service_name: service_name,
                method_name: method_name,
                timeout: timeout,
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
struct MessengerHandler {
    slab: Slab<Connection, Token>,
    index: HashMap<SocketAddr, Token>
}

impl MessengerHandler {
    fn new() -> MessengerHandler {
        MessengerHandler {
            slab: Slab::new(128),
            index: HashMap::new(),
        }
    }
}

impl Handler for MessengerHandler {

    type Timeout = ();
    type Message = Command;

    fn ready(&mut self, event_loop: &mut Loop, token: Token, events: EventSet) {
        if events.is_hup() {
            let connection = self.slab.remove(token);
            debug!("hup; connection: {:?}, events: {:?}", connection, events);
        } else if events.is_error() {
            let connection = self.slab.remove(token);
            warn!("error; connection: {:?}, events: {:?}", connection, events);
        } else {
            let connection = &mut self.slab[token];
            trace!("ready; connection: {:?}, events: {:?}", connection, events);
            connection.ready(events);
        }
    }

    fn notify(&mut self, event_loop: &mut Loop, command: Command) {
        match command {
            Command::Shutdown => {
                event_loop.shutdown();
            },
            Command::Request { addr, request } => {
                let token = self.index.get(&addr).map(|t| *t).unwrap_or_else(|| {
                    // No open connection for the socket address, create a new one.
                    if !self.slab.has_remaining() {
                        let count = self.slab.count();
                        self.slab.grow(count);
                    }
                    self.slab.insert_with(|token| Connection::new(event_loop, token, addr).unwrap())
                             .unwrap()
                });
                self.slab[token].send_request(request).unwrap()
            },
        }
    }
}
