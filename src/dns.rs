use std::collections::HashSet;
use std::net::{lookup_host, IpAddr, SocketAddr};
use std::thread;
use std::str::FromStr;

use kudu_pb::common::HostPortPB;

/// Resolves a sequence of hostnames into a set of socket addresses. If the hostname DNS lookup
/// fails, it is filtered from the result. The results are passed to the provided callback upon
/// completion.
pub fn resolve_hosts(hostports: &[HostPortPB]) -> HashSet<SocketAddr> {
    let mut addrs = HashSet::new();
    for hostport in hostports {
        // If the host is just a IP address, we may be able to parse it directly.
        if let Ok(addr) = IpAddr::from_str(hostport.get_host()) {
            // TODO: check for overflow on cast
            addrs.insert(SocketAddr::new(addr, hostport.get_port() as u16));
            continue;
        }
        // Otherwise fall back to a DNS lookup.
        match lookup_host(hostport.get_host()) {
            Ok(lookup_results) => {
                for result in lookup_results {
                    match result {
                        Ok(mut addr) => {
                            addr.set_port(hostport.get_port() as u16);
                            addrs.insert(addr);
                        },
                        Err(error) => warn!("unable to resolve host '{}': {}",
                                            hostport.get_host(), error),
                    }
                }
            },
            Err(error) => warn!("unable to resolve host '{}': {}",
                                hostport.get_host(), error),
        }
    }
    addrs
}

/// Asynchronously resolves a sequence of hostnames into a set of socket addresses. If the hostname
/// DNS lookup fails, it is filtered from the result. The results are passed to the provided
/// callback upon completion.
pub fn resolve_hosts_async<F>(registrations: Vec<HostPortPB>, f: F)
where F: FnOnce(HashSet<SocketAddr>) + Send + 'static {
    // TODO: is this done frequently enough to warrant a threadpool?
    thread::spawn(move || f(resolve_hosts(&registrations)));
}
