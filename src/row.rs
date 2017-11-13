use std::cmp;
use std::fmt;
use std::time::SystemTime;
use std::borrow::Cow;

use byteorder::{ByteOrder, LittleEndian};

use bit_set::BitSet;
use vec_map::VecMap;
#[cfg(any(feature="quickcheck", test))] use quickcheck;

use pb::RowOperationsPb;
use pb::row_operations_pb::{Type as OperationType};

use DataType;
use Error;
use RangePartitionBound;
use Result;
use Schema;
use Value;
use util;

/// Holds either owned or borrowed row data.
///
/// Similar to `std::borrow::Cow::<[u8]>`, but the borrowed variant holds a mutable reference, and
/// the owned variant holds a boxed slice instead of a vec.
enum Data<'data> {

    /// Borrowed row data.
    ///
    /// The data is laid out contiguously as follows:
    ///   - column-data
    ///   - is-null bitmap, if any columns are nullable
    Borrowed(&'data mut [u8]),

    /// Owned row data.
    ///
    /// The data is laid out contiguously as follows:
    ///   - is-set bitmap
    ///   - is-null bitmap, if any columns are nullable
    ///   - column-data
    Owned(Box<[u8]>),
}

impl <'data> Data<'data> {

    fn clone_borrowed_data(schema: &Schema, borrowed_data: &[u8]) -> Box<[u8]> {
        let row_len = schema.row_size();
        let bitset_len = BitSetT::byte_len(schema.num_columns());
        let data_len = row_len
                     + if schema.has_nullable_columns() { bitset_len } else { 0 }
                     + bitset_len;
        debug_assert_eq!(borrowed_data.len(), data_len - bitset_len);

        let data = Vec::with_capacity(data_len);
        // TODO: this leaves some trailing bits set, is that a problem?
        data.resize(bitset_len, 0xFF);
        if schema.has_nullable_columns() {
            data.extend_from_slice(&borrowed_data[row_len..]);
        }
        data.extend_from_slice(&borrowed_data[..row_len]);
        data.into_boxed_slice()
    }

    fn into_owned(&mut self, schema: &Schema) {
        if let Data::Borrowed(data) = *self {
            *self = Data::Owned(schema, data)
        }
    }
}

pub struct Row<'data> {
    data: Data<'data>,
    owned_data: VecMap<Vec<u8>>,
    schema: Schema,
}

// TODO: unset/unset_by_name.
impl <'data> Row<'data> {

    /// Creates a new owned row.
    pub(crate) fn new(schema: Schema) -> Row<'data> {
        let row_len = schema.row_size();
        let bitset_len = BitSetT::byte_len(schema.num_columns());
        let data_len = row_len
                     + if schema.has_nullable_columns() { bitset_len } else { 0 }
                     + bitset_len;

        let num_columns = schema.columns().len();

        let data = Data::Owned(vec![0; data_len].into_boxed_slice());
        Row {
            data,
            indirect_data: VecMap::new(),
            schema,
        }
    }

    pub fn set<V>(&mut self, idx: usize, value: V) -> Result<&mut Self> where V: Value<'data> {
        self.check_column_for_write::<V>(idx)?;
        unsafe {
            Ok(self.set_unchecked(idx, value))
        }
    }

    pub fn set_by_name<V>(&mut self, column: &str, value: V) -> Result<&mut Row<'data>> where V: Value<'data> {
        if let Some(idx) = self.schema.column_index(column) {
            self.set(idx, value)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub unsafe fn set_unchecked<'a, V>(&mut self,
                                       idx: usize,
                                       value: V)
                                       -> &mut Self where V: Value<'data> {
        debug_assert!(self.check_column_for_write::<V>(idx).is_ok());
        self.set_columns.insert(idx);
        if value.is_null() {
            self.null_columns.insert(idx);
            self.indirect_data.remove(idx);
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

    pub fn set_null(&mut self, idx: usize) -> Result<&mut Row<'data>> {
        self.check_column_for_nullability(idx)?;
        self.set_columns.insert(idx);
        self.null_columns.insert(idx);
        self.indirect_data.remove(idx);
        Ok(self)
    }

    pub fn set_null_unchecked(&mut self, idx: usize) -> &mut Row<'data> {
        debug_assert!(self.check_column_for_nullability(idx).is_ok());

    }

    pub fn set_null_by_name(&mut self, column: &str) -> Result<&mut Row<'data>> {
        if let Some(idx) = self.schema.column_index(column) {
            self.set_null(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub fn get<'self_, V>(&'self_ self, idx: usize) -> Result<V> where V: Value<'self_> {
        self.check_column_for_read::<V>(idx)?;
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

    pub fn get_by_name<'self_, V>(&'self_ self, column: &str) -> Result<Option<V>> where V: Value<'self_> {
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

    fn to_owned(&mut self) {
        if let Data::Borrowed(data) = *self {
            let row_len = self.schema.row_size();
            let bitset_len = BitSetT::byte_len(self.schema.num_columns());
            let data_len = row_len
                        + if self.schema.has_nullable_columns() { bitset_len } else { 0 }
                        + bitset_len;

            let data = Vec::with_capacity(data_len);
            // TODO: this leaves some trailing bits set, is that a problem?
            data.resize(bitset_len, 0xFF);
            if self.schema.has_nullable_columns() {
                data.extend_from_slice(&data[row_len..]);
            }
            data.extend_from_slice(&data[..row_len]);
            *self = Data::Owned(data.into_boxed_slice());
        }
    }

    pub fn into_owned(&self) -> Row<'static> {
        unimplemented!()
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_write<V>(&self, idx: usize) -> Result<()> where V: Value<'data> {
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

    fn check_column_for_nullability(&self, idx: usize) -> Result<()> {
        if idx >= self.schema.columns().len() {
            return Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                                      idx, self.schema)));
        }
        let column = &self.schema.columns()[idx];
        if !column.is_nullable() {
            return Err(Error::InvalidArgument(format!("column {:?} is not nullable", column)));
        }
        Ok(())
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_read<V>(&self, idx: usize) -> Result<()> where V: Value<'data> {
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

