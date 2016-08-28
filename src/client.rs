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
use std::thread;
use std::time::{Duration, Instant};

use kudu_pb::master::{
    DeleteTableRequestPB,
    GetTableSchemaRequestPB,
    IsAlterTableDoneRequestPB,
    IsCreateTableDoneRequestPB,
    ListMastersRequestPB,
    ListTablesRequestPB,
    ListTabletServersRequestPB,
    TableIdentifierPB,
};
use parking_lot::Mutex;

use Error;
use Operation;
use Result;
use Schema;
use Session;
use TableId;
use TabletServer;
use backoff::Backoff;
use master::Master;
use master::MasterProxy;
use meta_cache::MetaCache;
use partition::PartitionSchema;
use rpc::Messenger;
use table::AlterTableBuilder;
use table::Table;
use table::TableBuilder;

/// A Kudu database client.
///
/// Encapsulates the connection to a Kudu cluster. Only a single `Client` instance should be used
/// per application.
#[derive(Clone)]
pub struct Client {
    messenger: Messenger,
    master: MasterProxy,
    config: ClientConfig,
    meta_caches: Arc<Mutex<HashMap<TableId, MetaCache>>>,
    latest_observed_timestamp: Arc<Mutex<u64>>, // Replace with AtomicU64 when stable.
}

impl Client {

    /// Creates a new client with the provided configuration.
    pub fn new(config: ClientConfig) -> Client {
        let messenger = Messenger::new().unwrap();
        let master = MasterProxy::new(config.master_addresses(), messenger.clone());
        Client {
            master: master,
            messenger: messenger,
            config: config,
            meta_caches: Arc::new(Mutex::new(HashMap::new())),
            latest_observed_timestamp: Arc::new(Mutex::new(0)),
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

        let (send, recv) = sync_channel(1);
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
        let mut backoff = Backoff::with_duration_range(5, 5000);

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

    /// Deletes the table.
    pub fn delete_table<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_delete_table(identifier, deadline)
    }

    /// Deletes the table.
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

    pub fn alter_table<S>(&self,
                          table: S,
                          alter: AlterTableBuilder,
                          deadline: Instant)
                          -> Result<TableId>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_alter_table(identifier, alter, deadline)
    }

