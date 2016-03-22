#![cfg_attr(test, feature(static_mutex))]

extern crate kudu_sys;
#[cfg(test)] extern crate tempdir;
#[cfg(test)] mod test;

use std::error;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::result;
use std::slice;
use std::str;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

pub use kudu_sys::{
    CompressionType,
    DataType,
    EncodingType,
    ExternalConsistencyMode,
    FlushMode,
    OrderMode,
    ReadMode,
};


unsafe fn str_into_kudu_slice(s: &str) -> kudu_sys::kudu_slice {
    kudu_sys::kudu_slice { data: s.as_ptr(), len: s.len() }
}

unsafe fn slice_into_kudu_slice(s: &[u8]) -> kudu_sys::kudu_slice {
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

fn empty_kudu_slice() -> kudu_sys::kudu_slice {
    kudu_sys::kudu_slice { data: ptr::null(), len: 0 }
}

pub struct Error {
    inner: *mut kudu_sys::kudu_status,
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
    fn from_status(status: *mut kudu_sys::kudu_status) -> Result<()> {
        if status == ptr::null_mut() { return Ok(()) }
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
        let mut client = ptr::null_mut();;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_client_builder_build(self.inner, &mut client)));
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

    pub fn new_table_creator(&self) -> TableCreator {
        unsafe {
            TableCreator {
                inner: kudu_sys::kudu_client_new_table_creator(self.inner),
                marker: PhantomData,
            }
        }
    }

    pub fn delete_table(&self, table: &str) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_client_delete_table(self.inner,
                                                                  str_into_kudu_slice(table)))
        }
    }

    pub fn get_table_schema(&self, table: &str) -> Result<Schema> {
        unsafe {
            let mut schema = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_get_table_schema(self.inner,
                                                                           str_into_kudu_slice(table),
                                                                           &mut schema)));

            Ok(Schema { inner: schema })
        }
    }

    pub fn list_tables(&self, filter: &str) -> Result<Vec<String>> {
        unsafe {
            let mut list = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_list_tables(self.inner, str_into_kudu_slice(filter), &mut list)));
            let size = kudu_sys::kudu_table_list_size(list);
            let mut tables = Vec::with_capacity(size);

            for i in 0..size {
                tables.push(kudu_slice_into_str(kudu_sys::kudu_table_list_table_name(list, i)).to_owned());
            }
            kudu_sys::kudu_table_list_destroy(list);
            Ok(tables)
        }
    }

    pub fn list_tablet_servers(&self) -> Result<Vec<TabletServer>> {
        unsafe {
            let mut list = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_list_tablet_servers(self.inner, &mut list)));
            assert!(list != ptr::null_mut());
            let tservers =
                (0..kudu_sys::kudu_tablet_server_list_size(list)).map(|i| {
                    let tserver = kudu_sys::kudu_tablet_server_list_get(list, i);
                    TabletServer {
                        hostname: kudu_slice_into_str(kudu_sys::kudu_tablet_server_hostname(tserver)).to_owned(),
                        uuid: kudu_slice_into_str(kudu_sys::kudu_tablet_server_uuid(tserver)).to_owned(),
                    }
                }).collect();
            kudu_sys::kudu_tablet_server_list_destroy(list);
            Ok(tservers)
        }
    }

    pub fn new_session(&self) -> Session {
        unsafe {
            Session {
                inner: kudu_sys::kudu_client_new_session(self.inner),
                marker: PhantomData,
            }
        }
    }

    pub fn open_table<'a>(&'a self, table_name: &str) -> Result<Table<'a>> {
        unsafe {
            let mut table = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_open_table(self.inner,
                                                                     str_into_kudu_slice(table_name),
                                                                     &mut table)));
            Ok(Table {
                inner: table,
                marker: PhantomData,
            })
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

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

pub struct Table<'a> {
    inner: *mut kudu_sys::kudu_table,
    marker: PhantomData<&'a Client>,
}

impl <'a> Table<'a> {
    pub fn name(&self) -> &str {
        unsafe {
            kudu_slice_into_str(kudu_sys::kudu_table_name(self.inner))
        }
    }
    pub fn id(&self) -> &str {
        unsafe {
            kudu_slice_into_str(kudu_sys::kudu_table_id(self.inner))
        }
    }
    pub fn new_insert(&self) -> Insert<'a> {
        unsafe {
            Insert {
                inner: kudu_sys::kudu_table_new_insert(self.inner),
                marker: PhantomData,
            }
        }
    }
    pub fn new_update(&self) -> Update<'a> {
        unsafe {
            Update {
                inner: kudu_sys::kudu_table_new_update(self.inner),
                marker: PhantomData,
            }
        }
    }
    pub fn new_delete(&self) -> Delete<'a> {
        unsafe {
            Delete {
                inner: kudu_sys::kudu_table_new_delete(self.inner),
                marker: PhantomData,
            }
        }
    }
}

unsafe impl <'a> Send for Table<'a> {}
unsafe impl <'a> Sync for Table<'a> {}

impl <'a> fmt::Debug for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table({})", self.name())
    }
}

impl <'a> Drop for Table<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_table_destroy(self.inner);
        }
    }
}

pub struct Session<'a> {
    inner: *mut kudu_sys::kudu_session,
    marker: PhantomData<&'a Client>,
}

