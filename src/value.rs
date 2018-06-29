use std::mem;
use std::slice;
use std::str;
use std::time::SystemTime;
use std::{f32, f64};

use ieee754::Ieee754;

use util::{time_to_us, us_to_time};
use DataType;
use PhysicalType;
use Result;

/// Marker trait for types which can be stored in a Kudu column.
///
/// Idiomatic Rust types which correspond to the Kudu column types implement `Value`, for example
/// Kudu's 8 byte integer type maps to a Rust `i64`, Kudu's string type maps to `&str` and
/// `String`, and Kudu's timestamp type maps to [[SystemTime]].
///
/// Inline vs Cell format.
pub trait Value<'data>: Sized {
    /// The Kudu column type corresponding to the value type.
    const DATA_TYPE: DataType;
    const PHYSICAL_TYPE: PhysicalType;

    /// Returns `true` if the value type can be read from a column of the provided data type.
    #[inline]
    fn can_read_from(data_type: DataType) -> bool {
        Self::PHYSICAL_TYPE == data_type.physical_type()
    }

    /// Returns `true` if the value type can be written to a column of the provided data type.
    #[inline]
    fn can_write_to(data_type: DataType) -> bool {
        Self::PHYSICAL_TYPE == data_type.physical_type()
    }

    /// Returns `true` if the value type is nullable.
    #[inline]
    fn is_nullable() -> bool {
        false
    }

    /// Returns `true` if the value is NULL.
    #[inline]
    fn is_null(&self) -> bool {
        false
    }

    /// Returns `true` if the value can be compared with other values of the same type according to
    /// the Kudu value rules.
    #[inline]
    fn is_comparable(&self) -> bool {
        !self.is_null()
    }

    /// Creates a null value, or `None` if the value type is not nullable.
    #[inline]
    fn null() -> Option<Self> {
        None
    }

    /// Encodes the value to a buffer.
    fn encode(self) -> Vec<u8> {
        let len = mem::size_of::<Self>();
        let mut buf = Vec::with_capacity(len);
        unsafe {
            buf.set_len(len);
            self.write_cell(buf.as_mut_ptr());
        }
        buf
    }

    /// Encodes the incremented value to a buffer.
    fn encode_next(self) -> Option<Vec<u8>>;

    /// Decodes an instance of the value from a buffer.
    ///
    /// # Unsafety
    ///
    /// The data must have been encoded from a value of the same type with [Value::encode] or
    /// [Value::encode_next].
    unsafe fn decode(data: &'data [u8]) -> Self {
        debug_assert_eq!(data.len(), mem::size_of::<Self>());
        Self::read_cell(data.as_ptr()).unwrap()
    }

    /// Reads an instance of the value type from a raw row data array.
    unsafe fn read_cell(data: *const u8) -> Result<Self>;

    /// Writes the value to a raw row data array.
    unsafe fn write_cell(self, *mut u8);
}

impl<'data> Value<'data> for bool {
    const DATA_TYPE: DataType = DataType::Bool;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Bool;
    unsafe fn read_cell(data: *const u8) -> Result<bool> {
        Ok(*data != 0)
    }
    unsafe fn write_cell(self, data: *mut u8) {
        *data = self as u8;
    }
    fn encode_next(self) -> Option<Vec<u8>> {
        if self {
            None
        } else {
            Some(true.encode())
        }
    }
}

macro_rules! int_value {
    ($ty:ty, $data_type:ident) => {
        impl<'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            const PHYSICAL_TYPE: PhysicalType = PhysicalType::$data_type;
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
            unsafe fn read_cell(data: *const u8) -> Result<$ty> {
                Ok((data as *const $ty).read_unaligned().to_le())
            }
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
            unsafe fn write_cell(self, data: *mut u8) {
                (data as *mut $ty).write_unaligned(self.to_le());
            }
            fn encode_next(self) -> Option<Vec<u8>> {
                self.checked_add(1).map(Self::encode)
            }
        }
    };
}
int_value!(i8, Int8);
int_value!(i16, Int16);
int_value!(i32, Int32);
int_value!(i64, Int64);

