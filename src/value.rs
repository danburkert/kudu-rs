use std::str;
use std::time::SystemTime;

use byteorder::{ByteOrder, LittleEndian};

use DataType;
use Result;
use util::{time_to_us, us_to_time};

pub trait Value<'a>: Sized {
    fn data_type() -> DataType;
    fn size() -> usize;
    fn is_var_len() -> bool { false }
    fn is_nullable() -> bool { false }
    fn is_null(&self) -> bool { false }
    fn copy_data(&self, _dest: &mut [u8]) { unreachable!() }
    fn indirect_data(self) -> Vec<u8> { unreachable!() }
    fn from_data(data: &'a [u8]) -> Result<Self>;
    fn from_null() -> Self { unreachable!() }
}

impl <'a> Value<'a> for bool {
    fn data_type() -> DataType { DataType::Bool }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = if *self { 1 } else { 0 } }
    fn from_data(data: &[u8]) -> Result<bool> { if data[0] == 0 { Ok(false) } else { Ok(true) } }
}

impl <'a> Value<'a> for i8 {
    fn data_type() -> DataType { DataType::Int8 }
    fn size() -> usize { 1 }
    fn copy_data(&self, dest: &mut [u8]) { dest[0] = *self as u8}
    fn from_data(data: &[u8]) -> Result<i8> { Ok(data[0] as i8) }
}

impl <'a> Value<'a> for i16 {
    fn data_type() -> DataType { DataType::Int16 }
    fn size() -> usize { 2 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i16(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i16> { Ok(LittleEndian::read_i16(data)) }
}

impl <'a> Value<'a> for i32 {
    fn data_type() -> DataType { DataType::Int32 }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i32(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i32> { Ok(LittleEndian::read_i32(data)) }
}

impl <'a> Value<'a> for i64 {
    fn data_type() -> DataType { DataType::Int64 }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, *self) }
    fn from_data(data: &[u8]) -> Result<i64> { Ok(LittleEndian::read_i64(data)) }
}

impl <'a> Value<'a> for SystemTime {
    fn data_type() -> DataType { DataType::Timestamp }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_i64(dest, time_to_us(self)) }
    fn from_data(data: &[u8]) -> Result<SystemTime> { Ok(us_to_time(LittleEndian::read_i64(data))) }
}

impl <'a> Value<'a> for f32 {
    fn data_type() -> DataType { DataType::Float }
    fn size() -> usize { 4 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f32(dest, *self) }
    fn from_data(data: &[u8]) -> Result<f32> { Ok(LittleEndian::read_f32(data)) }
}

impl <'a> Value<'a> for f64 {
    fn data_type() -> DataType { DataType::Double }
    fn size() -> usize { 8 }
    fn copy_data(&self, dest: &mut [u8]) { LittleEndian::write_f64(dest, *self) }
    fn from_data(data: &[u8]) -> Result<f64> { Ok(LittleEndian::read_f64(data)) }
}

impl <'a> Value<'a> for &'a [u8] {
    fn data_type() -> DataType { DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.to_owned() }
    fn from_data(data: &'a [u8]) -> Result<&[u8]> { Ok(data) }
}

impl <'a> Value<'a> for Vec<u8> {
    fn data_type() -> DataType { DataType::Binary }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self }
    fn from_data(data: &[u8]) -> Result<Vec<u8>> { Ok(data.to_owned()) }
}

impl <'a> Value<'a> for &'a str {
    fn data_type() -> DataType { DataType::String }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.as_bytes().to_owned() }
    fn from_data(data: &[u8]) -> Result<&str> { str::from_utf8(data).map_err(From::from) }
}

impl <'a> Value<'a> for String {
    fn data_type() -> DataType { DataType::String }
    fn size() -> usize { 16 }
    fn is_var_len() -> bool { true }
    fn indirect_data(self) -> Vec<u8> { self.into_bytes() }
    fn from_data(data: &[u8]) -> Result<String> { str::from_utf8(data).map(str::to_owned).map_err(From::from) }
}

impl <'a, V> Value<'a> for Option<V> where V: Value<'a> {
    fn data_type() -> DataType { V::data_type() }
    fn size() -> usize { V::size() }
    fn is_var_len() -> bool { V::is_var_len() }
    fn is_nullable() -> bool { true }
    fn is_null(&self) -> bool { self.is_none() }
    fn indirect_data(self) -> Vec<u8> { self.unwrap().indirect_data() }
    fn from_data(data: &'a [u8]) -> Result<Option<V>> { V::from_data(data).map(Some) }
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
