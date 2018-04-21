use byteorder::{ByteOrder, LittleEndian};

use DataType;
use RangePartitionBound;
use Row;
use Schema;
use Value;
use bitmap;
use error::Status;
use pb::RowOperationsPb;
use pb::row_operations_pb::{Type as OperationTypePb};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperationKind {
    Insert,
    Update,
    Upsert,
    Delete,
}

impl OperationKind {
    pub(crate) fn as_pb(self) -> OperationTypePb {
        match self {
            OperationKind::Insert => OperationTypePb::Insert,
            OperationKind::Update => OperationTypePb::Update,
            OperationKind::Upsert => OperationTypePb::Upsert,
            OperationKind::Delete => OperationTypePb::Delete,
        }
    }
}

pub struct Operation<'data> {
    pub row: Row<'data>,
    pub kind: OperationKind,
}

impl <'data> Operation<'data> {
    pub fn into_owned(self) -> Operation<'static> {
        Operation {
            row: self.row.into_owned(),
            kind: self.kind,
        }
    }
}

pub struct OperationError {
    pub row: Row<'static>,
    pub kind: OperationKind,
    pub status: Status,
}


pub(crate) struct OperationEncoder {
    data: Vec<u8>,
    indirect_data: Vec<u8>,
}

impl OperationEncoder {

    pub fn new() -> OperationEncoder {
        OperationEncoder {
            data: Vec::new(),
            indirect_data: Vec::new(),
        }
    }

    pub fn encode_range_split(&mut self, row: &Row) {
        self.encode_row(OperationTypePb::SplitRow, row);
    }

    pub fn encode_range_partition(&mut self, lower: &RangePartitionBound, upper: &RangePartitionBound) {
        let (lower_bound, lower_bound_type) = match *lower {
            RangePartitionBound::Inclusive(ref row) => (row, OperationTypePb::RangeLowerBound),
            RangePartitionBound::Exclusive(ref row) => (row, OperationTypePb::ExclusiveRangeLowerBound),
        };
        let (upper_bound, upper_bound_type) = match *upper {
            RangePartitionBound::Inclusive(ref row) => (row, OperationTypePb::InclusiveRangeUpperBound),
            RangePartitionBound::Exclusive(ref row) => (row, OperationTypePb::RangeUpperBound),
        };

        self.encode_row(lower_bound_type, lower_bound);
        self.encode_row(upper_bound_type, upper_bound);
    }

    pub fn encode_range_partition_split(&mut self, split: &Row) {
        self.encode_row(OperationTypePb::SplitRow, split);
    }

    pub fn encode_row(&mut self, op_type: OperationTypePb, row: &Row) {
        self.data.reserve(1 + row.data.len());
        self.data.push(op_type as u8);
        let bitmap_len = row.schema.bitmap_len();
        let (bitmaps, row_data) = row.data.split_at(
            if row.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len });
        self.data.extend_from_slice(bitmaps);
        for (idx, column) in row.schema.columns().iter().enumerate() {
            if !bitmap::get(bitmaps, idx) ||
               (column.is_nullable() && bitmap::get(&bitmaps[bitmap_len..], idx)) { continue; }

            let data_type = column.data_type();
            let column_offset = row.schema.column_offsets()[idx];
            let width = data_type.size();
            if data_type.is_var_len() {
                let data: &[u8] = unsafe { Value::from_data(&row_data[column_offset..]) };
                let mut buf = [0u8; 16];
                LittleEndian::write_u64(&mut buf[..], self.indirect_data.len() as u64);
                LittleEndian::write_u64(&mut buf[8..], data.len() as u64);
                self.data.extend_from_slice(&buf[..]);
                self.indirect_data.extend_from_slice(data);
            } else {
                self.data.extend_from_slice(&row_data[column_offset..column_offset + width]);
            }
        }
    }

    /// Returns the encoded length for the row.
    /// TODO: it's pretty wasteful to do this as a separate step than encode_row.
    pub fn encoded_len(row: &Row) -> usize {
        let mut len = 1; // op type

        let bitmap_len = row.schema.bitmap_len();
        len += bitmap_len;

        let (bitmaps, row_data) = row.data.split_at(
            if row.schema.has_nullable_columns() { bitmap_len * 2 } else { bitmap_len });

        for (idx, column) in row.schema.columns().iter().enumerate() {
            if !bitmap::get(bitmaps, idx) ||
               (column.is_nullable() && bitmap::get(&bitmaps[bitmap_len..], idx)) { continue; }

            len += column.data_type().size();
            if column.data_type().is_var_len() {
                let offset = row.schema.column_offsets()[idx];
                let slice: &[u8] = unsafe { Value::from_data(&row_data[offset..]) };
                len += slice.len()
            }
        }
        len
    }

    pub fn len(&self) -> usize {
        self.data.len() + self.indirect_data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.indirect_data.clear();
    }

    pub fn into_pb(self) -> RowOperationsPb {
        RowOperationsPb {
            rows: Some(self.data),
            indirect_data: Some(self.indirect_data),
        }
    }
}

