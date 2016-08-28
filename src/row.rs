use std::cmp;
use std::fmt;
use std::time::SystemTime;

use byteorder::{ByteOrder, LittleEndian};
use kudu_pb::wire_protocol::{RowOperationsPB_Type as OperationType};

use bit_set::BitSet;
use vec_map::VecMap;
#[cfg(any(feature="quickcheck", test))] use quickcheck;

use DataType;
use Error;
use RangePartitionBound;
use Result;
use Schema;
use Value;
use operation::{Operation, OperationKind};
use util;

#[derive(Clone)]
pub struct Row {
    data: Box<[u8]>,
    indirect_data: VecMap<Vec<u8>>,
    set_columns: BitSet,
    null_columns: BitSet,
    schema: Schema,
}

/// TODO: unset/unset_by_name.  Should zero out existing values so that equality can still be fast.
/// TODO: remove varlen column bytes from `data` (right now it takes up 16 useless bytes) (is this
/// as easy as changing them to 0 width in the Value trait?).
impl Row {
    pub fn new(schema: Schema) -> Row {
        let num_columns = schema.columns().len();
        let null_columns = if schema.has_nullable_columns() { BitSet::with_capacity(num_columns) }
                           else { BitSet::with_capacity(0) };
        let data = vec![0; schema.row_size()].into_boxed_slice();
        Row {
            data: data,
            indirect_data: VecMap::with_capacity(num_columns),
            set_columns: BitSet::with_capacity(num_columns),
            null_columns: null_columns,
            schema: schema,
        }
    }

    pub fn set<'a, V>(&mut self, idx: usize, value: V) -> Result<&mut Row> where V: Value<'a> {
        try!(self.check_column_for_write::<V>(idx));
        unsafe {
            Ok(self.set_unchecked(idx, value))
        }
    }

    pub fn set_by_name<'a, V>(&mut self, column: &str, value: V) -> Result<&mut Row> where V: Value<'a> {
        if let Some(idx) = self.schema.column_index(column) {
            self.set(idx, value)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub unsafe fn set_unchecked<'a, V>(&mut self,
                                       idx: usize,
                                       value: V)
                                       -> &mut Row where V: Value<'a> {
        debug_assert_eq!(Ok(()), self.check_column_for_write::<V>(idx));
        self.set_columns.insert(idx);
        if value.is_null() {
            self.null_columns.insert(idx);
        } else {
            if self.schema.has_nullable_columns() { self.null_columns.remove(idx); }
            if V::is_var_len() {
                self.indirect_data.insert(idx, value.indirect_data());
            } else {
                let offset = self.schema.column_offsets()[idx];
                value.copy_data(&mut self.data[offset..offset+V::size()]);
            }
        }
        self
    }

