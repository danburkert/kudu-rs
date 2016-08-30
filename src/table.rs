use std::mem;
use std::sync::mpsc::sync_channel;
use std::time::Instant;

use kudu_pb::master::{
    AlterTableRequestPB,
    AlterTableRequestPB_StepType as StepType,
    CreateTableRequestPB
};
use kudu_pb::common::{
    PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
    PartitionSchemaPB_HashBucketSchemaPB as HashBucketSchemaPB
};

use Client;
use Column;
use Error;
use Operation;
use OperationKind;
use Result;
use Schema;
use TableId;
use Tablet;
use meta_cache::{Entry, MetaCache};
use partition::PartitionSchema;
use row::OperationEncoder;
use row::Row;

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

    #[doc(hidden)]
    pub fn new(name: String,
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

    pub fn id(&self) -> &TableId {
        &self.id
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

    pub fn insert(&self) -> Operation {
        Operation::new(self.meta_cache.clone(), OperationKind::Insert, self.schema.new_row())
    }

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
                    last_partition_key = tablet.partition().upper_bound_encoded().to_owned();
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

    #[doc(hidden)]
    pub fn meta_cache(&self) -> &MetaCache {
        &self.meta_cache
    }

    #[doc(hidden)]
    pub fn client(&self) -> &Client {
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
        self.add_hash_partition_with_seed(columns, num_partitions, 0)
    }

    pub fn add_hash_partition_with_seed<S>(&mut self,
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

    #[doc(hidden)]
    pub fn into_pb(self) -> Result<CreateTableRequestPB> {
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

        let mut range_partition_idxs = Vec::new();
        for column in &range_partition_columns {
            match schema.column_index(column) {
                Some(idx) => range_partition_idxs.push(idx),
                None => return Err(Error::InvalidArgument(
                        format!("range partition column '{}' not found in schema", column))),
            }
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

        let mut pb = CreateTableRequestPB::new();
        pb.set_name(name);
        pb.set_schema(schema.as_pb());

        let (data, indirect_data) = range_encoder.unwrap();
        pb.mut_split_rows_range_bounds().set_rows(data);
        pb.mut_split_rows_range_bounds().set_indirect_data(indirect_data);

        for column in range_partition_columns {
            let mut column_pb = ColumnIdentifierPB::new();
            column_pb.set_name(column);
            pb.mut_partition_schema().mut_range_schema().mut_columns().push(column_pb);
        }

        for (columns, num_partitions, seed) in hash_partitions {
            let mut hash_pb = HashBucketSchemaPB::new();
            for column in columns {
                let mut column_pb = ColumnIdentifierPB::new();
                column_pb.set_name(column);
                hash_pb.mut_columns().push(column_pb);
            }
            hash_pb.set_num_buckets(num_partitions as i32);
            if let Some(seed) = seed { hash_pb.set_seed(seed); }
            pb.mut_partition_schema().mut_hash_bucket_schemas().push(hash_pb);
        }

        if let Some(num_replicas) = num_replicas { pb.set_num_replicas(num_replicas as i32); }
        Ok(pb)
    }
}

pub struct AlterTableBuilder {
    #[doc(hidden)]
    pub error: Result<()>,
    #[doc(hidden)]
    pub schema: Option<Schema>,
    #[doc(hidden)]
    pub pb: AlterTableRequestPB,
}

impl AlterTableBuilder {

    pub fn new() -> AlterTableBuilder {
        AlterTableBuilder {
            error: Ok(()),
            schema: None,
            pb: AlterTableRequestPB::new(),
        }
    }

    pub fn rename_table<S>(mut self, new_name: S) -> AlterTableBuilder where S: Into<String> {
        self.rename_table_by_ref(new_name.into());
        self
    }

    pub fn rename_table_by_ref<S>(&mut self, new_name: S) -> &mut AlterTableBuilder where S: Into<String> {
        self.pb.set_new_table_name(new_name.into());
        self
    }

    pub fn add_column(mut self, column: Column) -> AlterTableBuilder {
        self.add_column_by_ref(column);
        self
    }

    pub fn add_column_by_ref(&mut self, column: Column) -> &mut AlterTableBuilder {
        {
            let mut step = self.pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::ADD_COLUMN);
            step.mut_add_column().set_schema(column.to_pb(false));
        }
        self
    }

    pub fn drop_column<S>(mut self, column: S) -> AlterTableBuilder where S: Into<String> {
        self.drop_column_by_ref(column.into());
        self
    }

    pub fn drop_column_by_ref<S>(&mut self, column: S) -> &mut AlterTableBuilder where S: Into<String> {
        {
            let mut step = self.pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::DROP_COLUMN);
            step.mut_drop_column().set_name(column.into());
        }
        self
    }

    pub fn rename_column<S1, S2>(mut self, old_name: S1, new_name: S2) -> AlterTableBuilder
    where S1: Into<String>,
          S2: Into<String> {
        self.rename_column_by_ref(old_name.into(), new_name.into());
        self
    }

    pub fn rename_column_by_ref<S1, S2>(&mut self, old_name: S1, new_name: S2) -> &mut AlterTableBuilder
    where S1: Into<String>,
          S2: Into<String> {
        {
            let mut step = self.pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::RENAME_COLUMN);
            step.mut_rename_column().set_old_name(old_name.into());
            step.mut_rename_column().set_new_name(new_name.into());
        }
        self
    }

    fn check_and_set_schema(&mut self, new_schema: &Schema) {
        if self.error.is_err() { return; }

        if let Some(ref schema) = self.schema {
            if schema != new_schema {
                self.error = Err(Error::InvalidArgument(
                        "schemas of range partition bounds must match".to_string()));
            }
            return;
        }

        self.pb.set_schema(new_schema.as_pb());
        self.schema = Some(new_schema.clone());
    }

    pub fn add_range_partition(mut self,
                               lower_bound: &RangePartitionBound,
                               upper_bound: &RangePartitionBound) -> AlterTableBuilder {
        self.add_range_partition_by_ref(lower_bound, upper_bound);
        self
    }

    pub fn add_range_partition_by_ref(&mut self,
                                      lower_bound: &RangePartitionBound,
                                      upper_bound: &RangePartitionBound) -> &mut AlterTableBuilder {
        self.check_and_set_schema(lower_bound.row().schema());
        self.check_and_set_schema(upper_bound.row().schema());
        if self.error.is_ok() {
            let mut step = self.pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::ADD_RANGE_PARTITION);

            let mut encoder = OperationEncoder::new();
            encoder.encode_range_partition(lower_bound, upper_bound);
            let (data, indirect_data) = encoder.unwrap();
            step.mut_add_range_partition().mut_range_bounds().set_rows(data);
            step.mut_add_range_partition().mut_range_bounds().set_indirect_data(indirect_data);
        }
        self
    }

    pub fn drop_range_partition(mut self,
                                lower_bound: &RangePartitionBound,
                                upper_bound: &RangePartitionBound) -> AlterTableBuilder {
        self.drop_range_partition_by_ref(lower_bound, upper_bound);
        self
    }

    pub fn drop_range_partition_by_ref(&mut self,
                                       lower_bound: &RangePartitionBound,
                                       upper_bound: &RangePartitionBound)
                                       -> &mut AlterTableBuilder {
        self.check_and_set_schema(lower_bound.row().schema());
        self.check_and_set_schema(upper_bound.row().schema());
        if self.error.is_ok() {
            let mut step = self.pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::DROP_RANGE_PARTITION);

            let mut encoder = OperationEncoder::new();
            encoder.encode_range_partition(lower_bound, upper_bound);
            let (data, indirect_data) = encoder.unwrap();
            step.mut_drop_range_partition().mut_range_bounds().set_rows(data);
            step.mut_drop_range_partition().mut_range_bounds().set_indirect_data(indirect_data);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use std::time::{Duration, Instant};

    use env_logger;

    use Client;
    use ClientConfig;
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
    fn list_tablets() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_masters(1)
                                                         .num_tservers(3));
        let client = Client::new(ClientConfig::new(cluster.master_addrs().to_owned()));

        // The tablet server is real slow coming up.
        // TODO: add MiniCluster::wait_for_startup() or equivalent.
        ::std::thread::sleep_ms(2000);

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

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline() + Duration::from_secs(10)).unwrap();
        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        let tablets = table.list_tablets(deadline()).unwrap();

        assert_eq!(8, tablets.len());
    }
}
