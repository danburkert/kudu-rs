use std::str;
use std::time::SystemTime;
use std::slice;

use byteorder::{ByteOrder, LittleEndian, NativeEndian};

use DataType;
use Result;
use util::{time_to_us, us_to_time};

/// Marker trait for types which can be stored in a Kudu column.
pub trait Value<'data>: Sized {
    #[doc(hidden)]
    fn data_type() -> DataType;
    /// Returns `true` if the value type can be read from a column of the provided data type.
    #[doc(hidden)]
    fn can_read_from(data_type: DataType) -> bool;
    /// Returns `true` if the value type can be written to a column of the provided data type.
    #[doc(hidden)]
    fn can_write_to(data_type: DataType) -> bool;
    #[doc(hidden)]
    fn size() -> usize;
    #[doc(hidden)]
    fn is_var_len() -> bool { false }
    #[doc(hidden)]
    fn is_nullable() -> bool { false }
    #[doc(hidden)]
    fn is_null(&self) -> bool { false }

    /// Encode the value to a buffer.
    ///
    /// The buffer must be of at least `size` length.
    #[doc(hidden)]
    fn copy_data(&self, dest: &mut [u8]);

    /// Returns the owned data, or `None` if this value is not owned.
    #[doc(hidden)]
    fn owned_data(self) -> Option<Vec<u8>> { None }

    /// Decode the value from the buff.
    ///
    /// Must only be used on trusted data which has been created with `copy_data`.
    unsafe fn from_data(data: &'data [u8]) -> Self;

    /// Creates a null value, or `None` if the value type is not nullable.
    #[doc(hidden)]
    fn null() -> Option<Self> { None }
}

impl <'data> Value<'data> for bool {
    fn data_type() -> DataType { DataType::Bool }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Bool }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Bool }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = if *self { 1 } else { 0 } }
    unsafe fn from_data(data: &[u8]) -> bool { if data[0] == 0 { false } else { true } }
}

impl <'data> Value<'data> for i8 {
    fn data_type() -> DataType { DataType::Int8 }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int8 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int8 }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = *self as u8}
    unsafe fn from_data(data: &[u8]) -> i8 { data[0] as i8 }
}

impl <'data> Value<'data> for i16 {
    fn data_type() -> DataType { DataType::Int16 }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int16 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int16 }
    fn size() -> usize { 2 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i16(dest, *self) }
    unsafe fn from_data(data: &[u8]) -> i16 { LittleEndian::read_i16(data) }
}

impl <'data> Value<'data> for i32 {
    fn data_type() -> DataType { DataType::Int32 }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int32 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int32 }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i32(dest, *self) }
    unsafe fn from_data(data: &[u8]) -> i32 { LittleEndian::read_i32(data) }
}

impl <'data> Value<'data> for i64 {
    fn data_type() -> DataType { DataType::Int64 }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int64 || data_type == DataType::Timestamp}
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int64 || data_type == DataType::Timestamp }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, *self) }
    unsafe fn from_data(data: &[u8]) -> i64 { LittleEndian::read_i64(data) }
}

impl <'data> Value<'data> for SystemTime {
    fn data_type() -> DataType { DataType::Timestamp }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, time_to_us(self)) }
    unsafe fn from_data(data: &[u8]) -> SystemTime { us_to_time(LittleEndian::read_i64(data)) }
}

impl <'data> Value<'data> for f32 {
    fn data_type() -> DataType { DataType::Float }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Float }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Float }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f32(dest, *self) }
    unsafe fn from_data(data: &[u8]) -> f32 { LittleEndian::read_f32(data) }
}

impl <'data> Value<'data> for f64 {
    fn data_type() -> DataType { DataType::Double }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Double }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Double }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f64(dest, *self) }
    unsafe fn from_data(data: &[u8]) -> f64 { LittleEndian::read_f64(data) }
}

impl <'data> Value<'data> for &'data [u8] {
    fn data_type() -> DataType { DataType::Binary }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary ||
                                                    data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn copy_data(&self, dest: &mut [u8]) {
        NativeEndian::write_u64(&mut dest[8..16], self.len() as u64);
        NativeEndian::write_u64(dest, self.as_ptr() as u64);
    }
    fn owned_data(self) -> Option<Vec<u8>> { None }
    unsafe fn from_data(data: &[u8]) -> &[u8] {
        let len = NativeEndian::read_u64(&data[8..16]) as usize;
        let ptr = NativeEndian::read_u64(data) as *const u8;
        slice::from_raw_parts(ptr, len)
    }
}

impl <'data> Value<'data> for Vec<u8> {
    fn data_type() -> DataType { DataType::Binary }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary ||
                                                    data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn copy_data(&self, dest: &mut [u8]) { self.as_slice().copy_data(dest) }
    fn owned_data(self) -> Option<Vec<u8>> { Some(self) }
    unsafe fn from_data(data: &[u8]) -> Vec<u8> { <&[u8] as Value>::from_data(data).to_owned() }
}

impl <'data> Value<'data> for &'data str {
    fn data_type() -> DataType { DataType::String }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String ||
                                                   data_type == DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn copy_data(&self, dest: &mut [u8]) { self.as_bytes().copy_data(dest) }
    unsafe fn from_data(data: &[u8]) -> &str {
        str::from_utf8_unchecked(<&[u8] as Value>::from_data(data))
    }
}

impl <'data> Value<'data> for String {
    fn data_type() -> DataType { DataType::String }
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String ||
                                                   data_type == DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn owned_data(self) -> Option<Vec<u8>> { Some(self.into_bytes()) }
    fn copy_data(&self, dest: &mut [u8]) { self.as_bytes().copy_data(dest) }
    unsafe fn from_data(data: &[u8]) -> String { <&str as Value>::from_data(data).to_owned() }
}

impl <'data, V> Value<'data> for Option<V> where V: Value<'data> {
    fn data_type() -> DataType { V::data_type() }
    fn can_read_from(data_type: DataType) -> bool { V::can_read_from(data_type) }
    fn can_write_to(data_type: DataType) -> bool { V::can_write_to(data_type) }
    fn size() -> usize { V::size() }
    fn is_var_len() -> bool { V::is_var_len() }
    fn is_nullable() -> bool { true }
    fn is_null(&self) -> bool { self.is_none() }
    fn owned_data(self) -> Option<Vec<u8>> { self.and_then(V::owned_data) }
    fn copy_data(&self, dest: &mut [u8]) {
        match *self {
            Some(ref value) => {
                value.copy_data(dest);
            },
            None => (),
        }
    }
    unsafe fn from_data(data: &'data [u8]) -> Option<V> { Some(V::from_data(data)) }
    fn null() -> Option<Self> { Some(None) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DataType;

    #[test]
    fn test_foo() {
        assert_eq!(DataType::Bool, bool::data_type());
    }
}
