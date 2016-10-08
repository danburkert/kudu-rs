use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use std::cmp;

use byteorder::{BigEndian, ByteOrder};
use kudu_pb::common::{
    PartitionPB,
    PartitionSchemaPB,
    PartitionSchemaPB_ColumnIdentifierPB as ColumnIdentifierPB,
    SchemaPB,
};

use Error;
use Result;
use Row;
use Schema;
use key;
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

/*
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
*/

#[derive(Clone)]
pub struct Partition {
    partition_schema: PartitionSchema,
    lower_bound_key: Vec<u8>,
    upper_bound_key: Vec<u8>,
    hash_partitions: Vec<u32>,
    range_lower_bound: Row,
    range_upper_bound: Row,
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({:?}), ({:?}))", &self.lower_bound_key, &self.upper_bound_key)
    }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        &self.lower_bound_key == &other.lower_bound_key &&
            &self.upper_bound_key == &other.upper_bound_key
    }
}

impl cmp::Eq for Partition { }

impl Partition {

    pub fn lower_bound_key(&self) -> &[u8] {
        &self.lower_bound_key
    }

    pub fn upper_bound_key(&self) -> &[u8] {
        &self.upper_bound_key
    }

    pub fn range_lower_bound(&self) -> &Row {
        &self.range_lower_bound
    }

    pub fn range_upper_bound(&self) -> &Row {
        &self.range_upper_bound
    }

    pub fn hash_partitions(&self) -> &[u32] {
        &self.hash_partitions
    }

    /// Formats the range partition.
    ///
    /// VALUES = 123
    /// VALUES = (123, 456)
    /// 123 <= VALUES < 999
    /// (123, 456) <= VALUES < (789, 102)
    pub fn fmt_range_partition(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let idxs = &self.partition_schema.range_partition_schema().columns;

        let lower_bound_idxs = idxs.iter().take_while(|&idx| self.range_lower_bound.is_set(*idx).unwrap()).count();
        let upper_bound_idxs = idxs.iter().take_while(|&idx| self.range_upper_bound.is_set(*idx).unwrap()).count();

        match (lower_bound_idxs, upper_bound_idxs) {
            (0, 0) => write!(f, "UNBOUNDED"),
            (0, count) => {
                try!(write!(f, "VALUES < "));
                fmt_row(f, &self.range_upper_bound, &idxs[..count])
            },
            (count, 0) => {
                try!(write!(f, "VALUES >= "));
                fmt_row(f, &self.range_lower_bound, &idxs[..count])
            },
            _ => {
                if key::is_row_incremented(&self.range_lower_bound, &self.range_upper_bound, idxs) {
                    try!(write!(f, "VALUES = "));
                    fmt_row(f, &self.range_lower_bound, idxs)
                } else {
                    try!(fmt_row(f, &self.range_lower_bound, idxs));
                    try!(write!(f, " <= VALUES < "));
                    fmt_row(f, &self.range_upper_bound, idxs)
                }
            }
        }
    }

    #[doc(hidden)]
    pub fn from_pb(primary_key_schema: &Schema,
                   partition_schema: PartitionSchema,
                   mut partition: PartitionPB)
                   -> Result<Partition> {
        let lower_bound_key = partition.take_partition_key_start();
        let upper_bound_key = partition.take_partition_key_end();

        let hash_partition_levels = partition_schema.hash_partition_schemas().len();
        let mut hash_partitions = Vec::with_capacity(hash_partition_levels);
        {
            let mut key = &lower_bound_key[..];
            for _ in partition_schema.hash_partition_schemas() {
                if key.is_empty() {
                    hash_partitions.push(0)
                } else if key.len() < 4 {
                    return Err(Error::Serialization(format!("invalid lower bound partition key: {:?}",
                                                            lower_bound_key)));
                } else {
                    hash_partitions.push(BigEndian::read_u32(key));
                    key = &key[4..];
                }
            }
        }

        let range_lower_bound = if lower_bound_key.len() > hash_partition_levels * 4 {
            try!(key::decode_range_partition_key(primary_key_schema,
                                                 partition_schema.range_partition_schema(),
                                                 &lower_bound_key[hash_partition_levels * 4..]))
        } else {
            primary_key_schema.new_row()
        };
        let range_upper_bound = if upper_bound_key.len() > hash_partition_levels * 4 {
            try!(key::decode_range_partition_key(primary_key_schema,
                                                 partition_schema.range_partition_schema(),
                                                 &upper_bound_key[hash_partition_levels * 4..]))
        } else {
            primary_key_schema.new_row()
        };

        Ok(Partition {
            partition_schema: partition_schema,
            lower_bound_key: lower_bound_key,
            upper_bound_key: upper_bound_key,
            hash_partitions: hash_partitions,
            range_lower_bound: range_lower_bound,
            range_upper_bound: range_upper_bound,
        })
    }
}

fn fmt_row(f: &mut fmt::Formatter, row: &Row, idxs: &[usize]) -> fmt::Result {
    debug_assert!(!idxs.is_empty());

    if idxs.len() == 1 {
        return util::fmt_cell(f, row, idxs[0]);
    }

    try!(write!(f, "("));
    let mut is_first = true;
    for &idx in idxs {
        if is_first { is_first = false; }
        else { try!(write!(f, ", ")) }
        try!(util::fmt_cell(f, row, idx));
    }
    write!(f, ")")
}
