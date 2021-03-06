#![allow(dead_code)]

//! Utility functions for working with keys.

use std::{f32, f64, i16, i32, i64, i8};

use byteorder::{BigEndian, ByteOrder, NativeEndian, WriteBytesExt};
use ieee754::Ieee754;

use partition::PartitionKey;
use DataType;
use Error;
use HashPartitionSchema;
use PartitionSchema;
use RangePartitionSchema;
use Result;
use Row;
use Schema;
use Value;

/// Murmur2 hash implementation returning 64-bit hashes.
pub(crate) fn murmur2_64(mut data: &[u8], seed: u64) -> u64 {
    const M: u64 = 0xc6a4_a793_5bd1_e995;
    const R: u8 = 47;

    let mut h: u64 = seed ^ ((data.len() as u64).wrapping_mul(M));

    while data.len() >= 8 {
        let mut k = NativeEndian::read_u64(data);

        k = k.wrapping_mul(M);
        k ^= k >> R;
        k = k.wrapping_mul(M);
        h ^= k;
        h = h.wrapping_mul(M);

        data = &data[8..];
    }

    let len = data.len();
    if len > 6 {
        h ^= u64::from(data[6]) << 48;
    }
    if len > 5 {
        h ^= u64::from(data[5]) << 40;
    }
    if len > 4 {
        h ^= u64::from(data[4]) << 32;
    }
    if len > 3 {
        h ^= u64::from(data[3]) << 24;
    }
    if len > 2 {
        h ^= u64::from(data[2]) << 16;
    }
    if len > 1 {
        h ^= u64::from(data[1]) << 8;
    }
    if len > 0 {
        h ^= u64::from(data[0]);
        h = h.wrapping_mul(M)
    }

    h ^= h >> R;
    h = h.wrapping_mul(M);
    h ^= h >> R;
    h
}

pub(crate) fn encode_primary_key(row: &Row) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    encode_columns(row, 0..row.schema().num_primary_key_columns(), &mut buf)?;
    Ok(buf)
}

pub(crate) fn encode_partition_key(
    partition_schema: &PartitionSchema,
    row: &Row,
) -> Result<PartitionKey> {
    let mut buf = Vec::new();
    for hash_schema in partition_schema.hash_partition_schemas() {
        encode_hash_partition_key(hash_schema, row, &mut buf)?;
    }
    encode_range_partition_key(partition_schema.range_partition_schema(), row, &mut buf)?;
    Ok(buf.into())
}

pub(crate) fn encode_range_partition_key(
    range_schema: &RangePartitionSchema,
    row: &Row,
    buf: &mut Vec<u8>,
) -> Result<()> {
    encode_columns(row, range_schema.columns().iter().cloned(), buf)
}

pub(crate) fn encode_hash_partition_key(
    hash_schema: &HashPartitionSchema,
    row: &Row,
    buf: &mut Vec<u8>,
) -> Result<()> {
    let len = buf.len();
    encode_columns(row, hash_schema.columns().iter().cloned(), buf)?;
    let bucket = murmur2_64(&buf[len..], u64::from(hash_schema.seed()))
        % u64::from(hash_schema.num_buckets());
    buf.truncate(len);
    buf.write_u32::<BigEndian>(bucket as u32).unwrap();
    Ok(())
}

fn encode_columns<I>(row: &Row, idxs: I, buf: &mut Vec<u8>) -> Result<()>
where
    I: Iterator<Item = usize> + ExactSizeIterator,
{
    let columns = idxs.len();
    let mut column = 1;
    for idx in idxs {
        encode_column(row, idx, column == columns, buf)?;
        column += 1;
    }
    Ok(())
}

