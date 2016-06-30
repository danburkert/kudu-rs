use std::collections::HashMap;

use kudu_pb::common::{
    PartitionSchemaPB,
    PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
    SchemaPB,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangePartitionSchema {
    columns: Vec<usize>,
}

impl RangePartitionSchema {
    fn new(columns: Vec<usize>) -> RangePartitionSchema {
        RangePartitionSchema { columns: columns }
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashPartitionSchema {
    columns: Vec<usize>,
    buckets: i32,
    seed: u32,
}

impl HashPartitionSchema {
    fn new(columns: Vec<usize>, buckets: i32, seed: u32) -> HashPartitionSchema {
        HashPartitionSchema {
            columns: columns,
            buckets: buckets,
            seed: seed,
        }
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    pub fn num_buckets(&self) -> i32 {
        self.buckets
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionSchema {
    range_partition: RangePartitionSchema,
    hash_partitions: Vec<HashPartitionSchema>,
}

impl PartitionSchema {
    pub fn from_pb(pb: &PartitionSchemaPB, schema: &SchemaPB) -> PartitionSchema {
        let mut columns_by_name = HashMap::new();
        let mut columns_by_id = HashMap::new();

        for (idx, column) in schema.get_columns().iter().enumerate() {
            assert!(column.has_name());
            assert!(column.has_id());
            columns_by_name.insert(column.get_name().to_owned(), idx);
            columns_by_id.insert(column.get_id(), idx);
        }

        let column_to_id = |column: &ColumnIdentifierPB| {
            if column.has_id() {
                columns_by_id[&(column.get_id() as u32)]
            } else if column.has_name() {
                columns_by_name[column.get_name()]
            } else {
                panic!("column identifier must have either a name or ID");
            }
        };

        let range_columns = pb.get_range_schema().get_columns()
                              .iter().map(&column_to_id).collect::<Vec<_>>();

        let mut hash_partitions = Vec::with_capacity(pb.get_hash_bucket_schemas().len());
        for hash in pb.get_hash_bucket_schemas().iter() {
            let columns = hash.get_columns().iter().map(&column_to_id).collect::<Vec<_>>();
            hash_partitions.push(HashPartitionSchema::new(columns, hash.get_num_buckets(), hash.get_seed()));
        }

        PartitionSchema {
            range_partition: RangePartitionSchema::new(range_columns),
            hash_partitions: hash_partitions,
        }
    }

    pub fn range_partition_schema(&self) -> &RangePartitionSchema {
        &self.range_partition
    }

    pub fn hash_partition_schemas(&self) -> &[HashPartitionSchema] {
        &self.hash_partitions
    }
}
