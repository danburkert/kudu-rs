use std::collections::HashSet;
use std::net::{SocketAddr, ToSocketAddrs};

use futures_cpupool::{CpuFuture, CpuPool};
use kudu_pb::common::HostPortPB;

use util;

pub type ResolveFuture = CpuFuture<Vec<SocketAddr>, ()>;

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
    pub fn resolve(&self, hostports: Vec<HostPortPB>) -> ResolveFuture {
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

pub fn resolve_hosts(hostports: &[HostPortPB]) -> HashSet<SocketAddr> {
    let mut addrs = HashSet::new();
    for hostport in hostports {
        match (hostport.get_host(), hostport.get_port() as u16).to_socket_addrs() {
            Ok(resolved_addrs) => addrs.extend(resolved_addrs),
            Err(error) => warn!("unable to resolve host '{}': {}",
                                hostport.get_host(), error),
        }
    }
    addrs
}

pub fn resolve_hostports(hostports: &[(String, u16)]) -> Vec<SocketAddr> {
    let mut addrs = Vec::new();
    for &(ref host, port) in hostports {
        match (host.as_str(), port).to_socket_addrs() {
            Ok(resolved_addrs) => addrs.extend(resolved_addrs),
            Err(error) => warn!("unable to resolve hostname '{:?}': {}", host, error),
        }
    }
    addrs.sort_by(util::cmp_socket_addrs);
    addrs.dedup();
    addrs
}