macro_rules! float_value {
    ($ty:ident, $data_type:ident, $unsigned_ty:ty) => {
        impl<'data> Value<'data> for $ty {
            const DATA_TYPE: DataType = DataType::$data_type;
            const PHYSICAL_TYPE: PhysicalType = PhysicalType::$data_type;
            fn is_comparable(&self) -> bool {
                !self.is_nan()
            }
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
            unsafe fn read_cell(data: *const u8) -> Result<$ty> {
                Ok($ty::from_bits(
                    (data as *const $unsigned_ty).read_unaligned().to_le(),
                ))
            }
            #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
            unsafe fn write_cell(self, data: *mut u8) {
                (data as *mut $unsigned_ty).write_unaligned(self.to_bits().to_le());
            }
            fn encode_next(self) -> Option<Vec<u8>> {
                if self == $ty::INFINITY || self.is_nan() {
                    None
                } else {
                    Some(self.next().encode())
                }
            }
        }
    };
}
float_value!(f32, Float, u32);
float_value!(f64, Double, u64);

impl<'data> Value<'data> for SystemTime {
    const DATA_TYPE: DataType = DataType::Timestamp;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Int64;
    unsafe fn read_cell(data: *const u8) -> Result<SystemTime> {
        Ok(us_to_time(i64::read_cell(data)?))
    }
    unsafe fn write_cell(self, data: *mut u8) {
        time_to_us(self).write_cell(data);
    }
    fn encode_next(self) -> Option<Vec<u8>> {
        time_to_us(self).encode_next()
    }
}

impl<'data> Value<'data> for &'data [u8] {
    const DATA_TYPE: DataType = DataType::Binary;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Binary;

    fn can_write_to(data_type: DataType) -> bool {
        data_type == DataType::Binary
    }
    unsafe fn read_cell(data: *const u8) -> Result<&'data [u8]> {
        let (ptr, len, _) = read_var_len_value(data);
        Ok(slice::from_raw_parts(ptr, len))
    }

    unsafe fn write_cell(self, data: *mut u8) {
        assert!(
            self.len() <= u32::max_value() as usize,
            "value length must not exceed u32::max_value()"
        );
        write_var_len_value(data, self.as_ptr(), self.len(), 0);
    }

    fn encode(self) -> Vec<u8> {
        self.to_owned()
    }

    fn encode_next(self) -> Option<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.len() + 1);
        buf.extend_from_slice(self);
        buf.push(0);
        Some(buf)
    }

    unsafe fn decode(data: &'data [u8]) -> &[u8] {
        data
    }
}

impl<'data> Value<'data> for Vec<u8> {
    const DATA_TYPE: DataType = DataType::Binary;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Binary;

    fn can_write_to(data_type: DataType) -> bool {
        data_type == DataType::Binary
    }
    unsafe fn read_cell(data: *const u8) -> Result<Vec<u8>> {
        Ok(<&[u8] as Value>::read_cell(data)?.to_owned())
    }
    unsafe fn write_cell(self, data: *mut u8) {
        assert!(
            self.capacity() <= u32::max_value() as usize,
            "owned value capacity must not exceed u32::max_value()"
        );
        write_var_len_value(data, self.as_ptr(), self.len(), self.capacity());
        mem::forget(self);
    }

    fn encode(self) -> Vec<u8> {
        self
    }

    fn encode_next(mut self) -> Option<Vec<u8>> {
        self.reserve_exact(1);
        self.push(0);
        Some(self)
    }

    unsafe fn decode(data: &'data [u8]) -> Vec<u8> {
        data.to_owned()
    }
}

impl<'data> Value<'data> for &'data str {
    const DATA_TYPE: DataType = DataType::String;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Binary;
    fn can_read_from(data_type: DataType) -> bool {
        data_type == DataType::String
    }
    unsafe fn read_cell(data: *const u8) -> Result<&'data str> {
        Ok(str::from_utf8(<&[u8] as Value>::read_cell(data)?)?)
    }
    unsafe fn write_cell(self, data: *mut u8) {
        self.as_bytes().write_cell(data)
    }
    fn encode(self) -> Vec<u8> {
        self.as_bytes().encode()
    }
    fn encode_next(self) -> Option<Vec<u8>> {
        self.as_bytes().encode_next()
    }

    unsafe fn decode(data: &'data [u8]) -> &'data str {
        str::from_utf8_unchecked(data)
    }
}

