extern crate kudu_sys;

use std::error;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;
use std::result;
use std::slice;
use std::str;
use std::time::Duration;

pub use kudu_sys::{DataType, CompressionType, EncodingType};

unsafe fn str_into_kudu_slice(s: &str) -> kudu_sys::kudu_slice {
    kudu_sys::kudu_slice { data: s.as_ptr(), len: s.len() }
}

unsafe fn kudu_slice_into_slice<'a>(s: kudu_sys::kudu_slice) -> &'a [u8] {
    slice::from_raw_parts(s.data, s.len)
}

unsafe fn kudu_slice_into_str<'a>(s: kudu_sys::kudu_slice) -> &'a str {
    // TODO: Check if Kudu has a UTF-8 invariant (and fix it if not).
    str::from_utf8(kudu_slice_into_slice(s)).unwrap()
}

unsafe fn slice_into_kudu_slice_list(s: &[kudu_sys::kudu_slice]) -> kudu_sys::kudu_slice_list {
    kudu_sys::kudu_slice_list { data: s.as_ptr(), len: s.len() }
}

pub struct Error {
    inner: *const kudu_sys::kudu_status,
}

impl Error {
    pub fn code(&self) -> i8 {
        unsafe {
            kudu_sys::kudu_status_code(self.inner)
        }
    }
    pub fn posix_code(&self) -> i16 {
        unsafe {
            kudu_sys::kudu_status_posix_code(self.inner)
        }
    }
    pub fn message(&self) -> &str {
        unsafe {
            kudu_slice_into_str(kudu_sys::kudu_status_message(self.inner))
        }
    }
    fn from_status(status: *const kudu_sys::kudu_status) -> Result<()> {
        if status == ptr::null() { return Ok(()) }
        else { return Err(Error { inner: status }) }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_status_destroy(self.inner);
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct ClientBuilder {
    inner: *mut kudu_sys::kudu_client_builder,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            inner: unsafe { kudu_sys::kudu_client_builder_create() },
        }
    }

    pub fn add_master_server_addr(&mut self, addr: &str) -> &mut ClientBuilder {
        // TODO: consider taking ToSocketAddrs instead
        // TODO: handle null error properly
        unsafe {
            kudu_sys::kudu_client_builder_add_master_server_addr(self.inner,
                                                                 str_into_kudu_slice(addr));
        }
        self
    }

    pub fn clear_master_server_addrs(&mut self) -> &mut ClientBuilder {
        unsafe {
            kudu_sys::kudu_client_builder_clear_master_server_addrs(self.inner);
        }
        self
    }

    pub fn set_default_admin_operation_timeout(&mut self, timeout: &Duration) -> &mut ClientBuilder {
        unsafe {
            kudu_sys::kudu_client_builder_set_default_admin_operation_timeout(
                self.inner,
                timeout.as_secs() as i64 * 1_000 + timeout.subsec_nanos() as i64 / 1_000_000);
        }
        self
    }

    pub fn set_default_rpc_timeout(&mut self, timeout: &Duration) -> &mut ClientBuilder {
        unsafe {
            kudu_sys::kudu_client_builder_set_default_rpc_timeout(
                self.inner,
                timeout.as_secs() as i64 * 1_000 + timeout.subsec_nanos() as i64 / 1_000_000);
        }
        self
    }

    // TODO: does this need to consume the builder?
    pub fn build(self) -> Result<Client> {
        let client = ptr::null_mut();;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_client_builder_build(self.inner, &client)));
        }

        Ok(Client {
            inner: client,
        })
    }
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientBuilder")
    }
}

impl Drop for ClientBuilder {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_client_builder_destroy(self.inner);
        }
    }
}

pub struct Client {
    inner: *mut kudu_sys::kudu_client,
}

impl Client {

    pub fn list_tables(&self) -> Result<Vec<&str>> {
        unsafe {
            let list = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_list_tables(self.inner, &list)));
            let size = kudu_sys::kudu_table_list_size(list);
            let mut tables = Vec::with_capacity(size);

