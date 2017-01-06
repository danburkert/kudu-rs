use std::net::{SocketAddr, ToSocketAddrs};
use std::vec;

use futures::Future;
use futures_cpupool::CpuPool;
use kudu_pb::common::HostPortPB;

use HostPort;
use Error;

#[derive(Clone)]
pub struct Resolver {
    pool: CpuPool,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            pool: CpuPool::new(4),
        }
    }

    /// Resolves a sequence of hostnames into a collection of socket addresses. If the hostname DNS
    /// lookup fails, it is filtered from the result.
    pub fn resolve(&self, hostport: HostPort) -> impl Future<Item=vec::IntoIter<SocketAddr>, Error=Error> {
        self.pool.spawn_fn(move || hostport.to_socket_addrs().map_err(Error::from))
    }


    /// Resolves a sequence of hostnames into a collection of socket addresses. If the hostname DNS
    /// lookup fails, it is filtered from the result.
    pub fn resolve_all(&self, hostports: Vec<HostPortPB>) -> impl Future<Item=Vec<SocketAddr>, Error=Error> {
        self.pool.spawn_fn(move || {
            let mut addrs = Vec::with_capacity(hostports.len());
            for hostport in hostports {
                match (hostport.get_host(), hostport.get_port() as u16).to_socket_addrs() {
                    Ok(resolved_addrs) => addrs.extend(resolved_addrs),
                    Err(error) => warn!("unable to resolve host '{}': {}",
                                        hostport.get_host(), error),
                }
            }
            Ok(addrs)
        })
    }
}