impl <'a> Session<'a> {
    pub fn set_flush_mode(&mut self, mode: FlushMode) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_set_flush_mode(self.inner, mode)) }
    }

    pub fn set_external_consistency_mode(&mut self, mode: ExternalConsistencyMode) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_set_external_consistency_mode(self.inner, mode)) }
    }

    pub fn set_timeout(&mut self, timeout: &Duration) {
        unsafe {
            kudu_sys::kudu_session_set_timeout_millis(self.inner,
                                                      timeout.as_secs() as i32 * 1_000 +
                                                      timeout.subsec_nanos() as i32 / 1_000_000);
        }
    }

    pub fn insert(&self, insert: Insert) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_insert(self.inner, insert.into_inner())) }
    }

    pub fn update(&self, update: Update) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_update(self.inner, update.into_inner())) }
    }

    pub fn delete(&self, delete: Delete) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_delete(self.inner, delete.into_inner())) }
    }

    pub fn flush(&self) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_session_flush(self.inner)) }
    }

    pub fn has_pending_operations(&self) -> bool {
        unsafe {
            if kudu_sys::kudu_session_has_pending_operations(self.inner) == 0 { false } else { true }
        }
    }

    pub fn count_buffered_operations(&self) -> i32 {
        unsafe {
            kudu_sys::kudu_session_count_buffered_operations(self.inner)
        }
    }

    pub fn count_pending_errors(&self) -> i32 {
        unsafe {
            kudu_sys::kudu_session_count_pending_errors(self.inner)
        }
    }

    pub fn close(self) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_session_close(self.inner))
        }
    }
}

impl <'a> fmt::Debug for Session<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Session")
    }
}

unsafe impl <'a> Send for Session<'a> {}
unsafe impl <'a> Sync for Session<'a> {}

impl <'a> Drop for Session<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_session_close(self.inner);
            kudu_sys::kudu_session_destroy(self.inner);
        }
    }
}

pub struct Insert<'a> {
    inner: *mut kudu_sys::kudu_insert,
    marker: PhantomData<Session<'a>>,
}

impl <'a> Insert<'a> {
    pub fn row(&mut self) -> PartialRow<'a> {
        unsafe {
            PartialRow {
                inner: kudu_sys::kudu_insert_mutable_row(self.inner),
                drop: false,
                marker: PhantomData,
            }
        }
    }
    fn into_inner(self) -> *mut kudu_sys::kudu_insert {
        let inner = self.inner;
        mem::forget(self);
        inner
    }
}

impl <'a> fmt::Debug for Insert<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insert")
    }
}

impl <'a> Drop for Insert<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_insert_destroy(self.inner);
        }
    }
}

pub struct Update<'a> {
    inner: *mut kudu_sys::kudu_update,
    marker: PhantomData<Session<'a>>,
}

impl <'a> Update<'a> {
    pub fn row(&mut self) -> PartialRow<'a> {
        unsafe {
            PartialRow {
                inner: kudu_sys::kudu_update_mutable_row(self.inner),
                drop: false,
                marker: PhantomData,
            }
        }
    }
    fn into_inner(self) -> *mut kudu_sys::kudu_update {
        let inner = self.inner;
        mem::forget(self);
        inner
    }
}

impl <'a> fmt::Debug for Update<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Update")
    }
}

impl <'a> Drop for Update<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_update_destroy(self.inner);
        }
    }
}

pub struct Delete<'a> {
    inner: *mut kudu_sys::kudu_delete,
    marker: PhantomData<Session<'a>>,
}

impl <'a> Delete<'a> {
    pub fn row(&mut self) -> PartialRow<'a> {
        unsafe {
            PartialRow {
                inner: kudu_sys::kudu_delete_mutable_row(self.inner),
                drop: false,
                marker: PhantomData,
            }
        }
    }
    fn into_inner(self) -> *mut kudu_sys::kudu_delete {
        let inner = self.inner;
        mem::forget(self);
        inner
    }
}

impl <'a> fmt::Debug for Delete<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Delete")
    }
}

impl <'a> Drop for Delete<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_delete_destroy(self.inner);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TabletServer {
    pub hostname: String,
    pub uuid: String,
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

    // TODO: this should return an Option
    pub fn column(&self, index: usize) -> ColumnSchema {
        ColumnSchema {
            inner: unsafe { kudu_sys::kudu_schema_column(self.inner, index) },
        }
    }

    /// Returns the index of the column, if it is in the schema.
    pub fn find_column(&self, column_name: &str) -> Result<usize> {
        unsafe {
            let mut index = 0;
            try!(Error::from_status(
                    kudu_sys::kudu_schema_find_column(self.inner,
                                                      str_into_kudu_slice(column_name),
                                                      &mut index)));
            Ok(index)
        }
    }

    pub fn new_row<'a>(&'a self) -> PartialRow<'a> {
        PartialRow {
            inner: unsafe { kudu_sys::kudu_schema_new_row(self.inner) },
            drop: true,
            marker: PhantomData,
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
    pub fn new() -> SchemaBuilder {
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

    pub fn build(&mut self) -> Result<Schema> {
        unsafe {
            let mut schema = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_schema_builder_build(self.inner, &mut schema)));
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
    phantom: PhantomData<&'a SchemaBuilder>,
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

pub struct TableCreator<'a> {
    inner: *mut kudu_sys::kudu_table_creator,
    marker: PhantomData<&'a Schema>,
}

impl <'a> TableCreator<'a> {
    pub fn table_name(&mut self, table_name: &str) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_table_name(self.inner, str_into_kudu_slice(table_name))
        }
        self
    }

    // Set the schema. The schema must outlive the creator.
    pub fn schema<'b>(&mut self, schema: &'b Schema) -> &mut Self where 'b: 'a {
        unsafe {
            kudu_sys::kudu_table_creator_schema(self.inner, schema.inner)
        }
        self
    }

    pub fn add_hash_partitions<S>(&mut self, column_names: &[S], num_buckets: i32, seed: i32) -> &mut Self
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

    pub fn add_split_row(&mut self, mut split_row: PartialRow) -> &mut Self {
        unsafe {
            kudu_sys::kudu_table_creator_add_split_row(self.inner, split_row.inner);
            assert!(split_row.drop);
            split_row.drop = false;
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

impl <'a> Drop for TableCreator<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_table_creator_destroy(self.inner);
        }
    }
}

