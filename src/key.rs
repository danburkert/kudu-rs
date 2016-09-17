//! Utility functions for working with keys.

use std::{i8, i16, i32, i64, f32, f64};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt, NativeEndian};
use ieee754::Ieee754;

use DataType;
use Error;
use HashPartitionSchema;
use PartitionSchema;
use RangePartitionSchema;
use Result;
use Row;
use Schema;

/// Murmur2 hash implementation returning 64-bit hashes.
pub fn murmur2_64(mut data: &[u8], seed: u64) -> u64 {
    let m: u64 = 0xc6a4a7935bd1e995;
    let r: u8 = 47;

    let mut h : u64 = seed ^ ((data.len() as u64).wrapping_mul(m));

    while data.len() >= 8 {
        let mut k = NativeEndian::read_u64(data);

        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h ^= k;
        h = h.wrapping_mul(m);

        data = &data[8..];
    };

    let len = data.len();
    if len > 6 { h ^= (data[6] as u64) << 48; }
    if len > 5 { h ^= (data[5] as u64) << 40; }
    if len > 4 { h ^= (data[4] as u64) << 32; }
    if len > 3 { h ^= (data[3] as u64) << 24; }
    if len > 2 { h ^= (data[2] as u64) << 16; }
    if len > 1 { h ^= (data[1] as u64) << 8; }
    if len > 0 { h ^= data[0] as u64;
                 h = h.wrapping_mul(m) }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;
    h
}

pub fn encode_primary_key(row: &Row) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    try!(encode_columns(row, 0..row.schema().num_primary_key_columns(), &mut buf));
    Ok(buf)
}

pub fn encode_partition_key(partition_schema: &PartitionSchema, row: &Row) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    for hash_schema in partition_schema.hash_partition_schemas() {
        try!(encode_hash_partition_key(&hash_schema, row, &mut buf));
    }
    try!(encode_range_partition_key(partition_schema.range_partition_schema(), row, &mut buf));
    Ok(buf)
}

pub fn encode_range_partition_key(range_schema: &RangePartitionSchema,
                                  row: &Row,
                                  buf: &mut Vec<u8>) -> Result<()> {
    encode_columns(row, range_schema.columns().iter().cloned(), buf)
}

pub fn encode_hash_partition_key(hash_schema: &HashPartitionSchema,
                                 row: &Row,
                                 buf: &mut Vec<u8>) -> Result<()> {
    let len = buf.len();
    try!(encode_columns(row, hash_schema.columns().iter().cloned(), buf));
    let bucket = murmur2_64(&buf[len..], hash_schema.seed() as u64) % hash_schema.num_buckets() as u64;
    buf.truncate(len);
    buf.write_u32::<BigEndian>(bucket as u32).unwrap();
    Ok(())
}

fn encode_columns<I>(row: &Row, idxs: I, buf: &mut Vec<u8>) -> Result<()>
where I: Iterator<Item=usize> + ExactSizeIterator {
    let columns = idxs.len();
    let mut column = 1;
    for idx in idxs {
        try!(encode_column(row, idx, column == columns, buf));
        column += 1;
    }
    Ok(())
}

fn encode_column(row: &Row, idx: usize, is_last: bool, buf: &mut Vec<u8>) -> Result<()> {
    match row.schema().columns()[idx].data_type() {
        DataType::Int8 => buf.push((try!(row.get::<i8>(idx)) ^ i8::MIN) as u8),
        DataType::Int16 => buf.write_i16::<BigEndian>(try!(row.get::<i16>(idx)) ^ i16::MIN).unwrap(),
        DataType::Int32 => buf.write_i32::<BigEndian>(try!(row.get::<i32>(idx)) ^ i32::MIN).unwrap(),
        DataType::Int64 | DataType::Timestamp => buf.write_i64::<BigEndian>(try!(row.get::<i64>(idx)) ^ i64::MIN).unwrap(),
        DataType::Binary | DataType::String => encode_binary(try!(row.get(idx)), is_last, buf),
        DataType::Bool | DataType::Float | DataType::Double => {
            panic!("illegal type {:?} in key", row.schema().columns()[idx].data_type());
        },
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
        buf[len-1] = 0;
    }
}

