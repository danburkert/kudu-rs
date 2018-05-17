use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::str;
use std::time::SystemTime;

#[cfg(any(feature="quickcheck", test))] use quickcheck;

use Error;
use Result;
use Schema;
use util;
use DataType;

/// Marker trait for types which can be stored in a Kudu column.
pub trait Value<'data>: Sized {
    /// The Kudu column type corresponding to the value type.
    const DATA_TYPE: DataType;

    /// Returns `true` if the value type can be read from a column of the provided data type.
    #[doc(hidden)]
    fn can_read_from(data_type: DataType) -> bool { data_type == Self::DATA_TYPE }

    /// Returns `true` if the value type can be written to a column of the provided data type.
    #[doc(hidden)]
    fn can_write_to(data_type: DataType) -> bool { data_type == Self::DATA_TYPE }

    /// Returns `true` if the value type is nullable.
    #[doc(hidden)]
    fn is_nullable() -> bool { false }

    /// Returns `true` if the value is NULL.
    #[doc(hidden)]
    fn is_null(&self) -> bool { false }

    /// Reads an instance of the value type from the row data array.
    #[doc(hidden)]
    unsafe fn read(data: *const u8) -> Result<Self>;

    /// Writes the value to the row data array.
    #[doc(hidden)]
    unsafe fn write(self, *mut u8) -> Result<()>;

    /// Creates a null value, or `None` if the value type is not nullable.
    #[doc(hidden)]
    fn null() -> Option<Self> { None }
}

impl <'data> Value<'data> for bool {
    const DATA_TYPE: DataType = DataType::Bool;
    unsafe fn read(data: *const u8) -> Result<bool> { Ok(*data != 0) }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        *data = if self { 1 } else { 0 };
        Ok(())
    }
}

macro_rules! int_value {
    ($ty:ty, $data_type:ident) => (
        impl <'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            unsafe fn read(data: *const u8) -> Result<$ty> {
                Ok((data as *const $ty).read_unaligned().to_le())
            }
            unsafe fn write(self, data: *mut u8) -> Result<()> {
                (data as *mut $ty).write_unaligned(self.to_le());
                Ok(())
            }
        }
    );
}
int_value!(i8, Int8);
int_value!(i16, Int16);
int_value!(i32, Int32);

impl <'data> Value<'data> for i64 {
    const DATA_TYPE: DataType = DataType::Int64;
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    unsafe fn read(data: *const u8) -> Result<i64> {
        Ok((data as *const i64).read_unaligned().to_le())
    }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        (data as *mut i64).write_unaligned(self.to_le());
        Ok(())
    }
}

macro_rules! float_value {
    ($ty:ident, $data_type:ident, $unsigned_ty:ty) => (
        impl <'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            unsafe fn read(data: *const u8) -> Result<$ty> {
                Ok($ty::from_bits((data as *const $unsigned_ty).read_unaligned().to_le()))
            }
            unsafe fn write(self, data: *mut u8) -> Result<()> {
                (data as *mut $unsigned_ty).write_unaligned(self.to_bits().to_le());
                Ok(())
            }
        }
    );
}
float_value!(f32, Float, u32);
float_value!(f64, Double, u64);

impl <'data> Value<'data> for SystemTime {
    const DATA_TYPE: DataType = DataType::Timestamp;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    unsafe fn read(data: *const u8) -> Result<SystemTime> { Ok(util::us_to_time(i64::read(data)?)) }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        util::time_to_us(self).write(data)?;
        Ok(())
    }
}

impl <'data> Value<'data> for &'data [u8] {
    const DATA_TYPE: DataType = DataType::Binary;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary
                                                 || data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }

    unsafe fn read(data: *const u8) -> Result<&'data [u8]> {
        let (ptr, len, _) = read_var_len_value(data);
        Ok(slice::from_raw_parts(ptr, len))
    }

    unsafe fn write(self, data: *mut u8) -> Result<()> {
        if self.len() > u32::max_value() as usize {
            return Err(Error::InvalidArgument(
                    "value length may not exceed u32::max_value()".to_string()));
        }

        write_var_len_value(data, self.as_ptr(), self.len(), 0);
        Ok(())
    }
}

impl <'data> Value<'data> for Vec<u8> {
    const DATA_TYPE: DataType = DataType::Binary;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary ||
                                                    data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }

    unsafe fn read(data: *const u8) -> Result<Vec<u8>> {
        Ok(<&[u8] as Value>::read(data)?.to_owned())
    }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        if self.capacity() > u32::max_value() as usize {
            return Err(Error::InvalidArgument(
                    "owned value capacity may not exceed u32::max_value()".to_string()));
        }

        write_var_len_value(data, self.as_ptr(), self.len(), self.capacity());
        mem::forget(self);
        Ok(())
    }
}

