use std::cmp;
use std::fmt;
use std::marker::PhantomData;
use std::time::SystemTime;

use byteorder::{ByteOrder, LittleEndian};

use bitmap;
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

pub struct Row<'data> {
    /// The row data.
    ///
    /// Laid out as follows:
    ///     - is-set bitmap
    ///     - is-null bitmap
    ///     - column values
    data: Box<[u8]>,
    owned_data: VecMap<Vec<u8>>,
    schema: Schema,
    _marker: PhantomData<&'data [u8]>,
}

// TODO: unset/unset_by_name.
impl <'data> Row<'data> {

    /// Creates a new owned row.
    pub(crate) fn new(schema: Schema) -> Row<'data> {
        let row_len = schema.row_len();
        let bitmap_len = schema.bitmap_len();
        let data_len = row_len
                     + if schema.has_nullable_columns() { bitmap_len } else { 0 }
                     + bitmap_len;

        let data = vec![0; data_len].into_boxed_slice();
        Row {
            data,
            owned_data: VecMap::new(),
            schema,
            _marker: Default::default(),
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
        bitmap::set(&mut *self.data, idx);

        let bitmap_len = self.schema.bitmap_len();
        if value.is_null() {
            bitmap::set(&mut self.data[bitmap_len..], idx);
            self.owned_data.remove(idx);
        } else {
            let mut row_offset = bitmap_len;
            if self.schema.has_nullable_columns() {
                bitmap::clear(&mut self.data[bitmap_len..], idx);
                row_offset += bitmap_len;
            }

            value.copy_data(&mut self.data[row_offset..]);
            if let Some(owned_data) = value.owned_data() {
                self.owned_data.insert(idx, owned_data);
            } else {
                self.owned_data.remove(idx);
            }
        }
        self
    }

    pub fn set_null(&mut self, idx: usize) -> Result<&mut Row<'data>> {
        self.check_column_for_nullability(idx)?;
        let bitmap_len = self.schema.bitmap_len();
        bitmap::set(&mut self.data[bitmap_len..], idx);
        bitmap::set(&mut *self.data, idx);
        self.owned_data.remove(idx);
        Ok(self)
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
        let bitmap_len = self.schema.bitmap_len();

        // Split the data slice into the is_set bitmap, the is_null bitmap, and the row.
        let (is_set, data) = self.data.split_at(bitmap_len);
        let (is_null, row) = if self.schema.has_nullable_columns() {
            data.split_at(bitmap_len)
        } else {
            let empty: &'static [u8] = &[];
            (empty, data)
        };

        if bitmap::get(is_set, idx) {
            Err(Error::InvalidArgument(format!("column '{}' ({}) is not set",
                                               self.schema.columns()[idx].name(), idx)))
        } else if self.schema.has_nullable_columns() && bitmap::get(is_null, idx) {
            if let Some(null) = V::null() {
                Ok(null)
            } else {
                Err(Error::InvalidArgument(format!("column '{}' ({}) is null",
                                                   self.schema.columns()[idx].name(), idx)))
            }
        } else {
            let offset = self.schema.column_offsets()[idx];
            let len = V::size();
            unsafe { Ok(V::from_data(&row[offset..offset+len])) }
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
            let bitmap_len = self.schema.bitmap_len();
            Ok(self.schema.has_nullable_columns() &&
               bitmap::get(&self.data[bitmap_len..], idx))
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
            Ok(bitmap::get(&*self.data, idx))
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

    pub fn into_owned(mut self) -> Row<'static> {
        // Find all borrowed var len columns, and copy them into owned_data.
        { // NLL hack
            let bitmap_len = self.schema.bitmap_len();
            let (bitmaps, row_data) = self.data.split_at_mut(
                if self.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len });

            for (idx, column) in self.schema.columns().iter().enumerate() {
                let data_type = column.data_type();
                if !data_type.is_var_len() ||
                !bitmap::get(bitmaps, idx) ||
                (column.is_nullable() && bitmap::get(&bitmaps[bitmap_len..], idx)) ||
                self.owned_data.contains_key(idx) {
                    continue;
                }
                let column_offset = self.schema.column_offsets()[idx];
                let data = unsafe { Vec::<u8>::from_data(&row_data[column_offset..]) };
                data.copy_data(&mut row_data[column_offset..]);
                self.owned_data.insert(idx, data);
            }
        }