impl <'a> fmt::Debug for TableCreator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TableCreator")
    }
}

pub struct ScanBuilder<'a> {
    inner: *mut kudu_sys::kudu_scanner,
    marker: PhantomData<Table<'a>>,
}

impl <'a> ScanBuilder<'a> {
    pub fn new(table: &'a Table) -> ScanBuilder<'a> {
        unsafe {
            ScanBuilder {
                inner: kudu_sys::kudu_scanner_create(table.inner),
                marker: PhantomData,
            }
        }
    }

    pub fn set_projected_column_names<S>(&mut self, column_names: &[S]) -> Result<()>
    where S: AsRef<str> {
        unsafe {
            let slices = column_names.iter()
                                     .map(|name| str_into_kudu_slice(name.as_ref()))
                                     .collect::<Vec<_>>();
            Error::from_status(kudu_sys::kudu_scanner_set_projected_column_names(
                    self.inner, slice_into_kudu_slice_list(&slices)))
        }
    }

    pub fn set_projected_column_indexes(&mut self, column_idxs: &[usize]) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_projected_column_indexes(
                    self.inner, column_idxs.as_ptr(), column_idxs.len()))
        }
    }

    pub fn add_lower_bound(&mut self, bound: &PartialRow) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_add_lower_bound(self.inner, bound.inner))
        }
    }

    pub fn add_upper_bound(&mut self, bound: &PartialRow) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_add_upper_bound(self.inner, bound.inner))
        }
    }

    pub fn set_cache_blocks(&mut self, cache_blocks: bool) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_cache_blocks(self.inner, if cache_blocks { 1 } else { 0 }))
        }
    }

    pub fn set_batch_size_bytes(&mut self, batch_size: usize) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_batch_size_bytes(self.inner, batch_size))
        }
    }

    pub fn set_read_mode(&mut self, mode: ReadMode) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_read_mode(self.inner, mode))
        }
    }

    pub fn set_order_mode(&mut self, mode: OrderMode) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_order_mode(self.inner, mode))
        }
    }

    pub fn set_fault_tolerant(&mut self) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_fault_tolerant(self.inner))
        }
    }

    pub fn set_snapshot(&mut self, timestamp: &SystemTime) -> Result<()> {
        unsafe {
            // TODO: handle this better
            let duration = timestamp.duration_since(UNIX_EPOCH).unwrap();
            let value = duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000;
            Error::from_status(kudu_sys::kudu_scanner_set_snapshot_micros(self.inner, value))
        }
    }

    pub fn set_snapshot_raw(&mut self, timestamp: u64) -> Result<()> {
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_snapshot_raw(self.inner, timestamp))
        }
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        let millis = timeout.as_secs() as i32 * 1_000 + timeout.subsec_nanos() as i32 / 1_000_000;
        unsafe {
            Error::from_status(kudu_sys::kudu_scanner_set_timeout_millis(self.inner, millis))
        }
    }

    fn into_inner(self) -> *mut kudu_sys::kudu_scanner {
        let inner = self.inner;
        mem::forget(self);
        inner
    }

    pub fn build(self) -> Result<Scanner<'a>> {
        unsafe { try!(Error::from_status(kudu_sys::kudu_scanner_open(self.inner))); }
        Ok(Scanner {
            inner: self.into_inner(),
            marker: PhantomData,
        })
    }
}

impl <'a> Drop for ScanBuilder<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_scanner_destroy(self.inner);
        }
    }
}

impl <'a> fmt::Debug for ScanBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanBuilder")
    }
}

pub struct Scanner<'a> {
    inner: *mut kudu_sys::kudu_scanner,
    marker: PhantomData<Table<'a>>,
}

impl <'a> Scanner<'a> {
    pub fn has_more_rows(&mut self) -> bool {
        unsafe {
            if kudu_sys::kudu_scanner_has_more_rows(self.inner) == 0 { false } else { true }
        }
    }
    pub fn next_batch(&mut self, batch: ScanBatch) -> Result<ScanBatch<'a>> {
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scanner_next_batch(self.inner, batch.inner)));
        }
        let b = ScanBatch {
            inner: batch.into_inner(),
            marker: PhantomData,
        };
        Ok(b)
    }
    pub fn get_projection_schema(&self) -> Schema {
        unsafe {
            Schema {
                inner: kudu_sys::kudu_scanner_get_projection_schema(self.inner),
            }
        }
    }
}

impl <'a> Drop for Scanner<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_scanner_close(self.inner);
            kudu_sys::kudu_scanner_destroy(self.inner);
        }
    }
}

impl <'a> fmt::Debug for Scanner<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scanner")
    }
}

pub struct ScanBatch<'a> {
    inner: *mut kudu_sys::kudu_scan_batch,
    marker: PhantomData<Scanner<'a>>,
}

impl <'a> ScanBatch<'a> {
    pub fn new() -> ScanBatch<'static> {
        unsafe {
            ScanBatch {
                inner: kudu_sys::kudu_scan_batch_create(),
                marker: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        unsafe { kudu_sys::kudu_scan_batch_num_rows(self.inner) }
    }
    pub fn get<'b>(&'b self, index: usize) -> Option<ScanBatchRow<'b>> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked<'b>(&'b self, index: usize) -> ScanBatchRow<'b> {
        ScanBatchRow {
            inner: kudu_sys::kudu_scan_batch_row(self.inner, index),
            marker: PhantomData,
        }
    }
    fn into_inner(self) -> *mut kudu_sys::kudu_scan_batch {
        let inner = self.inner;
        mem::forget(self);
        inner
    }
}

impl <'a> fmt::Debug for ScanBatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanBatch")
    }
}

