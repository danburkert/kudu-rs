use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use kudu_pb::common::{ColumnSchemaPB, SchemaPB};
#[cfg(any(feature="quickcheck", test))] use quickcheck;

use CompressionType;
use DataType;
use EncodingType;
use Error;
use Result;
use Row;

#[derive(Clone, PartialEq, Eq)]
pub struct Column {
    name: String,
    data_type: DataType,
    is_nullable: bool,
    compression: CompressionType,
    encoding: EncodingType,
    block_size: i32,
}

impl Column {

    fn new(name: String, data_type: DataType) -> Column {
        Column {
            name: name,
            data_type: data_type,
            is_nullable: true,
            compression: CompressionType::Default,
            encoding: EncodingType::Auto,
            block_size: 0,
        }
    }

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

    pub fn block_size(&self) -> Option<i32> {
        if self.block_size <= 0 {
            None
        } else {
            Some(self.block_size)
        }
    }

    pub fn to_pb(&self, is_key: bool) -> ColumnSchemaPB {
        let mut pb = ColumnSchemaPB::new();
        pb.set_name(self.name.clone());
        pb.set_field_type(self.data_type.to_pb());
        pb.set_is_nullable(self.is_nullable);
        pb.set_is_key(is_key);
        pb.set_encoding(self.encoding.to_pb());
        pb.set_compression(self.compression.to_pb());
        pb.set_cfile_block_size(self.block_size);
        pb
    }

    pub fn from_pb(mut pb: ColumnSchemaPB) -> Result<Column> {
        Ok(Column {
            name: pb.take_name(),
            data_type: try!(DataType::from_pb(pb.get_field_type())),
            is_nullable: pb.get_is_nullable(),
            compression: try!(CompressionType::from_pb(pb.get_compression())),
            encoding: try!(EncodingType::from_pb(pb.get_encoding())),
            block_size: pb.get_cfile_block_size(),
        })
    }
}

impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?} {:?}", self.name, self.data_type));
        if !self.is_nullable {
            try!(write!(f, " NOT NULL"));
        }
        if self.encoding != EncodingType::Auto {
            try!(write!(f, " ENCODING {:?}", self.encoding));
        }
        if self.compression != CompressionType::Default {
            try!(write!(f, " COMPRESSION {:?}", self.compression));
        }
        if let Some(block_size) = self.block_size() {
            try!(write!(f, " BLOCK SIZE {}", block_size));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Inner {
    columns: Vec<Column>,
    columns_by_name: HashMap<String, usize>,
    column_offsets: Vec<usize>,
    num_primary_key_columns: usize,
    row_size: usize,
    has_nullable_columns: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Schema {
    inner: Arc<Inner>,
}

impl Schema {
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

    #[doc(hidden)]
    pub fn num_primary_key_columns(&self) -> usize {
        self.inner.num_primary_key_columns
    }

    pub fn row_size(&self) -> usize {
        self.inner.row_size
    }

    pub fn has_nullable_columns(&self) -> bool {
        self.inner.has_nullable_columns
    }

    pub fn column_offsets(&self) -> &[usize] {
        &self.inner.column_offsets
    }

    pub fn new_row(&self) -> Row {
        Row::new(self.clone())
    }

    pub fn ref_eq(&self, other: &Schema) -> bool {
        let this: *const Inner = &*self.inner;
        let that: *const Inner = &*other.inner;
        this == that
    }

    #[doc(hidden)]
    pub fn as_pb(&self) -> SchemaPB {
        let mut pb = SchemaPB::new();
        for (idx, column) in self.inner.columns.iter().enumerate() {
            pb.mut_columns().push(column.to_pb(idx < self.inner.num_primary_key_columns));
        }
        pb
    }

    pub fn from_pb(mut pb: SchemaPB) -> Result<Schema> {
        let mut num_primary_key_columns = 0;
        let mut columns = Vec::with_capacity(pb.get_columns().len());
        for column in pb.take_columns().into_iter() {
            if column.get_is_key() { num_primary_key_columns += 1 }
            columns.push(try!(Column::from_pb(column)))
        }

        let mut columns_by_name = HashMap::with_capacity(columns.len());
        let mut column_offsets = Vec::with_capacity(columns.len());
        let mut index = 0;
        let mut row_size = 0;
        let mut has_nullable_columns = false;
        for column in &columns {
            columns_by_name.insert(column.name().to_string(), index);
            index += 1;
            column_offsets.push(row_size);
            row_size += column.data_type.size();
            has_nullable_columns |= column.is_nullable();
        }

        Ok(Schema {
            inner: Arc::new(Inner {
                columns: columns,
                columns_by_name: columns_by_name,
                column_offsets: column_offsets,
                num_primary_key_columns: num_primary_key_columns,
                row_size: row_size,
                has_nullable_columns: has_nullable_columns,
            })
        })
    }
}

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

#[cfg(any(feature="quickcheck", test))]
impl quickcheck::Arbitrary for Schema {
    fn arbitrary<G>(g: &mut G) -> Schema where G: quickcheck::Gen {
        use std::collections::HashSet;

        let mut builder = SchemaBuilder::new();

        let mut primary_key_columns: HashSet<String> = HashSet::arbitrary(g);
        while primary_key_columns.is_empty() {
            primary_key_columns = HashSet::arbitrary(g);
        }

        let mut columns: HashSet<String> = HashSet::arbitrary(g);
        while !primary_key_columns.is_disjoint(&columns) {
            columns = HashSet::arbitrary(g);
        }

        let mut columns = primary_key_columns.union(&columns).collect::<Vec<_>>();
        g.shuffle(&mut columns);

        for column in columns {
            let is_pk = primary_key_columns.contains(column);
            let data_type = if is_pk { DataType::arbitrary_primary_key(g) }
                                else { DataType::arbitrary(g) };

            let mut column = builder.add_column(column.as_str(), data_type);

            if is_pk || bool::arbitrary(g) { column.set_not_null() }
            else { column.set_nullable() };

            column.set_encoding(EncodingType::arbitrary(g, data_type));
            column.set_compression(CompressionType::arbitrary(g));
            if bool::arbitrary(g) {
                column.set_block_size(i32::arbitrary(g));
            }
        }

        let mut primary_key_columns: Vec<String> = primary_key_columns.iter().cloned().collect();
        g.shuffle(&mut primary_key_columns);

        builder.set_primary_key(primary_key_columns);
        builder.build().unwrap()
    }

    /// Returns an iterator containing versions of the schema with columns removed.
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        if self.columns().len() == 1 { return quickcheck::empty_shrinker(); }

        let mut schemas: Vec<Schema> = Vec::new();
        let start_idx = if self.num_primary_key_columns() > 1 { 0 } else { 1 };

        for idx in (start_idx..self.columns().len()).rev() {
            let mut builder = SchemaBuilder::new();

            for column in self.columns()[..idx].iter().chain(self.columns()[idx+1..].iter()) {
                let mut column_builder = builder.add_column(column.name(), column.data_type());
                if  column.is_nullable() { column_builder.set_nullable() }
                else { column_builder.set_not_null() };
                column_builder.set_encoding(column.encoding());
                column_builder.set_compression(column.compression());
                if let Some(block_size) = column.block_size() {
                    column_builder.set_block_size(block_size);
                }
            }

            let mut primary_key_columns = Vec::new();
            for pk_idx in 0..self.num_primary_key_columns() {
                if idx == pk_idx { continue; }
                primary_key_columns.push(self.columns()[pk_idx].name().to_owned());
            }
            builder.set_primary_key(primary_key_columns);
            schemas.push(builder.build().unwrap());
        }

        Box::new(schemas.into_iter())
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
        let column = ColumnBuilder::new(name.into(), data_type);
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

    pub fn build(mut self) -> Result<Schema> {
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
            if column.is_nullable { has_nullable_columns = true; }
        }

        Ok(Schema {
            inner: Arc::new(Inner {
                columns: columns,
                columns_by_name: columns_by_name,
                column_offsets: column_offsets,
                num_primary_key_columns: num_primary_key_columns,
                row_size: row_size,
                has_nullable_columns: has_nullable_columns,
            }),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnBuilder {
    name: String,
    data_type: DataType,
    is_nullable: bool,
    compression: CompressionType,
    encoding: EncodingType,
    block_size: i32,
}

impl ColumnBuilder {

    fn new(name: String, data_type: DataType) -> ColumnBuilder {
        ColumnBuilder {
            name: name,
            data_type: data_type,
            is_nullable: true,
            compression: CompressionType::Default,
            encoding: EncodingType::Auto,
            block_size: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }

    pub fn set_nullable(&mut self) -> &mut ColumnBuilder {
        self.is_nullable = true;
        self
    }

    pub fn set_not_null(&mut self) -> &mut ColumnBuilder {
        self.is_nullable = false;
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

    pub fn block_size(&self) -> i32 {
        self.block_size
    }

    pub fn set_block_size(&mut self, block_size: i32) -> &mut ColumnBuilder {
        self.block_size = block_size;
        self
    }

    fn build(self) -> Column {
        let ColumnBuilder { name, data_type, is_nullable, compression, encoding, block_size } = self;
        Column {
            name: name,
            data_type: data_type,
            is_nullable: is_nullable,
            compression: compression,
            encoding: encoding,
            block_size: block_size,
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use DataType;

    pub fn simple_schema() -> Schema {
        let mut builder = SchemaBuilder::new();
        builder.add_column("key", DataType::String).set_not_null();
        builder.add_column("val", DataType::String).set_not_null();
        builder.set_primary_key(vec!["key".to_string()]);
        builder.build().unwrap()
    }

    pub fn all_types_schema() -> Schema {
        let mut builder = SchemaBuilder::new();
        builder.add_column("key", DataType::Int32).set_not_null();

        builder.add_column("bool", DataType::Bool).set_not_null();
        builder.add_column("i8", DataType::Int8).set_not_null();
        builder.add_column("i16", DataType::Int16).set_not_null();
        builder.add_column("i32", DataType::Int32).set_not_null();
        builder.add_column("i64", DataType::Int64).set_not_null();
        builder.add_column("timestamp", DataType::Timestamp).set_not_null();
        builder.add_column("f32", DataType::Float).set_not_null();
        builder.add_column("f64", DataType::Double).set_not_null();
        builder.add_column("binary", DataType::Binary).set_not_null();
        builder.add_column("string", DataType::String).set_not_null();

        builder.add_column("nullable_bool", DataType::Bool).set_nullable();
        builder.add_column("nullable_i8", DataType::Int8).set_nullable();
        builder.add_column("nullable_i16", DataType::Int16).set_nullable();
        builder.add_column("nullable_i32", DataType::Int32).set_nullable();
        builder.add_column("nullable_i64", DataType::Int64).set_nullable();
        builder.add_column("nullable_timestamp", DataType::Timestamp).set_nullable();
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
