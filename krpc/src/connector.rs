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
use futures::stream::FuturesUnordered;
use tacho;
use tokio::reactor::Handle;

use Error;
use HostPort;
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

    pub fn connect(hostports: &[HostPort],
                   thread_pool: &CpuPool,
                   handle: Handle,
                   options: Options)
                   -> Connector {
        let mut resolving = FuturesUnordered::new();
        let mut connecting = FuturesUnordered::new();
        let negotiating = FuturesUnordered::new();
        let errors = Vec::new();

        let metrics = options.scope.as_ref().cloned().map(Metrics::new);

        for hostport in hostports {
            // Attempt to short-circuit DNS by parsing the host as an IP addr.
            if let Ok(addr) = IpAddr::from_str(&hostport.host) {
                connecting.push(Transport::connect(SocketAddr::new(addr, hostport.port),
                                                   options.clone(),
                                                   &handle));

            // Otherwise resolve the hostport.
            } else {
                let hostport = hostport.clone();
                resolving.push(thread_pool.spawn_fn(move || hostport.to_socket_addrs()));
            }
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