fn encode_column(row: &Row, idx: usize, is_last: bool, buf: &mut Vec<u8>) -> Result<()> {
    match row.schema().columns()[idx].data_type() {
        DataType::Int8 => buf.push((row.get::<_, i8>(idx)? ^ i8::MIN) as u8),
        DataType::Int16 => buf
            .write_i16::<BigEndian>(row.get::<_, i16>(idx)? ^ i16::MIN)
            .unwrap(),
        DataType::Int32 => buf
            .write_i32::<BigEndian>(row.get::<_, i32>(idx)? ^ i32::MIN)
            .unwrap(),
        DataType::Int64 | DataType::Timestamp => buf
            .write_i64::<BigEndian>(row.get::<_, i64>(idx)? ^ i64::MIN)
            .unwrap(),
        DataType::Binary | DataType::String => encode_binary(row.get(idx)?, is_last, buf),
        DataType::Bool | DataType::Float | DataType::Double => {
            panic!(
                "illegal type {:?} in key",
                row.schema().columns()[idx].data_type()
            );
        }
    }
    Ok(())
}

fn encode_binary(value: &[u8], is_last: bool, buf: &mut Vec<u8>) {
    if is_last {
        buf.extend_from_slice(value);
    } else {
        for split in value.split(|&x| x == 0) {
            buf.extend_from_slice(split);
            buf.extend_from_slice(&[0, 1]);
        }
        let len = buf.len();
        buf[len - 1] = 0;
    }
}

pub(crate) fn decode_primary_key(schema: &Schema, mut key: &[u8]) -> Result<Row<'static>> {
    let mut row = schema.new_row();

    let num_primary_key_columns = row.schema().num_primary_key_columns();
    for idx in 0..num_primary_key_columns {
        key = decode_column(&mut row, idx, idx + 1 == num_primary_key_columns, key)?;
    }

    if !key.is_empty() {
        return Err(Error::Serialization(
            "bytes remaining after decoding all primary key columns".to_owned(),
        ));
    }
    Ok(row)
}

pub(crate) fn decode_range_partition_key(
    schema: &Schema,
    range_partition_schema: &RangePartitionSchema,
    mut key: &[u8],
) -> Result<Row<'static>> {
    let mut row = schema.new_row();
    if range_partition_schema.columns().is_empty() {
        return Ok(row);
    }
    let last_idx = *range_partition_schema.columns().last().unwrap();

    for &idx in range_partition_schema.columns() {
        if key.is_empty() {
            break;
        }
        key = decode_column(&mut row, idx, idx == last_idx, key)?;
    }

    if !key.is_empty() {
        return Err(Error::Serialization(
            "bytes remaining after decoding all partition key columns".to_owned(),
        ));
    }
    Ok(row)
}

fn decode_column<'a>(row: &mut Row, idx: usize, is_last: bool, key: &'a [u8]) -> Result<&'a [u8]> {
    unsafe {
        // Use set_unchecked since the column type is already checked.
        match row.schema().columns()[idx].data_type() {
            DataType::Int8 => {
                row.set_unchecked(idx, (key[0] as i8) ^ i8::MIN);
                Ok(&key[1..])
            }
            DataType::Int16 => {
                row.set_unchecked(idx, BigEndian::read_i16(key) ^ i16::MIN);
                Ok(&key[2..])
            }
            DataType::Int32 => {
                row.set_unchecked(idx, BigEndian::read_i32(key) ^ i32::MIN);
                Ok(&key[4..])
            }
            DataType::Int64 | DataType::Timestamp => {
                row.set_unchecked(idx, BigEndian::read_i64(key) ^ i64::MIN);
                Ok(&key[8..])
            }
            DataType::Binary => {
                let (remaining, value) = decode_binary(key, is_last)?;
                row.set_unchecked(idx, value);
                Ok(remaining)
            }
            DataType::String => {
                let (remaining, value) = decode_binary(key, is_last)?;
                row.set_unchecked(idx, String::from_utf8(value)?);
                Ok(remaining)
            }
            DataType::Bool | DataType::Float | DataType::Double => {
                panic!(
                    "illegal type {:?} in key",
                    row.schema().columns()[idx].data_type()
                );
            }
        }
    }
}