            for i in 0..size {
                tables.push(kudu_slice_into_str(kudu_sys::kudu_table_list_table_name(list, i)));
            }
            kudu_sys::kudu_table_list_destroy(list);
            Ok(tables)
        }
    }

    pub fn table_schema(&self, table: &str) -> Result<Schema> {
        unsafe {
            let schema = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_table_schema(self.inner,
                                                                       str_into_kudu_slice(table),
                                                                       &schema)));

            Ok(Schema { inner: schema })
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_client_destroy(self.inner);
        }
    }
}

pub struct Schema {
    inner: *mut kudu_sys::kudu_schema,
}

impl Schema {
    pub fn num_columns(&self) -> usize {
        unsafe {
            return kudu_sys::kudu_schema_num_columns(self.inner);
        }
    }

    pub fn num_primary_key_columns(&self) -> usize {
        unsafe {
            return kudu_sys::kudu_schema_num_key_columns(self.inner);
        }
    }

    pub fn column(&self, index: usize) -> ColumnSchema {
        ColumnSchema {
            inner: unsafe { kudu_sys::kudu_schema_column(self.inner, index) },
        }
    }
}

impl fmt::Debug for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Schema")
    }
}

impl Drop for Schema {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_schema_destroy(self.inner);
        }
    }
}

pub struct ColumnSchema {
    inner: *mut kudu_sys::kudu_column_schema,
}

impl ColumnSchema {
    pub fn name(&self) -> &str {
        unsafe {
            kudu_slice_into_str(kudu_sys::kudu_column_schema_name(self.inner))
        }
    }

    pub fn data_type(&self) -> DataType {
        unsafe {
            kudu_sys::kudu_column_schema_data_type(self.inner)
        }
    }

    pub fn is_nullable(&self) -> bool {
        unsafe {
            kudu_sys::kudu_column_schema_is_nullable(self.inner) != 0
        }
    }

    pub fn encoding_type(&self) -> EncodingType {
        unsafe {
            kudu_sys::kudu_column_schema_encoding_type(self.inner)
        }
    }

    pub fn compression_type(&self) -> CompressionType {
        unsafe {
            kudu_sys::kudu_column_schema_compression_type(self.inner)
        }
    }
}

impl fmt::Debug for ColumnSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColumnSchema({})", self.name())
    }
}

impl Drop for ColumnSchema {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_column_schema_destroy(self.inner);
        }
    }
}

pub struct SchemaBuilder {
    inner: *mut kudu_sys::kudu_schema_builder,
}

impl SchemaBuilder {
    pub fn new(&mut self) -> SchemaBuilder {
        SchemaBuilder {
            inner: unsafe { kudu_sys::kudu_schema_builder_create() },
        }
    }

    pub fn add_column<'a>(&'a mut self, name: &str) -> ColumnSchemaBuilder<'a> {
        unsafe {
            ColumnSchemaBuilder {
                inner: kudu_sys::kudu_schema_builder_add_column(self.inner,
                                                                str_into_kudu_slice(name)),
                phantom: PhantomData,
            }
        }
    }

    pub fn set_primary_key_columns<S>(&mut self, column_names: &[S]) where S: AsRef<str> {
        unsafe {
            let slices = column_names.iter()
                                    .map(|name| str_into_kudu_slice(name.as_ref()))
                                    .collect::<Vec<_>>();
            kudu_sys::kudu_schema_builder_set_primary_key_columns(
                self.inner, slice_into_kudu_slice_list(&slices));
        }
    }

    pub fn kudu_schema_builder_build(&mut self) -> Result<Schema> {
        unsafe {
            let schema = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_schema_builder_build(self.inner, &schema)));
            Ok(Schema { inner: schema })
        }
    }
}

impl fmt::Debug for SchemaBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SchemaBuilder")
    }
}