    pub fn alter_table_by_id(&self,
                             id: &TableId,
                             alter: AlterTableBuilder,
                             deadline: Instant)
                             -> Result<()> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_alter_table(identifier, alter, deadline).map(|_| ())
    }

    fn do_alter_table(&self,
                      table: TableIdentifierPB,
                      alter: AlterTableBuilder,
                      deadline: Instant)
                      -> Result<TableId> {
        let AlterTableBuilder { error, mut pb, .. } = alter;
        try!(error);
        pb.set_table(table);

        let (send, recv) = sync_channel(0);
        self.master.alter_table(deadline, pb, move |resp| send.send(resp).unwrap());
        let table_id = recv.recv().unwrap().and_then(|resp| {
            str::from_utf8(resp.get_table_id())
                .map_err(|error| Error::Serialization(format!("{}", error)))
                .and_then(TableId::parse)
        });

        // If the table partitioning was altered and there is an existing meta cache for the table,
        // clear it.
        if alter.schema.is_some() {
            if let Ok(ref table_id) = table_id {
                let meta_cache = self.meta_caches.lock().get(table_id).cloned();
                if let Some(meta_cache) = meta_cache {
                    meta_cache.clear();
                }
            }
        }
        table_id
    }

    /// Returns `true` if the table is fully altered.
    pub fn is_alter_table_done<S>(&self, table: S, deadline: Instant) -> Result<bool>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_is_alter_table_done(identifier, deadline)
    }

    /// Returns `true` if the table is fully altered.
    pub fn is_alter_table_done_by_id(&self, id: &TableId, deadline: Instant) -> Result<bool> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_is_alter_table_done(identifier, deadline)
    }

    fn do_is_alter_table_done(&self, table: TableIdentifierPB, deadline: Instant) -> Result<bool> {
        let mut request = IsAlterTableDoneRequestPB::new();
        request.set_table(table);

        let (send, recv) = sync_channel(1);
        self.master.is_alter_table_done(deadline, request, move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|resp| resp.get_done())
    }

    /// Synchronously waits until the table is altered. If an error is returned,
    /// the table may not be altered yet.
    pub fn wait_for_table_alteration<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_wait_for_table_alteration(identifier, deadline)
    }

    /// Synchronously waits until the table is altered. If an error is returned,
    /// the table may not be altered yet.
    pub fn wait_for_table_alteration_by_id(&self, id: &TableId, deadline: Instant) -> Result<()> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_wait_for_table_alteration(identifier, deadline)
    }

    fn do_wait_for_table_alteration(&self, table: TableIdentifierPB, deadline: Instant) -> Result<()> {
        let mut backoff = Backoff::with_duration_range(5, 5000);

        let mut is_table_alter_done = self.do_is_alter_table_done(table.clone(), deadline);
        while let Ok(false) = is_table_alter_done {
            let sleep_ms = backoff.next_backoff_ms();
            debug!("alter table not yet complete, waiting {}ms", sleep_ms);
            // TODO: do delayed send instead of sleep
            thread::sleep(Duration::from_millis(sleep_ms));
            is_table_alter_done = self.do_is_alter_table_done(table.clone(), deadline);
        }
        is_table_alter_done.map(|_| ())
    }

    /// Lists all tables and their associated table ID.
    pub fn list_tables(&self, deadline: Instant) -> Result<Vec<(String, TableId)>> {
        self.do_list_tables(ListTablesRequestPB::new(), deadline)
    }

    /// Lists all tables with the a name matching the provided prefix, and their associated table
    /// ID.
    pub fn list_tables_with_prefix<S>(&self, name_prefix: S, deadline: Instant) ->
        Result<Vec<(String, TableId)>>
    where S: Into<String> {
        let mut request = ListTablesRequestPB::new();
        request.set_name_filter(name_prefix.into());
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

    pub fn list_masters(&self, deadline: Instant) -> Result<Vec<Master>> {
        let request = ListMastersRequestPB::new();
        let (send, recv) = sync_channel(1);
        self.master.list_masters(deadline, request, move |resp| send.send(resp).unwrap());
        let mut resp = try!(recv.recv().unwrap());
        let mut masters = Vec::with_capacity(resp.get_masters().len());
        for master in resp.take_masters().into_iter() {
            masters.push(try!(Master::from_pb(master)));
        }
        Ok(masters)
    }

    pub fn list_tablet_servers(&self, deadline: Instant) -> Result<Vec<TabletServer>> {
        let request = ListTabletServersRequestPB::new();
        let (send, recv) = sync_channel(1);
        self.master.list_tablet_servers(deadline, request, move |resp| send.send(resp).unwrap());
        let mut resp = try!(recv.recv().unwrap());
        let mut tablet_servers = Vec::with_capacity(resp.get_servers().len());
        for server in resp.take_servers().into_iter() {
            tablet_servers.push(try!(TabletServer::from_pb(server)));
        }
        Ok(tablet_servers)
    }

    /// Returns an open table.
    pub fn open_table<S>(&self, table: S, deadline: Instant) -> Result<Table>
    where S: Into<String> {
        let mut identifier = TableIdentifierPB::new();
        identifier.set_table_name(table.into());
        self.do_open_table(identifier, deadline)
    }


    /// Returns an open table.
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
        let meta_cache = self.meta_caches
                             .lock()
                             .entry(id.clone())
                             .or_insert_with(|| MetaCache::new(id.clone(),
                                                               schema.primary_key_projection(),
                                                               partition_schema.clone(),
                                                               self.master.clone()))
                             .clone();

        Ok(Table::new(name, id, schema, partition_schema, resp.get_num_replicas() as u32, self.master.clone(), meta_cache))
    }

    pub fn new_session<E>(&self) -> Session<E> where E: FnOnce(Operation, Error) + Send + 'static {
        Session::new(self.clone())
    }

    pub fn latest_observed_timestamp(&self) -> u64 {
        *self.latest_observed_timestamp.lock()
    }

    pub fn timestamp_observed(&self, timestamp: u64) {
        let mut latest = self.latest_observed_timestamp.lock();
        if timestamp > *latest {
            *latest = timestamp;
        }
    }

    #[doc(hidden)]
    pub fn master_proxy(&self) -> &MasterProxy {
        &self.master
    }

    /// This should only be called when the table has been guaranteed to have been opened.
    #[doc(hidden)]
    pub fn meta_cache(&self, table: &TableId) -> MetaCache {
        self.meta_caches.lock()[table].clone()
    }

    #[doc(hidden)]
    pub fn messenger(&self) -> &Messenger {
        &self.messenger
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}

