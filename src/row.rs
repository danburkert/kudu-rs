use std::cmp;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::str;

#[cfg(any(feature="quickcheck", test))] use quickcheck;

use ColumnSelector;
use Error;
use Result;
use Schema;
use util;
use value::{
    Value,
    read_var_len_value,
    write_var_len_value,
};

/// Returns the size of the data array for a partial row with the given schema.
fn partial_row_data_len(schema: &Schema) -> usize {
    let row_len = schema.row_len();
    let bitmap_len = schema.bitmap_len();
    row_len + schema.has_nullable_columns() as usize * bitmap_len + bitmap_len
}

pub struct Row<'data> {

    /// A pointer to the row data array, plus a 1 bit payload indicating whether this is a mutable
    /// partial row, or a constant contiguous row.
    ///
    /// The row data array consists of the individual columns laid out contiguously, followed by
    /// a null bitmap if there are nullable columns, followed by the is-set bitmap if the row is a
    /// mutable partial row.
    ///
    /// The tag payload is stored in the least-significant bit.
    tagged_ptr: i64,

    /// The schema of the row.
    schema: Schema,

    _marker: PhantomData<&'data [u8]>,
}

// TODO: unset
impl <'data> Row<'data> {

    /// Creates an empty mutable partial row.
    pub(crate) fn partial(schema: Schema) -> Row<'data> {
        let data_len = partial_row_data_len(&schema);

        // N.B. zeroing the bitmap portions of the data array is necessary.
        let data = vec![0u8; data_len];
        debug_assert_eq!(data.len(), data_len);
        debug_assert_eq!(data.capacity(), data_len);
        let ptr = data.as_ptr() as i64;
        mem::forget(data);

