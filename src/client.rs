#![allow(unused_imports)]

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

use cpupool::CpuPool;
use futures::Future;
use futures::future::{
    self,
    Either,
};
use krpc::HostPort;
use krpc;
use parking_lot::Mutex;
use timer::Timer;
use tokio;

use pb::master::{
    AlterTableResponsePb,
    CreateTableResponsePb,
    DeleteTableRequestPb,
    DeleteTableResponsePb,
    GetTableSchemaRequestPb,
    GetTableSchemaResponsePb,
    IsAlterTableDoneRequestPb,
    IsCreateTableDoneRequestPb,
    IsCreateTableDoneResponsePb,
    ListMastersRequestPb,
    ListMastersResponsePb,
    ListTablesRequestPb,
    ListTablesResponsePb,
    ListTabletServersResponsePb,
    MasterService,
    TableIdentifierPb,
};
use pb::ExpectField;

use Error;
use IntoMasterAddrs;
use Master;
use Options;
use Result;
use Schema;
use TableId;
use TabletServer;
use backoff::Backoff;
use master::MasterProxy;
use meta_cache::MetaCache;
use partition::PartitionSchema;
use table::AlterTableBuilder;
use table::Table;
use table::TableBuilder;

/// A Kudu database client.
///
/// Encapsulates the connection to a Kudu cluster. Only a single `Client` instance should be used
/// per application.
#[derive(Clone)]
pub struct Client {
    master: MasterProxy,
    options: Options,
    meta_caches: Arc<Mutex<HashMap<TableId, MetaCache>>>,
    latest_observed_timestamp: Arc<Mutex<u64>>, // Replace with AtomicU64 when stable.
}

impl Client {

    /// Creates a new client with the provided configuration.
    fn new(master_addresses: Vec<HostPort>, options: Options) -> Client {
        let master = MasterProxy::new(master_addresses, options.clone());
        Client {
            master,
            options,
            meta_caches: Arc::new(Mutex::new(HashMap::new())),
            latest_observed_timestamp: Arc::new(Mutex::new(0)),
        }
    }

    /// Creates a new Kudu table with the schema and options specified by `builder`. Returns the
    /// new table's ID, or an error on failure.
    pub fn create_table(&mut self, builder: TableBuilder) -> impl Future<Item=TableId, Error=Error> {
        let pb = match builder.into_pb().map(Box::new) {
            Ok(pb) => pb,
            Err(error) => return Either::B(future::err(error)),
        };
        let request = MasterService::create_table(pb, Instant::now() + self.options.admin_timeout, &[]);

        let response = self.master
                           .send(request)
                           .and_then(|response: CreateTableResponsePb| -> Result<TableId> {
                               TableId::parse_bytes(&response.table_id.expect_field("CreateTableResponsePb",
                                                                                    "table_id")?)
                           });
        Either::A(response)
    }

    /// Returns `true` if the table is fully created.
    pub fn is_create_table_done<S>(&mut self, table: S) -> impl Future<Item=bool, Error=Error>
    where S: Into<String> {
        self.do_is_create_table_done(table.into().into())
    }

    /// Returns `true` if the table is fully created.
    pub fn is_create_table_done_by_id(&mut self, id: TableId) -> impl Future<Item=bool, Error=Error> {
        self.do_is_create_table_done(id.into())
    }

    fn do_is_create_table_done(&mut self, table: TableIdentifierPb) -> impl Future<Item=bool, Error=Error> {
        let request = MasterService::is_create_table_done(Box::new(IsCreateTableDoneRequestPb { table }),
                                                          Instant::now() + self.options.admin_timeout,
                                                          &[]);

        self.master
            .send(request)
            .map(|response: IsCreateTableDoneResponsePb| response.done())
    }

/*
    /// Synchronously waits until the table is created. If an error is returned,
    /// the table may not be created yet.
    pub fn wait_for_table_creation<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_name(table.into());
        self.do_wait_for_table_creation(identifier, deadline)
    }

    /// Synchronously waits until the table is created. If an error is returned,
    /// the table may not be created yet.
    pub fn wait_for_table_creation_by_id(&self, id: &TableId, deadline: Instant) -> Result<()> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_wait_for_table_creation(identifier, deadline)
    }

    fn do_wait_for_table_creation(&self, table: TableIdentifierPb, deadline: Instant) -> Result<()> {
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
*/

