use std::cmp;
use std::fmt;
use std::marker::PhantomData;

use bitmap;
use vec_map::VecMap;
#[cfg(any(feature="quickcheck", test))] use quickcheck;

use Error;
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
    pub(crate) data: Box<[u8]>,
    pub(crate) owned_data: VecMap<Vec<u8>>,
    pub(crate) schema: Schema,
    _marker: PhantomData<&'data [u8]>,
}

// TODO: unset/unset_by_name.
impl <'data> Row<'data> {

    /// Creates a new empty row.
    ///
    /// Initially all values will be unset.
    pub(crate) fn new(schema: Schema) -> Row<'data> {
        let row_len = schema.row_len();
        let bitmap_len = schema.bitmap_len();
        let data_len = row_len
                     + if schema.has_nullable_columns() { bitmap_len } else { 0 }
                     + bitmap_len;

        // N.B. zeroing the bitmap portions of the data array is necessary.
        let data = vec![0; data_len].into_boxed_slice();

        Row {
            data,
            owned_data: VecMap::new(),
            schema,
            _marker: Default::default(),
        }
    }

    /// Sets the value of the column at index `idx`.
    ///
    /// Returns an error if the column does not exist, or the value's type is wrong.
    pub fn set<V>(&mut self, idx: usize, value: V) -> Result<&mut Self> where V: Value<'data> {
        self.check_column_for_write::<V>(idx)?;
        unsafe {
            Ok(self.set_unchecked(idx, value))
        }
    }

    /// Sets the value of the column by name.
    ///
    /// Returns an error if the column does not exist, or the value's type is wrong.
    pub fn set_by_name<V>(&mut self, column: &str, value: V) -> Result<&mut Row<'data>> where V: Value<'data> {
        match self.schema.column_index(column) {
            Some(idx) => self.set(idx, value),
            None => Err(Error::InvalidArgument(format!("unknown column {}", column))),
        }
    }

    /// Sets the value of the column at index `idx` without checking the column's type.
    ///
    /// # Unsafety
    ///
    /// This method may invoke undefined behavior if the column does not exist, or the value type
    /// does not match the column type.
    pub unsafe fn set_unchecked<V>(&mut self,
                                   idx: usize,
                                   value: V)
                                   -> &mut Self where V: Value<'data> {
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

            value.copy_data(&mut self.data[row_offset + self.schema.column_offsets()[idx]..]);
            if let Some(owned_data) = value.owned_data() {
                self.owned_data.insert(idx, owned_data);
            } else {
                self.owned_data.remove(idx);
            }
        }
        self
    }

    /// Sets the column at index `idx` to null.
    ///
    /// Returns an error if the column does not exist, or the column is not nullable.
    pub fn set_null(&mut self, idx: usize) -> Result<&mut Row<'data>> {
        self.check_column_for_nullability(idx)?;
        let bitmap_len = self.schema.bitmap_len();
        bitmap::set(&mut self.data[bitmap_len..], idx);
        bitmap::set(&mut *self.data, idx);
        self.owned_data.remove(idx);
        Ok(self)
    }

    /// Sets the column by name to null.
    ///
    /// Returns an error if the column does not exist, or the column is not nullable.
    pub fn set_null_by_name(&mut self, column: &str) -> Result<&mut Row<'data>> {
        if let Some(idx) = self.schema.column_index(column) {
            self.set_null(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column {}", column)))
        }
    }

    /// Gets the value of the column at index `idx`.
    ///
    /// Returns an error if the column does not exist, the column is unset, the column type
    /// does not match the value type.
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

        if !bitmap::get(is_set, idx) {
            Err(Error::InvalidArgument(format!("column {} ({}) is not set",
                                               self.schema.columns()[idx].name(), idx)))
        } else if self.schema.has_nullable_columns() && bitmap::get(is_null, idx) {
            if let Some(null) = V::null() {
                Ok(null)
            } else {
                Err(Error::InvalidArgument(format!("column {} ({}) is null",
                                                   self.schema.columns()[idx].name(), idx)))
            }
        } else {
            let offset = self.schema.column_offsets()[idx];
            let size = V::size();
            unsafe { Ok(V::from_data(&row[offset..offset + size])) }
        }
    }

    /// Gets the value of the column by name.
    ///
    /// Returns an error if the column does not exist, the column is unset, the column type
    /// does not match the value type.
    pub fn get_by_name<'self_, V>(&'self_ self, column: &str) -> Result<V> where V: Value<'self_> {
        if let Some(idx) = self.schema.column_index(column) {
            self.get(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column {}", column)))
        }
    }

    /// Returns `true` if the column at index `idx` is null.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_null(&self, idx: usize) -> Result<bool> {
        if idx >= self.schema.columns().len() {
            Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                               idx, self.schema)))
        } else {
            Ok(self.schema.has_nullable_columns() &&
               bitmap::get(&self.data[self.schema.bitmap_len()..], idx))
        }
    }

    /// Returns `true` if the column is null.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_null_by_name(&self, column: &str) -> Result<bool> {
        if let Some(idx) = self.schema.column_index(column) {
            self.is_null(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column {}", column)))
        }
    }

    /// Returns `true` if the column at index `idx` is set.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_set(&self, idx: usize) -> Result<bool> {
        if idx >= self.schema.columns().len() {
            Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                               idx, self.schema)))
        } else {
            Ok(bitmap::get(&*self.data, idx))
        }
    }

    /// Returns `true` if the column is set.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_set_by_name(&self, column: &str) -> Result<bool> {
        if let Some(idx) = self.schema.column_index(column) {
            self.is_set(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column {}", column)))
        }
    }

    /// Returns the schema of the row.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Copies all borrowed values into a new row with a `'static` lifetime.
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

    /// Checks that the column with the specified index is nullable.
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
        use DataType;
        use quickcheck::Arbitrary;
        let mut row = schema.new_row();
        for (idx, column) in schema.columns().iter().enumerate() {
            // TODO: set null values.
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
        struct ColumnWrapper<'a>(&'a str);
        impl <'a> fmt::Debug for ColumnWrapper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(self.0)
            }
        }


        struct CellWrapper<'a>(&'a Row<'a>, usize);
        impl <'a> fmt::Debug for CellWrapper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                util::fmt_cell(f, self.0, self.1)
            }
        }

        let mut map = f.debug_map();
        for (idx, column) in self.schema.columns().iter().enumerate() {
            // TODO: add iterator to bitmap.
            if !bitmap::get(&*self.data, idx) { continue; }
            map.entry(&ColumnWrapper(column.name()), &CellWrapper(self, idx));
        }
        map.finish()
    }
}

