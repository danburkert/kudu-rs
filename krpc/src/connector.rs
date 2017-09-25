use std::io;
use std::net::{
    IpAddr,
    SocketAddr,
    ToSocketAddrs,
};
use std::str::FromStr;
use std::vec;

use cpupool::{CpuFuture, CpuPool};
use itertools::Itertools;
use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered; use Error;
use tacho;
use tokio::reactor::Handle;

use Options;
use connection::Connection;
use negotiator::Negotiator;
use transport::Transport;
use transport::TransportNew;

type SocketAddrs = vec::IntoIter<SocketAddr>;

pub(crate) struct Connector {
    handle: Handle,
    options: Options,
    resolving: FuturesUnordered<CpuFuture<SocketAddrs, io::Error>>,
    connecting: FuturesUnordered<TransportNew>,
    negotiating: FuturesUnordered<Negotiator>,
    errors: Vec<Error>,
    metrics: Option<Metrics>,
}

impl Connector {

    pub fn connect(hostports: &[String],
                   thread_pool: &CpuPool,
                   handle: Handle,
                   options: Options)
                   -> Connector {
        let mut resolving = FuturesUnordered::new();
        let mut connecting = FuturesUnordered::new();
        let negotiating = FuturesUnordered::new();
        let mut errors = Vec::new();

        let mut metrics = options.scope.as_ref().cloned().map(Metrics::new);

        for hostport in hostports {

            // Attempt to short-circuit DNS by parsing the hostport as a socket addr or IP addr.
            if let Ok(addr) = SocketAddr::from_str(&hostport) {
                connecting.push(Transport::connect(addr, options.clone(), &handle));
                continue;
            }
            if let Ok(addr) = IpAddr::from_str(&hostport) {
                if let Some(port) = options.default_port {
                    connecting.push(Transport::connect(SocketAddr::new(addr, port),
                                                       options.clone(),
                                                       &handle));
                } else {
                    let error = io::Error::new(io::ErrorKind::InvalidInput,
                                               format!("invalid hostport (no port): {:?}", hostport));
                    warn!("{}", error);
                    if let Some(ref mut metrics) = metrics {
                        metrics.resolve_errors.incr(1);
                    }
                    errors.push(error.into());
                }
                continue;
            }

            // Otherwise it is a hostname which needs to be split into (host, port) pair and
            // resolved.
            let mut parts_iter = hostport.rsplitn(2, ':');
            let mut port = parts_iter.next();
            let host = parts_iter.next().unwrap_or_else(|| port.take().unwrap()).to_owned();

            // Parse the port from the hostport if it's present, or use the default port it it's
            // present, or fail.
            let port = match port.map(u16::from_str).or_else(|| options.default_port.map(Result::Ok)) {
                Some(Ok(port)) => port,
                Some(Err(_)) => {
                    let error = io::Error::new(io::ErrorKind::InvalidInput,
                                               format!("invalid hostport: {:?}", hostport));
                    warn!("{}", error);
                    if let Some(ref mut metrics) = metrics {
                        metrics.resolve_errors.incr(1);
                    }
                    errors.push(error.into());
                    continue;
                },
                None => {
                    let error = io::Error::new(io::ErrorKind::InvalidInput,
                                               format!("invalid hostport (no port): {:?}", hostport));
                    warn!("{}", error);
                    if let Some(ref mut metrics) = metrics {
                        metrics.resolve_errors.incr(1);
                    }
                    errors.push(error.into());
                    continue;
                }
            };

            resolving.push(thread_pool.spawn_fn(move || (&host[..], port).to_socket_addrs()));
        }

        Connector {
            handle,
            options,
            resolving,
            connecting,
            negotiating,
            errors,
            metrics,
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
                        let transport = Transport::connect(addr, self.options.clone(), &self.handle);
                        self.connecting.push(transport);
                    }
                },
                // No futures are ready.
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => break,
                Err(error) => {
                    warn!("{}", error);
                    if let Some(ref mut metrics) = self.metrics {
                        metrics.resolve_errors.incr(1);
                    }
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
                    if let Some(ref mut metrics) = self.metrics {
                        metrics.connect_errors.incr(1);
                    }
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
                    if let Some(ref mut metrics) = self.metrics {
                        metrics.negotiate_errors.incr(1);
                    }
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

struct Metrics {
    /// Number of failures while attempting DNS resolution.
    resolve_errors: tacho::Counter,

    /// Number of failures while attempting to connect.
    connect_errors: tacho::Counter,

    /// Number of failures while negotiating.
    negotiate_errors: tacho::Counter,
}

impl Metrics {
    fn new(scope: tacho::Scope) -> Metrics {
        let scope = scope.prefixed("krpc");
        let resolve_errors = scope.clone().labeled("kind", "resolve").counter("errors");
        let connect_errors = scope.clone().labeled("kind", "connect").counter("errors");
        let negotiate_errors = scope.labeled("kind", "negotiate").counter("errors");

        Metrics {
            resolve_errors,
            connect_errors,
            negotiate_errors,
        }
    }
}