    /// Deletes the table.
    pub fn delete_table<S>(&mut self, table: S) -> impl Future<Item=(), Error=Error>
    where S: Into<String> {
        self.do_delete_table(table.into().into())
    }

    /// Deletes the table.
    pub fn delete_table_by_id(&mut self, id: TableId) -> impl Future<Item=(), Error=Error> {
        self.do_delete_table(id.into())
    }

    fn do_delete_table(&mut self, table: TableIdentifierPb) -> impl Future<Item=(), Error=Error>{
        let request = MasterService::delete_table(Box::new(DeleteTableRequestPb { table }),
                                                           Instant::now() + self.options.admin_timeout,
                                                           &[]);

        self.master.send(request).map(|_: DeleteTableResponsePb| ())
    }

    pub fn alter_table<S>(&mut self, table: S, alter: AlterTableBuilder) -> impl Future<Item=TableId, Error=Error>
    where S: Into<String> {
        self.do_alter_table(table.into().into(), alter)
    }

    pub fn alter_table_by_id(&mut self, id: TableId, alter: AlterTableBuilder) -> impl Future<Item=(), Error=Error> {
        self.do_alter_table(id.into(), alter).map(|_| ())
    }

    pub fn do_alter_table(&mut self, identifier: TableIdentifierPb, alter: AlterTableBuilder) -> impl Future<Item=TableId, Error=Error> {
        let AlterTableBuilder { result, mut pb, schema } = alter;
        if let Err(error) = result {
            return Either::B(future::err(error));
        }

        pb.table = identifier;
        let request = MasterService::alter_table(Box::new(pb), Instant::now() + self.options.admin_timeout, &[]);
        let meta_caches = self.meta_caches.clone();
        let result = self.master.send(request).and_then(move |resp: AlterTableResponsePb| {
            let table_id = str::from_utf8(resp.table_id())
                               .map_err(|error| Error::Serialization(format!("{}", error)))
                               .and_then(TableId::parse)?;

            // If the table partitioning was altered and there is an existing meta cache for the table,
            // clear it.
            if schema.is_some() {
                let meta_cache = meta_caches.lock().get(&table_id).cloned();
                if let Some(meta_cache) = meta_cache {
                    meta_cache.clear();
                }
            }

            Ok(table_id)
        });
        Either::A(result)
    }

/*
    /// Returns `true` if the table is fully altered.
    pub fn is_alter_table_done<S>(&self, table: S, deadline: Instant) -> Result<bool>
    where S: Into<String> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_name(table.into());
        self.do_is_alter_table_done(identifier, deadline)
    }

    /// Returns `true` if the table is fully altered.
    pub fn is_alter_table_done_by_id(&self, id: &TableId, deadline: Instant) -> Result<bool> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_is_alter_table_done(identifier, deadline)
    }

    fn do_is_alter_table_done(&self, table: TableIdentifierPb, deadline: Instant) -> Result<bool> {
        let mut request = IsAlterTableDoneRequestPb::new();
        request.set_table(table);

        let (send, recv) = sync_channel(1);
        self.master.is_alter_table_done(deadline, request, move |resp| send.send(resp).unwrap());
        recv.recv().unwrap().map(|resp| resp.get_done())
    }

    /// Synchronously waits until the table is altered. If an error is returned,
    /// the table may not be altered yet.
    pub fn wait_for_table_alteration<S>(&self, table: S, deadline: Instant) -> Result<()>
    where S: Into<String> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_name(table.into());
        self.do_wait_for_table_alteration(identifier, deadline)
    }

    /// Synchronously waits until the table is altered. If an error is returned,
    /// the table may not be altered yet.
    pub fn wait_for_table_alteration_by_id(&self, id: &TableId, deadline: Instant) -> Result<()> {
        let mut identifier = TableIdentifierPb::new();
        identifier.set_table_id(id.to_string().into_bytes());
        self.do_wait_for_table_alteration(identifier, deadline)
    }

    fn do_wait_for_table_alteration(&self, table: TableIdentifierPb, deadline: Instant) -> Result<()> {
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
    */

    /// Lists all tables and their associated table ID.
    pub fn list_tables(&mut self) -> impl Future<Item=Vec<(String, TableId)>, Error=Error> {
        self.do_list_tables(Default::default())
    }

