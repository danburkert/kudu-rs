use std::fmt;
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
};
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};
use std::thread;

use kudu_pb::master::{
    DeleteTableRequestPB,
    IsCreateTableDoneRequestPB,
};
use uuid::Uuid;

use backoff::Backoff;
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

    /// Creates a new Kudu table with the schema and options specified by `builder`. Returns the
    /// new table's ID, or an error on failure.
    fn create_table(&self, builder: TableBuilder, deadline: Instant) -> Result<Uuid> {
        let (send, recv) = sync_channel(0);
        self.master.create_table(deadline, builder.into_pb(), move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|resp| Uuid::parse_str(&String::from_utf8_lossy(resp.get_table_id())).unwrap())
    }

    /// Returns true if the table is fully created.
    fn is_create_table_done(&self, table: Uuid, deadline: Instant) -> Result<bool> {
        let mut request = IsCreateTableDoneRequestPB::new();
        request.mut_table().set_table_id(table.simple().to_string().into_bytes());

        let (send, recv) = sync_channel(0);
        self.master.is_create_table_done(deadline, request, move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|resp| resp.get_done())
    }

    /// Synchronously waits until the table is created. If an error is returned, the table may not
    /// be created yet.
    fn wait_for_table_creation(&self, table: Uuid, deadline: Instant) -> Result<()> {
        let mut backoff = Backoff::with_duration_range(16, 4096);

        let mut is_create_table_done = self.is_create_table_done(table.clone(), deadline);
        while let Ok(false) = is_create_table_done {
            let sleep_ms = backoff.next_backoff_ms();
            debug!("create table not yet complete, waiting {}ms", sleep_ms);
            // TODO: don't sleep if it would expire the deadline.
            thread::sleep(Duration::from_millis(sleep_ms));
            is_create_table_done = self.is_create_table_done(table.clone(), deadline);
        }
        is_create_table_done.map(|_| ())
    }

    /// Deletes the Kudu table.
    fn delete_table(&self, table: Uuid, deadline: Instant) -> Result<()> {
        let mut request = DeleteTableRequestPB::new();
        request.mut_table().set_table_id(table.simple().to_string().into_bytes());

        let (send, recv) = sync_channel(0);
        self.master.delete_table(deadline, request, move |resp| send.send(resp).unwrap());
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
    fn new(master_addresses: Vec<SocketAddr>) -> ClientConfig {
        ClientConfig {
            master_addresses: master_addresses,
            ..Default::default()
        }
    }

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

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use mini_cluster::MiniCluster;
    use SchemaBuilder;
    use super::*;
    use TableBuilder;
    use DataType;

    use env_logger;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn create_and_delete_table() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let mut schema_builder = SchemaBuilder::new();
        schema_builder.add_column("key", DataType::Int32).set_not_null();
        schema_builder.add_column("val", DataType::Int32);
        schema_builder.set_primary_key(vec!["key".to_owned()]);

        let mut table_builder = TableBuilder::new("create_and_delete_table",
                                                  schema_builder.build().unwrap());
        table_builder.add_hash_partitions(vec!["key".to_owned()], 4);
        table_builder.set_num_replicas(1);

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation(table_id, deadline()).unwrap();

        client.delete_table(table_id, deadline()).unwrap();
    }
}
