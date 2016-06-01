use std::borrow::Cow;

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

    fn set<V>(&'a mut self, idx: usize, value: V) -> Result<&mut Row<'a>> where V: Value<'a> {
        try!(self.check_column(idx, V::data_type()));
        self.set_columns.insert(idx);
        if self.schema.has_nullable_columns() { self.null_columns.remove(idx); }
        if V::is_var_len() {
            self.indirect_data.insert(idx, value.indirect_data().unwrap());
        } else {
            let offset = self.schema.column_offsets()[idx];
            let len = V::size();
            value.copy_data(&mut self.data[offset..offset+len]);
        }
        Ok(self)
    }

    fn get<'b, V>(&'b mut self, idx: usize) -> Result<V> where V: Value<'b> {
        try!(self.check_column(idx, V::data_type()));
        if !self.set_columns.get(idx) {
            return Err(Error::InvalidArgument(format!("column '{:?}' ({}) is not set",
                                                      self.schema.columns()[idx].name(), idx)));
        }
        if V::is_var_len() {
            V::from_data(&self.indirect_data[idx])
        } else {
            let offset = self.schema.column_offsets()[idx];
            let len = V::size();
            V::from_data(&self.data[offset..offset+len])
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

        row.set::<&str>(9, "foo").unwrap();
        assert_eq!("foo".to_owned(), row.get::<String>(9).unwrap());
    }
}