impl <'data> Value<'data> for &'data str {
    const DATA_TYPE: DataType = DataType::String;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String
                                                || data_type == DataType::Binary }
    unsafe fn read(data: *const u8) -> Result<&'data str> {
        Ok(str::from_utf8(<&[u8] as Value>::read(data)?)?)
    }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        self.as_bytes().write(data)
    }
}

impl <'data> Value<'data> for String {
    const DATA_TYPE: DataType = DataType::String;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String
                                                || data_type == DataType::Binary }
    unsafe fn read(data: *const u8) -> Result<String> { Ok(<&str as Value>::read(data)?.to_owned()) }
    unsafe fn write(self, data: *mut u8) -> Result<()> { self.into_bytes().write(data) }
}

impl <'data, V> Value<'data> for Option<V> where V: Value<'data> {
    const DATA_TYPE: DataType = V::DATA_TYPE;
    fn can_read_from(data_type: DataType) -> bool { V::can_read_from(data_type) }
    fn can_write_to(data_type: DataType) -> bool { V::can_write_to(data_type) }
    fn is_nullable() -> bool { true }
    fn is_null(&self) -> bool { self.is_none() }
    unsafe fn read(data: *const u8) -> Result<Option<V>> { Ok(Some(V::read(data)?)) }
    unsafe fn write(self, data: *mut u8) -> Result<()> {
        if let Some(value) = self {
            value.write(data)?;
        }
        Ok(())
    }
    fn null() -> Option<Self> { Some(None) }
}

unsafe fn write_var_len_value(data: *mut u8, ptr: *const u8, len: usize, cap: usize) {
    debug_assert!(len < u32::max_value() as usize,
                  "value length may not exceed u32::max_value()");
    debug_assert!(cap == 0 || cap < u32::max_value() as usize,
                  "owned value capacity may not exceed u32::max_value()");
    debug_assert!(cap == 0 || len <= cap,
                  "owned value capacity may not exceed u32::max_value()");

    let data = data as *mut u64;

    // The pointer.
    let ptr = ptr as u64;

    // The (length, capacity) pair as a u64.
    let len_cap = (len as u64) | ((cap as u64) << 32);

    data.offset(0).write_unaligned(ptr.to_le());
    data.offset(1).write_unaligned(len_cap.to_le());
}

unsafe fn read_var_len_value(data: *const u8) -> (*const u8, usize, usize) {
    let data = data as *const u64;

    // Read the pointer.
    let ptr = data.offset(0).read_unaligned().to_le() as *const u8;

    // Read the (length, capacity) pair, and separate.
    let len_cap = data.offset(1).read_unaligned().to_le();
    let len = len_cap as u32 as usize;
    let cap = (len_cap >> 32) as usize;

    debug_assert!(cap == 0 || len <= cap, "owned value length exceeds the capacity");

    (ptr, len, cap)
}

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

// TODO: unset/unset_by_name.
impl <'data> Row<'data> {

    /// Creates a new mutable partial row.
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

