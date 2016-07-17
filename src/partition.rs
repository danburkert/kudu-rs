use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use std::fmt;
use std::cmp;

use byteorder::{BigEndian, ByteOrder};
use kudu_pb::common::{
    PartitionPB,
    PartitionSchemaPB,
    PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
    SchemaPB,
};

use DataType;
use Result;
use Row;
use Schema;
use key::decode_range_partition_key;
use util;

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
    buckets: u32,
    seed: u32,
}

impl HashPartitionSchema {
    fn new(columns: Vec<usize>, buckets: u32, seed: u32) -> HashPartitionSchema {
        HashPartitionSchema {
            columns: columns,
            buckets: buckets,
            seed: seed,
        }
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    pub fn num_buckets(&self) -> u32 {
        self.buckets
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Inner {
    range_partition: RangePartitionSchema,
    hash_partitions: Vec<HashPartitionSchema>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionSchema {
    inner: Arc<Inner>,
}

impl PartitionSchema {

    pub fn range_partition_schema(&self) -> &RangePartitionSchema {
        &self.inner.range_partition
    }

    pub fn hash_partition_schemas(&self) -> &[HashPartitionSchema] {
        &self.inner.hash_partitions
    }

    #[doc(hidden)]
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
            hash_partitions.push(HashPartitionSchema::new(columns,
                                                          hash.get_num_buckets() as u32,
                                                          hash.get_seed()));
        }

        PartitionSchema {
            inner: Arc::new(Inner {
                range_partition: RangePartitionSchema::new(range_columns),
                hash_partitions: hash_partitions,
            })
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PartitionKey {
    hash_buckets: Vec<u32>,
    row: Row,
    partition_schema: PartitionSchema,
}

impl PartitionKey {

    pub fn hash_buckets(&self) -> &[u32] {
        &self.hash_buckets
    }

    pub fn range_row(&self) -> &Row {
        &self.row
    }

    pub fn partition_schema(&self) -> &PartitionSchema {
        &self.partition_schema
    }

    fn from_encoded(schema: &Schema,
                    partition_schema: &PartitionSchema,
                    mut encoded_key: &[u8]) -> Result<PartitionKey> {

        let mut hash_buckets = Vec::with_capacity(partition_schema.hash_partition_schemas().len());
        for _ in partition_schema.hash_partition_schemas() {
            if encoded_key.is_empty() { break; }
            hash_buckets.push(BigEndian::read_u32(encoded_key));
            encoded_key = &encoded_key[4..];
        }

        let range_row = try!(decode_range_partition_key(schema,
                                                        partition_schema.range_partition_schema(),
                                                        encoded_key));

        Ok(PartitionKey {
            hash_buckets: hash_buckets,
            row: range_row,
            partition_schema: partition_schema.clone(),
        })
    }
}

impl fmt::Debug for PartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;

        for bucket in &self.hash_buckets {
            if is_first { is_first = false; }
            else { try!(write!(f, ", ")); }
            try!(write!(f, "bucket={}", bucket));
        }

        let row = &self.row;
        for &idx in self.partition_schema().range_partition_schema().columns() {
            if !row.is_set(idx).unwrap() { break; }

            if is_first { is_first = false; }
            else { try!(write!(f, ", ")) }

            let column = &row.schema().columns()[idx];

            try!(write!(f, "{:?}=", column.name()));
            match column.data_type() {
                DataType::Bool => try!(write!(f, "{}", row.get::<bool>(idx).unwrap())),
                DataType::Int8 => try!(write!(f, "{}", row.get::<i8>(idx).unwrap())),
                DataType::Int16 => try!(write!(f, "{}", row.get::<i16>(idx).unwrap())),
                DataType::Int32 => try!(write!(f, "{}", row.get::<i32>(idx).unwrap())),
                DataType::Int64 => try!(write!(f, "{}", row.get::<i64>(idx).unwrap())),
                DataType::Timestamp => try!(util::fmt_timestamp(f, row.get::<SystemTime>(idx).unwrap())),
                DataType::Float => try!(write!(f, "{}", row.get::<f32>(idx).unwrap())),
                DataType::Double => try!(write!(f, "{}", row.get::<f64>(idx).unwrap())),
                DataType::Binary => try!(util::fmt_hex(f, row.get::<&[u8]>(idx).unwrap())),
                DataType::String => try!(write!(f, "{:?}", row.get::<&str>(idx).unwrap())),
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Partition {
    lower_bound_encoded: Vec<u8>,
    upper_bound_encoded: Vec<u8>,
    lower_bound: PartitionKey,
    upper_bound: PartitionKey,
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({:?}), ({:?}))", &self.lower_bound, &self.upper_bound)
    }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.lower_bound_encoded == other.lower_bound_encoded &&
            self.upper_bound_encoded == other.upper_bound_encoded &&
            self.lower_bound.partition_schema == other.lower_bound.partition_schema
    }
}

impl cmp::Eq for Partition { }

impl Partition {

    pub fn lower_bound(&self) -> &PartitionKey {
        &self.lower_bound
    }

    pub fn upper_bound(&self) -> &PartitionKey {
        &self.upper_bound
    }

    pub fn lower_bound_encoded(&self) -> &[u8] {
        &self.lower_bound_encoded
    }

    pub fn upper_bound_encoded(&self) -> &[u8] {
        &self.upper_bound_encoded
    }

    pub fn from_pb(primary_key_schema: &Schema,
                   partition_schema: &PartitionSchema,
                   mut partition: PartitionPB)
                   -> Result<Partition> {
        let lower_bound = try!(PartitionKey::from_encoded(primary_key_schema,
                                                          partition_schema,
                                                          partition.get_partition_key_start()));
        let upper_bound = try!(PartitionKey::from_encoded(primary_key_schema,
                                                          partition_schema,
                                                          partition.get_partition_key_end()));

        Ok(Partition {
            lower_bound_encoded: partition.take_partition_key_start(),
            upper_bound_encoded: partition.take_partition_key_end(),
            lower_bound: lower_bound,
            upper_bound: upper_bound,
        })
    }
}
