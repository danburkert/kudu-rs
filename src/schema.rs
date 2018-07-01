use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use bitmap;
use pb::{ColumnSchemaPb, SchemaPb};
use CompressionType;
use DataType;
use EncodingType;
use Error;
use Result;
use Row;

/// `Column` instances hold schema and metadata about a column in a Kudu table.
///
/// `Column` also serves as a builder API for specifying new columns during create and alter
/// table operations.
#[derive(Clone, PartialEq, Eq)]
pub struct Column {
    name: String,
    data_type: DataType,
    is_nullable: bool,
    compression: CompressionType,
    encoding: EncodingType,
    cfile_block_size: u32,
}

impl Column {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    pub fn encoding(&self) -> EncodingType {
        self.encoding
    }

    pub fn compression(&self) -> CompressionType {
        self.compression
    }

    pub fn cfile_block_size(&self) -> Option<u32> {
        if self.cfile_block_size == 0 {
            None
        } else {
            Some(self.cfile_block_size)
        }
    }

    /// Returns a new column.
    pub fn new<S>(name: S, data_type: DataType) -> Column
    where
        S: Into<String>,
    {
        Column {
            name: name.into(),
            data_type,
            is_nullable: true,
            compression: CompressionType::Default,
            encoding: EncodingType::Auto,
            cfile_block_size: 0,
        }
    }

    pub fn set_is_nullable(mut self, is_nullable: bool) -> Column {
        self.is_nullable = is_nullable;
        self
    }

    pub fn set_nullable(mut self) -> Column {
        self.is_nullable = true;
        self
    }

    pub fn set_not_null(mut self) -> Column {
        self.is_nullable = false;
        self
    }

    pub fn set_encoding(mut self, encoding: EncodingType) -> Column {
        self.encoding = encoding;
        self
    }

    pub fn set_compression(mut self, compression: CompressionType) -> Column {
        self.compression = compression;
        self
    }

    pub fn set_cfile_block_size(mut self, cfile_block_size: u32) -> Column {
        self.cfile_block_size = cfile_block_size;
        self
    }

    pub(crate) fn into_pb(self, is_key: bool) -> ColumnSchemaPb {
        ColumnSchemaPb {
            name: self.name,
            type_: self.data_type.to_pb(),
            is_nullable: Some(self.is_nullable),
            is_key: Some(is_key),
            encoding: Some(self.encoding.to_pb()),
            compression: Some(self.compression.to_pb()),
            // TODO: checked cast.
            cfile_block_size: Some(self.cfile_block_size as i32),
            ..Default::default()
        }
    }

    pub(crate) fn from_pb(pb: ColumnSchemaPb) -> Result<Column> {
        Ok(Column {
            is_nullable: pb.is_nullable(),
            data_type: DataType::from_pb(pb.type_())?,
            compression: CompressionType::from_pb(pb.compression())?,
            encoding: EncodingType::from_pb(pb.encoding())?,
            cfile_block_size: pb.cfile_block_size() as u32,
            name: pb.name,
        })
    }
}

impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.name, self.data_type)?;
        if !self.is_nullable {
            write!(f, " NOT NULL")?;
        }
        if self.encoding != EncodingType::Auto {
            write!(f, " ENCODING {:?}", self.encoding)?;
        }
        if self.compression != CompressionType::Default {
            write!(f, " COMPRESSION {:?}", self.compression)?;
        }
        if let Some(cfile_block_size) = self.cfile_block_size() {
            write!(f, " CFILE BLOCK SIZE {}", cfile_block_size)?;
        }
        Ok(())
    }
}

struct Inner {
    // TODO: switch columns and column_offsets to be a Box<[]>.
    columns: Vec<Column>,
    columns_by_name: HashMap<String, usize>,
    // TODO: inline this into the Column struct instances
    column_offsets: Vec<usize>,
    // TODO: replace this with a bitset containing indices of var len columns
    var_len_column_offsets: Vec<usize>,
    num_primary_key_columns: usize,

    /// Length of the encoded columns.
    row_len: usize,
    has_nullable_columns: bool,
}

pub trait ColumnSelector {
    fn column_index(self, schema: &Schema) -> Result<usize>;
}

impl ColumnSelector for usize {
    #[inline]
    fn column_index(self, schema: &Schema) -> Result<usize> {
        schema.check_index(self).map(|_| self)
    }
}

