use std::fmt;
use std::io;
use std::net::{
    IpAddr,
    SocketAddr,
    ToSocketAddrs,
};
use std::str::FromStr;
use std::vec;

/// A host and port pair.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HostPort {
    /// The host.
    host: Box<str>,
    /// The port.
    port: u16,
}

impl HostPort {
    /// Creates a new host, port pair.
    pub fn new(host: String, port: u16) -> HostPort {
        HostPort {
            host: host.into_boxed_str(),
            port,
        }
    }

    /// Returns the host.
    pub fn host(&self) -> &str {
        &*self.host
    }

    /// Returns the port.
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl From<SocketAddr> for HostPort {
    fn from(addr: SocketAddr) -> HostPort {
        HostPort::new(addr.ip().to_string(), addr.port())
    }
}

impl From<(String, u16)> for HostPort {
    fn from((host, port): (String, u16)) -> HostPort {
        HostPort::new(host, port)
    }
}

impl HostPort {

    pub fn parse(hostport: &str, default_port: u16) -> io::Result<HostPort> {
        let hostport = hostport.trim();

        // Attempt to parse as a SocketAddr.
        if let Ok(addr) = SocketAddr::from_str(hostport) {
            return Ok(addr.into());
        }

        // Attempt to parse as an IpAddr.
        if let Ok(addr) = IpAddr::from_str(&hostport) {
            return Ok(HostPort {
                host: format!("{}", addr).into_boxed_str(),
                port: default_port,
            });
        }

        // Otherwise it is a hostname which needs to be split into (host, port) pair and resolved.
        let mut parts = hostport.splitn(2, ':');
        let host = parts.next().unwrap().to_owned();
        let port = parts.next()
                        .map(u16::from_str)
                        .unwrap_or(Ok(default_port))
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput,
                                                    format!("invalid hostport: {:?}", hostport)))?;
        Ok(HostPort::new(host, port))
    }
}

impl ToSocketAddrs for HostPort {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        (self.host(), self.port()).to_socket_addrs()
    }
}

impl fmt::Debug for HostPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for HostPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host(), self.port())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hostport_parse() {

        let hostport = HostPort {
            host: "host01.example.com".to_string().into_boxed_str(),
            port: 80,
        };

        let addr = HostPort::from(SocketAddr::from_str("127.0.0.1:80").unwrap());

        assert_eq!(hostport, HostPort::parse("host01.example.com:80  ", 42).unwrap());
        assert_eq!(hostport, HostPort::parse("  host01.example.com", 80).unwrap());

        assert_eq!(addr, HostPort::parse("127.0.0.1:80", 42).unwrap());
        assert_eq!(addr, HostPort::parse("127.0.0.1", 80).unwrap());
    }
}
