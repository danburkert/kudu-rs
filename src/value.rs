use std::str;
use std::time::SystemTime;

use byteorder::{ByteOrder, LittleEndian};

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
    #[doc(hidden)]
    fn copy_data(&self, _dest: &mut [u8]) { unreachable!() }
    #[doc(hidden)]
    fn indirect_data(self) -> Vec<u8> { unreachable!() }
    #[doc(hidden)]
    fn from_data(data: &'data [u8]) -> Result<Self>;
    #[doc(hidden)]
    fn from_null() -> Self { unreachable!() }
}

impl <'data> Value<'data> for bool {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Bool }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Bool }
    fn data_type() -> DataType { DataType::Bool }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = if *self { 1 } else { 0 } }
    fn from_data(data: &[u8]) -> Result<bool> { if data[0] == 0 { Ok(false) } else { Ok(true) } }
}

impl <'data> Value<'data> for i8 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int8 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int8 }
    fn data_type() -> DataType { DataType::Int8 }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = *self as u8}
    fn from_data(data: &[u8]) -> Result<i8> { Ok(data[0] as i8) }
}

impl <'data> Value<'data> for i16 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int16 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int16 }
    fn data_type() -> DataType { DataType::Int16 }
    fn size() -> usize { 2 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i16(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i16> { Ok(LittleEndian::read_i16(data)) }
}

impl <'data> Value<'data> for i32 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int32 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int32 }
    fn data_type() -> DataType { DataType::Int32 }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i32(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i32> { Ok(LittleEndian::read_i32(data)) }
}

impl <'data> Value<'data> for i64 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Int64 || data_type == DataType::Timestamp}
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Int64 || data_type == DataType::Timestamp }
    fn data_type() -> DataType { DataType::Int64 }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i64> { Ok(LittleEndian::read_i64(data)) }
}

impl <'data> Value<'data> for SystemTime {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Timestamp || data_type == DataType::Int64 }
    fn data_type() -> DataType { DataType::Timestamp }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, time_to_us(self)) }
    fn from_data(data: &[u8]) -> Result<SystemTime> { Ok(us_to_time(LittleEndian::read_i64(data))) }
}

impl <'data> Value<'data> for f32 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Float }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Float }
    fn data_type() -> DataType { DataType::Float }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f32(dest, *self) }
    fn from_data(data: &[u8]) -> Result<f32> { Ok(LittleEndian::read_f32(data)) }
}

impl <'data> Value<'data> for f64 {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Double }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Double }
    fn data_type() -> DataType { DataType::Double }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f64(dest, *self) }
    fn from_data(data: &[u8]) -> Result<f64> { Ok(LittleEndian::read_f64(data)) }
}

impl <'data> Value<'data> for &'data [u8] {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary ||
                                                    data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }
    fn data_type() -> DataType { DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.to_owned() }
    fn from_data(data: &'data [u8]) -> Result<&[u8]> { Ok(data) }
}

impl <'data> Value<'data> for Vec<u8> {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::Binary ||
                                                    data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::Binary }
    fn data_type() -> DataType { DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self }
    fn from_data(data: &[u8]) -> Result<Vec<u8>> { Ok(data.to_owned()) }
}

impl <'data> Value<'data> for &'data str {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String ||
                                                   data_type == DataType::Binary }
    fn data_type() -> DataType { DataType::String }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.as_bytes().to_owned() }
    fn from_data(data: &[u8]) -> Result<&str> { str::from_utf8(data).map_err(From::from) }
}

impl <'data> Value<'data> for String {
    fn can_read_from(data_type: DataType) -> bool { data_type == DataType::String }
    fn can_write_to(data_type: DataType) -> bool { data_type == DataType::String ||
                                                   data_type == DataType::Binary }
    fn data_type() -> DataType { DataType::String }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.into_bytes() }
    fn from_data(data: &[u8]) -> Result<String> { str::from_utf8(data).map(str::to_owned).map_err(From::from) }
}

impl <'data, V> Value<'data> for Option<V> where V: Value<'data> {
    fn data_type() -> DataType { V::data_type() }
    fn can_read_from(data_type: DataType) -> bool { V::can_read_from(data_type) }
    fn can_write_to(data_type: DataType) -> bool { V::can_write_to(data_type) }
    fn size() -> usize { V::size() }
    fn is_var_len() -> bool { V::is_var_len() }
    fn is_nullable() -> bool { true }
    fn is_null(&self) -> bool { self.is_none() }
    fn indirect_data(self) -> Vec<u8> { self.unwrap().indirect_data() }
    fn from_data(data: &'data [u8]) -> Result<Option<V>> { V::from_data(data).map(Some) }
    fn from_null() -> Option<V> { None }
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
