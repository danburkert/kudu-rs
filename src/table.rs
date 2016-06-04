use kudu_pb::master::CreateTableRequestPB;
use kudu_pb::common::{PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
                      PartitionSchemaPB_HashBucketSchemaPB as HashBucketSchemaPB};

use Schema;
use row::OperationEncoder;

use row::Row;

pub struct TableBuilder {
    name: String,
    schema: Schema,
    range_partition_columns: Vec<String>,
    range_encoder: OperationEncoder,
    hash_partitions: Vec<(Vec<String>, i32, Option<u32>)>,
    num_replicas: Option<i32>,
}

impl TableBuilder {

    pub fn new(name: String, schema: Schema) -> TableBuilder {
        TableBuilder {
            name: name,
            schema: schema,
            range_partition_columns: Vec::new(),
            range_encoder: OperationEncoder::new(),
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

    pub fn add_range_split<F>(&mut self, f: F) -> &mut TableBuilder where F: FnOnce(&mut Row) {
        {
            let mut row = Row::new(&self.schema);
            f(&mut row);
            self.range_encoder.encode_range_split(row);
        }
        self
    }

    pub fn add_range_bound<F>(&mut self, f: F) -> &mut TableBuilder where F: FnOnce(&mut Row, &mut Row) {
        {
            let mut lower = Row::new(&self.schema);
            let mut upper = Row::new(&self.schema);
            f(&mut lower, &mut upper);
            self.range_encoder.encode_range_bound(lower, upper);
        }
        self
    }

    pub fn clear_range_splits_and_bounds(&mut self) -> &mut TableBuilder {
        self.range_encoder.clear();
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

    pub fn into_pb(self) -> CreateTableRequestPB {
        let TableBuilder { name, schema, range_partition_columns,
                           range_encoder, hash_partitions, num_replicas } = self;
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
        pb
    }
}