impl <'a> Drop for ScanBatch<'a> {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_scan_batch_destroy(self.inner);
        }
    }
}

pub trait ColumnType<'a>: Sized {
    fn get(&'a PartialRow, column_idx: usize) -> Result<Self>;
    fn set(self, row: &mut PartialRow<'a>, column_idx: usize) -> Result<()>;
    fn get_by_name(&'a PartialRow, column: &str) -> Result<Self>;
    fn set_by_name(self, row: &mut PartialRow<'a>, column: &str) -> Result<()>;
    fn get_batch(&'a ScanBatchRow, column_idx: usize) -> Result<Self>;
    fn get_batch_by_name(&'a ScanBatchRow, column: &str) -> Result<Self>;
}

pub trait VarLengthColumnType<'a>: Sized + ColumnType<'a> {
    fn set_copy(self, row: &mut PartialRow, column_idx: usize) -> Result<()>;
    fn set_copy_by_name(self, row: &mut PartialRow, column: &str) -> Result<()>;
}

impl <'a> ColumnType<'a> for bool {
    fn get(row: &PartialRow, column_idx: usize) -> Result<bool> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_bool(row.inner, column_idx, &mut value)));
        }
        Ok(if value == 0 { false } else { true })
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_bool(row.inner, column_idx, if self { 1 } else { 0 })) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<bool> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_bool_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(if value == 0 { false } else { true })
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_bool_by_name(row.inner, str_into_kudu_slice(column), if self { 1 } else { 0 })) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<bool> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_bool(&row.inner, column_idx, &mut value)));
        }
        Ok(if value == 0 { false } else { true })
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<bool> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_bool_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(if value == 0 { false } else { true })
    }
}

impl <'a> ColumnType<'a> for i8 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<i8> {
        let mut value: i8 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int8(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int8(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<i8> {
        let mut value: i8 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int8_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int8_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<i8> {
        let mut value: i8 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int8(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<i8> {
        let mut value: i8 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int8_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for i16 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<i16> {
        let mut value: i16 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int16(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int16(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<i16> {
        let mut value: i16 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int16_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int16_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<i16> {
        let mut value: i16 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int16(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<i16> {
        let mut value: i16 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int16_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for i32 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<i32> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int32(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int32(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<i32> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int32_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int32_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<i32> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int32(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<i32> {
        let mut value: i32 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int32_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for i64 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<i64> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int64(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int64(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<i64> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_int64_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_int64_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<i64> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int64(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<i64> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_int64_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for SystemTime {
    fn get(row: &PartialRow, column_idx: usize) -> Result<SystemTime> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_timestamp(row.inner, column_idx, &mut value)));
        }

        if value < 0 {
            let value = !(value as u64) + 1;
            Ok(UNIX_EPOCH - Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        } else {
            let value = value as u64;
            Ok(UNIX_EPOCH + Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        }
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        let value = match self.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                (duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64
            },
            Err(err) => {
                let duration = err.duration();
                let micros = duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000;
                -(micros as i64)
            }
        };

        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_timestamp(row.inner, column_idx, value)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<SystemTime> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_timestamp_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }

        if value < 0 {
            let value = !(value as u64) + 1;
            Ok(UNIX_EPOCH - Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        } else {
            let value = value as u64;
            Ok(UNIX_EPOCH + Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        }
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        let value = match self.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                (duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000) as i64
            },
            Err(err) => {
                let duration = err.duration();
                let micros = duration.as_secs() * 1000_000 + duration.subsec_nanos() as u64 / 1000;
                -(micros as i64)
            }
        };

        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_timestamp_by_name(row.inner, str_into_kudu_slice(column), value)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<SystemTime> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_timestamp(&row.inner, column_idx, &mut value)));
        }

        if value < 0 {
            let value = !(value as u64) + 1;
            Ok(UNIX_EPOCH - Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        } else {
            let value = value as u64;
            Ok(UNIX_EPOCH + Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        }
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<SystemTime> {
        let mut value: i64 = 0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_timestamp_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }

        if value < 0 {
            let value = !(value as u64) + 1;
            Ok(UNIX_EPOCH - Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        } else {
            let value = value as u64;
            Ok(UNIX_EPOCH + Duration::new(value / 1000_000, (value % 1000_000) as u32 * 1000))
        }
    }
}

impl <'a> ColumnType<'a> for f32 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<f32> {
        let mut value: f32 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_float(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_float(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<f32> {
        let mut value: f32 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_float_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_float_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<f32> {
        let mut value: f32 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_float(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<f32> {
        let mut value: f32 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_float_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for f64 {
    fn get(row: &PartialRow, column_idx: usize) -> Result<f64> {
        let mut value: f64 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_double(row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_double(row.inner, column_idx, self)) }
    }
    fn get_by_name(row: &PartialRow, column: &str) -> Result<f64> {
        let mut value: f64 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_double_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_double_by_name(row.inner, str_into_kudu_slice(column), self)) }
    }
    fn get_batch(row: &ScanBatchRow, column_idx: usize) -> Result<f64> {
        let mut value: f64 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_double(&row.inner, column_idx, &mut value)));
        }
        Ok(value)
    }
    fn get_batch_by_name(row: &ScanBatchRow, column: &str) -> Result<f64> {
        let mut value: f64 = 0.0;
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_double_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
        }
        Ok(value)
    }
}

impl <'a> ColumnType<'a> for &'a str {
    fn get(row: &'a PartialRow, column_idx: usize) -> Result<&'a str> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_string(row.inner, column_idx, &mut value)));
            Ok(kudu_slice_into_str(value))
        }
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_string(row.inner, column_idx, str_into_kudu_slice(self))) }
    }
    fn get_by_name(row: &'a PartialRow, column: &str) -> Result<&'a str> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_string_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
            Ok(kudu_slice_into_str(value))
        }
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_string_by_name(row.inner, str_into_kudu_slice(column), str_into_kudu_slice(self))) }
    }
    fn get_batch(row: &'a ScanBatchRow, column_idx: usize) -> Result<&'a str> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_string(&row.inner, column_idx, &mut value)));
            Ok(kudu_slice_into_str(value))
        }
    }
    fn get_batch_by_name(row: &'a ScanBatchRow, column: &str) -> Result<&'a str> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_string_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
            Ok(kudu_slice_into_str(value))
        }
    }
}

impl <'a> VarLengthColumnType<'a> for &'a str {
    fn set_copy(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_string_copy(row.inner, column_idx, str_into_kudu_slice(self))) }
    }
    fn set_copy_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_string_copy_by_name(row.inner, str_into_kudu_slice(column), str_into_kudu_slice(self))) }
    }
}