        // Copy the fields to a new row with a static lifetime.
        Row {
            data: self.data,
            owned_data: self.owned_data,
            schema: self.schema,
            _marker: PhantomData::default(),
        }
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
    pub fn arbitrary<G>(g: &mut G, schema: &Schema) -> Row<'static> where G: quickcheck::Gen {
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
        // Copy the inline and borrowed data.
        let mut row = Row {
            data: self.data.clone(),
            owned_data: VecMap::with_capacity(self.owned_data.len()),
            schema: self.schema.clone(),
            _marker: PhantomData::default(),
        };

        // Copy the owned data.
        for (idx, data) in &self.owned_data {
            unsafe {
                row.set_unchecked(idx, data.to_owned());
            }
        }

        row
    }
}

impl <'data> fmt::Debug for Row<'data> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;
        for (idx, column) in self.schema.columns().iter().enumerate() {
            if !bitmap::get(&*self.data, idx) { continue; }

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
        unimplemented!()
        /*
        self.schema == other.schema &&
            self.set_columns == other.set_columns &&
            self.null_columns == other.null_columns &&
            self.data == other.data &&
            self.indirect_data == other.indirect_data
            */
    }
}

impl <'data> cmp::Eq for Row<'data>  {}

/// `Row`s can be compared based on primary key column values. If the schemas do not match or if
/// some of the primary key columns are not set, the ordering is not defined.
impl <'data> cmp::PartialOrd for Row<'data> {
    fn partial_cmp(&self, other: &Row) -> Option<cmp::Ordering> {
        unimplemented!()
        /*
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
        */
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
        self.data.reserve(1 + row.data.len());
        self.data.push(op_type as u8);
        let bitmap_len = row.schema.bitmap_len();
        let (bitmaps, row_data) = row.data.split_at(
            if row.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len });
        self.data.extend_from_slice(bitmaps);
        let mut offset = self.data.len();
        for (idx, column) in row.schema.columns().iter().enumerate() {
            if !bitmap::get(bitmaps, idx) ||
               (column.is_nullable() && bitmap::get(&bitmaps[bitmap_len..], idx)) { continue; }

            let data_type = column.data_type();
            let column_offset = row.schema.column_offsets()[idx];
            let width = data_type.size();
            if data_type.is_var_len() {
                let data: &[u8] = unsafe { Value::from_data(&row_data[column_offset..]) };
                LittleEndian::write_u64(&mut self.data[offset..], self.indirect_data.len() as u64);
                LittleEndian::write_u64(&mut self.data[offset+8..], data.len() as u64);
                self.indirect_data.extend_from_slice(data);
            } else {
                self.data.extend_from_slice(&row_data[column_offset..column_offset + width]);
            }
            offset += width;
        }
    }

    /// Returns the direct and indirect encoded length for the row.
    pub fn encoded_len(row: &Row) -> usize {
        let mut len = 1; // op type

        let bitmap_len = row.schema.bitmap_len();
        len += bitmap_len;

        let (bitmaps, row_data) = row.data.split_at(
            if row.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len });

        for (idx, column) in row.schema.columns().iter().enumerate() {
            if !bitmap::get(bitmaps, idx) ||
               (column.is_nullable() && bitmap::get(&bitmaps[bitmap_len..], idx)) { continue; }

            len += column.data_type().size();
            if column.data_type().is_var_len() {
                let offset = row.schema.column_offsets()[idx];
                let slice: &[u8] = unsafe { Value::from_data(&row_data[offset..]) };
                len += slice.len()
            }
        }
        len
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