    /// Sets the value of the column at index `idx`.
    ///
    /// Returns an error if the column does not exist, or the value's type is wrong.
    pub fn set<V>(&mut self, idx: usize, value: V) -> Result<&mut Self> where V: Value<'data> {
        self.check_column_for_write::<V>(idx)?;
        unsafe {
            self.set_unchecked(idx, value)
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
                                   -> Result<&mut Self> where V: Value<'data> {
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

            value.write(data.offset(self.schema.column_offset(idx)))?;
        }
        Ok(self)
    }

    /// Sets the column at index `idx` to null.
    ///
    /// Returns an error if the column does not exist, or the column is not nullable.
    pub fn set_null(&mut self, idx: usize) -> Result<&mut Row<'data>> {
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
    /// The result is undefined if the `idx` is not valid.
    #[inline]
    unsafe fn is_null_unchecked(&self, idx: usize) -> bool {
        self.schema.has_nullable_columns()
            && bitmap_get(self.data().offset(self.is_null_offset() as isize), idx)
    }

    /// Returns `true` if the column at index `idx` is null.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_null(&self, idx: usize) -> Result<bool> {
        self.check_index(idx)?;
        Ok(unsafe { self.is_null_unchecked(idx) })
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
    /// The result is undefined if the `idx` is not valid.
    #[inline]
    unsafe fn is_set_unchecked(&self, idx: usize) -> bool {
        self.is_contiguous_row()
            || bitmap_get(self.data().offset(self.is_set_offset()), idx)
    }

    /// Returns `true` if the column at index `idx` is set.
    ///
    /// Returns an error if the column does not exist.
    pub fn is_set(&self, idx: usize) -> Result<bool> {
        self.check_index(idx)?;
        Ok(unsafe { self.is_set_unchecked(idx) })
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

    /// Returns the offset of the is-null bitmap in the data array.
    #[inline]
    fn is_null_offset(&self) -> isize {
        debug_assert!(self.schema.has_nullable_columns());
        self.schema.row_len() as isize
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
    fn is_partial_row(&self) -> bool {
        self.tagged_ptr & 1 == 1
    }

    #[inline]
    fn is_contiguous_row(&self) -> bool {
        !self.is_partial_row()
    }

    fn check_index(&self, idx: usize) -> Result<()> {
        if idx >= self.schema.columns().len() {
            Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                               idx, self.schema)))
        } else {
            Ok(())
        }
    }

    #[inline]
    fn data(&self) -> *const u8 {
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
        self.check_index(idx)?;
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
        self.check_index(idx)?;
        let column = &self.schema.columns()[idx];
        if !column.is_nullable() {
            return Err(Error::InvalidArgument(format!("column {:?} is not nullable", column)));
        }
        Ok(())
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column_for_read<V>(&self, idx: usize) -> Result<()> where V: Value<'data> {
        self.check_index(idx)?;
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
        let mut row = schema.new_row2();
        for (idx, column) in schema.columns().iter().enumerate() {
            // TODO: set null values.
            unsafe {
                // Use set_unchecked since the column type is already checked.
                match column.data_type() {
                    DataType::Bool => row.set_unchecked(idx, bool::arbitrary(g)).unwrap(),
                    DataType::Int8 => row.set_unchecked(idx, i8::arbitrary(g)).unwrap(),
                    DataType::Int16 => row.set_unchecked(idx, i16::arbitrary(g)).unwrap(),
                    DataType::Int32 => row.set_unchecked(idx, i32::arbitrary(g)).unwrap(),
                    DataType::Int64 => row.set_unchecked(idx, i64::arbitrary(g)).unwrap(),
                    DataType::Timestamp => row.set_unchecked(idx, i64::arbitrary(g)).unwrap(),
                    DataType::Float => row.set_unchecked(idx, f32::arbitrary(g)).unwrap(),
                    DataType::Double => row.set_unchecked(idx, f64::arbitrary(g)).unwrap(),
                    DataType::Binary => row.set_unchecked(idx, Vec::arbitrary(g)).unwrap(),
                    DataType::String => row.set_unchecked(idx, String::arbitrary(g)).unwrap(),
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
                util::fmt_cell2(f, self.0, self.1)
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

#[cfg(test)]
mod tests {

    use quickcheck::{quickcheck, TestResult, StdGen};
    use rand;
    use schema;
    use super::*;

    #[test]
    fn test_get_set() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row2();

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

    /*
    #[test]
    fn test_eq() {
        let schema = schema::tests::all_types_schema();
        let mut a = schema.new_row2();
        let mut b = schema.new_row2();
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
    */

    #[test]
    fn test_null_values() {
        let schema = schema::tests::all_types_schema();
        let mut row = schema.new_row2();
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
        let mut row = schema.new_row2();

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
        let mut row = schema.new_row2();

        row.set::<i32>(0, 12).unwrap();
        assert!(row.is_set(0).unwrap_or(false));
        assert_eq!(12, row.get::<i32>(0).unwrap());

        // Test a borrowed string.
        row.set(10, "foo_borrowed").unwrap();
        assert_eq!("foo_borrowed".to_owned(), row.get::<String>(10).unwrap());

        // Test an owned string.
        row.set(10, "foo_owned".to_owned()).unwrap();
        assert_eq!("foo_owned", row.get::<&str>(10).unwrap());

        assert_eq!("{key: 12, string: \"foo_owned\"}",
                   &format!("{:?}", row));

        assert_eq!("{key: 12, string: \"foo_owned\"}",
                   &format!("{:?}", row.clone()));

        // Test another borrowed string to ensure the owned string is deallocated.
        row.set(10, "foo_borrowed2").unwrap();
        assert_eq!("foo_borrowed2", row.get::<&str>(10).unwrap());

        row.set_null_by_name("nullable_i32").unwrap();

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
}