impl <'a> ColumnType<'a> for &'a [u8] {
    fn get(row: &'a PartialRow, column_idx: usize) -> Result<&'a [u8]> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_binary(row.inner, column_idx, &mut value)));
            Ok(kudu_slice_into_slice(value))
        }
    }
    fn set(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_binary(row.inner, column_idx, slice_into_kudu_slice(self))) }
    }
    fn get_by_name(row: &'a PartialRow, column: &str) -> Result<&'a [u8]> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_partial_row_get_binary_by_name(row.inner, str_into_kudu_slice(column), &mut value)));
            Ok(kudu_slice_into_slice(value))
        }
    }
    fn set_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_binary_by_name(row.inner, str_into_kudu_slice(column), slice_into_kudu_slice(self))) }
    }
    fn get_batch(row: &'a ScanBatchRow, column_idx: usize) -> Result<&'a [u8]> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_binary(&row.inner, column_idx, &mut value)));
            Ok(kudu_slice_into_slice(value))
        }
    }
    fn get_batch_by_name(row: &'a ScanBatchRow, column: &str) -> Result<&'a [u8]> {
        let mut value: kudu_sys::kudu_slice = empty_kudu_slice();
        unsafe {
            try!(Error::from_status(kudu_sys::kudu_scan_batch_row_ptr_get_binary_by_name(&row.inner, str_into_kudu_slice(column), &mut value)));
            Ok(kudu_slice_into_slice(value))
        }
    }
}

impl <'a> VarLengthColumnType<'a> for &'a [u8] {
    fn set_copy(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_binary_copy(row.inner, column_idx, slice_into_kudu_slice(self))) }
    }
    fn set_copy_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_binary_copy_by_name(row.inner, str_into_kudu_slice(column), slice_into_kudu_slice(self))) }
    }
}

impl <'a, T> ColumnType<'a> for Option<T> where T: ColumnType<'a> {
    fn get(row: &'a PartialRow, column_idx: usize) -> Result<Option<T>> {
        if row.is_null(column_idx) {
            Ok(None)
        } else {
            ColumnType::get(row, column_idx).map(|t| Some(t))
        }
    }
    fn set(self, row: &mut PartialRow<'a>, column_idx: usize) -> Result<()> {
        match self {
            Some(t) => t.set(row, column_idx),
            None => row.set_null(column_idx),
        }
    }
    fn get_by_name(row: &'a PartialRow, column: &str) -> Result<Option<T>> {
        if row.is_null_by_name(column) {
            Ok(None)
        } else {
            ColumnType::get_by_name(row, column).map(|t| Some(t))
        }
    }
    fn set_by_name(self, row: &mut PartialRow<'a>, column: &str) -> Result<()> {
        match self {
            Some(t) => t.set_by_name(row, column),
            None => row.set_null_by_name(column),
        }
    }
    fn get_batch(row: &'a ScanBatchRow, column_idx: usize) -> Result<Option<T>> {
        if row.is_null(column_idx) {
            Ok(None)
        } else {
            ColumnType::get_batch(row, column_idx).map(|t| Some(t))
        }
    }
    fn get_batch_by_name(row: &'a ScanBatchRow, column: &str) -> Result<Option<T>> {
        if row.is_null_by_name(column) {
            Ok(None)
        } else {
            ColumnType::get_batch_by_name(row, column).map(|t| Some(t))
        }
    }
}

impl <'a, T> VarLengthColumnType<'a> for Option<T> where T: VarLengthColumnType<'a> {
    fn set_copy(self, row: &mut PartialRow, column_idx: usize) -> Result<()> {
        match self {
            Some(t) => t.set_copy(row, column_idx),
            None => row.set_null(column_idx),
        }
    }
    fn set_copy_by_name(self, row: &mut PartialRow, column: &str) -> Result<()> {
        match self {
            Some(t) => t.set_copy_by_name(row, column),
            None => row.set_null_by_name(column),
        }
    }
}

pub struct PartialRow<'a> {
    inner: *mut kudu_sys::kudu_partial_row,
    drop: bool,
    marker: PhantomData<&'a Schema>,
}

impl <'a> PartialRow<'a> {

    // by column index

