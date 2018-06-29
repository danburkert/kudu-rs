use pb::master::alter_table_request_pb::{
    AddColumn, AddRangePartition, DropColumn, DropRangePartition, RenameColumn, Step, StepType,
};
use pb::master::{AlterTableRequestPb, CreateTableRequestPb};
use pb::partition_schema_pb::{ColumnIdentifierPb, HashBucketSchemaPb, RangeSchemaPb};
use pb::PartitionSchemaPb;

use meta_cache::{Entry, Lookup, TableLocations};
use partition::PartitionSchema;
use scanner::ScanBuilder;
use tablet::TabletInfo;
use Column;
use Error;
use OperationEncoder;
use Result;
use Row;
use Schema;
use TableId;
use Writer;
use WriterConfig;

use futures::{Async, Future, Poll, Stream};

#[derive(Clone)]
pub struct Table {
    name: String,
    id: TableId,
    schema: Schema,
    partition_schema: PartitionSchema,
    num_replicas: u32,
    table_locations: TableLocations,
}

impl Table {
    pub(crate) fn new(
        name: String,
        id: TableId,
        schema: Schema,
        partition_schema: PartitionSchema,
        num_replicas: u32,
        table_locations: TableLocations,
    ) -> Table {
        Table {
            name,
            id,
            schema,
            partition_schema,
            num_replicas,
            table_locations,
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

    pub fn new_writer(&self, config: WriterConfig) -> Writer {
        Writer::new(self.clone(), config)
    }

    pub fn scan_builder(&self) -> ScanBuilder {
        ScanBuilder::new(self.schema.clone(), self.table_locations.clone())
    }

    // TODO: should this be a stream?
    pub fn tablets(&self) -> Tablets {
        Tablets {
            table_locations: self.table_locations.clone(),
            primary_key_schema: self.schema.primary_key_projection(),
            partition_schema: self.partition_schema.clone(),
            lookup: Some(self.table_locations.entry(&[])),
        }
    }

    pub(crate) fn table_locations(&self) -> &TableLocations {
        &self.table_locations
    }
}

pub struct Tablets {
    table_locations: TableLocations,
    primary_key_schema: Schema,
    partition_schema: PartitionSchema,
    lookup: Option<Lookup<Entry>>,
}

impl Stream for Tablets {
    type Item = TabletInfo;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<TabletInfo>, Error> {
        loop {
            let entry = match self.lookup.as_mut() {
                Some(lookup) => try_ready!(lookup.poll()),
                None => return Ok(Async::Ready(None)),
            };

            self.lookup = if entry.upper_bound().is_empty() {
                None
            } else {
                Some(self.table_locations.entry(entry.upper_bound()))
            };

            match entry {
                Entry::Tablet(ref tablet) => {
                    return Ok(Async::Ready(Some(
                        tablet.info(&self.primary_key_schema, self.partition_schema.clone())?,
                    )));
                }
                Entry::NonCoveredRange {
                    ref upper_bound, ..
                } if upper_bound.is_empty() =>
                {
                    return Ok(Async::Ready(None));
                }
                Entry::NonCoveredRange { .. } => (),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangePartitionBound {
    Inclusive(Row<'static>),
    Exclusive(Row<'static>),
}

impl RangePartitionBound {
    fn row(&self) -> &Row<'static> {
        match *self {
            RangePartitionBound::Inclusive(ref row) | RangePartitionBound::Exclusive(ref row) => {
                row
            }
        }
    }
}

pub struct TableBuilder {
    name: String,
    schema: Schema,
    hash_partitions: Vec<(Vec<String>, u32, Option<u32>)>,
    range_partition_columns: Vec<String>,
    range_partitions: Vec<(RangePartitionBound, RangePartitionBound)>,
    range_partition_splits: Vec<Row<'static>>,
    num_replicas: Option<u32>,
}

impl TableBuilder {
    /// Creates a new table builder with the provided table name and schema.
    pub fn new<S>(name: S, schema: Schema) -> TableBuilder
    where
        S: Into<String>,
    {
        TableBuilder {
            name: name.into(),
            schema,
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
    pub fn add_hash_partitions<S>(
        &mut self,
        columns: Vec<S>,
        num_partitions: u32,
    ) -> &mut TableBuilder
    where
        S: Into<String>,
    {
        self.add_hash_partitions_with_seed(columns, num_partitions, 0)
    }

    pub fn add_hash_partitions_with_seed<S>(
        &mut self,
        columns: Vec<S>,
        num_partitions: u32,
        seed: u32,
    ) -> &mut TableBuilder
    where
        S: Into<String>,
    {
        let columns = columns.into_iter().map(Into::into).collect();
        self.hash_partitions
            .push((columns, num_partitions, Some(seed)));
        self
    }

    /// Range partitions the table by the specified columns.
    ///
    /// Range partitioned tables must have at least one partition added with
    /// `TableBuilder::add_range_partition`.
    pub fn set_range_partition_columns<S>(&mut self, columns: Vec<S>) -> &mut TableBuilder
    where
        S: Into<String>,
    {
        self.range_partition_columns = columns.into_iter().map(Into::into).collect();
        self
    }

    /// Adds a range partition to the table with the specified bounds.
    pub fn add_range_partition(
        &mut self,
        lower_bound: RangePartitionBound,
        upper_bound: RangePartitionBound,
    ) -> &mut TableBuilder {
        self.range_partitions.push((lower_bound, upper_bound));
        self
    }

    pub fn add_range_partition_split(&mut self, split: Row<'static>) -> &mut TableBuilder {
        self.range_partition_splits.push(split);
        self
    }

    pub fn set_num_replicas(&mut self, num_replicas: u32) {
        self.num_replicas = Some(num_replicas);
    }

    pub(crate) fn into_pb(self) -> Result<CreateTableRequestPb> {
        let TableBuilder {
            name,
            schema,
            range_partition_columns,
            range_partitions,
            range_partition_splits,
            hash_partitions,
            num_replicas,
        } = self;

        let mut range_encoder = OperationEncoder::new();

        if range_partition_columns.is_empty() && !range_partitions.is_empty() {
            return Err(Error::InvalidArgument(
                "range partitions specified without range partitioning columns".to_string(),
            ));
        } else if range_partition_columns.is_empty() && !range_partition_splits.is_empty() {
            return Err(Error::InvalidArgument(
                "range partition splits specified without range partitioning columns".to_string(),
            ));
        }

        for (lower, upper) in range_partitions {
            if &schema != lower.row().schema() || &schema != upper.row().schema() {
                return Err(Error::InvalidArgument(
                    "range partition bound schema does not match the table schema".to_string(),
                ));
            }
            range_encoder.encode_range_partition(&lower, &upper);
        }

        for split in range_partition_splits {
            if &schema != split.schema() {
                return Err(Error::InvalidArgument(
                    "range partition split schema does not match the table schema".to_string(),
                ));
            }
            range_encoder.encode_range_partition_split(&split);
        }

        let split_rows_range_bounds = range_encoder.into_pb();

        let partition_schema = PartitionSchemaPb {
            hash_bucket_schemas: hash_partitions
                .into_iter()
                .map(|(columns, num_partitions, seed)| {
                    let columns = columns.into_iter().map(ColumnIdentifierPb::from).collect();

                    HashBucketSchemaPb {
                        columns,
                        num_buckets: num_partitions as i32,
                        seed,
                        ..Default::default()
                    }
                })
                .collect(),
            range_schema: Some(RangeSchemaPb {
                columns: range_partition_columns
                    .into_iter()
                    .map(ColumnIdentifierPb::from)
                    .collect(),
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

    pub fn rename_table<S>(&mut self, new_name: S) -> &mut AlterTableBuilder
    where
        S: Into<String>,
    {
        self.pb.new_table_name = Some(new_name.into());
        self
    }

    pub fn add_column(&mut self, column: Column) -> &mut AlterTableBuilder {
        self.pb.alter_schema_steps.push(Step {
            type_: Some(StepType::AddColumn as i32),
            add_column: Some(AddColumn {
                schema: column.into_pb(false),
            }),
            ..Default::default()
        });
        self
    }

    pub fn drop_column<S>(&mut self, column: S) -> &mut AlterTableBuilder
    where
        S: Into<String>,
    {
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
    where
        S1: Into<String>,
        S2: Into<String>,
    {
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
        if self.result.is_err() {
            return;
        }

        if let Some(ref schema) = self.schema {
            if schema != new_schema {
                self.result = Err(Error::InvalidArgument(
                    "schemas of range partition bounds must match".to_string(),
                ));
            }
            return;
        }

        self.pb.schema = Some(new_schema.as_pb());
        self.schema = Some(new_schema.clone());
    }

    pub fn add_range_partition(
        &mut self,
        lower_bound: &RangePartitionBound,
        upper_bound: &RangePartitionBound,
    ) -> &mut AlterTableBuilder {
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

    pub fn drop_range_partition(
        &mut self,
        lower_bound: &RangePartitionBound,
        upper_bound: &RangePartitionBound,
    ) -> &mut AlterTableBuilder {
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

    use env_logger;
    use futures::Stream;
    use tokio::runtime::current_thread::Runtime;

    use super::*;
    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use schema::tests::simple_schema;
    use Client;
    use DataType;
    use Options;
    use SchemaBuilder;
    use TableBuilder;

    #[test]
    fn create_table_builder() {
        let _ = env_logger::try_init();

        let mut table_builder = TableBuilder::new("t", simple_schema());

        let mut split_row = table_builder.schema().new_row();
        split_row.set("key", "foo").unwrap();

        table_builder.add_range_partition_split(split_row);
    }

    #[test]
    fn tablets_unpartitioned() {
        let _ = env_logger::try_init();
        let mut cluster = MiniCluster::default();
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime
            .block_on(Client::new(cluster.master_addrs(), Options::default()))
            .expect("client");

        let schema = simple_schema();

        let mut table = TableBuilder::new("tablets_unpartitioned", schema.clone());
        table.set_num_replicas(1);
        let table_id = runtime
            .block_on(client.create_table(table))
            .expect("create_table");

        let table = runtime
            .block_on(client.open_table_by_id(table_id))
            .expect("open_table");
        let tablets = runtime.block_on(table.tablets().collect()).unwrap();
        assert_eq!(1, tablets.len());
    }

    #[test]
    fn tablets_non_covered_ranges() {
        let _ = env_logger::try_init();
        let mut cluster =
            MiniCluster::new(MiniClusterConfig::default().num_masters(1).num_tservers(3));
        let mut runtime = Runtime::new().unwrap();

        let mut client = runtime
            .block_on(Client::new(cluster.master_addrs(), Options::default()))
            .expect("client");

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
        table_builder.add_range_partition(
            RangePartitionBound::Inclusive(lower_bound),
            RangePartitionBound::Exclusive(upper_bound),
        );

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();
        lower_bound.set(0, 200i32).unwrap();
        upper_bound.set(0, 300i32).unwrap();
        table_builder.add_range_partition(
            RangePartitionBound::Inclusive(lower_bound),
            RangePartitionBound::Exclusive(upper_bound),
        );

        let table_id = runtime
            .block_on(client.create_table(table_builder))
            .unwrap();

        let table = runtime.block_on(client.open_table_by_id(table_id)).unwrap();
        let tablets = runtime.block_on(table.tablets().collect()).unwrap();

        assert_eq!(8, tablets.len());
    }
}
