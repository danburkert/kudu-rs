use std::borrow::Cow;

use byteorder::{ByteOrder, LittleEndian};
use kudu_pb::wire_protocol::{RowOperationsPB_Type as OperationType};

use bit_set::BitSet;
use vec_map::VecMap;

use DataType;
use Error;
use Result;
use Schema;
use value::Value;

pub struct Row<'a> {
    data: Box<[u8]>,
    indirect_data: VecMap<Cow<'a, [u8]>>,
    set_columns: BitSet,
    null_columns: BitSet,
    schema: &'a Schema,
}

impl <'a> Row<'a> {
    pub fn new(schema: &'a Schema) -> Row<'a> {
        let num_columns = schema.columns().len();

        let null_columns = if schema.has_nullable_columns() { BitSet::with_capacity(num_columns) }
                           else { BitSet::with_capacity(0) };

        let data = vec![0; schema.row_size()].into_boxed_slice();

        Row {
            data: data,
            indirect_data: VecMap::with_capacity(num_columns),
            set_columns: BitSet::with_capacity(num_columns),
            null_columns: null_columns,
            schema: schema,
        }
    }

    pub fn set<V>(&mut self, idx: usize, value: V) -> Result<&mut Row<'a>> where V: Value<'a> {
        try!(self.check_column(idx, V::data_type()));
        if value.is_null() {
            if !self.schema.columns()[idx].is_nullable() {
                return Err(Error::InvalidArgument(format!("null value provided for column {:?}",
                                                  self.schema.columns()[idx])))
            }
            self.set_columns.insert(idx);
            self.null_columns.insert(idx);
        } else {
            self.set_columns.insert(idx);
            if self.schema.has_nullable_columns() { self.null_columns.remove(idx); }

            if V::is_var_len() {
                self.indirect_data.insert(idx, value.indirect_data().unwrap());
            } else {
                let offset = self.schema.column_offsets()[idx];
                let len = V::size();
                value.copy_data(&mut self.data[offset..offset+len]);
            }

        }
        Ok(self)
    }

    pub fn set_by_name<V>(&mut self, column: &str, value: V) -> Result<&mut Row<'a>> where V: Value<'a> {
        if let Some(idx) = self.schema.column_index(column) {
            self.set(idx, value)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    pub fn get<'b, V>(&'b self, idx: usize) -> Result<V> where V: Value<'b> {
        try!(self.check_column(idx, V::data_type()));
        if !self.set_columns.get(idx) {
            Err(Error::InvalidArgument(format!("column '{}' ({}) is not set",
                                               self.schema.columns()[idx].name(), idx)))
        } else if self.schema.has_nullable_columns() && self.null_columns.get(idx) {
            if V::is_nullable() {
                Ok(V::from_null())
            }
            else {
                Err(Error::InvalidArgument(format!("column '{}' ({}) is null",
                                                   self.schema.columns()[idx].name(), idx)))
            }
        } else if V::is_var_len() {
            V::from_data(&self.indirect_data[idx])
        } else {
            let offset = self.schema.column_offsets()[idx];
            let len = V::size();
            V::from_data(&self.data[offset..offset+len])
        }
    }

    pub fn get_by_name<'b, V>(&'b self, column: &str) -> Result<Option<V>> where V: Value<'b> {
        if let Some(idx) = self.schema.column_index(column) {
            self.get(idx)
        } else {
            Err(Error::InvalidArgument(format!("unknown column '{}'", column)))
        }
    }

    /// Checks that the column with the specified index has the expected type.
    fn check_column(&self, idx: usize, data_type: DataType) -> Result<()> {
        if idx >= self.schema.columns().len() {
            return Err(Error::InvalidArgument(format!("index {} is invalid for schema {:?}",
                                                      idx, self.schema)));
        }
        if data_type != self.schema.columns()[idx].data_type() {
            return Err(Error::InvalidArgument(format!("type {:?} is invalid for column {:?}",
                                                      data_type,
                                                      self.schema.columns()[idx])));
        }
        Ok(())
    }
}

pub struct OperationEncoder {
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

    pub fn encode_insert(&mut self, row: Row) {
        self.encode(OperationType::INSERT, row);
    }
    pub fn encode_update(&mut self, row: Row) {
        self.encode(OperationType::UPDATE, row);
    }
    pub fn encode_delete(&mut self, row: Row) {
        self.encode(OperationType::DELETE, row);
    }
    pub fn encode_upsert(&mut self, row: Row) {
        self.encode(OperationType::UPSERT, row);
    }
    pub fn encode_range_split(&mut self, row: Row) {
        self.encode(OperationType::SPLIT_ROW, row);
    }
    pub fn encode_range_bound(&mut self, lower: Row, upper: Row) {
        self.encode(OperationType::RANGE_LOWER_BOUND, lower);
        self.encode(OperationType::RANGE_UPPER_BOUND, upper);
    }

    fn encode(&mut self, op_type: OperationType, row: Row) {
        let Row { mut data, indirect_data, set_columns, null_columns, schema } = row;

        self.data.push(op_type as u8);
        self.data.extend_from_slice(set_columns.data());
        self.data.extend_from_slice(null_columns.data());

        // Iterate through the indirect data, serializing it to the indirect data buffer, and
        // rewriting the corresponding data pointer and length.
        for (idx, indirect_datum) in indirect_data {
            let offset = schema.column_offsets()[idx];
            LittleEndian::write_u64(&mut data[offset..], self.indirect_data.len() as u64);
            LittleEndian::write_u64(&mut data[offset+8..], indirect_datum.len() as u64);
            self.indirect_data.extend_from_slice(&indirect_datum);
        }

        self.data.extend_from_slice(&data);
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.indirect_data.clear();
    }

    pub fn unwrap(self) -> (Vec<u8>, Vec<u8>) {
        let OperationEncoder { data, indirect_data } = self;
        (data, indirect_data)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use schema;

    #[test]
    fn test_partial_row() {
        let schema = schema::tests::all_types_schema();
        let mut row = Row::new(&schema);

        row.set::<i32>(0, 12).unwrap();
        assert_eq!(12, row.get::<i32>(0).unwrap());

        row.set(9, "foo").unwrap();
        assert_eq!("foo".to_owned(), row.get::<String>(9).unwrap());
    }
}