impl Drop for SchemaBuilder {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_schema_builder_destroy(self.inner);
        }
    }
}

pub struct ColumnSchemaBuilder<'a> {
    inner: *mut kudu_sys::kudu_column_schema_builder,
    phantom: PhantomData<&'a ()>,
}

impl <'a> ColumnSchemaBuilder<'a> {
    pub fn data_type(&mut self, data_type: DataType) -> &mut Self {
        unsafe {
            kudu_sys::kudu_column_schema_builder_data_type(self.inner, data_type);
        }
        self
    }

    pub fn encoding_type(&mut self, encoding_type: EncodingType) -> &mut Self {
        unsafe {
            kudu_sys::kudu_column_schema_builder_encoding_type(self.inner, encoding_type);
        }
        self
    }

    pub fn compression_type(&mut self, compression_type: CompressionType) -> &mut Self {
        unsafe {
            kudu_sys::kudu_column_schema_builder_compression_type(self.inner, compression_type);
        }
        self
    }

    pub fn block_size(&mut self, block_size: i32) -> &mut Self {
        unsafe {
            kudu_sys::kudu_column_schema_builder_block_size(self.inner, block_size);
        }
        self
    }

    pub fn nullable(&mut self, nullable: bool) -> &mut Self {
        unsafe {
            kudu_sys::kudu_column_schema_builder_nullable(self.inner, if nullable { 1 } else { 0 });
        }
        self
    }
}

impl <'a> fmt::Debug for ColumnSchemaBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColumnSchemaBuilder")
    }
}

pub struct TableCreator {
    inner: *mut kudu_sys::kudu_table_creator,
}

impl TableCreator {
    pub fn table_name(&mut self, table_name: &str) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_table_name(self.inner, str_into_kudu_slice(table_name))
        }
        self
    }

    // TODO: this is unsafe, because it doesn't ensure that the schema outlives
    // the table creator.
    pub fn schema(&mut self, schema: &Schema) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_schema(self.inner, schema.inner)
        }
        self
    }

    pub fn add_hash_partitions<S>(&mut self,
                                  column_names: &[S],
                                  num_buckets: i32,
                                  seed: i32)
                                  -> &mut Self
    where S: AsRef<str> {
        unsafe {
            let slices = column_names.iter()
                                     .map(|name| str_into_kudu_slice(name.as_ref()))
                                     .collect::<Vec<_>>();
            kudu_sys::kudu_table_creator_add_hash_partitions(
                self.inner, slice_into_kudu_slice_list(&slices), num_buckets, seed);
        }
        self
    }

    pub fn set_range_partition_columns<S>(&mut self, column_names: &[S]) -> &mut Self
    where S: AsRef<str> {
        unsafe {
            let slices = column_names.iter()
                                     .map(|name| str_into_kudu_slice(name.as_ref()))
                                     .collect::<Vec<_>>();
            kudu_sys::kudu_table_creator_set_range_partition_columns(
                self.inner, slice_into_kudu_slice_list(&slices));
        }
        self
    }

    pub fn num_replicas(&mut self, num_replicas: i32) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_num_replicas(self.inner, num_replicas);
        }
        self
    }

    pub fn timeout(&mut self, timeout: &Duration) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_timeout(
                self.inner,
                timeout.as_secs() as i64 * 1_000 + timeout.subsec_nanos() as i64 / 1_000_000);
        }
        self
    }

    pub fn wait(&mut self, wait: bool) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_wait(self.inner, if wait { 1 } else { 0 });
        }
        self
    }

    pub fn create(&mut self) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_table_creator_create(self.inner))
        }
    }
}

impl Drop for TableCreator {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_table_creator_destroy(self.inner);
        }
    }
}

impl fmt::Debug for TableCreator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TableCreator")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unreachable_master() {
        let mut builder = ClientBuilder::new();
        builder.add_master_server_addr("kudu.example.com");
        let client = builder.build();
        println!("client: {:?}", client);
        assert!(client.is_err());
    }
}