    pub fn set<T>(&mut self, column_idx: usize, value: T) -> Result<()> where T: ColumnType<'a> {
        T::set(value, self, column_idx)
    }

    pub fn set_copy<'b, T>(&mut self, column_idx: usize, value: T) -> Result<()> where T: VarLengthColumnType<'b> {
        T::set_copy(value, self, column_idx)
    }

    pub fn set_null(&mut self, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_null(self.inner, column_idx)) }
    }

    pub fn unset(&mut self, column_idx: usize) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_unset(self.inner, column_idx)) }
    }

    pub fn get<T>(&'a self, column_idx: usize) -> Result<T> where T: ColumnType<'a> {
        T::get(self, column_idx)
    }

    pub fn is_null(&self, column_idx: usize) -> bool {
        unsafe {
            if kudu_sys::kudu_partial_row_is_null(self.inner, column_idx) == 0 { false } else { true }
        }
    }

    pub fn is_set(&self, column_idx: usize) -> bool {
        unsafe {
            if kudu_sys::kudu_partial_row_is_set(self.inner, column_idx) == 0 { false } else { true }
        }
    }

    // by name

    pub fn set_by_name<T>(&mut self, column: &str, value: T) -> Result<()> where T: ColumnType<'a> {
        T::set_by_name(value, self, column)
    }

    pub fn set_copy_by_name<'b, T>(&mut self, column: &str, value: T) -> Result<()> where T: VarLengthColumnType<'b> {
        T::set_copy_by_name(value, self, column)
    }

    pub fn set_null_by_name(&mut self, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_set_null_by_name(self.inner, str_into_kudu_slice(column))) }
    }

    pub fn unset_by_name(&mut self, column: &str) -> Result<()> {
        unsafe { Error::from_status(kudu_sys::kudu_partial_row_unset_by_name(self.inner, str_into_kudu_slice(column))) }
    }

    pub fn get_by_name<T>(&'a self, column: &str) -> Result<T> where T: ColumnType<'a> {
        T::get_by_name(self, column)
    }

    pub fn is_null_by_name(&self, column: &str) -> bool {
        unsafe {
            if kudu_sys::kudu_partial_row_is_null_by_name(self.inner, str_into_kudu_slice(column)) == 0 { false } else { true }
        }
    }

    pub fn is_set_by_name(&self, column: &str) -> bool {
        unsafe {
            if kudu_sys::kudu_partial_row_is_set_by_name(self.inner, str_into_kudu_slice(column)) == 0 { false } else { true }
        }
    }
}

impl <'a> Drop for PartialRow<'a> {
    fn drop(&mut self) {
        if self.drop {
            unsafe { kudu_sys::kudu_partial_row_destroy(self.inner) };
        }
    }
}

impl <'a> fmt::Debug for PartialRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PartialRow")
    }
}

pub struct ScanBatchRow<'a> {
    inner: kudu_sys::kudu_scan_batch_row_ptr,
    marker: PhantomData<&'a Schema>,
}

impl <'a> ScanBatchRow<'a> {

    pub fn get<T>(&'a self, column_idx: usize) -> Result<T> where T: ColumnType<'a> {
        T::get_batch(self, column_idx)
    }

    pub fn is_null(&self, column_idx: usize) -> bool {
        unsafe {
            if kudu_sys::kudu_scan_batch_row_ptr_is_null(&self.inner, column_idx) == 0 { false } else { true }
        }
    }

    pub fn get_by_name<T>(&'a self, column: &str) -> Result<T> where T: ColumnType<'a> {
        T::get_batch_by_name(self, column)
    }

    pub fn is_null_by_name(&self, column: &str) -> bool {
        unsafe {
            if kudu_sys::kudu_scan_batch_row_ptr_is_null_by_name(&self.inner, str_into_kudu_slice(column)) == 0 { false } else { true }
        }
    }
}

impl <'a> fmt::Debug for ScanBatchRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanBatchRow")
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use std::time::{Duration, SystemTime};

    use super::*;
    use test::{MiniCluster, MiniClusterConfig};

    fn threadsafe<T>(_: T) where T: Sync + Send { }

    #[test]
    fn test_unreachable_master() {
        let mut builder = ClientBuilder::new();
        builder.add_master_server_addr("kudu.example.com");
        let client = builder.build();
        assert!(client.is_err());
    }

    #[test]
    fn test_create_client() {
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let client = cluster.client();

        // Client instances are thread safe
        threadsafe(client);
    }

    const COLUMN_NAMES: &'static [&'static str] = &["int8","timestamp", "string", "int16", "int32",
                                                    "int64", "bool", "float", "double", "binary"];

    fn all_types_schema() -> Schema {
        let mut builder = SchemaBuilder::new();
        builder.add_column(COLUMN_NAMES[0]).data_type(DataType::Int8)
               .encoding_type(EncodingType::Prefix)
               .compression_type(CompressionType::Lz4)
               .block_size(1024)
               .nullable(false);
        builder.add_column(COLUMN_NAMES[1])
               .data_type(DataType::Timestamp)
               .encoding_type(EncodingType::GroupVarint)
               .compression_type(CompressionType::Default)
               .block_size(1024)
               .nullable(false);
        builder.add_column(COLUMN_NAMES[2]).data_type(DataType::String)
               .encoding_type(EncodingType::Dictionary)
               .compression_type(CompressionType::Snappy)
               .block_size(1024)
               .nullable(false);
        builder.add_column(COLUMN_NAMES[3]).data_type(DataType::Int16);
        builder.add_column(COLUMN_NAMES[4]).data_type(DataType::Int32);
        builder.add_column(COLUMN_NAMES[5]).data_type(DataType::Int64);
        builder.add_column(COLUMN_NAMES[6]).data_type(DataType::Bool);
        builder.add_column(COLUMN_NAMES[7]).data_type(DataType::Float);
        builder.add_column(COLUMN_NAMES[8]).data_type(DataType::Double);
        builder.add_column(COLUMN_NAMES[9]).data_type(DataType::Binary);

        builder.set_primary_key_columns(&[COLUMN_NAMES[0], COLUMN_NAMES[1], COLUMN_NAMES[2]]);
        builder.build().unwrap()
    }

    fn kv_schema() -> Schema {
        let mut builder = SchemaBuilder::new();
        builder.add_column("key").data_type(DataType::String).nullable(false);
        builder.add_column("value").data_type(DataType::String).nullable(true);
        builder.set_primary_key_columns(&["key"]);
        builder.build().unwrap()
    }