pub fn decode_primary_key(schema: &Schema, mut key: &[u8]) -> Result<Row> {
    let mut row = schema.new_row();

    let num_primary_key_columns = row.schema().num_primary_key_columns();
    for idx in 0..num_primary_key_columns {
        key = try!(decode_column(&mut row, idx, idx + 1 == num_primary_key_columns, key));
    }

    if !key.is_empty() {
        return Err(Error::Serialization(
                "bytes remaining after decoding all primary key columns".to_owned()));
    }
    Ok(row)
}

pub fn decode_range_partition_key(schema: &Schema,
                                  range_partition_schema: &RangePartitionSchema,
                                  mut key: &[u8]) -> Result<Row> {
    let mut row = schema.new_row();
    if range_partition_schema.columns().is_empty() { return Ok(row) }
    let last_idx = *range_partition_schema.columns().last().unwrap();

    for &idx in range_partition_schema.columns() {
        if key.is_empty() { break; }
        key = try!(decode_column(&mut row, idx, idx == last_idx, key));
    }

    if !key.is_empty() {
        return Err(Error::Serialization(
                "bytes remaining after decoding all partition key columns".to_owned()));
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
            },
            DataType::Int16 => {
                row.set_unchecked(idx, BigEndian::read_i16(key) ^ i16::MIN);
                Ok(&key[2..])
            },
            DataType::Int32 => {
                row.set_unchecked(idx, BigEndian::read_i32(key) ^ i32::MIN);
                Ok(&key[4..])
            },
            DataType::Int64 | DataType::Timestamp =>  {
                row.set_unchecked(idx, BigEndian::read_i64(key) ^ i64::MIN);
                Ok(&key[8..])
            },
            DataType::Binary => {
                let (remaining, value) = try!(decode_binary(key, is_last));
                row.set_unchecked(idx, value);
                Ok(remaining)
            },
            DataType::String => {
                let (remaining, value) = try!(decode_binary(key, is_last));
                row.set_unchecked(idx, try!(String::from_utf8(value)));
                Ok(remaining)
            },
            DataType::Bool | DataType::Float | DataType::Double => {
                panic!("illegal type {:?} in key", row.schema().columns()[idx].data_type());
            },
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
                            key = &key[idx+2..];
                            break;
                        },
                        1 => {
                            ret.extend_from_slice(&key[..idx+1]);
                            key = &key[idx+2..];
                        }
                        _ => return Err(Error::Serialization(format!(
                                    "unexpected binary sequence: {:?}", &[0, b])))

                    },
                    None => return Err(Error::Serialization(
                            "expected binary column separator sequence".to_owned())),
                },
                None => return Err(Error::Serialization(
                        "expected binary column separator sequence".to_owned())),
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
            let val = row.get(idx).unwrap();
            if val { return false };
            row.set(idx, true).unwrap();
        },
        DataType::Int8 => {
            let val = row.get::<i8>(idx).unwrap();
            if val == i8::MAX { return false; }
            row.set(idx, val + 1).unwrap();
        },
        DataType::Int16 => {
            let val = row.get::<i16>(idx).unwrap();
            if val == i16::MAX { return false; }
            row.set(idx, val + 1).unwrap();
        },
        DataType::Int32 => {
            let val = row.get::<i32>(idx).unwrap();
            if val == i32::MAX { return false; }
            row.set(idx, val + 1).unwrap();
        },
        DataType::Int64 | DataType::Timestamp =>  {
            let val = row.get::<i64>(idx).unwrap();
            if val == i64::MAX { return false; }
            row.set(idx, val + 1).unwrap();
        },
        DataType::Binary | DataType::String => {
            let mut val = row.get::<Vec<u8>>(idx).unwrap();
            val.push(0u8);
            row.set(idx, val).unwrap();
        },
        DataType::Float => {
            let val = row.get::<f32>(idx).unwrap();
            if val == f32::INFINITY || val.is_nan() { return false; }
            row.set(idx, val.next()).unwrap();
        },
        DataType::Double => {
            let val = row.get::<f64>(idx).unwrap();
            if val == f64::INFINITY || val.is_nan() { return false; }
            row.set(idx, val.next()).unwrap();
        },
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
            .add_column(Column::builder("a", DataType::String).set_not_null())
            .add_column(Column::builder("b", DataType::Int32).set_not_null())
            .add_column(Column::builder("c", DataType::String).set_not_null())
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