    pub fn get<'a, V>(&'a self, idx: usize) -> Result<V> where V: Value<'a> {
        try!(self.check_column_for_read::<V>(idx));
        if !self.set_columns.get(idx) {
            Err(Error::InvalidArgument(format!("column '{}' ({}) is not set",
                                               self.schema.columns()[idx].name(), idx)))
        } else if self.schema.has_nullable_columns() && self.null_columns.get(idx) {
            if V::is_nullable() {
                Ok(V::from_null())
            }
            else {
                Err(Error::InvalidArgument(format!("column '{}' ({}) is null",
                                                   self.schema.columns()[idx].name(), idx)))
            }
        } else if V::is_var_len() {
            V::from_data(&self.indirect_data[idx])
        } else {
            let offset = self.schema.column_offsets()[idx];
            let len = V::size();
            V::from_data(&self.data[offset..offset+len])
        }
    }

    pub fn get_by_name<'a, V>(&'a self, column: &str) -> Result<Option<V>> where V: Value<'a> {
        if let Some(idx) = self.schema.column_index(column) {
            self.get(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub fn is_null(&self, idx: usize) -> Result<bool> {
        if idx >= self.schema.columns().len() {
            Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                               idx, self.schema)))
        } else {
            Ok(self.schema.has_nullable_columns() && self.null_columns.get(idx))
        }
    }

    pub fn is_null_by_name(&self, column: &str) -> Result<bool> {
        if let Some(idx) = self.schema.column_index(column) {
            self.is_null(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub fn is_set(&self, idx: usize) -> Result<bool> {
        if idx >= self.schema.columns().len() {
            Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                               idx, self.schema)))
        } else {
            Ok(self.set_columns.get(idx))
        }
    }

    pub fn is_set_by_name(&self, column: &str) -> Result<bool> {
        if let Some(idx) = self.schema.column_index(column) {
            self.is_set(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_write<'a, V>(&self, idx: usize) -> Result<()> where V: Value<'a> {
        if idx >= self.schema.columns().len() {
            return Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                                      idx, self.schema)));
        }
        let column = &self.schema.columns()[idx];
        if !V::can_write_to(column.data_type()) {
            return Err(Error::InvalidArgument(format!("type {:?} is invalid for column {:?}",
                                                      V::data_type(),
                                                      column)));
        }
        if V::is_nullable() && !column.is_nullable() {
            return Err(Error::InvalidArgument(format!("nullable type is invalid for column {:?}",
                                                      column)));
        }
        Ok(())
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_read<'a, V>(&self, idx: usize) -> Result<()> where V: Value<'a> {
        if idx >= self.schema.columns().len() {
            return Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                                      idx, self.schema)));
        }
        let column = &self.schema.columns()[idx];
        if !V::can_read_from(column.data_type()) {
            return Err(Error::InvalidArgument(format!("type {:?} is invalid for column {:?}",
                                                      V::data_type(),
                                                      column)));
        }
        if V::is_nullable() && !column.is_nullable() {
            return Err(Error::InvalidArgument(format!("nullable type is invalid for column {:?}",
                                                      column)));
        }
        Ok(())
    }

    #[cfg(any(feature="quickcheck", test))]
    pub fn arbitrary<G>(g: &mut G, schema: &Schema) -> Row where G: quickcheck::Gen {
        use quickcheck::Arbitrary;
        let mut row = schema.new_row();
        for (idx, column) in schema.columns().iter().enumerate() {
            unsafe {
                // Use set_unchecked since the column type is already checked.
                match column.data_type() {
                    DataType::Bool => row.set_unchecked(idx, bool::arbitrary(g)),
                    DataType::Int8 => row.set_unchecked(idx, i8::arbitrary(g)),
                    DataType::Int16 => row.set_unchecked(idx, i16::arbitrary(g)),
                    DataType::Int32 => row.set_unchecked(idx, i32::arbitrary(g)),
                    DataType::Int64 => row.set_unchecked(idx, i64::arbitrary(g)),
                    DataType::Timestamp => row.set_unchecked(idx, i64::arbitrary(g)),
                    DataType::Float => row.set_unchecked(idx, f32::arbitrary(g)),
                    DataType::Double => row.set_unchecked(idx, f64::arbitrary(g)),
                    DataType::Binary => row.set_unchecked(idx, Vec::arbitrary(g)),
                    DataType::String => row.set_unchecked(idx, String::arbitrary(g)),
                };
            }
        }
        row
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;
        for (idx, column) in self.schema.columns().iter().enumerate() {
            if !self.set_columns.get(idx) { continue; }

            if is_first { is_first = false; }
            else { try!(write!(f, ", ")) }

            try!(write!(f, "{:?} {:?}=", column.data_type(), column.name()));
            if self.is_null(idx).unwrap() {
                try!(write!(f, "NULL"))
            } else {
                match column.data_type() {
                    DataType::Bool => try!(write!(f, "{}", self.get::<bool>(idx).unwrap())),
                    DataType::Int8 => try!(write!(f, "{}", self.get::<i8>(idx).unwrap())),
                    DataType::Int16 => try!(write!(f, "{}", self.get::<i16>(idx).unwrap())),
                    DataType::Int32 => try!(write!(f, "{}", self.get::<i32>(idx).unwrap())),
                    DataType::Int64 => try!(write!(f, "{}", self.get::<i64>(idx).unwrap())),
                    DataType::Timestamp => try!(util::fmt_timestamp(f, self.get::<SystemTime>(idx).unwrap())),
                    DataType::Float => try!(write!(f, "{}", self.get::<f32>(idx).unwrap())),
                    DataType::Double => try!(write!(f, "{}", self.get::<f64>(idx).unwrap())),
                    DataType::Binary => try!(util::fmt_hex(f, self.get::<&[u8]>(idx).unwrap())),
                    DataType::String => try!(write!(f, "{:?}", self.get::<&str>(idx).unwrap())),
                }
            }
        }
        Ok(())
    }
}

impl cmp::PartialEq for Row {
    fn eq(&self, other: &Row) -> bool {
        self.schema == other.schema &&
            self.set_columns == other.set_columns &&
            self.null_columns == other.null_columns &&
            self.data == other.data &&
            self.indirect_data == other.indirect_data
    }
}

impl cmp::Eq for Row {}

/// `Row`s can be compared based on primary key column values. If the schemas do not match or if
/// some of the primary key columns are not set, the ordering is not defined.
impl cmp::PartialOrd for Row {
    fn partial_cmp(&self, other: &Row) -> Option<cmp::Ordering> {
        if self.schema != other.schema { return None; }

        for (idx, column) in self.schema.primary_key().iter().enumerate() {
            if self.set_columns.get(idx) && other.set_columns.get(idx) {
                let ord = match column.data_type() {
                    DataType::Int8 => self.get::<i8>(idx).unwrap().cmp(&other.get::<i8>(idx).unwrap()),
                    DataType::Int16 => self.get::<i16>(idx).unwrap().cmp(&other.get::<i16>(idx).unwrap()),
                    DataType::Int32 => self.get::<i32>(idx).unwrap().cmp(&other.get::<i32>(idx).unwrap()),
                    DataType::Int64 => self.get::<i64>(idx).unwrap().cmp(&other.get::<i64>(idx).unwrap()),
                    DataType::Timestamp => self.get::<SystemTime>(idx).unwrap().cmp(&other.get::<SystemTime>(idx).unwrap()),
                    DataType::Binary => self.get::<&[u8]>(idx).unwrap().cmp(other.get::<&[u8]>(idx).unwrap()),
                    DataType::String => self.get::<&str>(idx).unwrap().cmp(other.get::<&str>(idx).unwrap()),
                    _ => unreachable!("primary key column of type {:?}", column.data_type()),
                };
                if ord != cmp::Ordering::Equal { return Some(ord); }
            } else { return None; }
        }
        Some(cmp::Ordering::Equal)
    }
}