    #[test]
    fn test_create_schema() {
        let schema = all_types_schema();
        assert_eq!(10, schema.num_columns());
        assert_eq!(3, schema.num_primary_key_columns());

        for (index, name) in COLUMN_NAMES.iter().enumerate() {
            assert_eq!(index, schema.find_column(name).unwrap());
            assert_eq!(COLUMN_NAMES[index], schema.column(index).name());
        }
    }

    #[test]
    fn test_partial_row() {
        let schema = all_types_schema();
        let mut row = schema.new_row();
        let now = SystemTime::now();

        for i in 0..schema.num_columns() {
            assert!(!row.is_set(i));
        }

        row.set(0, 42i8).unwrap();
        row.set(1, now).unwrap();
        row.set(2, "hebejebe").unwrap();
        row.set(3, 42i16).unwrap();
        row.set(4, 42i32).unwrap();
        row.set(5, 42i64).unwrap();
        row.set(6, true).unwrap();
        row.set(7, 123.456f32).unwrap();
        row.set(8, 123.456).unwrap();
        row.set(9, &b"hebejebe"[..]).unwrap();

        for i in 0..schema.num_columns() {
            println!("testing i = {}" , i);
            assert!(row.is_set(i));
        }

        assert_eq!(42i8, row.get(0).unwrap());
        assert_eq!(now, row.get(1).unwrap());
        assert_eq!("hebejebe", row.get::<&str>(2).unwrap());
        assert_eq!(42i16, row.get(3).unwrap());
        assert_eq!(42i32, row.get(4).unwrap());
        assert_eq!(42i64, row.get(5).unwrap());
        assert_eq!(true, row.get(6).unwrap());
        assert_eq!(123.456f32, row.get(7).unwrap());
        assert_eq!(123.456, row.get(8).unwrap());
        assert_eq!(b"hebejebe", row.get::<&[u8]>(9).unwrap());

        row.set::<Option<bool>>(6, None).unwrap();
        assert_eq!(None, row.get::<Option<bool>>(6).unwrap());
        assert!(row.is_set(6));
        assert!(row.is_null(6));

        row.set(6, Some(true)).unwrap();
        assert_eq!(true, row.get(6).unwrap());
        assert!(row.is_set(6));
        assert!(!row.is_null(6));

        row.set_null(6).unwrap();
        assert!(row.is_set(6));
        assert!(row.is_null(6));

        assert!(row.set_null(0).is_err());
    }

    #[test]
    fn test_partial_row_by_name() {
        let schema = all_types_schema();
        let mut row = schema.new_row();
        let now = SystemTime::now();

        for i in 0..schema.num_columns() {
            assert!(!row.is_set_by_name(schema.column(i).name()));
        }

        row.set_by_name("int8", 42i8).unwrap();
        row.set_by_name("timestamp", now).unwrap();
        row.set_by_name("string", "hebejebe").unwrap();
        row.set_by_name("int16", 42i16).unwrap();
        row.set_by_name("int32", 42i32).unwrap();
        row.set_by_name("int64", 42i64).unwrap();
        row.set_by_name("bool", true).unwrap();
        row.set_by_name("float", 123.456f32).unwrap();
        row.set_by_name("double", 123.456).unwrap();
        row.set_by_name("binary", &b"hebejebe"[..]).unwrap();

        for i in 0..schema.num_columns() {
            println!("testing i = {}" , i);
            assert!(row.is_set_by_name(schema.column(i).name()));
        }

        assert_eq!(42i8, row.get_by_name("int8").unwrap());
        assert_eq!(now, row.get_by_name("timestamp").unwrap());
        assert_eq!("hebejebe", row.get_by_name::<&str>("string").unwrap());
        assert_eq!(42i16, row.get_by_name("int16").unwrap());
        assert_eq!(42i32, row.get_by_name("int32").unwrap());
        assert_eq!(42i64, row.get_by_name("int64").unwrap());
        assert_eq!(true, row.get_by_name("bool").unwrap());
        assert_eq!(123.456f32, row.get_by_name("float").unwrap());
        assert_eq!(123.456, row.get_by_name("double").unwrap());
        assert_eq!(b"hebejebe", row.get_by_name::<&[u8]>("binary").unwrap());

        row.set_by_name::<Option<bool>>("bool", None).unwrap();
        assert_eq!(None, row.get_by_name::<Option<bool>>("bool").unwrap());
        assert!(row.is_set_by_name("bool"));
        assert!(row.is_null_by_name("bool"));

        row.set_by_name("bool", Some(true)).unwrap();
        assert_eq!(true, row.get_by_name("bool").unwrap());
        assert!(row.is_set_by_name("bool"));
        assert!(!row.is_null_by_name("bool"));

        row.set_null_by_name("bool").unwrap();
        assert!(row.is_set_by_name("bool"));
        assert!(row.is_null_by_name("bool"));

        assert!(row.set_null_by_name("int8").is_err());
    }

    #[test]
    fn test_list_tablet_servers() {
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let tablet_servers = cluster.client().list_tablet_servers().unwrap();
        assert_eq!(vec!["127.0.0.1"], tablet_servers.iter().map(|ts| &ts.hostname).collect::<Vec<_>>());
        assert!(tablet_servers.iter().all(|ts| ts.uuid.len() == 32));
    }

