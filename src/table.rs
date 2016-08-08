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

use Column;
use Error;
use master::MasterProxy;
use meta_cache::{Entry, MetaCache};
use partition::PartitionSchema;
use Result;
use row::OperationEncoder;
use row::Row;
use Schema;
use TableId;
use Tablet;

#[derive(Clone)]
pub struct Table {
    name: String,
    id: TableId,
    schema: Schema,
    partition_schema: PartitionSchema,
    num_replicas: u32,
    master_proxy: MasterProxy,
    meta_cache: MetaCache,
}

impl Table {

    #[doc(hidden)]
    pub fn new(name: String,
               id: TableId,
               schema: Schema,
               partition_schema: PartitionSchema,
               num_replicas: u32,
               master_proxy: MasterProxy,
               meta_cache: MetaCache) -> Table {
        Table {
            name: name,
            id: id,
            schema: schema,
            partition_schema: partition_schema,
            num_replicas: num_replicas,
            master_proxy: master_proxy,
            meta_cache: meta_cache,
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

    pub fn list_tablets(&self, deadline: Instant) -> Result<Vec<Tablet>> {
        let mut tablets = Vec::new();
        let (send, recv) = sync_channel(1);
        let mut last_partition_key = Vec::new();

        loop {
            let send = send.clone();
            self.meta_cache.get(&last_partition_key, deadline,
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
}

pub struct TableBuilder {
    name: String,
    schema: Schema,
    range_partition_columns: Vec<String>,
    range_splits: Vec<Row>,
    range_bounds: Vec<(Row, Row)>,
    hash_partitions: Vec<(Vec<String>, i32, Option<u32>)>,
    num_replicas: Option<i32>,
}

impl TableBuilder {

    pub fn new<S>(name: S, schema: Schema) -> TableBuilder where S: Into<String> {
        TableBuilder {
            name: name.into(),
            schema: schema,
            range_partition_columns: Vec::new(),
            range_splits: Vec::new(),
            range_bounds: Vec::new(),
            hash_partitions: Vec::new(),
            num_replicas: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn set_range_partition_columns<S>(&mut self, columns: Vec<S>) -> &mut TableBuilder
    where S: Into<String> {
        self.range_partition_columns = columns.into_iter().map(Into::into).collect();
        self
    }

    pub fn clear_range_partition_columns(&mut self) -> &mut TableBuilder {
        self.range_partition_columns.clear();
        self
    }

    pub fn add_range_split(&mut self, split_row: Row) -> &mut TableBuilder {
        self.range_splits.push(split_row);
        self
    }

    pub fn add_range_bound(&mut self, lower_bound: Row, upper_bound: Row) -> &mut TableBuilder {
        self.range_bounds.push((lower_bound, upper_bound));
        self
    }

    pub fn clear_range_splits(&mut self) -> &mut TableBuilder {
        self.range_splits.clear();
        self
    }

    pub fn clear_range_bounds(&mut self) -> &mut TableBuilder {
        self.range_bounds.clear();
        self
    }

    pub fn add_hash_partition<S>(&mut self, columns: Vec<S>, num_buckets: i32) -> &mut TableBuilder
    where S: Into<String> {
        self.add_hash_partition_with_seed(columns, num_buckets, 0)
    }

    pub fn add_hash_partition_with_seed<S>(&mut self,
                                           columns: Vec<S>,
                                           num_buckets: i32,
                                           seed: u32)
                                           -> &mut TableBuilder
    where S: Into<String> {
        let columns = columns.into_iter().map(Into::into).collect();
        self.hash_partitions.push((columns, num_buckets, Some(seed)));
        self
    }

    pub fn clear_hash_partitions(&mut self) -> &mut TableBuilder {
        self.hash_partitions.clear();
        self
    }

    pub fn set_num_replicas(&mut self, num_replicas: i32) {
        self.num_replicas = Some(num_replicas);
    }

    #[doc(hidden)]
    pub fn into_pb(self) -> Result<CreateTableRequestPB> {
        let TableBuilder { name, schema, range_partition_columns, range_splits,
                           range_bounds, hash_partitions, num_replicas } = self;

        let mut range_encoder = OperationEncoder::new();

        for split in range_splits {
            if !split.schema().ref_eq(&schema) {
                return Err(Error::InvalidArgument(
                        format!("schema of range split row {:?} does not match the table schema {:?}",
                                split, schema)));
            }
            range_encoder.encode_range_split(&split);
        }

        for (lower, upper) in range_bounds {
            if !lower.schema().ref_eq(&schema) {
                return Err(Error::InvalidArgument(
                        format!("schema of range lower bound row {:?} does not match the table schema {:?}",
                                lower, schema)));
            }
            if !upper.schema().ref_eq(&schema) {
                return Err(Error::InvalidArgument(
                        format!("schema of range upper bound row {:?} does not match the table schema {:?}",
                                upper, schema)));
            }
            range_encoder.encode_range_bound(&lower, &upper);
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

        for (columns, num_buckets, seed) in hash_partitions {
            let mut hash_pb = HashBucketSchemaPB::new();
            for column in columns {
                let mut column_pb = ColumnIdentifierPB::new();
                column_pb.set_name(column);
                hash_pb.mut_columns().push(column_pb);
            }
            hash_pb.set_num_buckets(num_buckets);
            if let Some(seed) = seed { hash_pb.set_seed(seed); }
            pb.mut_partition_schema().mut_hash_bucket_schemas().push(hash_pb);
        }

        if let Some(num_replicas) = num_replicas { pb.set_num_replicas(num_replicas); }
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

    pub fn add_range_partition(mut self, lower_bound: &Row, upper_bound: &Row) -> AlterTableBuilder {
        self.add_range_partition_by_ref(lower_bound, upper_bound);
        self
    }

    pub fn add_range_partition_by_ref(&mut self, lower_bound: &Row, upper_bound: &Row) -> &mut AlterTableBuilder {
        fn inner(schema: &mut Option<Schema>, pb: &mut AlterTableRequestPB, lower_bound: &Row, upper_bound: &Row) -> Result<()> {
            let schema_error =
                || Error::InvalidArgument("schemas of range partition bounds must match".to_string());

            if let Some(ref schema) = *schema {
                if schema != lower_bound.schema() || schema != upper_bound.schema() {
                    return Err(schema_error());
                }
            } else if lower_bound.schema() != upper_bound.schema() {
                return Err(schema_error());
            } else {
                pb.set_schema(lower_bound.schema().as_pb());
                *schema = Some(lower_bound.schema().clone());
            }

            let mut step = pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::ADD_RANGE_PARTITION);

            let mut encoder = OperationEncoder::new();
            encoder.encode_range_bound(lower_bound, upper_bound);
            let (data, indirect_data) = encoder.unwrap();
            step.mut_add_range_partition().mut_range_bounds().set_rows(data);
            step.mut_add_range_partition().mut_range_bounds().set_indirect_data(indirect_data);
            Ok(())
        }

        {
            let AlterTableBuilder { ref mut error, ref mut schema, ref mut pb } = *self;
            if error.is_ok() {
                *error = inner(schema, pb, lower_bound, upper_bound);
            }
        }
        self
    }

    pub fn drop_range_partition(mut self, lower_bound: &Row, upper_bound: &Row) -> AlterTableBuilder {
        self.drop_range_partition_by_ref(lower_bound, upper_bound);
        self
    }

    pub fn drop_range_partition_by_ref(&mut self, lower_bound: &Row, upper_bound: &Row) -> &mut AlterTableBuilder {
        fn inner(schema: &mut Option<Schema>, pb: &mut AlterTableRequestPB, lower_bound: &Row, upper_bound: &Row) -> Result<()> {
            let schema_error =
                || Error::InvalidArgument("schemas of range partition bounds must match".to_string());

            if let Some(ref schema) = *schema {
                if schema != lower_bound.schema() || schema != upper_bound.schema() {
                    return Err(schema_error());
                }
            } else if lower_bound.schema() != upper_bound.schema() {
                return Err(schema_error());
            } else {
                pb.set_schema(lower_bound.schema().as_pb());
                *schema = Some(lower_bound.schema().clone());
            }

            let mut step = pb.mut_alter_schema_steps().push_default();
            step.set_field_type(StepType::DROP_RANGE_PARTITION);

            let mut encoder = OperationEncoder::new();
            encoder.encode_range_bound(lower_bound, upper_bound);
            let (data, indirect_data) = encoder.unwrap();
            step.mut_drop_range_partition().mut_range_bounds().set_rows(data);
            step.mut_drop_range_partition().mut_range_bounds().set_indirect_data(indirect_data);
            Ok(())
        }

        {
            let AlterTableBuilder { ref mut error, ref mut schema, ref mut pb } = *self;
            if error.is_ok() {
                *error = inner(schema, pb, lower_bound, upper_bound);
            }
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

        table_builder.add_range_split(split_row);
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
        table_builder.add_hash_partition(vec!["key"], 4);
        table_builder.set_num_replicas(3);

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();
        lower_bound.set(0, 0i32).unwrap();
        upper_bound.set(0, 100i32).unwrap();
        table_builder.add_range_bound(lower_bound, upper_bound);

        let mut lower_bound = schema.new_row();
        let mut upper_bound = schema.new_row();
        lower_bound.set(0, 200i32).unwrap();
        upper_bound.set(0, 300i32).unwrap();
        table_builder.add_range_bound(lower_bound, upper_bound);

        let table_id = client.create_table(table_builder, deadline()).unwrap();
        client.wait_for_table_creation_by_id(&table_id, deadline() + Duration::from_secs(10)).unwrap();
        let table = client.open_table_by_id(&table_id, deadline()).unwrap();

        let tablets = table.list_tablets(deadline()).unwrap();

        assert_eq!(8, tablets.len());
    }
}