pub(crate) struct OperationDecoder<'a> {
    schema: &'a Schema,
    data: &'a [u8],
    indirect_data: &'a [u8],
    bitmap_len: usize,
    row_len: usize,
}

impl <'a> OperationDecoder<'a> {
    pub(crate) fn new(schema: &'a Schema, data: &'a [u8], indirect_data: &'a [u8]) -> OperationDecoder<'a> {
        let bitmap_len = schema.bitmap_len();
        let row_len = 0
            + 1 // op type
            + schema.bitmap_len() // is set bitmap
            + if schema.has_nullable_columns() { bitmap_len } else { 0 } // is null bitmap
            + schema.row_len(); // row length
        debug_assert_eq!(data.len() % row_len, 0);

        OperationDecoder {
            schema,
            data,
            indirect_data,
            bitmap_len,
            row_len,
        }
    }

    pub(crate) fn decode_operation(&self, index: usize) -> Operation<'a> {
        let mut offset = index * self.row_len;

        let op_type = i32::from(self.data[offset]);
        offset += 1;

        let mut row = self.schema.new_row();

        // Split the data slice into the is_set bitmap, the is_null bitmap, and the row.
        let (is_set, is_null) = self.data[offset..].split_at(self.bitmap_len);

        offset += self.bitmap_len;
        if self.schema.has_nullable_columns() {
            offset += self.bitmap_len;
        }

        for (idx, column) in self.schema.columns().iter().enumerate() {
            if !bitmap::get(is_set, idx) { continue; }
            if column.is_nullable() && bitmap::get(is_null, idx) {
                // TODO: set_null_unchecked
                row.set_null(idx).unwrap();
                continue;
            }

            unsafe {
                match column.data_type() {
                    DataType::Bool => {
                        row.set_unchecked(idx, bool::from_data(&self.data[offset..]));
                    },
                    DataType::Int8 => {
                        row.set_unchecked(idx, i8::from_data(&self.data[offset..]));
                    },
                    DataType::Int16 => {
                        row.set_unchecked(idx, i16::from_data(&self.data[offset..]));
                    },
                    DataType::Int32 => {
                        row.set_unchecked(idx, i32::from_data(&self.data[offset..]));
                    },
                    DataType::Int64 | DataType::Timestamp => {
                        row.set_unchecked(idx, i64::from_data(&self.data[offset..]));
                    },
                    DataType::Float => {
                        row.set_unchecked(idx, f32::from_data(&self.data[offset..]));
                    },
                    DataType::Double => {
                        row.set_unchecked(idx, f64::from_data(&self.data[offset..]));
                    },
                    DataType::Binary | DataType::String => {
                        let indirect_offset = LittleEndian::read_u64(&self.data[offset..]) as usize;
                        let len = LittleEndian::read_u64(&self.data[offset+8..]) as usize;
                        row.set_unchecked(idx, &self.indirect_data[indirect_offset..indirect_offset+len]);
                    },
                }
            }
        }

        let kind = match OperationTypePb::from_i32(op_type).unwrap_or_default() {
            OperationTypePb::Insert => OperationKind::Insert,
            OperationTypePb::Update => OperationKind::Update,
            OperationTypePb::Delete => OperationKind::Delete,
            OperationTypePb::Upsert => OperationKind::Upsert,
            other => panic!("unexpected operation type: {:?}", other),
        };

        Operation {
            row,
            kind,
        }
    }
}