fn decode_binary(mut key: &[u8], is_last: bool) -> Result<(&[u8], Vec<u8>)> {
    if is_last {
        Ok((&[], key.to_owned()))
    } else {
        let mut ret = Vec::new();

        loop {
            match key.iter().position(|&x| x == 0) {
                Some(idx) => match key.get(idx + 1) {
                    Some(&b) => match b {
                        0 => {
                            ret.extend_from_slice(&key[..idx]);
                            key = &key[idx + 2..];
                            break;
                        }
                        1 => {
                            ret.extend_from_slice(&key[..idx + 1]);
                            key = &key[idx + 2..];
                        }
                        _ => {
                            return Err(Error::Serialization(format!(
                                "unexpected binary sequence: {:?}",
                                &[0, b]
                            )))
                        }
                    },
                    None => {
                        return Err(Error::Serialization(
                            "expected binary column separator sequence".to_owned(),
                        ))
                    }
                },
                None => {
                    return Err(Error::Serialization(
                        "expected binary column separator sequence".to_owned(),
                    ))
                }
            }
        }

        Ok((key, ret))
    }
}

/// Increments the value of a cell in a row.
///
/// Returns `true` if the increment succeeds, or `false` if the value is already the maximum
/// or the value is unset or null.
fn increment_cell(row: &mut Row, idx: usize) -> bool {
    if !row.is_set(idx).unwrap() || row.is_null(idx).unwrap() {
        return false;
    }
    match row.schema().columns()[idx].data_type() {
        DataType::Bool => {
            let val: bool = row.get(idx).unwrap();
            if val {
                return false;
            };
            row.set(idx, true).unwrap();
        }
        DataType::Int8 => {
            let val: i8 = row.get(idx).unwrap();
            if val == i8::MAX {
                return false;
            }
            row.set(idx, val + 1).unwrap();
        }
        DataType::Int16 => {
            let val: i16 = row.get(idx).unwrap();
            if val == i16::MAX {
                return false;
            }
            row.set(idx, val + 1).unwrap();
        }
        DataType::Int32 => {
            let val: i32 = row.get(idx).unwrap();
            if val == i32::MAX {
                return false;
            }
            row.set(idx, val + 1).unwrap();
        }
        DataType::Int64 | DataType::Timestamp => {
            let val: i64 = row.get(idx).unwrap();
            if val == i64::MAX {
                return false;
            }
            row.set(idx, val + 1).unwrap();
        }
        DataType::Binary | DataType::String => {
            let mut val: Vec<u8> = row.get(idx).unwrap();
            val.push(0u8);
            row.set(idx, val).unwrap();
        }
        DataType::Float => {
            let val: f32 = row.get(idx).unwrap();
            if val == f32::INFINITY || val.is_nan() {
                return false;
            }
            row.set(idx, val.next()).unwrap();
        }
        DataType::Double => {
            let val: f64 = row.get(idx).unwrap();
            if val == f64::INFINITY || val.is_nan() {
                return false;
            }
            row.set(idx, val.next()).unwrap();
        }
    }
    true
}