impl <'data> Clone for Row<'data> {
    fn clone(&self) -> Row<'data> {
        let schema = self.schema.clone();
        // Copy the direct data.
        let data = match self.data {
            Data::Owned(data) => Data::Owned(data.clone()),
            Data::Borrowed(data) => Data::Owned(Data::clone_borrowed_data(&schema, data)),
        };

        let mut row = Row {
            data,
            owned_data: VecMap::with_capacity(self.owned_data.capacity()),
            schema,
        };

        // Copy the indirect data, and fixup the data pointers to match.
        for (&idx, data) in &self.owned_data {
            self.set_unchecked(idx, data.clone());
        }
        row
    }
}

impl <'data> fmt::Debug for Row<'data> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;
        for (idx, column) in self.schema.columns().iter().enumerate() {
            if !self.set_columns.get(idx) { continue; }

            if is_first { is_first = false; }
            else { write!(f, ", ")? }

            write!(f, "{:?} {:?}=", column.data_type(), column.name())?;
            if self.is_null(idx).unwrap() {
                write!(f, "NULL")?
            } else {
                util::fmt_cell(f, self, idx)?;
            }
        }
        Ok(())
    }
}

impl <'data> cmp::PartialEq for Row<'data> {
    fn eq(&self, other: &Row) -> bool {
        self.schema == other.schema &&
            self.set_columns == other.set_columns &&
            self.null_columns == other.null_columns &&
            self.data == other.data &&
            self.indirect_data == other.indirect_data
    }
}

impl <'data> cmp::Eq for Row<'data>  {}

/// `Row`s can be compared based on primary key column values. If the schemas do not match or if
/// some of the primary key columns are not set, the ordering is not defined.
impl <'data> cmp::PartialOrd for Row<'data> {
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

pub(crate) struct OperationEncoder {
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

    pub fn encode_range_split(&mut self, row: &Row) {
        self.encode_row(OperationType::SplitRow, row);
    }

    pub fn encode_range_partition(&mut self, lower: &RangePartitionBound, upper: &RangePartitionBound) {
        let (lower_bound, lower_bound_type) = match *lower {
            RangePartitionBound::Inclusive(ref row) => (row, OperationType::RangeLowerBound),
            RangePartitionBound::Exclusive(ref row) => (row, OperationType::ExclusiveRangeLowerBound),
        };
        let (upper_bound, upper_bound_type) = match *upper {
            RangePartitionBound::Inclusive(ref row) => (row, OperationType::InclusiveRangeUpperBound),
            RangePartitionBound::Exclusive(ref row) => (row, OperationType::RangeUpperBound),
        };

        self.encode_row(lower_bound_type, lower_bound);
        self.encode_row(upper_bound_type, upper_bound);
    }

    pub fn encode_range_partition_split(&mut self, split: &Row) {
        self.encode_row(OperationType::SplitRow, split);
    }

    pub fn encode_row(&mut self, op_type: OperationType, row: &Row) {
        let Row { ref data, ref indirect_data, ref set_columns, ref null_columns, ref schema, .. } = *row;

        self.data.push(op_type as u8);
        self.data.extend_from_slice(set_columns.data());
        self.data.extend_from_slice(null_columns.data());

        let mut offset = self.data.len();
        // TODO: use reserve, and get rid of truncate
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

    /// Returns the direct and indirect encoded length for the row.
    pub fn encoded_len(row: &Row) -> (usize, usize) {
        let Row { ref indirect_data, ref set_columns, ref null_columns, ref schema, .. } = *row;

        let mut direct = 1; // op type

        direct += set_columns.data().len();
        direct += null_columns.data().len();

        for (idx, column) in schema.columns().iter().enumerate() {
            if !set_columns.get(idx) { continue; }
            if column.is_nullable() && null_columns.get(idx) { continue; }
            direct += column.data_type().size();
        }
        (direct, indirect_data.values().map(|data| data.len()).sum())
    }

    pub fn len(&self) -> usize {
        self.data.len() + self.indirect_data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.indirect_data.clear();
    }

    pub fn unwrap(self) -> (Vec<u8>, Vec<u8>) {
        let OperationEncoder { data, indirect_data } = self;
        (data, indirect_data)
    }

    pub fn into_pb(self) -> RowOperationsPb {
        RowOperationsPb {
            rows: Some(self.data),
            indirect_data: Some(self.indirect_data),
        }
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