    #[test]
    fn test_create_drop_list_table() {
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let client = cluster.client();

        // Table list is initially empty
        assert_eq!(Vec::<String>::new(), cluster.client().list_tables("").unwrap());

        let schema = all_types_schema();
        let mut split_row = schema.new_row();
        split_row.set_by_name("string", "m").unwrap();
        client.new_table_creator()
              .table_name("t")
              .schema(&schema)
              .num_replicas(1)
              .wait(true)
              .set_range_partition_columns(&["string"])
              .add_hash_partitions(&["int8", "timestamp"], 2, 0)
              .add_split_row(split_row)
              .create()
              .unwrap();

        assert_eq!(vec!["t".to_owned()], cluster.client().list_tables("").unwrap());
        assert_eq!(vec!["t".to_owned()], cluster.client().list_tables("t").unwrap());
        assert_eq!(Vec::<String>::new(), cluster.client().list_tables("a").unwrap());

        client.delete_table("t").unwrap();
        assert_eq!(Vec::<String>::new(), cluster.client().list_tables("").unwrap());
        assert!(client.delete_table("t").is_err());
    }

    #[test]
    fn test_insert_update_delete() {
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let client = cluster.client();

        let schema = kv_schema();
        client.new_table_creator()
              .table_name("table")
              .schema(&schema)
              .num_replicas(1)
              .wait(true)
              .create()
              .unwrap();

        let table = client.open_table("table").unwrap();
        assert_eq!("table", table.name());

        let mut session = client.new_session();
        session.set_timeout(&Duration::from_secs(10));

        let mut insert = table.new_insert();
        insert.row().set(0, "my-key").unwrap();
        insert.row().set(1, "my-val").unwrap();
        session.insert(insert).unwrap();

        let mut update = table.new_update();
        update.row().set(0, "my-key").unwrap();
        update.row().set(1, "new-val").unwrap();
        session.update(update).unwrap();

        let mut insert = table.new_insert();
        insert.row().set(0, "my-key").unwrap();
        insert.row().set(1, "duplicate").unwrap();
        assert!(session.insert(insert).is_err());

        session.set_flush_mode(FlushMode::ManualFlush).unwrap();

        let mut insert = table.new_insert();
        insert.row().set(0, "my-key").unwrap();
        insert.row().set(1, "bogus").unwrap();
        session.insert(insert).unwrap();
        assert!(session.flush().is_err());

        let mut delete = table.new_delete();
        delete.row().set(0, "my-key").unwrap();
        session.delete(delete).unwrap();
        session.flush().unwrap();

        let mut delete = table.new_delete();
        delete.row().set(0, "my-key").unwrap();
        session.delete(delete).unwrap();
        assert!(session.flush().is_err());

        let mut update = table.new_update();
        update.row().set(0, "my-key").unwrap();
        session.update(update).unwrap();
        assert!(session.flush().is_err());

        let mut insert = table.new_insert();
        insert.row().set(0, "my-key").unwrap();
        insert.row().set(1, "bogus").unwrap();
        session.insert(insert).unwrap();
        session.flush().unwrap();

        session.close().unwrap();
    }

    #[test]
    fn test_scan() {
        let cluster = MiniCluster::new(MiniClusterConfig::default());
        let client = cluster.client();

        let schema = kv_schema();
        client.new_table_creator()
              .table_name("table")
              .schema(&schema)
              .num_replicas(1)
              .wait(true)
              .create()
              .unwrap();

        let table = client.open_table("table").unwrap();
        assert_eq!("table", table.name());

        let mut session = client.new_session();
        session.set_timeout(&Duration::from_secs(10));

        let mut keys = HashSet::new();

        for i in 0..100 {
            let mut insert = table.new_insert();
            let key = format!("key-{:0>2}", i);
            insert.row().set_copy(0, &key[..]).unwrap();
            keys.insert(key);
            insert.row().set_copy(1, &format!("val-{:0>2}", i)[..]).unwrap();
            session.insert(insert).unwrap();
        }
        session.flush().unwrap();
        session.close().unwrap();

        { // no bounds, no predicates
            let mut scanner = ScanBuilder::new(&table).build().unwrap();
            let projection_schema = scanner.get_projection_schema();
            assert_eq!(2, projection_schema.num_columns());

            let mut batch = ScanBatch::new();
            let mut count = 0;
            let mut scanned_keys = HashSet::new();
            while scanner.has_more_rows() {
                batch = scanner.next_batch(batch).unwrap();
                for offset in 0..batch.len() {
                    let row = batch.get(offset).unwrap();
                    let k = row.get::<&str>(0).unwrap();
                    scanned_keys.insert(k.to_owned());
                }
                count += batch.len();
            }
            assert_eq!(100, count);
            assert_eq!(keys, scanned_keys);
        }

        { // lower bound
            let mut bound = schema.new_row();
            bound.set(0, "key-50").unwrap();
            let mut scanner = {
                let mut builder = ScanBuilder::new(&table);
                builder.add_lower_bound(&bound).unwrap();
                builder.build().unwrap()
            };
            let mut batch = ScanBatch::new();
            let mut count = 0;
            while scanner.has_more_rows() {
                batch = scanner.next_batch(batch).unwrap();
                count += batch.len();
            }
            assert_eq!(50, count);
        }

        { // upper bound
            let mut bound = schema.new_row();
            bound.set(0, "key-50").unwrap();
            let mut scanner = {
                let mut builder = ScanBuilder::new(&table);
                builder.add_upper_bound(&bound).unwrap();
                builder.build().unwrap()
            };
            let mut batch = ScanBatch::new();
            let mut count = 0;
            while scanner.has_more_rows() {
                batch = scanner.next_batch(batch).unwrap();
                count += batch.len();
            }
            assert_eq!(50, count);
        }

        { // upper bound
            let mut bound = schema.new_row();
            bound.set(0, "key-50").unwrap();
            let mut scanner = {
                let mut builder = ScanBuilder::new(&table);
                builder.add_upper_bound(&bound).unwrap();
                builder.build().unwrap()
            };
            let mut batch = ScanBatch::new();
            let mut count = 0;
            while scanner.has_more_rows() {
                batch = scanner.next_batch(batch).unwrap();
                count += batch.len();
            }
            assert_eq!(50, count);
        }
    }
}
