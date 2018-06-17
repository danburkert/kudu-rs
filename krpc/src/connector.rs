use std::io;
use std::net::{
    IpAddr,
    SocketAddr,
    ToSocketAddrs,
};
use std::str::FromStr;
use std::vec;

use futures::{
    Async,
    Future,
    Poll,
    Stream,
    future,
    self,
};
use futures::stream::FuturesUnordered;
use itertools::Itertools;
use threadpool;

use Error;
use HostPort;
use Options;
use connection::Connection;
use negotiator::Negotiator;
use transport::Transport;
use transport::TransportNew;

type SocketAddrs = vec::IntoIter<SocketAddr>;

pub(crate) struct Connector {
    options: Options,
    resolving: FuturesUnordered<Box<Future<Item=SocketAddrs, Error=Error> + Send + 'static>>,
    connecting: FuturesUnordered<TransportNew>,
    negotiating: FuturesUnordered<Negotiator>,
    errors: Vec<Error>,
}

impl Connector {

    pub fn connect(hostports: &[HostPort], options: Options) -> Connector {
        let mut resolving = FuturesUnordered::<Box<Future<Item=_, Error=_> + Send + 'static>>::new();
        let mut connecting = FuturesUnordered::new();
        let negotiating = FuturesUnordered::new();
        let errors = Vec::new();

        for hostport in hostports {
            // Attempt to short-circuit DNS by parsing the host as an IP addr.
            if let Ok(addr) = IpAddr::from_str(&hostport.host()) {
                connecting.push(Transport::connect(SocketAddr::new(addr, hostport.port()),
                                                   options.clone()));

            // Otherwise resolve the hostport.
            } else {
                let hostport: HostPort = hostport.clone();
                // TODO(tokio-rs/tokio#432): use tokio_threadpool::blocking.
                let (send, recv) = futures::sync::oneshot::channel();
                ::std::thread::spawn(move || {
                    send.send(hostport.to_socket_addrs().map_err(Error::from)).unwrap()
                });
                resolving.push(Box::new(recv.map_err(|_| -> Error { unreachable!() }).flatten()))
            }
        }

        Connector {
            options,
            resolving,
            connecting,
            negotiating,
            errors,
        }
    }
}

impl Future for Connector {
    type Item = Connection;
    type Error = Error;

    fn poll(&mut self) -> Poll<Connection, Error> {

        // Poll the resolving futures.
        loop {
            match self.resolving.poll() {
                Ok(Async::Ready(Some(addrs))) => {
                    for addr in addrs {
                        let transport = Transport::connect(addr, self.options.clone());
                        self.connecting.push(transport);
                    }
                },
                // No futures are ready.
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => break,
                Err(error) => {
                    warn!("{}", error);
                    self.errors.push(error.into())
                },
            }
        }

        // Poll the connecting futures.
        loop {
            match self.connecting.poll() {
                Ok(Async::Ready(Some(transport))) => {
                    self.negotiating.push(Negotiator::negotiate(transport));
                },
                // No futures are ready.
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => break,
                Err(error) => {
                    warn!("{}", error);
                    self.errors.push(error.into())
                },
            }
        }

        // Poll the negotiating futures.
        loop {
            match self.negotiating.poll() {
                Ok(Async::Ready(Some(transport))) => {
                    return Ok(Async::Ready(Connection::new(transport, &self.options)));
                },
                // No futures are ready.
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => break,
                Err(error) => {
                    warn!("{}", error);
                    self.errors.push(error);
                },
            }
        }

        // If all futures resulted in an error, return the error.
        if self.resolving.is_empty() && self.connecting.is_empty() && self.negotiating.is_empty() {
            if self.errors.len() == 1 {
                return Err(self.errors.pop().unwrap().into());
            } else {
                return Err(io::Error::new(io::ErrorKind::ConnectionRefused,
                                          format!("failed to connect, errors: [{}]",
                                                  self.errors.iter().format(", "))).into());
            }
        }

        Ok(Async::NotReady)
    }
}