    /// Lists all tables with the a name matching the provided prefix, and their associated table ID.
    pub fn list_tables_with_prefix<S>(&mut self, name_prefix: S) -> impl Future<Item=Vec<(String, TableId)>, Error=Error>
    where S: Into<String> {
        self.do_list_tables(Box::new(ListTablesRequestPb { name_filter: Some(name_prefix.into()) }))
    }

    fn do_list_tables(&mut self, request: Box<ListTablesRequestPb>) -> impl Future<Item=Vec<(String, TableId)>, Error=Error> {
        let request = MasterService::list_tables(request,
                                                 Instant::now() + self.options.admin_timeout,
                                                 &[]);

        self.master.send(request).and_then(|response: ListTablesResponsePb| {
            let mut tables = Vec::with_capacity(response.tables.len());
            for table in response.tables {
                tables.push((table.name, TableId::parse_bytes(&table.id)?));
            }
            Ok(tables)
        })
    }

    pub fn list_masters(&mut self) -> impl Future<Item=Vec<Master>, Error=Error> {
        let request = MasterService::list_masters(Default::default(),
                                                  Instant::now() + self.options.admin_timeout,
                                                  &[]);

        self.master.send(request).and_then(|response: ListMastersResponsePb| {
            let mut servers = Vec::with_capacity(response.masters.len());
            for server in response.masters {
                servers.push(Master::from_pb(server)?);
            }
            Ok(servers)
        })
    }

    pub fn list_tablet_servers(&mut self) -> impl Future<Item=Vec<TabletServer>, Error=Error> {
        let request = MasterService::list_tablet_servers(Default::default(),
                                                         Instant::now() + self.options.admin_timeout,
                                                         &[]);

        self.master.send(request).and_then(|response: ListTabletServersResponsePb| {
            let mut servers = Vec::with_capacity(response.servers.len());
            for server in response.servers {
                servers.push(TabletServer::from_pb(server)?);
            }
            Ok(servers)
        })
    }

    /// Returns an open table.
    pub fn open_table<S>(&mut self, table: S) -> impl Future<Item=Table, Error=Error>
    where S: Into<String> {
        self.do_open_table(table.into().into())
    }

    /// Returns an open table.
    pub fn open_table_by_id(&mut self, id: TableId) -> impl Future<Item=Table, Error=Error> {
        self.do_open_table(id.into())
    }

    fn do_open_table(&mut self, table: TableIdentifierPb) -> impl Future<Item=Table, Error=Error> {
        let request = MasterService::get_table_schema(Box::new(GetTableSchemaRequestPb { table }),
                                                      Instant::now() + self.options.admin_timeout,
                                                      &[]);

        let client = self.clone();
        self.master.send(request).and_then(move |resp: GetTableSchemaResponsePb| -> Result<Table> {
            static MESSAGE: &'static str = "GetTableSchemaResponsePb";

            let num_replicas = resp.num_replicas() as u32;
            let name = resp.table_name.expect_field(MESSAGE, "table_name")?;

            let id = TableId::parse_bytes(&resp.table_id.expect_field(MESSAGE, "table_id")?)?;

            let schema = resp.schema.expect_field(MESSAGE, "schema")?;
            let partition_schema = PartitionSchema::from_pb(&resp.partition_schema.expect_field(MESSAGE, "partition_schema")?,
                                                            &schema);
            let schema = Schema::from_pb(schema)?;

            let meta_cache = client.meta_caches
                                   .lock()
                                   .entry(id.clone())
                                   .or_insert_with(|| MetaCache::new(id.clone(),
                                                                     schema.primary_key_projection(),
                                                                     partition_schema.clone(),
                                                                     client.master.clone()))
                                   .clone();

            Ok(Table::new(name,
                          id,
                          schema,
                          partition_schema,
                          num_replicas,
                          meta_cache,
                          client))
        })
    }

    pub fn latest_observed_timestamp(&self) -> u64 {
        *self.latest_observed_timestamp.lock()
    }

    pub fn observe_timestamp(&self, timestamp: u64) {
        let mut latest = self.latest_observed_timestamp.lock();
        if timestamp > *latest {
            *latest = timestamp;
        }
    }

    pub(crate) fn master_proxy(&self) -> &MasterProxy {
        &self.master
    }