impl<'data> Value<'data> for String {
    const DATA_TYPE: DataType = DataType::String;
    const PHYSICAL_TYPE: PhysicalType = PhysicalType::Binary;
    fn can_read_from(data_type: DataType) -> bool {
        data_type == DataType::String
    }
    unsafe fn read_cell(data: *const u8) -> Result<String> {
        Ok(<&str as Value>::read_cell(data)?.to_owned())
    }
    unsafe fn write_cell(self, data: *mut u8) {
        self.into_bytes().write_cell(data)
    }
    fn encode(self) -> Vec<u8> {
        self.into_bytes().encode()
    }
    fn encode_next(self) -> Option<Vec<u8>> {
        self.into_bytes().encode_next()
    }
    unsafe fn decode(data: &'data [u8]) -> String {
        <&str as Value>::decode(data).to_owned()
    }
}

impl<'data, V> Value<'data> for Option<V>
where
    V: Value<'data>,
{
    const DATA_TYPE: DataType = V::DATA_TYPE;
    const PHYSICAL_TYPE: PhysicalType = V::PHYSICAL_TYPE;
    fn can_read_from(data_type: DataType) -> bool {
        V::can_read_from(data_type)
    }
    fn can_write_to(data_type: DataType) -> bool {
        V::can_write_to(data_type)
    }
    fn is_nullable() -> bool {
        true
    }
    fn is_null(&self) -> bool {
        self.is_none()
    }
    fn is_comparable(&self) -> bool {
        self.as_ref().map_or(false, V::is_comparable)
    }
    unsafe fn read_cell(data: *const u8) -> Result<Option<V>> {
        Ok(Some(V::read_cell(data)?))
    }
    unsafe fn write_cell(self, data: *mut u8) {
        if let Some(value) = self {
            value.write_cell(data);
        }
    }
    fn null() -> Option<Self> {
        Some(None)
    }
    fn encode(self) -> Vec<u8> {
        self.unwrap().encode()
    }
    fn encode_next(self) -> Option<Vec<u8>> {
        self.unwrap().encode_next()
    }
    unsafe fn decode(data: &'data [u8]) -> Option<V> {
        Some(V::decode(data))
    }
}

pub(crate) unsafe fn write_var_len_value(data: *mut u8, ptr: *const u8, len: usize, cap: usize) {
    debug_assert!(
        len < u32::max_value() as usize,
        "value length may not exceed u32::max_value()"
    );
    debug_assert!(
        cap == 0 || cap < u32::max_value() as usize,
        "owned value capacity may not exceed u32::max_value()"
    );
    debug_assert!(
        cap == 0 || len <= cap,
        "owned value capacity may not exceed u32::max_value()"
    );

    #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
    let data = data as *mut u64;

    // The pointer.
    let ptr = ptr as u64;

    // The (length, capacity) pair as a u64.
    let len_cap = (len as u64) | ((cap as u64) << 32);

    data.offset(0).write_unaligned(ptr.to_le());
    data.offset(1).write_unaligned(len_cap.to_le());
}

pub(crate) unsafe fn read_var_len_value(data: *const u8) -> (*const u8, usize, usize) {
    #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
    let data = data as *const u64;

    // Read the pointer.
    let ptr = data.offset(0).read_unaligned().to_le() as *const u8;

    // Read the (length, capacity) pair, and separate.
    let len_cap = data.offset(1).read_unaligned().to_le();
    let len = len_cap as u32 as usize;
    let cap = (len_cap >> 32) as usize;

    debug_assert!(
        cap == 0 || len <= cap,
        "owned value length exceeds the capacity"
    );

    (ptr, len, cap)
}