/// Compares the cell values in two rows. If either cell is not set or null, returns false.
fn is_cell_equal(a: &Row, b: &Row, idx: usize) -> bool {
    assert_eq!(a.schema(), b.schema());

    if !a.is_set(idx).unwrap()
        || a.is_null(idx).unwrap()
        || !b.is_set(idx).unwrap()
        || b.is_null(idx).unwrap()
    {
        return false;
    }

    fn cmp<'a, T>(a: &'a Row, b: &'a Row, idx: usize) -> bool
    where
        T: Value<'a> + PartialEq,
    {
        a.get::<_, T>(idx).unwrap() == b.get::<_, T>(idx).unwrap()
    }

    match a.schema().columns()[idx].data_type() {
        DataType::Bool => cmp::<bool>(a, b, idx),
        DataType::Int8 => cmp::<i8>(a, b, idx),
        DataType::Int16 => cmp::<i16>(a, b, idx),
        DataType::Int32 => cmp::<i32>(a, b, idx),
        DataType::Int64 | DataType::Timestamp => cmp::<i64>(a, b, idx),
        DataType::Binary | DataType::String => cmp::<&[u8]>(a, b, idx),
        // TODO: do bitwise cmp for floats?
        DataType::Float => cmp::<f32>(a, b, idx),
        DataType::Double => cmp::<f64>(a, b, idx),
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
fn is_cell_incremented(lower: &Row, upper: &Row, idx: usize) -> bool {
    assert_eq!(lower.schema(), upper.schema());

    if !lower.is_set(idx).unwrap()
        || lower.is_null(idx).unwrap()
        || !upper.is_set(idx).unwrap()
        || upper.is_null(idx).unwrap()
    {
        return false;
    }

    match lower.schema().columns()[idx].data_type() {
        DataType::Bool => {
            let lower: bool = lower.get(idx).unwrap();
            let upper: bool = upper.get(idx).unwrap();
            !lower && upper
        }
        DataType::Int8 => {
            let lower: i8 = lower.get(idx).unwrap();
            let upper: i8 = upper.get(idx).unwrap();
            lower < i8::MAX && lower + 1 == upper
        }
        DataType::Int16 => {
            let lower: i16 = lower.get(idx).unwrap();
            let upper: i16 = upper.get(idx).unwrap();
            lower < i16::MAX && lower + 1 == upper
        }
        DataType::Int32 => {
            let lower: i32 = lower.get(idx).unwrap();
            let upper: i32 = upper.get(idx).unwrap();
            lower < i32::MAX && lower + 1 == upper
        }
        DataType::Int64 | DataType::Timestamp => {
            let lower: i64 = lower.get(idx).unwrap();
            let upper: i64 = upper.get(idx).unwrap();
            lower < i64::MAX && lower + 1 == upper
        }
        DataType::Binary | DataType::String => {
            let lower: &[u8] = lower.get(idx).unwrap();
            let upper: &[u8] = upper.get(idx).unwrap();
            lower.len() + 1 == upper.len()
                && lower == &upper[..lower.len()]
                && upper[upper.len() - 1] == 0
        }
        DataType::Float => {
            let lower: f32 = lower.get(idx).unwrap();
            let upper: f32 = upper.get(idx).unwrap();
            lower != f32::INFINITY && lower.next() == upper
        }
        DataType::Double => {
            let lower: f64 = lower.get(idx).unwrap();
            let upper: f64 = upper.get(idx).unwrap();
            lower != f64::INFINITY && lower.next() == upper
        }
    }
}

pub(crate) fn is_row_incremented(lower: &Row, upper: &Row, idxs: &[usize]) -> bool {
    let mut equals = false;
    for &idx in idxs.iter().rev() {
        if equals {
            if is_cell_equal(lower, upper, idx) {
                continue;
            } else {
                return false;
            }
        }

        if !lower.is_set(idx).unwrap() && !upper.is_set(idx).unwrap() {
            continue;
        }

        if !is_cell_incremented(lower, upper, idx) {
            return false;
        }
        equals = true;
    }
    true
}

#[cfg(test)]
mod test {

    use super::*;

    use Column;
    use DataType;
    use SchemaBuilder;

    #[test]
    fn test_murmur2_64() {
        assert_eq!(7115271465109541368, murmur2_64(b"ab", 0));
        assert_eq!(2601573339036254301, murmur2_64(b"abcdefg", 0));
        assert_eq!(3575930248840144026, murmur2_64(b"quick brown fox", 42));
    }

    #[test]
    fn primary_key_encode_decode() {
        let schema = SchemaBuilder::new()
            .add_column(Column::new("a", DataType::String).set_not_null())
            .add_column(Column::new("b", DataType::Int32).set_not_null())
            .add_column(Column::new("c", DataType::String).set_not_null())
            .set_primary_key(vec!["a", "b", "c"])
            .build()
            .unwrap();

        {
            let mut row = schema.new_row();
            assert_eq!(row.schema(), &schema);
            row.set(0, "fuzz\0\0\0\0buster").unwrap();
            row.set(1, 99).unwrap();
            row.set(2, "calibri\0\0\0").unwrap();
            let key = encode_primary_key(&row).unwrap();

            let decoded_row = decode_primary_key(&schema, &key).unwrap();
            assert_eq!(row, decoded_row);
        }
    }
}