    /// This should only be called when the table has been guaranteed to have been opened.
    pub(crate) fn meta_cache(&self, table: &TableId) -> MetaCache {
        self.meta_caches.lock()[table].clone()
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}

/// Client configuration options.
#[derive(Debug)]
pub struct ClientBuilder {
    master_addresses: Result<Vec<HostPort>>,
    reactor: tokio::reactor::Remote,
    rpc: krpc::Options,
    threadpool: Option<CpuPool>,
    timer: Option<Timer>,
    admin_timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn new<A>(master_addresses: A,
                  reactor: tokio::reactor::Remote)
                  -> ClientBuilder where A: IntoMasterAddrs {
        ClientBuilder {
            master_addresses: master_addresses.into_master_addrs(),
            reactor,
            rpc: krpc::Options::default(),
            threadpool: None,
            timer: None,
            admin_timeout: None,
        }
    }

    pub fn build(self) -> Result<Client> {
        let master_addresses = self.master_addresses?;

        // TODO: This is a terrible default.
        let threadpool = self.threadpool.unwrap_or_else(CpuPool::new_num_cpus);
        let timer = self.timer.unwrap_or_default();
        let admin_timeout = self.admin_timeout.unwrap_or(Duration::from_secs(60));

        let options = Options {
            rpc: self.rpc,
            remote: self.reactor,
            threadpool,
            timer,
            admin_timeout,
        };

        Ok(Client::new(master_addresses, options))
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
    use tokio::reactor::Core;

    #[test]
    fn table_lifecycle() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::default();
        let mut reactor = Core::new().unwrap();

        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("table_lifecycle", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(1);

        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");
        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.rename_table("table_lifecycle_renamed");
        reactor.run(client.alter_table_by_id(table_id, alter_builder)).expect("alter_table_by_id");

        let table = reactor.run(client.open_table("table_lifecycle_renamed")).expect("open_table");
        assert_eq!(table_id, table.id());

        let tables = reactor.run(client.list_tables()).expect("list_tables");
        assert_eq!(vec![("table_lifecycle_renamed".to_string(), table_id)], tables);

        reactor.run(client.delete_table_by_id(table_id)).expect("delete_table");
        assert!(reactor.run(client.list_tables()).expect("list_tables").is_empty());
    }

    #[test]
    fn list_servers() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(3)
                                                             .num_tservers(3));
        let mut reactor = Core::new().unwrap();
        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let tablet_servers = reactor.run(client.list_tablet_servers()).expect("list_table_servers");
        assert_eq!(3, tablet_servers.len());

        let masters = reactor.run(client.list_masters()).expect("list_masters");
        assert_eq!(3, masters.len());
    }

    #[test]
    fn alter_table() {
        let _ = env_logger::init();
        let mut cluster = MiniCluster::new(MiniClusterConfig::default()
                                                             .num_masters(1)
                                                             .num_tservers(1));
        let mut reactor = Core::new().unwrap();
        let mut client = ClientBuilder::new(cluster.master_addrs(), reactor.remote())
                                       .build()
                                       .expect("client");

        let mut table_builder = TableBuilder::new("t", simple_schema());
        table_builder.set_num_replicas(1);
        table_builder.set_range_partition_columns(vec!["key"]);
        let table_id = reactor.run(client.create_table(table_builder)).expect("create_table");

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.add_column(Column::builder("c0", DataType::Int32));

        let _ = reactor.run(client.alter_table("t", alter_builder)).expect("add column");

        // TODO: wait
        thread::sleep(Duration::from_millis(2000));

        let schema = reactor.run(client.open_table("t")).expect("open_table").schema().clone();
        assert_eq!(3, schema.columns().len());

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.drop_range_partition(&RangePartitionBound::Inclusive(schema.new_row()),
                                           &RangePartitionBound::Exclusive(schema.new_row()));

        reactor.run(client.alter_table_by_id(table_id, alter_builder)).expect("drop range partition");

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();

        lower_bound.set_by_name("key", "a").unwrap();
        upper_bound.set_by_name("key", "z").unwrap();

        let mut alter_builder = AlterTableBuilder::new();
        alter_builder.add_range_partition(&RangePartitionBound::Inclusive(lower_bound),
                                          &RangePartitionBound::Exclusive(upper_bound))
                     .rename_table("u")
                     .drop_column("c0");
        reactor.run(client.alter_table_by_id(table_id, alter_builder)).unwrap();

        // TODO: wait
        thread::sleep(Duration::from_millis(2000));

        let schema = reactor.run(client.open_table("u")).unwrap().schema().clone();
        assert_eq!(2, schema.columns().len());
    }
}
