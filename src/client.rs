use std::fmt;
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
};
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};

use master::MasterProxy;
use rpc::Messenger;
use table::TableBuilder;
use Result;

#[derive(Clone)]
pub struct Client {
    messenger: Messenger,
    master: MasterProxy,
    config: ClientConfig,
}

impl Client {

    fn new(config: ClientConfig) -> Client {
        let messenger = Messenger::new().unwrap();
        let master = MasterProxy::new(config.master_addresses(), messenger.clone());
        Client {
            master: master,
            messenger: messenger,
            config: config,
        }
    }

    fn create_table(&self, builder: TableBuilder) -> Result<()> {
        let (send, recv) = sync_channel(0);

        self.master.create_table(Instant::now() + self.config.default_admin_operation_timeout(),
                                 builder.into_pb(),
                                 move |resp| send.send(resp).unwrap());

        recv.recv().unwrap().map(|_| ())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientConfig {

    /// A seed set of master addresses. Must contain at least one active master in the cluster.
    master_addresses: Vec<SocketAddr>,

    /// The default timeout for admin operations.
    default_admin_operation_timeout: Duration,
}

impl ClientConfig {
    fn master_addresses(&self) -> &[SocketAddr] {
        &self.master_addresses
    }

    fn set_master_addresses(&mut self, master_addresses: Vec<SocketAddr>) -> &mut ClientConfig {
        self.master_addresses = master_addresses;
        self
    }

    fn default_admin_operation_timeout(&self) -> Duration {
        self.default_admin_operation_timeout
    }

    fn set_default_admin_operation_timeout(&mut self, timeout: Duration) -> &mut ClientConfig {
        self.default_admin_operation_timeout = timeout;
        self
    }
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            master_addresses: vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7051)],
            default_admin_operation_timeout: Duration::from_secs(30),
        }
    }
}
