use std::fmt;
use std::slice;

use byteorder::{ByteOrder, LittleEndian};

use bitmap;
use pb::row_operations_pb::Type as OperationTypePb;
use pb::RowOperationsPb;
use value::read_var_len_value;
use DataType;
use Error;
use RangePartitionBound;
use Row;
use Schema;
use Value;

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

#[derive(Debug)]
pub struct Operation<'data> {
    pub row: Row<'data>,
    pub kind: OperationKind,
}

impl<'data> Operation<'data> {
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
    pub error: Error,
}

pub(crate) struct OperationEncoder {
    pub(crate) data: Vec<u8>,
    pub(crate) indirect_data: Vec<u8>,
}

impl OperationEncoder {
    pub fn new() -> OperationEncoder {
        OperationEncoder {
            data: Vec::new(),
            indirect_data: Vec::new(),
        }
    }

    pub fn encode_range_partition(
        &mut self,
        lower: &RangePartitionBound,
        upper: &RangePartitionBound,
    ) {
        let (lower_bound, lower_bound_type) = match *lower {
            RangePartitionBound::Inclusive(ref row) => (row, OperationTypePb::RangeLowerBound),
            RangePartitionBound::Exclusive(ref row) => {
                (row, OperationTypePb::ExclusiveRangeLowerBound)
            }
        };
        let (upper_bound, upper_bound_type) = match *upper {
            RangePartitionBound::Inclusive(ref row) => {
                (row, OperationTypePb::InclusiveRangeUpperBound)
            }
            RangePartitionBound::Exclusive(ref row) => (row, OperationTypePb::RangeUpperBound),
        };

        self.encode_row(lower_bound_type, lower_bound);
        self.encode_row(upper_bound_type, upper_bound);
    }

    pub fn encode_range_partition_split(&mut self, split: &Row) {
        self.encode_row(OperationTypePb::SplitRow, split);
    }

    pub fn encode_row(&mut self, op_type: OperationTypePb, row: &Row) {
        let schema = row.schema();
        let bitmap_len = schema.bitmap_len();
        self.data
            .reserve(1 + schema.row_len() + schema.has_nullable_columns() as usize * bitmap_len);

        self.data.push(op_type as u8);

        match row.is_set_bitmap() {
            Some(bitmap) => self.data.extend_from_slice(bitmap),
            None => {
                let len = self.data.len() + schema.bitmap_len();
                self.data.resize(len, 0xFF);
            }
        }
        self.data.extend_from_slice(row.is_null_bitmap());

        unsafe {
            for (idx, column) in row.schema().columns().iter().enumerate() {
                if !row.is_set_unchecked(idx)
                    || (column.is_nullable() && row.is_null_unchecked(idx))
                {
                    continue;
                }
                let data_type = column.data_type();
                if data_type.is_var_len() {
                    let data: &[u8] = row.get(idx).unwrap();
                    let mut buf = [0u8; 16];
                    LittleEndian::write_u64(&mut buf[..], self.indirect_data.len() as u64);
                    LittleEndian::write_u64(&mut buf[8..], data.len() as u64);
                    self.data.extend_from_slice(&buf[..]);
                    self.indirect_data.extend_from_slice(data);
                } else {
                    let column_offset = row.schema().column_offsets()[idx] as isize;
                    let data =
                        slice::from_raw_parts(row.data().offset(column_offset), data_type.size());
                    self.data.extend_from_slice(data);
                }
            }
        }
    }

    /// Returns the encoded length for the row.
    /// TODO: it's pretty wasteful to do this as a separate step than encode_row.
    pub fn encoded_len(row: &Row) -> usize {
        let mut len = 1; // op type

        let bitmap_len = row.schema().bitmap_len();
        len += bitmap_len;
        if row.schema().has_nullable_columns() {
            len += bitmap_len;
        }

        for (idx, column) in row.schema().columns().iter().enumerate() {
            unsafe {
                if !row.is_set_unchecked(idx)
                    || (column.is_nullable() && row.is_null_unchecked(idx))
                {
                    continue;
                }
            }

            len += column.data_type().size();
            if column.data_type().is_var_len() {
                len += row.get::<_, &[u8]>(idx).unwrap().len();
            }
        }

        len
    }

    pub fn len(&self) -> usize {
        self.data.len() + self.indirect_data.len()
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
    offset: usize,
}

impl<'a> Iterator for OperationDecoder<'a> {
    type Item = Operation<'a>;

    fn next(&mut self) -> Option<Operation<'a>> {
        let Self {
            schema,
            data,
            ref mut offset,
            ..
        } = *self;

        if *offset >= data.len() {
            return None;
        }

        let op_type = i32::from(data[*offset]);
        *offset += 1;

        let mut row = schema.new_row();

        // Split the data slice into the is_set bitmap, the is_null bitmap, and the row.
        let bitmap_len = schema.bitmap_len();
        let (is_set, is_null) = data[*offset..].split_at(bitmap_len);

        *offset += bitmap_len;
        if self.schema.has_nullable_columns() {
            *offset += bitmap_len;
        }

        for (idx, column) in self.schema.columns().iter().enumerate() {
            if !bitmap::get(is_set, idx) {
                continue;
            }
            if column.is_nullable() && bitmap::get(is_null, idx) {
                // TODO: set_null_unchecked
                row.set_null(idx).unwrap();
                continue;
            }

            unsafe {
                let data = data.as_ptr().offset(*offset as isize);
                match column.data_type() {
                    DataType::Bool => {
                        row.set_unchecked(idx, bool::read(data).unwrap());
                    }
                    DataType::Int8 => {
                        row.set_unchecked(idx, i8::read(data).unwrap());
                    }
                    DataType::Int16 => {
                        row.set_unchecked(idx, i16::read(data).unwrap());
                    }
                    DataType::Int32 => {
                        row.set_unchecked(idx, i32::read(data).unwrap());
                    }
                    DataType::Int64 | DataType::Timestamp => {
                        row.set_unchecked(idx, i64::read(data).unwrap());
                    }
                    DataType::Float => {
                        row.set_unchecked(idx, f32::read(data).unwrap());
                    }
                    DataType::Double => {
                        row.set_unchecked(idx, f64::read(data).unwrap());
                    }
                    DataType::Binary | DataType::String => {
                        let (ptr, len, _) = read_var_len_value(data);
                        row.set_unchecked(idx, slice::from_raw_parts(ptr, len));
                    }
                }
            }
            *offset += column.data_type().size();
        }

        let kind = match OperationTypePb::from_i32(op_type).unwrap_or_default() {
            OperationTypePb::Insert => OperationKind::Insert,
            OperationTypePb::Update => OperationKind::Update,
            OperationTypePb::Delete => OperationKind::Delete,
            OperationTypePb::Upsert => OperationKind::Upsert,
            other => panic!("unexpected operation type: {:?}", other),
        };

        Some(Operation { row, kind })
    }
}

impl<'a> OperationDecoder<'a> {
    pub(crate) fn new(
        schema: &'a Schema,
        data: &'a [u8],
        indirect_data: &'a [u8],
    ) -> OperationDecoder<'a> {
        OperationDecoder {
            schema,
            data,
            indirect_data,
            offset: 0,
        }
    }
}

impl<'a> fmt::Debug for OperationDecoder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OperationDecoder")
            .field("schema", self.schema)
            .field("data", &self.data.len())
            .field("indirect_data", &self.indirect_data.len())
            .field("offset", &self.offset)
            .finish()
    }
}
