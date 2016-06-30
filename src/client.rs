use std::collections::HashMap;
use std::fmt;
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
};
use std::str;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};
use std::thread;

use kudu_pb::master::{
    DeleteTableRequestPB,
    GetTableSchemaRequestPB,
    IsCreateTableDoneRequestPB,
    ListTablesRequestPB,
    TableIdentifierPB,
};
use parking_lot::Mutex;

use backoff::Backoff;
use Error;
use master::MasterProxy;
use meta_cache::MetaCache;
use partition::PartitionSchema;
use Result;
use rpc::Messenger;
use Schema;
use table::Table;
use table::TableBuilder;
use TableId;

#[derive(Clone)]
pub struct Client {
    messenger: Messenger,
    master: MasterProxy,
    config: ClientConfig,
    meta_caches: Arc<Mutex<HashMap<TableId, MetaCache>>>,
}

impl Client {

    pub fn new(config: ClientConfig) -> Client {
        let messenger = Messenger::new().unwrap();
        let master = MasterProxy::new(config.master_addresses(), messenger.clone());
        Client {
            master: master,
            messenger: messenger,
            config: config,
            meta_caches: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new Kudu table with the schema and options specified by `builder`. Returns the
    /// new table's ID, or an error on failure.
    pub fn create_table(&self, builder: TableBuilder, deadline: Instant) -> Result<TableId> {
        let (send, recv) = sync_channel(0);
        self.master.create_table(deadline,
                                 try!(builder.into_pb()),
                                 move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().and_then(|resp| {
            str::from_utf8(resp.get_table_id())
                .map_err(|error| Error::Serialization(format!("{}", error)))
                .and_then(TableId::parse)
        })
    }

    /// Returns `true` if the table is fully created.
    pub fn is_create_table_done<S>(&self, table: S, deadline: Instant) -> Result<bool>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_is_create_table_done(identifier, deadline)
    }

    /// Returns `true` if the table is fully created.
    pub fn is_create_table_done_by_id(&self, id: &TableId, deadline: Instant) -> Result<bool> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_is_create_table_done(identifier, deadline)
    }

    fn do_is_create_table_done(&self, table: TableIdentifierPB, deadline: Instant) -> Result<bool> {
        let mut request = IsCreateTableDoneRequestPB::new();
        request.set_table(table);

        let (send, recv) = sync_channel(0);
        self.master.is_create_table_done(deadline, request, move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|resp| resp.get_done())
    }

    /// Synchronously waits until the table is created. If an error is returned,
    /// the table may not be created yet.
    pub fn wait_for_table_creation<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_wait_for_table_creation(identifier, deadline)
    }

