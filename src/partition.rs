use std::borrow;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use std::sync::Arc;

use byteorder::{BigEndian, ByteOrder};
use pb::partition_schema_pb::column_identifier_pb::Identifier;
use pb::partition_schema_pb::ColumnIdentifierPb;
use pb::{PartitionPb, PartitionSchemaPb, SchemaPb};

use key;
use util;
use Error;
use Result;
use Row;
use Schema;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangePartitionSchema {
    columns: Box<[usize]>,
}

impl RangePartitionSchema {
    fn new(columns: Vec<usize>) -> RangePartitionSchema {
        RangePartitionSchema {
            columns: columns.into_boxed_slice(),
        }
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashPartitionSchema {
    columns: Box<[usize]>,
    buckets: u32,
    seed: u32,
}

impl HashPartitionSchema {
    fn new(columns: Vec<usize>, buckets: u32, seed: u32) -> HashPartitionSchema {
        HashPartitionSchema {
            columns: columns.into_boxed_slice(),
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

#[derive(Debug, PartialEq, Eq)]
struct Inner {
    range_partition: RangePartitionSchema,
    hash_partitions: Box<[HashPartitionSchema]>,
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

    pub(crate) fn from_pb(pb: &PartitionSchemaPb, schema: &SchemaPb) -> PartitionSchema {
        let mut columns_by_name = HashMap::new();
        let mut columns_by_id = HashMap::new();

        for (idx, column) in schema.columns.iter().enumerate() {
            assert!(column.id.is_some());
            columns_by_name.insert(column.name.clone(), idx);
            columns_by_id.insert(column.id(), idx);
        }

        let column_to_id = |column: &ColumnIdentifierPb| match *column
            .identifier
            .as_ref()
            .expect("column identifier must have either a name or ID")
        {
            Identifier::Id(id) => columns_by_id[&(id as u32)],
            Identifier::Name(ref name) => columns_by_name[name],
        };

        let range_columns = if let Some(range_schema) = pb.range_schema.as_ref() {
            range_schema
                .columns
                .iter()
                .map(&column_to_id)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let mut hash_partitions = Vec::with_capacity(pb.hash_bucket_schemas.len());
        for hash in &pb.hash_bucket_schemas {
            let columns = hash.columns.iter().map(&column_to_id).collect::<Vec<_>>();
            hash_partitions.push(HashPartitionSchema::new(
                columns,
                hash.num_buckets as u32,
                hash.seed(),
            ));
        }

        PartitionSchema {
            inner: Arc::new(Inner {
                range_partition: RangePartitionSchema::new(range_columns),
                hash_partitions: hash_partitions.into_boxed_slice(),
            }),
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) struct PartitionKey(Box<[u8]>);

impl PartitionKey {
    pub fn empty() -> PartitionKey {
        Vec::new().into()
    }
}

impl ops::Deref for PartitionKey {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.0
    }
}

impl AsRef<[u8]> for PartitionKey {
    fn as_ref(&self) -> &[u8] {
        &*self.0
    }
}

impl Into<PartitionKey> for Vec<u8> {
    fn into(self) -> PartitionKey {
        PartitionKey(self.into_boxed_slice())
    }
}

impl Into<PartitionKey> for Box<[u8]> {
    fn into(self) -> PartitionKey {
        PartitionKey(self)
    }
}

impl borrow::Borrow<[u8]> for PartitionKey {
    fn borrow(&self) -> &[u8] {
        &*self.0
    }
}

pub(crate) trait IntoPartitionKey {
    fn into_partition_key(&self) -> PartitionKey;
}

impl IntoPartitionKey for [u8] {
    fn into_partition_key(&self) -> PartitionKey {
        PartitionKey(self.to_owned().into())
    }
}

#[derive(Clone)]
pub struct Partition {
    partition_schema: PartitionSchema,
    lower_bound: PartitionKey,
    upper_bound: PartitionKey,
    hash_partitions: Vec<u32>,
    range_lower_bound: Row<'static>,
    range_upper_bound: Row<'static>,
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[({:?}), ({:?}))", self.lower_bound, self.upper_bound)
    }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.lower_bound == other.lower_bound && self.upper_bound == other.upper_bound
    }
}

impl cmp::Eq for Partition {}

impl Partition {
    pub fn lower_bound(&self) -> &[u8] {
        &self.lower_bound
    }

    pub fn upper_bound(&self) -> &[u8] {
        &self.upper_bound
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

        let lower_bound_idxs = idxs
            .iter()
            .take_while(|&idx| self.range_lower_bound.is_set(*idx).unwrap())
            .count();
        let upper_bound_idxs = idxs
            .iter()
            .take_while(|&idx| self.range_upper_bound.is_set(*idx).unwrap())
            .count();

        match (lower_bound_idxs, upper_bound_idxs) {
            (0, 0) => write!(f, "UNBOUNDED"),
            (0, count) => {
                try!(write!(f, "VALUES < "));
                fmt_row(f, &self.range_upper_bound, &idxs[..count])
            }
            (count, 0) => {
                try!(write!(f, "VALUES >= "));
                fmt_row(f, &self.range_lower_bound, &idxs[..count])
            }
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

    pub(crate) fn from_bounds(
        primary_key_schema: &Schema,
        partition_schema: PartitionSchema,
        lower_bound: PartitionKey,
        upper_bound: PartitionKey,
    ) -> Result<Partition> {
        let hash_partition_levels = partition_schema.hash_partition_schemas().len();
        let mut hash_partitions = Vec::with_capacity(hash_partition_levels);
        {
            let mut key = &lower_bound[..];
            for _ in partition_schema.hash_partition_schemas() {
                if key.is_empty() {
                    hash_partitions.push(0)
                } else if key.len() < 4 {
                    return Err(Error::Serialization(format!(
                        "invalid lower bound partition key: {:?}",
                        lower_bound
                    )));
                } else {
                    hash_partitions.push(BigEndian::read_u32(key));
                    key = &key[4..];
                }
            }
        }

        let range_lower_bound = if lower_bound.len() > hash_partition_levels * 4 {
            try!(key::decode_range_partition_key(
                primary_key_schema,
                partition_schema.range_partition_schema(),
                &lower_bound[hash_partition_levels * 4..]
            ))
        } else {
            primary_key_schema.new_row()
        };
        let range_upper_bound = if upper_bound.len() > hash_partition_levels * 4 {
            try!(key::decode_range_partition_key(
                primary_key_schema,
                partition_schema.range_partition_schema(),
                &upper_bound[hash_partition_levels * 4..]
            ))
        } else {
            primary_key_schema.new_row()
        };

        Ok(Partition {
            partition_schema,
            lower_bound,
            upper_bound,
            hash_partitions,
            range_lower_bound,
            range_upper_bound,
        })
    }

    pub(crate) fn from_pb(
        primary_key_schema: &Schema,
        partition_schema: PartitionSchema,
        mut partition: PartitionPb,
    ) -> Result<Partition> {
        let lower_bound = partition
            .partition_key_start
            .take()
            .unwrap()
            .into_partition_key();
        let upper_bound = partition
            .partition_key_end
            .take()
            .unwrap()
            .into_partition_key();
        Partition::from_bounds(
            primary_key_schema,
            partition_schema,
            lower_bound,
            upper_bound,
        )
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
        if is_first {
            is_first = false;
        } else {
            try!(write!(f, ", "))
        }
        try!(util::fmt_cell(f, row, idx));
    }
    write!(f, ")")
}
