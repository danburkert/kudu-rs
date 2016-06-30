
use kudu_pb::master::CreateTableRequestPB;
use kudu_pb::common::{PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
                      PartitionSchemaPB_HashBucketSchemaPB as HashBucketSchemaPB};

use Error;
use meta_cache::MetaCache;
use partition::PartitionSchema;
use Result;
use row::OperationEncoder;
use row::Row;
use Schema;
use TableId;

pub struct Table {
    name: String,
    id: TableId,
    schema: Schema,
    partition_schema: PartitionSchema,
    meta_cache: MetaCache,
}

impl Table {
    pub fn new(name: String,
               id: TableId,
               schema: Schema,
               partition_schema: PartitionSchema,
               meta_cache: MetaCache) -> Table {
        Table {
            name: name,
            id: id,
            schema: schema,
            partition_schema: partition_schema,
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

    pub fn set_range_partition_columns(&mut self, columns: Vec<String>) -> &mut TableBuilder {
        self.range_partition_columns = columns;
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

    pub fn add_hash_partitions(&mut self, columns: Vec<String>, num_buckets: i32) -> &mut TableBuilder {
        self.hash_partitions.push((columns, num_buckets, None));
        self
    }

    pub fn add_hash_partitions_with_seed(&mut self,
                                         columns: Vec<String>,
                                         num_buckets: i32,
                                         seed: u32) -> &mut TableBuilder {
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

#[cfg(test)]
mod tests {

    use env_logger;

    use schema::tests::simple_schema;
    use super::*;

    #[test]
    fn create_table_builder() {
        let _ = env_logger::init();


        let val = String::from("foo");

        let key_val: &str = &val;

        let mut table_builder = TableBuilder::new("t", simple_schema());

        let mut split_row = table_builder.schema().new_row();
        split_row.set_by_name("key", key_val).unwrap();

        table_builder.add_range_split(split_row);
    }
}