    /// Synchronously waits until the table is created. If an error is returned,
    /// the table may not be created yet.
    pub fn wait_for_table_creation_by_id(&self, id: &TableId, deadline: Instant) -> Result<()> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_wait_for_table_creation(identifier, deadline)
    }

    fn do_wait_for_table_creation(&self, table: TableIdentifierPB, deadline: Instant) -> Result<()> {
        let mut backoff = Backoff::with_duration_range(16, 4096);

        let mut is_create_table_done = self.do_is_create_table_done(table.clone(), deadline);
        while let Ok(false) = is_create_table_done {
            let sleep_ms = backoff.next_backoff_ms();
            debug!("create table not yet complete, waiting {}ms", sleep_ms);
            // TODO: do delayed send instead of sleep
            thread::sleep(Duration::from_millis(sleep_ms));
            is_create_table_done = self.do_is_create_table_done(table.clone(), deadline);
        }
        is_create_table_done.map(|_| ())
    }

    /// Deletes the Kudu table.
    pub fn delete_table<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_delete_table(identifier, deadline)
    }

    /// Deletes the Kudu table.
    pub fn delete_table_by_id(&self, id: &TableId, deadline: Instant) -> Result<()> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_delete_table(identifier, deadline)
    }

    fn do_delete_table(&self, table: TableIdentifierPB, deadline: Instant) -> Result<()> {
        let mut request = DeleteTableRequestPB::new();
        request.set_table(table);

        let (send, recv) = sync_channel(0);
        self.master.delete_table(deadline, request, move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|_| ())
    }

    pub fn list_tables(&self, deadline: Instant) -> Result<Vec<(String, TableId)>> {
        self.do_list_tables(ListTablesRequestPB::new(), deadline)
    }

    pub fn list_tables_with_name_filter<S>(&self, name_filter: S, deadline: Instant) -> Result<Vec<(String, TableId)>>
    where S: Into<String> {
        let mut request = ListTablesRequestPB::new();
        request.set_name_filter(name_filter.into());
        self.do_list_tables(request, deadline)
    }

    fn do_list_tables(&self, request: ListTablesRequestPB, deadline: Instant) -> Result<Vec<(String, TableId)>> {
        let (send, recv) = sync_channel(0);
        self.master.list_tables(deadline, request, move |resp| send.send(resp).unwrap());
        let mut resp = try!(recv.recv().unwrap());
        let mut tables = Vec::with_capacity(resp.get_tables().len());
        for mut table in resp.take_tables().into_iter() {
            tables.push((table.take_name(), try!(TableId::parse_bytes(table.get_id()))))
        }
        Ok(tables)
    }

    /// Returns the schema of a Kudu table.
    pub fn open_table<S>(&self, table: S, deadline: Instant) -> Result<Table>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_open_table(identifier, deadline)
    }

    /// Returns the schema of a Kudu table.
    pub fn open_table_by_id(&self, id: &TableId, deadline: Instant) -> Result<Table> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_open_table(identifier, deadline)
    }

    fn do_open_table(&self, table: TableIdentifierPB, deadline: Instant) -> Result<Table> {
        let mut request = GetTableSchemaRequestPB::new();
        request.set_table(table);

        let (send, recv) = sync_channel(0);
        self.master.get_table_schema(deadline, request, move |resp| send.send(resp).unwrap());

        let mut resp = try!(recv.recv().unwrap());
        let name = resp.take_table_name();
        let id = try!(TableId::parse_bytes(resp.get_table_id()));
        let partition_schema = PartitionSchema::from_pb(resp.get_partition_schema(),
                                                        resp.get_schema());
        let schema = try!(Schema::from_pb(resp.take_schema()));
        let meta_cache = self.meta_cache(&id);
        Ok(Table::new(name, id, schema, partition_schema, meta_cache))
    }

    pub fn master_proxy(&self) -> &MasterProxy {
        &self.master
    }

    fn meta_cache(&self, table: &TableId) -> MetaCache {
        self.meta_caches
            .lock()
            .entry(table.clone())
            .or_insert_with(|| MetaCache::new(table.clone(), self.master.clone()))
            .clone()
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
    pub fn new(master_addresses: Vec<SocketAddr>) -> ClientConfig {
        ClientConfig {
            master_addresses: master_addresses,
            ..Default::default()
        }
    }

    pub fn master_addresses(&self) -> &[SocketAddr] {
        &self.master_addresses
    }

    pub fn set_master_addresses(&mut self, master_addresses: Vec<SocketAddr>) -> &mut ClientConfig {
        self.master_addresses = master_addresses;
        self
    }

    pub fn default_admin_operation_timeout(&self) -> Duration {
        self.default_admin_operation_timeout
    }

    pub fn set_default_admin_operation_timeout(&mut self, timeout: Duration) -> &mut ClientConfig {
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

        let schema = schema_builder.build().unwrap();

        let mut table_builder = TableBuilder::new("create_and_delete_table", schema.clone());
        table_builder.add_hash_partitions(vec!["key".to_owned()], 4);
        table_builder.set_num_replicas(1);

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline()).unwrap();

        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        assert_eq!("create_and_delete_table", table.name());
        assert_eq!(&table_id, table.id());
        assert_eq!(&schema, table.schema());
        assert_eq!(1, table.partition_schema().hash_partition_schemas().len());
        assert_eq!(&[0], table.partition_schema().hash_partition_schemas()[0].columns());
        assert_eq!(4, table.partition_schema().hash_partition_schemas()[0].num_buckets());
        assert_eq!(0, table.partition_schema().hash_partition_schemas()[0].seed());
        assert_eq!(&[0], table.partition_schema().range_partition_schema().columns());

        let tables = client.list_tables(deadline()).unwrap();
        assert_eq!(1, tables.len());
        assert_eq!("create_and_delete_table", &tables[0].0);

        client.delete_table_by_id(&table_id, deadline()).unwrap();
    }
}