/// Client configuration options.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientConfig {
    /// A seed set of master addresses. Must contain at least one active master in the cluster.
    master_addresses: Vec<SocketAddr>,
}

impl ClientConfig {
    pub fn new(master_addresses: Vec<SocketAddr>) -> ClientConfig {
        ClientConfig {
            master_addresses: master_addresses,
        }
    }

    pub fn master_addresses(&self) -> &[SocketAddr] {
        &self.master_addresses
    }

    pub fn set_master_addresses(&mut self, master_addresses: Vec<SocketAddr>) -> &mut ClientConfig {
        self.master_addresses = master_addresses;
        self
    }
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            master_addresses: vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7051)],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use AlterTableBuilder;
    use Column;
    use DataType;
    use RangePartitionBound;
    use SchemaBuilder;
    use TableBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use schema::tests::simple_schema;
    use super::*;

    use env_logger;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn create_and_delete_table() {
        let _ = env_logger::init();
        let cluster = MiniCluster::default();

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("create_and_delete_table", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
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

    #[test]
    fn list_tablet_servers() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(3));
        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

        let tablet_servers = client.list_tablet_servers(deadline()).unwrap();
        assert_eq!(3, tablet_servers.len());
    }

    #[test]
    fn list_masters() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(3)
                                                         .num_tservers(0));
        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        let masters = client.list_masters(deadline()).unwrap();
        assert_eq!(3, masters.len());
    }

    #[test]
    fn alter_table() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(1));

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));
        let mut table_builder = TableBuilder::new("t", simple_schema());
        table_builder.set_num_replicas(1);
        table_builder.set_range_partition_columns(vec!["key"]);
        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation("t", deadline()).unwrap();

        client.alter_table("t", AlterTableBuilder::new()
                           .add_column(Column::builder("c0", DataType::Int32)),
                           deadline()).unwrap();
        client.wait_for_table_alteration("t", deadline()).unwrap();

        let schema = client.open_table("t", deadline()).unwrap().schema().clone();
        assert_eq!(3, schema.columns().len());

        let alter = AlterTableBuilder::new().drop_range_partition(
            &RangePartitionBound::Inclusive(schema.new_row()),
            &RangePartitionBound::Exclusive(schema.new_row()));

        client.alter_table_by_id(&table_id, alter, deadline()).unwrap();
        client.wait_for_table_alteration("t", deadline()).unwrap();

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();

        lower_bound.set_by_name("key", "a").unwrap();
        upper_bound.set_by_name("key", "z").unwrap();

        let alter = AlterTableBuilder::new()
            .add_range_partition(&RangePartitionBound::Inclusive(lower_bound),
                                 &RangePartitionBound::Exclusive(upper_bound))
            .rename_table("u")
            .drop_column("c0");

        client.alter_table_by_id(&table_id, alter, deadline()).unwrap();
        client.wait_for_table_alteration("u", deadline()).unwrap();
        let schema = client.open_table("u", deadline()).unwrap().schema().clone();
        assert_eq!(2, schema.columns().len());
    }
}