// TODO: move below to own file

pub struct OperationEncoder {
    data: Vec<u8>,
    indirect_data: Vec<u8>,
}

impl OperationEncoder {
    pub fn new() -> OperationEncoder {
        OperationEncoder {
            data: Vec::new(),
            indirect_data: Vec::new(),
        }
    }

    pub fn with_capacity(data: usize, indirect_data: usize) -> OperationEncoder {
        OperationEncoder {
            data: Vec::with_capacity(data),
            indirect_data: Vec::with_capacity(indirect_data),
        }
    }

    pub fn encode(&mut self, operation: &Operation) {
        let op_type = match operation.kind() {
            OperationKind::Insert => OperationType::INSERT,
            OperationKind::Update => OperationType::UPDATE,
            OperationKind::Upsert => OperationType::UPSERT,
            OperationKind::Delete => OperationType::DELETE,
        };

        self.encode_row(op_type, operation.row());
    }

    pub fn encode_range_split(&mut self, row: &Row) {
        self.encode_row(OperationType::SPLIT_ROW, row);
    }

    pub fn encode_range_partition(&mut self, lower: &RangePartitionBound, upper: &RangePartitionBound) {
        let (lower_bound, lower_bound_type) = match *lower {
            RangePartitionBound::Inclusive(ref row) => (row, OperationType::RANGE_LOWER_BOUND),
            RangePartitionBound::Exclusive(ref row) => (row, OperationType::EXCLUSIVE_RANGE_LOWER_BOUND),
        };
        let (upper_bound, upper_bound_type) = match *upper {
            RangePartitionBound::Inclusive(ref row) => (row, OperationType::INCLUSIVE_RANGE_UPPER_BOUND),
            RangePartitionBound::Exclusive(ref row) => (row, OperationType::RANGE_UPPER_BOUND),
        };

        self.encode_row(lower_bound_type, &lower_bound);
        self.encode_row(upper_bound_type, &upper_bound);
    }
    pub fn encode_range_partition_split(&mut self, split: &Row) {
        self.encode_row(OperationType::SPLIT_ROW, split);
    }

    fn encode_row(&mut self, op_type: OperationType, row: &Row) {
        let Row { ref data, ref indirect_data, ref set_columns, ref null_columns, ref schema } = *row;

        self.data.push(op_type as u8);
        self.data.extend_from_slice(set_columns.data());
        self.data.extend_from_slice(null_columns.data());

        let mut offset = self.data.len();
        self.data.resize(offset + schema.row_size(), 0);
        for (idx, column) in schema.columns().iter().enumerate() {
            if !set_columns.get(idx) { continue; }
            if column.is_nullable() && null_columns.get(idx) { continue; }

            let data_type = column.data_type();
            let size = data_type.size();
            if data_type.is_var_len() {
                let data = &indirect_data[idx];
                LittleEndian::write_u64(&mut self.data[offset..], self.indirect_data.len() as u64);
                LittleEndian::write_u64(&mut self.data[offset+8..], data.len() as u64);
                self.indirect_data.extend_from_slice(data);
            } else {
                let column_offset = schema.column_offsets()[idx];
                self.data[offset..offset+size].copy_from_slice(&data[column_offset..column_offset+size]);
            }
            offset += size;
        }
        self.data.truncate(offset);
    }

    pub fn encoded_len(operation: &Operation) -> usize {
        let Row { ref indirect_data, ref set_columns, ref null_columns, ref schema, .. } = *operation.row();

        let mut len = 1; // op type

        len += set_columns.data().len();
        len += null_columns.data().len();
        len += schema.row_size();
        len += indirect_data.values().map(|data| data.len()).sum();

        len
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.indirect_data.clear();
    }

    pub fn unwrap(self) -> (Vec<u8>, Vec<u8>) {
        let OperationEncoder { data, indirect_data } = self;
        (data, indirect_data)
    }
}

#[cfg(test)]
mod tests {

    use quickcheck::{quickcheck, TestResult, StdGen};
    use rand;
    use schema;
    use super::*;

    #[test]
    fn test_partial_row() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set::<i32>(0, 12).unwrap();
        assert_eq!(12, row.get::<i32>(0).unwrap());

        row.set(10, "foo").unwrap();
        assert_eq!("foo".to_owned(), row.get::<String>(10).unwrap());
    }

    #[test]
    fn check_to_string() {

        fn rows_to_string(schema: schema::Schema) -> TestResult {
            let mut g = StdGen::new(rand::thread_rng(), 100);

            for _ in 0..10 {
                let row = Row::arbitrary(&mut g, &schema);
                format!("{:?}", row);
            }

            TestResult::passed()
        }

        quickcheck(rows_to_string as fn(schema::Schema) -> TestResult);
    }
}