impl <'data> cmp::PartialEq for Row<'data> {
    fn eq(&self, other: &Row) -> bool {
        if self.schema != other.schema {
            return false;
        }

        let bitmap_len = self.schema.bitmap_len();
        let row_offset = if self.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len };

        // Check that all of the columns are set the same, and set null the same.
        if self.data[..row_offset] != other.data[..row_offset] {
            return false;
        }

        for (idx, (column, &column_offset)) in self.schema.columns().iter().zip(self.schema.column_offsets()).enumerate() {
            // Check if the column is unset or null.
            if !bitmap::get(&*self.data, idx) ||
               (column.is_nullable() && bitmap::get(&self.data[bitmap_len..], idx)) {
                continue;
            }

            let start = row_offset + column_offset;
            let end = start + column.data_type().size();

            if column.data_type().is_var_len() {
                unsafe {
                    let a: &[u8] = Value::from_data(&self.data[start..end]);
                    let b: &[u8] = Value::from_data(&other.data[start..end]);
                    if a != b {
                        return false;
                    }
                }
            } else if self.data[start..end] != other.data[start..end] {
                return false;
            }
        }
        true
    }
}

impl <'data> cmp::Eq for Row<'data>  {}

/// `Row`s can be compared based on primary key column values. If the schemas do not match or if
/// some of the primary key columns are not set, the ordering is not defined.
impl <'data> cmp::PartialOrd for Row<'data> {
    fn partial_cmp(&self, _other: &Row) -> Option<cmp::Ordering> {
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

#[cfg(test)]
mod tests {

    use quickcheck::{quickcheck, TestResult, StdGen};
    use rand;
    use schema;
    use super::*;

    #[test]
    fn test_get_set() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        for idx in 0..schema.columns().len() {
            assert!(!row.is_set(idx).unwrap());
        }

        // Set/get 'key' by index.
        assert_eq!(0, schema.column_index("key").unwrap());
        row.set(0, 32i32).unwrap();
        assert_eq!(32i32, row.get(0).unwrap());
        assert!(row.is_set(0).unwrap());

        // Set/get 'string' by index.
        assert_eq!(10, schema.column_index("string").unwrap());
        row.set(10, "foo").unwrap();
        assert_eq!("foo", row.get::<&str>(10).unwrap());
        assert_eq!("foo".to_string(), row.get::<String>(10).unwrap());
        assert!(row.is_set(10).unwrap());

        // Set/get 'nullable_string'.
        row.set_by_name("nullable_string", "bar".to_string()).unwrap();
        assert_eq!("bar", row.get_by_name::<&str>("nullable_string").unwrap());
        assert_eq!("bar".to_string(), row.get_by_name::<String>("nullable_string").unwrap());
        assert_eq!(Some("bar"), row.get_by_name::<Option<&str>>("nullable_string").unwrap());
        assert_eq!(Some("bar".to_string()), row.get_by_name::<Option<String>>("nullable_string").unwrap());

        row.set_by_name("nullable_binary", &b""[..]).unwrap();
        assert_eq!(&b""[..], row.get_by_name::<&[u8]>("nullable_binary").unwrap());
        assert_eq!(Vec::<u8>::new(), row.get_by_name::<Vec<u8>>("nullable_binary").unwrap());
        assert_eq!(Some(&b""[..]), row.get_by_name::<Option<&[u8]>>("nullable_binary").unwrap());
        assert_eq!(Some(Vec::new()), row.get_by_name::<Option<Vec<u8>>>("nullable_binary").unwrap());
    }

    #[test]
    fn test_eq() {
        let schema = schema::tests::all_types_schema();
        let mut a = schema.new_row();
        let mut b = schema.new_row();
        assert_eq!(a, b);

        a.set_by_name("i32", 100i32).unwrap();
        assert_ne!(a, b);
        b.set_by_name("i32", 100i32).unwrap();
        assert_eq!(a, b);

        a.set_null_by_name("nullable_i32").unwrap();
        assert_ne!(a, b);
        b.set_null_by_name("nullable_i32").unwrap();
        assert_eq!(a, b);

        a.set_by_name("string", "foo").unwrap();
        assert_ne!(a, b);
        b.set_by_name("string", "bar").unwrap();
        assert_ne!(a, b);
        b.set_by_name("string", "foo".to_string()).unwrap();
        assert_eq!(a, b);

        let mut c = a.clone();
        assert_eq!(a, c);
        assert_eq!(b, c);

        let mut a = a.into_owned();
        assert_eq!(a, b);
        assert_eq!(a, c);

        a.set_by_name("nullable_bool", true).unwrap();
        b.set_by_name("nullable_bool", false).unwrap();
        assert_ne!(a, b);
        assert_ne!(a, c);

        c.set_by_name("nullable_bool", true).unwrap();
        assert_eq!(a, c);
        assert_ne!(b, a);
    }

    #[test]
    fn test_null_values() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();
        for (idx, column) in schema.columns().iter().enumerate() {
            assert!(!row.is_null(idx).unwrap());
            assert!(!row.is_null_by_name(column.name()).unwrap());
            assert!(!row.is_set(idx).unwrap());
        }

        assert!(row.is_null(schema.columns().len()).is_err());
        assert!(row.is_null_by_name("bogus").is_err());

        let nullable_columns = &[
            "nullable_bool",
            "nullable_i8",
            "nullable_i16",
            "nullable_i32",
            "nullable_i64",
            "nullable_timestamp",
            "nullable_f32",
            "nullable_f64",
            "nullable_binary",
            "nullable_string",
        ];

        for nullable_column in nullable_columns {
            row.set_null_by_name(nullable_column).unwrap();
            assert!(row.is_null(schema.column_index(nullable_column).unwrap()).unwrap());
            assert!(row.is_null_by_name(nullable_column).unwrap());
            assert!(row.is_set_by_name(nullable_column).unwrap());
        }

        let clone = row.clone();
        for nullable_column in nullable_columns {
            assert!(clone.is_null(schema.column_index(nullable_column).unwrap()).unwrap());
            assert!(clone.is_null_by_name(nullable_column).unwrap());
            assert!(clone.is_set_by_name(nullable_column).unwrap());
        }

        assert!(!row.is_null_by_name("key").unwrap());
    }

    #[test]
    fn test_debug_fmt() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set_by_name("key", 42).unwrap();
        row.set_by_name("bool", true).unwrap();
        row.set_by_name("i64", -123i64).unwrap();
        row.set_by_name("timestamp", 1514768523456789i64).unwrap();
        row.set_by_name("binary", &[0x01, 0x02, 0x42, 0x88, 0xf7][..]).unwrap();
        row.set_by_name("string", "foo").unwrap();
        row.set_by_name("nullable_i32", None::<i32>).unwrap();
        row.set_null_by_name("nullable_string").unwrap();

        assert_eq!("{key: 42, \
                     bool: true, \
                     i64: -123, \
                     timestamp: 2018-01-01T01:02:03.456789Z, \
                     binary: 0x01024288f7, \
                     string: \"foo\", \
                     nullable_i32: NULL, \
                     nullable_string: NULL}",
                   format!("{:?}", row));
    }

    #[test]
    fn test_row() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set::<i32>(0, 12).unwrap();
        println!("row is set: {:?}", row.is_set(0));
        assert!(row.is_set(0).unwrap_or(false));
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