impl<'a> ColumnSelector for &'a str {
    fn column_index(self, schema: &Schema) -> Result<usize> {
        schema
            .column_index(self)
            .ok_or_else(|| Error::InvalidArgument(format!("unknown column {}", self)))
    }
}

#[derive(Clone)]
pub struct Schema {
    inner: Arc<Inner>,
}

impl Schema {
    pub(crate) fn new(columns: Vec<Column>, num_primary_key_columns: usize) -> Schema {
        let mut columns_by_name = HashMap::with_capacity(columns.len());
        let mut column_offsets = Vec::with_capacity(columns.len());
        let mut var_len_column_offsets = Vec::new();
        let mut row_len = 0;
        let mut has_nullable_columns = false;
        for (idx, column) in columns.iter().enumerate() {
            columns_by_name.insert(column.name().to_string(), idx);
            column_offsets.push(row_len);
            if column.data_type.is_var_len() {
                var_len_column_offsets.push(row_len);
            }
            row_len += column.data_type.size();
            has_nullable_columns |= column.is_nullable();
        }

        Schema {
            inner: Arc::new(Inner {
                columns,
                columns_by_name,
                column_offsets,
                var_len_column_offsets,
                num_primary_key_columns,
                row_len,
                has_nullable_columns,
            }),
        }
    }

    pub fn columns(&self) -> &[Column] {
        &self.inner.columns
    }

    pub fn column(&self, index: usize) -> Option<&Column> {
        self.inner.columns.get(index)
    }

    pub fn column_by_name(&self, name: &str) -> Option<&Column> {
        self.column_index(name).map(|idx| &self.inner.columns[idx])
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.inner.columns_by_name.get(name).cloned()
    }

    pub fn primary_key(&self) -> &[Column] {
        &self.inner.columns[0..self.inner.num_primary_key_columns]
    }

    pub(crate) fn num_primary_key_columns(&self) -> usize {
        self.inner.num_primary_key_columns
    }

    pub fn primary_key_projection(&self) -> Schema {
        Schema::new(
            self.primary_key().to_owned(),
            self.num_primary_key_columns(),
        )
    }