        Row {
            tagged_ptr: (ptr << 1) | 1,
            schema,
            _marker: PhantomData::default(),
        }
    }

    /// Creates a constant contiguous row referencing the provided data.
    pub(crate) fn contiguous(schema: Schema, data: &[u8]) -> Row {
        let tagged_ptr = (data.as_ptr() as i64) << 1;
        Row {
            tagged_ptr,
            schema,
            _marker: PhantomData::default(),
        }
    }

    /// Sets the value of the column.
    ///
    /// Returns an error if the column does not exist, or the value's type is wrong.
    pub fn set<C, V>(&mut self, column: C, value: V) -> Result<&mut Self>
    where C: ColumnSelector,
          V: Value<'data>
    {
        let idx = column.column_index(&self.schema)?;
        self.check_column_for_write::<V>(idx)?;
        unsafe {
            Ok(self.set_unchecked(idx, value))
        }
    }

    /// Sets the value of the column at index `idx` without checking the column's type.
    ///
    /// # Unsafety
    ///
    /// This method may invoke undefined behavior if the column does not exist, or the value type
    /// does not match the column type.
    pub unsafe fn set_unchecked<V>(&mut self, idx: usize, value: V) -> &mut Self
    where V: Value<'data>
    {
        if self.is_contiguous_row() {
            self.into_partial_row();
        } else if V::DATA_TYPE.is_var_len() {
            // Deallocate the existing value, if it has already been set to an owned value.
            self.deallocate(idx);
        }

        let data = self.data_mut();
        bitmap_set(data.offset(self.is_set_offset()), idx);
        if value.is_null() {
            bitmap_set(data.offset(self.is_null_offset()), idx);
        } else {
            if self.schema.has_nullable_columns() {
                bitmap_clear(data.offset(self.is_null_offset()), idx);
            }

            value.write(data.offset(self.schema.column_offset(idx)));
        }
        self
    }

    /// Sets the column to null.
    ///
    /// Returns an error if the column does not exist, or the column is not nullable.
    pub fn set_null<C>(&mut self, column: C) -> Result<&mut Row<'data>>
    where C: ColumnSelector
    {
        let idx = column.column_index(&self.schema)?;
        self.check_column_for_nullability(idx)?;

        if self.is_contiguous_row() {
            self.into_partial_row();
        }

        unsafe {
            // Deallocate the existing value, if it has already been set to an owned value.
            self.deallocate(idx);
            bitmap_set(self.data_mut().offset(self.is_set_offset()), idx);
            bitmap_set(self.data_mut().offset(self.is_null_offset()), idx);
        }
        Ok(self)
    }

    /// Gets the value of the column.
    ///
    /// Returns an error if the column does not exist, the column is unset, the column type
    /// does not match the value type.
    pub fn get<'self_, C, V>(&'self_ self, column: C) -> Result<V>
    where C: ColumnSelector,
          V: Value<'self_>
    {
        let idx = column.column_index(&self.schema)?;
        self.check_column_for_read::<V>(idx)?;

        unsafe {
            if !self.is_set_unchecked(idx) {
                Err(Error::InvalidArgument(format!("column {} ({}) is not set",
                                                   self.schema.columns()[idx].name(), idx)))
            } else if self.is_null_unchecked(idx) {
                if let Some(null) = V::null() {
                    Ok(null)
                } else {
                    Err(Error::InvalidArgument(format!("column {} ({}) is null",
                                                    self.schema.columns()[idx].name(), idx)))
                }
            } else {
                Ok(V::read(self.data().offset(self.schema.column_offset(idx)))?)
            }
        }
    }

    /// Returns `true` if the column at index `idx` is null.
    ///
    /// The result is undefined if the `idx` is not valid.
    #[inline]
    pub unsafe fn is_null_unchecked(&self, idx: usize) -> bool {
        self.schema.has_nullable_columns()
            && bitmap_get(self.data().offset(self.is_null_offset() as isize), idx)
    }

    /// Returns `true` if the column is null.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_null<C>(&self, column: C) -> Result<bool> where C: ColumnSelector {
        let idx = column.column_index(&self.schema)?;
        Ok(unsafe { self.is_null_unchecked(idx) })
    }

    /// Returns `true` if the column at index `idx` is set.
    ///
    /// The result is undefined if the `idx` is not valid.
    #[inline]
    pub unsafe fn is_set_unchecked(&self, idx: usize) -> bool {
        self.is_contiguous_row() || bitmap_get(self.data().offset(self.is_set_offset()), idx)
    }

    /// Returns `true` if the column is set.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_set<C>(&self, column: C) -> Result<bool> where C: ColumnSelector {
        let idx = column.column_index(&self.schema)?;
        Ok(unsafe { self.is_set_unchecked(idx) })
    }

    /// Returns the schema of the row.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Returns the offset of the is-null bitmap in the data array.
    #[inline]
    fn is_null_offset(&self) -> isize {
        debug_assert!(self.schema.has_nullable_columns());
        self.schema.row_len() as isize
    }

    pub(crate) fn is_null_bitmap(&self) -> &[u8] {
        if self.schema.has_nullable_columns() {
            unsafe {
                slice::from_raw_parts(self.data().offset(self.schema.row_len() as isize),
                                      self.schema.bitmap_len())
            }
        } else {
            &[]
        }
    }

    /// Returns the offset of the is-set bitmap in the data array.
    ///
    /// Must only be called on a partial row.
    #[inline]
    fn is_set_offset(&self) -> isize {
        debug_assert!(self.is_partial_row());
        (self.schema.row_len()
            + self.schema.has_nullable_columns() as usize * self.schema.bitmap_len()) as isize
    }

    #[inline]
    pub(crate) fn is_set_bitmap(&self) -> Option<&[u8]> {
        let len = self.schema.bitmap_len();
        if self.is_partial_row() {
            let offset = self.schema.row_len() + self.schema.has_nullable_columns() as usize * len;
            unsafe { Some(slice::from_raw_parts(self.data().offset(offset as isize), len)) }
        } else {
            None
        }
    }

    #[inline]
    fn is_partial_row(&self) -> bool {
        self.tagged_ptr & 1 == 1
    }

    #[inline]
    fn is_contiguous_row(&self) -> bool {
        !self.is_partial_row()
    }

    #[inline]
    pub(crate) fn data(&self) -> *const u8 {
        (self.tagged_ptr >> 1) as _
    }

    #[inline]
    fn data_mut(&mut self) -> *mut u8 {
        debug_assert!(self.is_partial_row());
        (self.tagged_ptr >> 1) as _
    }

    #[inline(never)]
    fn into_partial_row(&mut self) {
        debug_assert!(self.is_partial_row());
        unimplemented!()
    }

    /// Copies all borrowed values into a new row with a `'static` lifetime.
    pub fn into_owned(mut self) -> Row<'static> {
        if self.is_contiguous_row() {
            self.into_partial_row();
        }

        let data = self.data_mut();
        unsafe {
            for (idx, column) in self.schema.columns().iter().enumerate() {
                if !column.data_type().is_var_len()
                    || !self.is_set_unchecked(idx)
                    || self.is_null_unchecked(idx) {
                    continue;
                }

                let data = data.offset(self.schema.column_offset(idx));
                let (ptr, len, cap) = read_var_len_value(data);
                // If capacity is 0, then this is a borrowed value. Copy it and write the new owned
                // value back to the row.
                if cap == 0 {
                    // Copy it into a Vec and write it to the new data array.
                    let copy: Vec<u8> = slice::from_raw_parts(ptr, len).to_owned();
                    write_var_len_value(data, copy.as_ptr(), copy.len(), copy.capacity());
                    mem::forget(copy);
                }
            }

            // Extend the lifetime, which is safe now that there are no borrowed values.
            mem::transmute(self)
        }
    }

    /// Deallocates the value at the provided index. Does nothing if the value is not set, or is
    /// not owned by the row.
    ///
    /// # Preconditions
    ///
    ///   * must be a mutable partial row
    unsafe fn deallocate(&mut self, idx: usize) {
        debug_assert!(self.is_partial_row());

        // Check that the value is set, and that it's variable length.
        if !self.schema.columns()[idx].data_type().is_var_len()
            || !self.is_set_unchecked(idx)
            || self.is_null_unchecked(idx) {
            return;
        }

        let data = self.data().offset(self.schema.column_offset(idx));
        let (ptr, len, cap) = read_var_len_value(data);

        // If capacity is greater than 0, then this is an owned value.
        if cap > 0 {
            // Drop the owned value.
            drop(Vec::<u8>::from_raw_parts(ptr as *mut u8, len, cap));
        }
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_write<V>(&self, idx: usize) -> Result<()> where V: Value<'data> {
        self.schema.check_index(idx)?;
        let column = &self.schema.columns()[idx];
        if !V::can_write_to(column.data_type()) {
            return Err(Error::InvalidArgument(format!("type {:?} is invalid for column {:?}",
                                                      V::DATA_TYPE,
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
        self.schema.check_index(idx)?;
        let column = &self.schema.columns()[idx];
        if !column.is_nullable() {
            return Err(Error::InvalidArgument(format!("column {:?} is not nullable", column)));
        }
        Ok(())
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_read<V>(&self, idx: usize) -> Result<()> where V: Value<'data> {
        let column = &self.schema.columns()[idx];
        if !V::can_read_from(column.data_type()) {
            return Err(Error::InvalidArgument(format!("type {:?} is invalid for column {:?}",
                                                      V::DATA_TYPE,
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
        if self.is_contiguous_row() {
            // A cloned contiguous row can reference the same const data.
            Row {
                tagged_ptr: self.tagged_ptr,
                schema: self.schema.clone(),
                _marker: PhantomData::default(),
            }
        } else {
            unsafe {
                // Copy the data and owned data.
                let data_len = partial_row_data_len(&self.schema);
                let mut data = slice::from_raw_parts(self.data(), data_len).to_owned();
                debug_assert_eq!(data.len(), data_len);
                debug_assert_eq!(data.capacity(), data_len);

                let ptr = data.as_mut_ptr();

                mem::forget(data);

                // Copy each owned value to the new data array.
                for (idx, column) in self.schema.columns().iter().enumerate() {
                    if !column.data_type().is_var_len()
                        || !self.is_set_unchecked(idx)
                        || self.is_null_unchecked(idx) {
                        continue;
                    }

                    let data = ptr.offset(self.schema.column_offset(idx));
                    let (ptr, len, cap) = read_var_len_value(data);
                    // If capacity is not 0, then this is a an owned value. Copy it and write the
                    // new owned value back to the new row.
                    if cap > 0 {
                        // Copy it into a Vec and write it to the new data array.
                        let copy: Vec<u8> = slice::from_raw_parts(ptr, len).to_owned();
                        write_var_len_value(data, copy.as_ptr(), copy.len(), copy.capacity());
                        mem::forget(copy);
                    }
                }

                Row {
                    tagged_ptr: ((ptr as i64) << 1) | 1,
                    schema: self.schema.clone(),
                    _marker: PhantomData::default(),
                }
            }
        }
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
            if unsafe { self.is_set_unchecked(idx) } {
                map.entry(&ColumnWrapper(column.name()), &CellWrapper(self, idx));
            }
        }
        map.finish()
    }
}

impl <'data> cmp::PartialEq for Row<'data> {
    fn eq(&self, other: &Row) -> bool {
        if self.schema != other.schema {
            return false;
        }

        if self.tagged_ptr == other.tagged_ptr {
            // TODO: is this right?  Can't a row be compared to itself?
            debug_assert!(self.is_contiguous_row());
            return true;
        }

        let num_columns = self.schema.columns().len();

        unsafe {
            // Check the is-set bitmap (if this is a partial row).
            if self.is_partial_row() {
                assert!(other.is_partial_row(), "TODO");
                let is_set_offset = self.is_set_offset();
                if !bitmap_eq(self.data().offset(is_set_offset), other.data().offset(is_set_offset), num_columns) {
                    return false;
                }
            }

            // Check the is-null bitmap (if there are nullable columns).
            if self.schema.has_nullable_columns() {
                let offset = self.is_null_offset();
                if !bitmap_eq(self.data().offset(offset), other.data().offset(offset), num_columns) {
                    return false;
                }
            }

            for (idx, (column, &offset)) in self.schema.columns().iter().zip(self.schema.column_offsets()).enumerate() {
                // Check if the column is unset or null.
                if !self.is_set_unchecked(idx) || self.is_null_unchecked(idx) {
                    continue;
                }

                let size = column.data_type().size();
                let offset = offset as isize;

                let a = self.data().offset(offset);
                let b = other.data().offset(offset);

                if slice::from_raw_parts(a, size) == slice::from_raw_parts(b, size) {
                    continue;
                }

                if column.data_type().is_var_len()
                    && <&[u8] as Value>::read(a).unwrap() == <&[u8] as Value>::read(b).unwrap() {
                    continue;
                }

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

impl <'data> Drop for Row<'data> {
    fn drop(&mut self) {
        if self.is_partial_row() {
            unsafe {
                for idx in 0..self.schema.columns().len() {
                    self.deallocate(idx);
                }
                let data_len = partial_row_data_len(&self.schema);
                Vec::from_raw_parts(self.data_mut(), data_len, data_len);
            }
        }
    }
}

/// Returns the number of bytes required to hold a number of bits.
fn bitmap_len(num_bits: usize) -> usize {
    (num_bits + 7) / 8
}

/// Returns the value of the bit at the index.
unsafe fn bitmap_get(bits: *const u8, idx: usize) -> bool {
    *bits.offset((idx >> 3) as isize) & (1 << (idx & 7)) > 0
}

/// Sets the value of the bit at the index to 1.
unsafe fn bitmap_set(bits: *mut u8, idx: usize) {
    *bits.offset((idx >> 3) as isize) |= 1 << (idx & 7)
}

/// Sets the value of the bit at the index to 0.
unsafe fn bitmap_clear(bits: *mut u8, idx: usize) {
    *bits.offset((idx >> 3) as isize) &= !(1 << (idx & 7));
}

unsafe fn bitmap_eq(a: *const u8, b: *const u8, bits: usize) -> bool {
    let bytes = bits >> 3;
    let bytes_eq = slice::from_raw_parts(a, bytes) == slice::from_raw_parts(b, bytes);
    let bytes = bytes as isize;
    let remainder_eq = bits & 0x07 == 0 || *a.offset(bytes) & 0x07 == *b.offset(bytes) & 0x07;
    bytes_eq && remainder_eq
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
        assert_eq!("foo", row.get::<_, &str>(10).unwrap());
        assert_eq!("foo".to_string(), row.get::<_, String>(10).unwrap());
        assert!(row.is_set(10).unwrap());

        // Set/get 'nullable_string'.
        row.set("nullable_string", "bar".to_string()).unwrap();
        assert_eq!("bar", row.get::<_, &str>("nullable_string").unwrap());
        assert_eq!("bar".to_string(), row.get::<_, String>("nullable_string").unwrap());
        assert_eq!(Some("bar"), row.get::<_, Option<&str>>("nullable_string").unwrap());
        assert_eq!(Some("bar".to_string()), row.get::<_, Option<String>>("nullable_string").unwrap());

        row.set("nullable_binary", &b""[..]).unwrap();
        assert_eq!(&b""[..], row.get::<_, &[u8]>("nullable_binary").unwrap());
        assert_eq!(Vec::<u8>::new(), row.get::<_, Vec<u8>>("nullable_binary").unwrap());
        assert_eq!(Some(&b""[..]), row.get::<_, Option<&[u8]>>("nullable_binary").unwrap());
        assert_eq!(Some(Vec::new()), row.get::<_, Option<Vec<u8>>>("nullable_binary").unwrap());
    }

    #[test]
    fn test_eq() {
        let schema = schema::tests::all_types_schema();
        let mut a = schema.new_row();
        let mut b = schema.new_row();
        assert_eq!(a, b);

        a.set("i32", 100i32).unwrap();
        assert_ne!(a, b);
        b.set("i32", 100i32).unwrap();
        assert_eq!(a, b);

        a.set_null("nullable_i32").unwrap();
        assert_ne!(a, b);
        b.set_null("nullable_i32").unwrap();
        assert_eq!(a, b);

        a.set("string", "foo").unwrap();
        assert_ne!(a, b);
        b.set("string", "bar").unwrap();
        assert_ne!(a, b);
        b.set("string", "foo".to_string()).unwrap();
        assert_eq!(a, b);

        let mut c = a.clone();
        assert_eq!(a, c);
        assert_eq!(b, c);

        let mut a = a.into_owned();
        assert_eq!(a, b);
        assert_eq!(a, c);

        a.set("nullable_bool", true).unwrap();
        b.set("nullable_bool", false).unwrap();
        assert_ne!(a, b);
        assert_ne!(a, c);

        c.set("nullable_bool", true).unwrap();
        assert_eq!(a, c);
        assert_ne!(b, a);
    }

    #[test]
    fn test_null_values() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();
        for (idx, column) in schema.columns().iter().enumerate() {
            assert!(!row.is_null(idx).unwrap());
            assert!(!row.is_null(column.name()).unwrap());
            assert!(!row.is_set(idx).unwrap());
        }

        assert!(row.is_null(schema.columns().len()).is_err());
        assert!(row.is_null("bogus").is_err());

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

        for &nullable_column in nullable_columns {
            row.set_null(nullable_column).unwrap();
            assert!(row.is_null(nullable_column).unwrap());
            assert!(row.is_set(nullable_column).unwrap());
        }

        let clone = row.clone();
        for &nullable_column in nullable_columns {
            assert!(clone.is_null(nullable_column).unwrap());
            assert!(clone.is_set(nullable_column).unwrap());
        }

        assert!(!row.is_null("key").unwrap());
    }

    #[test]
    fn test_debug_fmt() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row();

        row.set("key", 42i32).unwrap();
        row.set("bool", true).unwrap();
        row.set("i64", -123i64).unwrap();
        row.set("timestamp", 1514768523456789i64).unwrap();
        row.set("binary", &[0x01, 0x02, 0x42, 0x88, 0xf7][..]).unwrap();
        row.set("string", "foo").unwrap();
        row.set("nullable_i32", None::<i32>).unwrap();
        row.set_null("nullable_string").unwrap();

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

        row.set(0, 12i32).unwrap();
        assert!(row.is_set(0).unwrap_or(false));
        assert_eq!(12i32, row.get(0).unwrap());

        // Test a borrowed string.
        row.set(10, "foo_borrowed").unwrap();
        assert_eq!("foo_borrowed".to_owned(), row.get::<_, String>(10).unwrap());

        // Test an owned string.
        row.set(10, "foo_owned".to_owned()).unwrap();
        assert_eq!("foo_owned", row.get::<_, &str>(10).unwrap());

        assert_eq!("{key: 12, string: \"foo_owned\"}",
                   &format!("{:?}", row));

        assert_eq!("{key: 12, string: \"foo_owned\"}",
                   &format!("{:?}", row.clone()));

        // Test another borrowed string to ensure the owned string is deallocated.
        row.set(10, "foo_borrowed2").unwrap();
        assert_eq!("foo_borrowed2", row.get::<_, &str>(10).unwrap());

        row.set_null("nullable_i32").unwrap();

        assert_eq!("{key: 12, string: \"foo_borrowed2\", nullable_i32: NULL}",
                   &format!("{:?}", row));
        assert_eq!("{key: 12, string: \"foo_borrowed2\", nullable_i32: NULL}",
                   &format!("{:?}", row.clone()));
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

    #[test]
    fn check_eq() {
        fn rows_to_string(schema: schema::Schema) -> TestResult {
            let mut g = StdGen::new(rand::thread_rng(), 100);

            for _ in 0..10 {
                let row = Row::arbitrary(&mut g, &schema);
                assert_eq!(row, row.clone());
            }

            TestResult::passed()
        }

        quickcheck(rows_to_string as fn(schema::Schema) -> TestResult);
    }
}
