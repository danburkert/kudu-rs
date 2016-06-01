use std::collections::HashMap;

use bit_set::BitSet;
use CompressionType;
use DataType;
use EncodingType;
use Error;
use Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column {
    name: String,
    data_type: DataType,
    nullable: bool,
    compression: CompressionType,
    encoding: EncodingType,
    block_size: usize,
}

impl Column {
    fn new(name: String, data_type: DataType) -> Column {
        Column {
            name: name,
            data_type: data_type,
            nullable: true,
            compression: CompressionType::Default,
            encoding: EncodingType::Default,
            block_size: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }

    pub fn encoding(&self) -> EncodingType {
        self.encoding
    }

    pub fn compression(&self) -> CompressionType {
        self.compression
    }

    pub fn block_size(&self) -> Option<usize> {
        if self.block_size == 0 {
            None
        } else {
            Some(self.block_size)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    columns: Vec<Column>,
    columns_by_name: HashMap<String, usize>,
    column_offsets: Vec<usize>,
    num_primary_key_columns: usize,
    row_size: usize,
    has_nullable_columns: bool,
}

impl Schema {
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    pub fn column_by_name(&self, name: &str) -> Option<&Column> {
        self.columns_by_name.get(name).and_then(|&idx| self.columns.get(idx))
    }

    pub fn primary_key(&self) -> &[Column] {
        &self.columns[0..self.num_primary_key_columns]
    }

    pub fn row_size(&self) -> usize {
        self.row_size
    }

    pub fn has_nullable_columns(&self) -> bool {
        self.has_nullable_columns
    }

    pub fn column_offsets(&self) -> &[usize] {
        &self.column_offsets
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaBuilder {
    columns: Vec<ColumnBuilder>,
    primary_key: Vec<String>,
    range_partition_columns: Vec<String>,
}

impl SchemaBuilder {

    pub fn new() -> SchemaBuilder {
        SchemaBuilder {
            columns: Vec::new(),
            primary_key: Vec::new(),
            range_partition_columns: Vec::new(),
        }
    }

    pub fn columns(&self) -> &[ColumnBuilder] {
        &self.columns
    }

    pub fn columns_mut(&mut self) -> &mut [ColumnBuilder] {
        &mut self.columns
    }

    pub fn add_column<S>(&mut self, name: S, data_type: DataType) -> &mut ColumnBuilder
    where S: Into<String> {
        let mut column = ColumnBuilder::new(name.into(), data_type);
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn primary_key(&self) -> &[String] {
        &self.primary_key
    }

    pub fn set_primary_key(&mut self, columns: Vec<String>) -> &mut SchemaBuilder {
        self.primary_key = columns;
        self
    }

    fn build(mut self) -> Result<Schema> {
        if self.primary_key.is_empty() {
            return Err(Error::InvalidArgument(
                    "primary key must have at least one column".to_owned()));
        }

        let num_primary_key_columns = self.primary_key.len();

        let mut columns = Vec::with_capacity(self.columns.len());
        let mut columns_by_name = HashMap::with_capacity(self.columns.len());
        let mut column_offsets = Vec::with_capacity(self.columns.len());
        let mut row_size = 0;
        let mut has_nullable_columns = false;

        for column_name in &self.primary_key {
            let idx = self.columns.iter().position(|col| col.name() == column_name);
            if let Some(idx) = idx {
                columns.push(self.columns.remove(idx).build());
            } else {
                return Err(Error::InvalidArgument(
                    format!("primary key column '{}' has no corresponding column builder",
                            column_name)))
            }
        }

        for column in self.columns {
            columns.push(column.build())
        }

        for (idx, column) in columns.iter().enumerate() {
            columns_by_name.insert(column.name().to_owned(), idx);
            column_offsets.push(row_size);
            row_size += column.data_type.size();
            if column.nullable { has_nullable_columns = true; }
        }

        Ok(Schema {
            columns: columns,
            columns_by_name: columns_by_name,
            column_offsets: column_offsets,
            num_primary_key_columns: num_primary_key_columns,
            row_size: row_size,
            has_nullable_columns: has_nullable_columns,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnBuilder {
    name: String,
    data_type: DataType,
    nullable: bool,
    compression: CompressionType,
    encoding: EncodingType,
    block_size: usize,
}

impl ColumnBuilder {

    fn new(name: String, data_type: DataType) -> ColumnBuilder {
        ColumnBuilder {
            name: name,
            data_type: data_type,
            nullable: true,
            compression: CompressionType::Default,
            encoding: EncodingType::Default,
            block_size: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }

    pub fn set_nullable(&mut self) -> &mut ColumnBuilder {
        self.nullable = true;
        self
    }

    pub fn set_not_null(&mut self) -> &mut ColumnBuilder {
        self.nullable = false;
        self
    }

    pub fn encoding(&self) -> EncodingType {
        self.encoding
    }

    pub fn set_encoding(&mut self, encoding: EncodingType) -> &mut ColumnBuilder {
        self.encoding = encoding;
        self
    }

    pub fn compression(&self) -> CompressionType {
        self.compression
    }

    pub fn set_compression(&mut self, compression: CompressionType) -> &mut ColumnBuilder {
        self.compression = compression;
        self
    }

    pub fn block_size(&self) -> Option<usize> {
        if self.block_size == 0 {
            None
        } else {
            Some(self.block_size)
        }
    }

    pub fn set_block_size(&mut self, block_size: usize) -> &mut ColumnBuilder {
        self.block_size = block_size;
        self
    }

    fn build(self) -> Column {
        let ColumnBuilder { name, data_type, nullable, compression, encoding, block_size } = self;
        Column {
            name: name,
            data_type: data_type,
            nullable: nullable,
            compression: compression,
            encoding: encoding,
            block_size: block_size,
        }
    }
}

/*
impl fmt::Debug for ColumnBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "'{}' {:?}", &self.name, self.data_type));
        if !self.nullable {
            try!(write!(f, " NOT NULL"));
        }
        if self.encoding != EncodingType::Default {
            try!(write!(f, " ENCODING {:?}", self.encoding));
        }
        if self.compression != CompressionType::Default {
            try!(write!(f, " COMPRESSION {:?}", self.compression));
        }
        if self.block_size > 0 {
            try!(write!(f, " BLOCK SIZE {}", self.block_size));
        }
        Ok(())
    }
}
*/

#[cfg(test)]
pub mod tests {

    use super::*;
    use DataType;

    pub fn all_types_schema() -> Schema {
        let mut builder = SchemaBuilder::new();
        builder.add_column("key", DataType::Int32);

        builder.add_column("bool", DataType::Bool);
        builder.add_column("i8", DataType::Int8);
        builder.add_column("i16", DataType::Int16);
        builder.add_column("i32", DataType::Int32);
        builder.add_column("i64", DataType::Int64);
        builder.add_column("f32", DataType::Float);
        builder.add_column("f64", DataType::Double);
        builder.add_column("binary", DataType::Binary);
        builder.add_column("string", DataType::String);

        builder.add_column("nullable_bool", DataType::Bool).set_nullable();
        builder.add_column("nullable_i8", DataType::Int8).set_nullable();
        builder.add_column("nullable_i16", DataType::Int16).set_nullable();
        builder.add_column("nullable_i32", DataType::Int32).set_nullable();
        builder.add_column("nullable_i64", DataType::Int64).set_nullable();
        builder.add_column("nullable_f32", DataType::Float).set_nullable();
        builder.add_column("nullable_f64", DataType::Double).set_nullable();
        builder.add_column("nullable_binary", DataType::Binary).set_nullable();
        builder.add_column("nullable_string", DataType::String).set_nullable();

        builder.set_primary_key(vec!["key".to_string()]);

        builder.build().unwrap()
    }

    #[test]
    fn test_create_schema() {
        all_types_schema();
    }
}