    pub(crate) fn check_index(&self, idx: usize) -> Result<()> {
        if idx >= self.columns().len() {
            Err(Error::InvalidArgument(format!(
                "index {} is invalid for schema {:?}",
                idx, self
            )))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub(crate) fn bitmap_len(&self) -> usize {
        bitmap::len(self.inner.columns.len())
    }

    #[inline]
    pub(crate) fn row_len(&self) -> usize {
        self.inner.row_len
    }

    #[inline]
    pub fn has_nullable_columns(&self) -> bool {
        self.inner.has_nullable_columns
    }

    #[inline]
    pub(crate) fn column_offsets(&self) -> &[usize] {
        &self.inner.column_offsets
    }

    #[inline]
    pub(crate) fn var_len_column_offsets(&self) -> &[usize] {
        &self.inner.var_len_column_offsets
    }

    #[inline]
    pub(crate) fn column_offset(&self, idx: usize) -> isize {
        self.inner.column_offsets[idx] as isize
    }

    pub fn new_row<'a>(&self) -> Row<'a> {
        Row::partial(self.clone())
    }

    pub fn ref_eq(&self, other: &Schema) -> bool {
        let this: *const Inner = &*self.inner;
        let that: *const Inner = &*other.inner;
        this == that
    }

    pub(crate) fn as_pb(&self) -> SchemaPb {
        let mut pb = SchemaPb::default();
        for (idx, column) in self.inner.columns.iter().enumerate() {
            pb.columns.push(
                column
                    .clone()
                    .into_pb(idx < self.inner.num_primary_key_columns),
            );
        }
        pb
    }

    pub(crate) fn from_pb(pb: SchemaPb) -> Result<Schema> {
        let mut num_primary_key_columns = 0;
        let mut columns = Vec::with_capacity(pb.columns.len());
        for column in pb.columns {
            if column.is_key() {
                num_primary_key_columns += 1
            }
            columns.push(try!(Column::from_pb(column)))
        }
        Ok(Schema::new(columns, num_primary_key_columns))
    }
}

impl cmp::PartialEq for Schema {
    fn eq(&self, other: &Schema) -> bool {
        self.ref_eq(other)
            || (self.inner.num_primary_key_columns == other.inner.num_primary_key_columns
                && self.inner.columns == other.inner.columns)
    }
}

impl cmp::Eq for Schema {}

impl fmt::Debug for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        let mut is_first = true;
        for column in self.columns() {
            if is_first {
                is_first = false;
                try!(write!(f, "{:?}", column));
            } else {
                try!(write!(f, ", {:?}", column));
            }
        }
        try!(write!(f, ") PRIMARY KEY ("));
        is_first = true;
        for column in self.primary_key() {
            if is_first {
                is_first = false;
                try!(write!(f, "{}", column.name()));
            } else {
                try!(write!(f, ", {}", column.name()));
            }
        }
        write!(f, ")")
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaBuilder {
    columns: Vec<Column>,
    primary_key: Vec<String>,
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder {
            columns: Vec::new(),
            primary_key: Vec::new(),
        }
    }

    pub fn add_column(mut self, column: Column) -> SchemaBuilder {
        self.columns.push(column);
        self
    }

    pub fn set_primary_key<S>(mut self, columns: Vec<S>) -> SchemaBuilder
    where
        S: Into<String>,
    {
        self.primary_key = columns.into_iter().map(Into::into).collect();
        self
    }

    pub fn build(mut self) -> Result<Schema> {
        if self.primary_key.is_empty() {
            return Err(Error::InvalidArgument(
                "primary key must have at least one column".to_owned(),
            ));
        }

        let mut columns = Vec::with_capacity(self.columns.len());

        for column_name in &self.primary_key {
            let idx = self
                .columns
                .iter()
                .position(|col| col.name() == column_name);
            if let Some(idx) = idx {
                columns.push(self.columns.remove(idx));
            } else {
                return Err(Error::InvalidArgument(format!(
                    "primary key column '{}' has no corresponding column",
                    column_name
                )));
            }
        }

        columns.extend(self.columns.drain(..));

        Ok(Schema::new(columns, self.primary_key.len()))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use DataType;

    pub fn simple_schema() -> Schema {
        SchemaBuilder::new()
            .add_column(Column::new("key", DataType::String).set_not_null())
            .add_column(Column::new("val", DataType::String).set_not_null())
            .set_primary_key(vec!["key"])
            .build()
            .unwrap()
    }

    pub fn all_types_schema() -> Schema {
        SchemaBuilder::new()
            .add_column(Column::new("key", DataType::Int32).set_not_null())
            .add_column(Column::new("bool", DataType::Bool).set_not_null())
            .add_column(Column::new("i8", DataType::Int8).set_not_null())
            .add_column(Column::new("i16", DataType::Int16).set_not_null())
            .add_column(Column::new("i32", DataType::Int32).set_not_null())
            .add_column(Column::new("i64", DataType::Int64).set_not_null())
            .add_column(Column::new("timestamp", DataType::Timestamp).set_not_null())
            .add_column(Column::new("f32", DataType::Float).set_not_null())
            .add_column(Column::new("f64", DataType::Double).set_not_null())
            .add_column(Column::new("binary", DataType::Binary).set_not_null())
            .add_column(Column::new("string", DataType::String).set_not_null())
            .add_column(Column::new("nullable_bool", DataType::Bool).set_nullable())
            .add_column(Column::new("nullable_i8", DataType::Int8).set_nullable())
            .add_column(Column::new("nullable_i16", DataType::Int16).set_nullable())
            .add_column(Column::new("nullable_i32", DataType::Int32).set_nullable())
            .add_column(Column::new("nullable_i64", DataType::Int64).set_nullable())
            .add_column(Column::new("nullable_timestamp", DataType::Timestamp).set_nullable())
            .add_column(Column::new("nullable_f32", DataType::Float).set_nullable())
            .add_column(Column::new("nullable_f64", DataType::Double).set_nullable())
            .add_column(Column::new("nullable_binary", DataType::Binary).set_nullable())
            .add_column(Column::new("nullable_string", DataType::String).set_nullable())
            .set_primary_key(vec!["key"])
            .build()
            .unwrap()
    }

    #[test]
    fn test_create_schema() {
        all_types_schema();
    }
}
