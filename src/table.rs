use pb::master::{
    AlterTableRequestPb,
    CreateTableRequestPb,
};
use pb::master::alter_table_request_pb::{
    AddColumn,
    AddRangePartition,
    DropColumn,
    DropRangePartition,
    RenameColumn,
    Step,
    StepType,
};
use pb::PartitionSchemaPb;
use pb::partition_schema_pb::{
    ColumnIdentifierPb,
    HashBucketSchemaPb,
    RangeSchemaPb,
};

use Client;
use Column;
use Error;
use meta_cache::{Entry, MetaCache};
use partition::PartitionSchema;
use Result;
use row::OperationEncoder;
use row::Row;
use Schema;
use TableId;
//use Tablet;
//use Writer;
//use WriterConfig;

#[derive(Clone)]
pub struct Table {
    name: String,
    id: TableId,
    schema: Schema,
    partition_schema: PartitionSchema,
    num_replicas: u32,
    meta_cache: MetaCache,
    client: Client,
}

impl Table {

    pub(crate) fn new(name: String,
                      id: TableId,
                      schema: Schema,
                      partition_schema: PartitionSchema,
                      num_replicas: u32,
                      meta_cache: MetaCache,
                      client: Client) -> Table {
        Table {
            name: name,
            id: id,
            schema: schema,
            partition_schema: partition_schema,
            num_replicas: num_replicas,
            meta_cache: meta_cache,
            client: client,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> TableId {
        self.id
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn partition_schema(&self) -> &PartitionSchema {
        &self.partition_schema
    }

    pub fn num_replicas(&self) -> u32 {
        self.num_replicas
    }

    /*
    pub fn new_writer(&self, config: WriterConfig) -> Writer {
        Writer::new(self.clone(), config)
    }
    */

    /*
    pub fn list_tablets(&self, deadline: Instant) -> Result<Vec<Tablet>> {
        let mut tablets = Vec::new();
        let (send, recv) = sync_channel(1);
        let mut last_partition_key = Vec::new();

        loop {
            let send = send.clone();
            self.meta_cache.entry(mem::replace(&mut last_partition_key, Vec::new()),
                                  deadline,
                                  move |entry| send.send(entry).unwrap());

            match try!(recv.recv().unwrap()) {
                Entry::Tablet(tablet) => {
                    last_partition_key = tablet.partition().upper_bound_key().to_owned();
                    tablets.push(tablet);
                },
                Entry::NonCoveredRange { partition_upper_bound, .. } => {
                    last_partition_key = partition_upper_bound;
                },
            };
            if last_partition_key.is_empty() { break; }
        }

        Ok(tablets)
    }
    */

    pub(crate) fn meta_cache(&self) -> &MetaCache {
        &self.meta_cache
    }

    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangePartitionBound {
    Inclusive(Row),
    Exclusive(Row),
}

impl RangePartitionBound {
    fn row(&self) -> &Row {
        match *self {
            RangePartitionBound::Inclusive(ref row) => row,
            RangePartitionBound::Exclusive(ref row) => row,
        }
    }
}

pub struct TableBuilder {
    name: String,
    schema: Schema,
    hash_partitions: Vec<(Vec<String>, u32, Option<u32>)>,
    range_partition_columns: Vec<String>,
    range_partitions: Vec<(RangePartitionBound, RangePartitionBound)>,
    range_partition_splits: Vec<Row>,
    num_replicas: Option<u32>,
}

impl TableBuilder {

    /// Creates a new table builder with the provided table name and schema.
    pub fn new<S>(name: S, schema: Schema) -> TableBuilder where S: Into<String> {
        TableBuilder {
            name: name.into(),
            schema: schema,
            hash_partitions: Vec::new(),
            range_partition_columns: Vec::new(),
            range_partitions: Vec::new(),
            range_partition_splits: Vec::new(),
            num_replicas: None,
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Hash partitions the table by the specfied columns.
    pub fn add_hash_partitions<S>(&mut self,
                                  columns: Vec<S>,
                                  num_partitions: u32) -> &mut TableBuilder
    where S: Into<String> {
        self.add_hash_partitions_with_seed(columns, num_partitions, 0)
    }

    pub fn add_hash_partitions_with_seed<S>(&mut self,
                                            columns: Vec<S>,
                                            num_partitions: u32,
                                            seed: u32)
                                            -> &mut TableBuilder
    where S: Into<String> {
        let columns = columns.into_iter().map(Into::into).collect();
        self.hash_partitions.push((columns, num_partitions, Some(seed)));
        self
    }

    /// Range partitions the table by the specified columns.
    ///
    /// Range partitioned tables must have at least one partition added with
    /// `TableBuilder::add_range_partition`.
    pub fn set_range_partition_columns<S>(&mut self, columns: Vec<S>) -> &mut TableBuilder
    where S: Into<String> {
        self.range_partition_columns = columns.into_iter().map(Into::into).collect();
        self
    }

    /// Adds a range partition to the table with the specified bounds.
    pub fn add_range_partition(&mut self,
                               lower_bound: RangePartitionBound,
                               upper_bound: RangePartitionBound)
                               -> &mut TableBuilder {
        self.range_partitions.push((lower_bound, upper_bound));
        self
    }

    pub fn add_range_partition_split(&mut self, split: Row) -> &mut TableBuilder {
        self.range_partition_splits.push(split);
        self
    }

    pub fn set_num_replicas(&mut self, num_replicas: u32) {
        self.num_replicas = Some(num_replicas);
    }

    pub(crate) fn into_pb(self) -> Result<CreateTableRequestPb> {
        let TableBuilder { name, schema, range_partition_columns, range_partitions,
                           range_partition_splits, hash_partitions, num_replicas } = self;

        let mut range_encoder = OperationEncoder::new();

        if range_partition_columns.is_empty() && !range_partitions.is_empty() {
            return Err(Error::InvalidArgument(
                    "range partitions specified without range partitioning columns".to_string()));
        } else if range_partition_columns.is_empty() && !range_partition_splits.is_empty() {
            return Err(Error::InvalidArgument(
                    "range partition splits specified without range partitioning columns".to_string()));
        }

        for (lower, upper) in range_partitions {
            if &schema != lower.row().schema() || &schema != upper.row().schema() {
                return Err(Error::InvalidArgument(
                        "range partition bound schema does not match the table schema".to_string()));
            }
            range_encoder.encode_range_partition(&lower, &upper);
        }

        for split in range_partition_splits {
            if &schema != split.schema() {
                return Err(Error::InvalidArgument(
                        "range partition split schema does not match the table schema".to_string()));
            }
            range_encoder.encode_range_partition_split(&split);
        }

        let split_rows_range_bounds = range_encoder.into_pb();

        let partition_schema = PartitionSchemaPb {
            hash_bucket_schemas: hash_partitions.into_iter().map(|(columns, num_partitions, seed)| {
                let columns = columns.into_iter().map(ColumnIdentifierPb::from).collect();

                HashBucketSchemaPb {
                    columns,
                    num_buckets: num_partitions as i32,
                    seed: seed,
                    ..Default::default()
                }
            }).collect(),
            range_schema: Some(RangeSchemaPb {
                columns: range_partition_columns.into_iter().map(ColumnIdentifierPb::from).collect(),
            }),
        };

        Ok(CreateTableRequestPb {
            name,
            schema: schema.as_pb(),
            split_rows_range_bounds: Some(split_rows_range_bounds),
            partition_schema: Some(partition_schema),
            num_replicas: num_replicas.map(|n| n as i32),
        })
    }
}

pub struct AlterTableBuilder {
    pub(crate) result: Result<()>,
    pub(crate) schema: Option<Schema>,
    pub(crate) pb: AlterTableRequestPb,
}

impl AlterTableBuilder {

    pub fn new() -> AlterTableBuilder {
        AlterTableBuilder {
            result: Ok(()),
            schema: None,
            pb: AlterTableRequestPb::default(),
        }
    }

    pub fn rename_table<S>(&mut self, new_name: S) -> &mut AlterTableBuilder where S: Into<String> {
        self.pb.new_table_name = Some(new_name.into());
        self
    }

    pub fn add_column(&mut self, column: Column) -> &mut AlterTableBuilder {
        self.pb.alter_schema_steps.push(Step {
            type_: Some(StepType::AddColumn as i32),
            add_column: Some(AddColumn {
                schema: column.to_pb(false),
            }),
            ..Default::default()
        });
        self
    }

    pub fn drop_column<S>(&mut self, column: S) -> &mut AlterTableBuilder where S: Into<String> {
        self.pb.alter_schema_steps.push(Step {
            type_: Some(StepType::DropColumn as i32),
            drop_column: Some(DropColumn {
                name: column.into(),
            }),
            ..Default::default()
        });
        self
    }

    pub fn rename_column<S1, S2>(&mut self, old_name: S1, new_name: S2) -> &mut AlterTableBuilder
    where S1: Into<String>,
          S2: Into<String> {
        self.pb.alter_schema_steps.push(Step {
            type_: Some(StepType::RenameColumn as i32),
            rename_column: Some(RenameColumn {
                old_name: old_name.into(),
                new_name: new_name.into(),
            }),
            ..Default::default()
        });
        self
    }

    fn check_and_set_schema(&mut self, new_schema: &Schema) {
        if self.result.is_err() { return; }

        if let Some(ref schema) = self.schema {
            if schema != new_schema {
                self.result = Err(Error::InvalidArgument(
                        "schemas of range partition bounds must match".to_string()));
            }
            return;
        }

        self.pb.schema = Some(new_schema.as_pb());
        self.schema = Some(new_schema.clone());
    }

    pub fn add_range_partition(&mut self,
                               lower_bound: &RangePartitionBound,
                               upper_bound: &RangePartitionBound) -> &mut AlterTableBuilder {
        self.check_and_set_schema(lower_bound.row().schema());
        self.check_and_set_schema(upper_bound.row().schema());
        if self.result.is_ok() {
            let mut encoder = OperationEncoder::new();
            encoder.encode_range_partition(lower_bound, upper_bound);

            self.pb.alter_schema_steps.push(Step {
                type_: Some(StepType::AddRangePartition as i32),
                add_range_partition: Some(AddRangePartition {
                    range_bounds: Some(encoder.into_pb()),
                }),
                ..Default::default()
            });
        }
        self
    }

    pub fn drop_range_partition_by_ref(&mut self,
                                       lower_bound: &RangePartitionBound,
                                       upper_bound: &RangePartitionBound)
                                       -> &mut AlterTableBuilder {
        self.check_and_set_schema(lower_bound.row().schema());
        self.check_and_set_schema(upper_bound.row().schema());
        if self.result.is_ok() {
            let mut encoder = OperationEncoder::new();
            encoder.encode_range_partition(lower_bound, upper_bound);
            self.pb.alter_schema_steps.push(Step {
                type_: Some(StepType::DropRangePartition as i32),
                drop_range_partition: Some(DropRangePartition {
                    range_bounds: Some(encoder.into_pb()),
                }),
                ..Default::default()
            });
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};

    use env_logger;
    use tokio::reactor::Core;

    use Client;
    use ClientBuilder;
    use Column;
    use DataType;
    use SchemaBuilder;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use schema::tests::simple_schema;
    use super::*;

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(5)
    }

    #[test]
    fn create_table_builder() {
        let _ = env_logger::init();

        let mut table_builder = TableBuilder::new("t", simple_schema());

        let mut split_row = table_builder.schema().new_row();
        split_row.set_by_name("key", "foo").unwrap();

        table_builder.add_range_partition_split(split_row);
    }

    #[test]
    fn create_table() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(3));
        let reactor = Core::new().unwrap();

        let client = ClientBuilder::new(cluster.master_addrs(), reactor.remote()).build().unwrap();
    }

    /*
    #[test]
    fn list_tablets() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(3));
        let reactor = Core::new().unwrap();

        let client = ClientBuilder::new(cluster.master_addrs().to_owned(), reactor).build().unwrap();

        let schema = SchemaBuilder::new()
            .add_column(Column::builder("key", DataType::Int32).set_not_null())
            .add_column(Column::builder("val", DataType::Int32))
            .set_primary_key(vec!["key"])
            .build()
            .unwrap();

        let mut table_builder = TableBuilder::new("list_tablets", schema.clone());
        table_builder.add_hash_partitions(vec!["key"], 4);
        table_builder.set_num_replicas(3);

        table_builder.set_range_partition_columns(vec!["key"]);
        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();
        lower_bound.set(0, 0i32).unwrap();
        upper_bound.set(0, 100i32).unwrap();
        table_builder.add_range_partition(RangePartitionBound::Inclusive(lower_bound),
                                          RangePartitionBound::Exclusive(upper_bound));

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();
        lower_bound.set(0, 200i32).unwrap();
        upper_bound.set(0, 300i32).unwrap();
        table_builder.add_range_partition(RangePartitionBound::Inclusive(lower_bound),
                                          RangePartitionBound::Exclusive(upper_bound));

        let table_id = client.create_table(table_builder).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline() + Duration::from_secs(10)).unwrap();
        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        let tablets = table.list_tablets(deadline()).unwrap();

        assert_eq!(8, tablets.len());
    }
    */
}
