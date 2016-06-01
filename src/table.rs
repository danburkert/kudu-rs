use std::fmt;

use Schema;
use SchemaBuilder;

use DataType;
use EncodingType;
use CompressionType;

use row::Row;

struct TableBuilder<'a> {
    name: String,
    schema: Schema,
    range_partition_columns: Vec<String>,
    split_rows: Vec<Row<'a>>,
    hash_partitions: Vec<(Vec<String>, u32, Option<u32>)>,
}

impl <'a> TableBuilder<'a> {

    pub fn new(name: String, schema: Schema) -> TableBuilder<'a> {
        TableBuilder {
            name: name,
            schema: schema,
            range_partition_columns: Vec::new(),
            split_rows: Vec::new(),
            hash_partitions: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn set_range_partition_columns(&mut self, columns: Vec<String>) -> &mut TableBuilder<'a> {
        self.range_partition_columns = columns;
        self
    }

    pub fn clear_range_partition_columns(&mut self) -> &mut TableBuilder<'a> {
        self.range_partition_columns.clear();
        self
    }

    pub fn add_split_row(&'a mut self) -> &mut Row {
        let row = Row::new(&self.schema);
        self.split_rows.push(row);
        self.split_rows.last_mut().unwrap()
    }

    pub fn clear_split_rows(&mut self) -> &mut TableBuilder<'a> {
        self.split_rows.clear();
        self
    }

    pub fn add_hash_partitions(&mut self, columns: Vec<String>, num_buckets: u32) -> &mut TableBuilder<'a> {
        self.hash_partitions.push((columns, num_buckets, None));
        self
    }

    pub fn add_hash_partitions_with_seed(&mut self,
                                         columns: Vec<String>,
                                         num_buckets: u32,
                                         seed: u32) -> &mut TableBuilder<'a> {
        self.hash_partitions.push((columns, num_buckets, Some(seed)));
        self
    }

    pub fn clear_hash_partitions(&mut self) -> &mut TableBuilder<'a> {
        self.hash_partitions.clear();
        self
    }
}

struct ColumnBuilder {
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
}

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
