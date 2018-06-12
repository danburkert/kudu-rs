use std::mem;
use std::slice;
use std::str;
use std::time::SystemTime;

use DataType;
use Result;
use util::{time_to_us, us_to_time};

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
    unsafe fn write(self, *mut u8);

    /// Creates a null value, or `None` if the value type is not nullable.
    #[doc(hidden)]
    fn null() -> Option<Self> { None }
}

impl <'data> Value<'data> for bool {
    const DATA_TYPE: DataType = DataType::Bool;
    unsafe fn read(data: *const u8) -> Result<bool> { Ok(*data != 0) }
    unsafe fn write(self, data: *mut u8) {
        *data = if self { 1 } else { 0 };
    }
}

macro_rules! int_value {
    ($ty:ty, $data_type:ident) => (
        impl <'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            unsafe fn read(data: *const u8) -> Result<$ty> {
                Ok((data as *const $ty).read_unaligned().to_le())
            }
            unsafe fn write(self, data: *mut u8) {
                (data as *mut $ty).write_unaligned(self.to_le());
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
    unsafe fn write(self, data: *mut u8) {
        (data as *mut i64).write_unaligned(self.to_le());
    }
}

macro_rules! float_value {
    ($ty:ident, $data_type:ident, $unsigned_ty:ty) => (
        impl <'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            unsafe fn read(data: *const u8) -> Result<$ty> {
                Ok($ty::from_bits((data as *const $unsigned_ty).read_unaligned().to_le()))
            }
            unsafe fn write(self, data: *mut u8) {
                (data as *mut $unsigned_ty).write_unaligned(self.to_bits().to_le());
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
    unsafe fn read(data: *const u8) -> Result<SystemTime> { Ok(us_to_time(i64::read(data)?)) }
    unsafe fn write(self, data: *mut u8) {
        time_to_us(self).write(data);
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

    unsafe fn write(self, data: *mut u8) {
        assert!(self.len() <= u32::max_value() as usize,
                "value length must not exceed u32::max_value()");
        write_var_len_value(data, self.as_ptr(), self.len(), 0);
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
    unsafe fn write(self, data: *mut u8) {
        assert!(self.capacity() <= u32::max_value() as usize,
                "owned value capacity must not exceed u32::max_value()");
        write_var_len_value(data, self.as_ptr(), self.len(), self.capacity());
        mem::forget(self);
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
    unsafe fn write(self, data: *mut u8) {
        self.as_bytes().write(data)
    }
}

impl <'data> Value<'data> for String {
    const DATA_TYPE: DataType = DataType::String;
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String
                                                || data_type == DataType::Binary }
    unsafe fn read(data: *const u8) -> Result<String> { Ok(<&str as Value>::read(data)?.to_owned()) }
    unsafe fn write(self, data: *mut u8) { self.into_bytes().write(data) }
}

impl <'data, V> Value<'data> for Option<V> where V: Value<'data> {
    const DATA_TYPE: DataType = V::DATA_TYPE;
    fn can_read_from(data_type: DataType) -> bool { V::can_read_from(data_type) }
    fn can_write_to(data_type: DataType) -> bool { V::can_write_to(data_type) }
    fn is_nullable() -> bool { true }
    fn is_null(&self) -> bool { self.is_none() }
    unsafe fn read(data: *const u8) -> Result<Option<V>> { Ok(Some(V::read(data)?)) }
    unsafe fn write(self, data: *mut u8) {
        if let Some(value) = self {
            value.write(data);
        }
    }
    fn null() -> Option<Self> { Some(None) }
}

pub(crate) unsafe fn write_var_len_value(data: *mut u8, ptr: *const u8, len: usize, cap: usize) {
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

pub(crate) unsafe fn read_var_len_value(data: *const u8) -> (*const u8, usize, usize) {
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
