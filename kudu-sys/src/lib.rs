#![allow(non_camel_case_types)]

pub enum kudu_client {}
pub enum kudu_client_builder {}
pub enum kudu_column_schema {}
pub enum kudu_schema {}
pub enum kudu_status {}
pub enum kudu_table_list {}
pub enum kudu_schema_builder {}
pub enum kudu_column_schema_builder {}
pub enum kudu_table_creator {}

#[repr(C)]
pub struct kudu_slice {
    pub data: *const u8,
    pub len: usize,
}

#[repr(C)]
pub struct kudu_slice_list {
    pub data: *const kudu_slice,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum DataType {
    Int8 = 0,
    Int16 = 1,
    Int32 = 2,
    Int64 = 3,
    String = 4,
    Bool = 5,
    Float = 6,
    Double = 7,
    Binary = 8,
    Timestamp = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum CompressionType {
  Default = 0,
  None = 1,
  Snappy = 2,
  LZ4 = 3,
  ZLIB = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum EncodingType {
  Default = 0,
  Plain = 1,
  Prefix = 2,
  GroupVarint = 3,
  RunLength = 4,
  Dictionary = 5,
  BitShuffle = 6,
}

#[link(name="kudu_client")]
extern "C" {

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Status
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_status_destroy(status: *const kudu_status);
    pub fn kudu_status_code(status: *const kudu_status) -> i8;
    pub fn kudu_status_posix_code(status: *const kudu_status) -> i16;
    pub fn kudu_status_message(status: *const kudu_status) -> kudu_slice;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Client Builder
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_client_builder_create() -> *mut kudu_client_builder;
    pub fn kudu_client_builder_destroy(builder: *mut kudu_client_builder);
    pub fn kudu_client_builder_add_master_server_addr(builder: *mut kudu_client_builder,
                                                      addr: kudu_slice);
    pub fn kudu_client_builder_clear_master_server_addrs(builder: *mut kudu_client_builder);
    pub fn kudu_client_builder_set_default_admin_operation_timeout(builder: *mut kudu_client_builder,
                                                                   timeout_millis: i64);
    pub fn kudu_client_builder_set_default_rpc_timeout(builder: *mut kudu_client_builder,
                                                       timeout_millis: i64);
    pub fn kudu_client_builder_build(builder: *mut kudu_client_builder,
                                     client: *const *mut kudu_client)
                                     -> *const kudu_status;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Table List
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_table_list_destroy(list: *mut kudu_table_list);
    pub fn kudu_table_list_size(list: *const kudu_table_list) -> usize;
    pub fn kudu_table_list_table_name(list: *const kudu_table_list,
                                      index: usize)
                                      -> kudu_slice;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Schema
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_schema_destroy(schema: *mut kudu_schema);
    pub fn kudu_schema_num_columns(schema: *const kudu_schema) -> usize;
    pub fn kudu_schema_num_key_columns(schema: *const kudu_schema) -> usize;
    pub fn kudu_schema_column(schema: *const kudu_schema, index: usize) -> *mut kudu_column_schema;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Column Schema
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_column_schema_destroy(column_schema: *mut kudu_column_schema);
    pub fn kudu_column_schema_name(column_schema: *const kudu_column_schema) -> kudu_slice;
    pub fn kudu_column_schema_is_nullable(column_schema: *const kudu_column_schema) -> i32;
    pub fn kudu_column_schema_data_type(column_schema: *const kudu_column_schema) -> DataType;
    pub fn kudu_column_schema_encoding_type(column_schema: *const kudu_column_schema) -> EncodingType;
    pub fn kudu_column_schema_compression_type(column_schema: *const kudu_column_schema) -> CompressionType;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Schema Builder
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_schema_builder_create() -> *mut kudu_schema_builder;
    pub fn kudu_schema_builder_destroy(builder: *mut kudu_schema_builder);
    pub fn kudu_schema_builder_add_column(builder: *mut kudu_schema_builder,
                                          name: kudu_slice)
                                          -> *mut kudu_column_schema_builder;
    pub fn kudu_schema_builder_set_primary_key_columns(builder: *mut kudu_schema_builder,
                                                       column_names: kudu_slice_list);
    pub fn kudu_schema_builder_build(builder: *mut kudu_schema_builder,
                                     schema: *const *mut kudu_schema) -> *const kudu_status;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Column Schema Builder
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_column_schema_builder_data_type(builder: *mut kudu_column_schema_builder,
                                                data_type: DataType);
    pub fn kudu_column_schema_builder_encoding_type(builder: *mut kudu_column_schema_builder,
                                                    encoding_type: EncodingType);
    pub fn kudu_column_schema_builder_compression_type(builder: *mut kudu_column_schema_builder,
                                                       compression_type: CompressionType);
    pub fn kudu_column_schema_builder_block_size(builder: *mut kudu_column_schema_builder,
                                                 block_size: i32);
    pub fn kudu_column_schema_builder_nullable(builder: *mut kudu_column_schema_builder,
                                               nullable: /*bool*/i32);

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Client
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_client_destroy(client: *mut kudu_client);
    pub fn kudu_client_list_tables(client: *const kudu_client,
                                   tables: *const *mut kudu_table_list)
                                   -> *const kudu_status;
    pub fn kudu_client_table_schema(client: *const kudu_client,
                                    table: kudu_slice,
                                    schema: *const *mut kudu_schema)
                                    -> *const kudu_status;
    pub fn kudu_client_new_table_creator(client: *mut kudu_client) -> *mut kudu_table_creator;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Table Creator
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_table_creator_destroy(creator: *mut kudu_table_creator);
    pub fn kudu_table_creator_table_name(creator: *mut kudu_table_creator, table_name: kudu_slice);
    pub fn kudu_table_creator_schema(creator: *mut kudu_table_creator, schema: *const kudu_schema);
    pub fn kudu_table_creator_add_hash_partitions(creator: *mut kudu_table_creator,
                                                  columns: kudu_slice_list,
                                                  num_buckets: i32,
                                                  seed: i32);
    pub fn kudu_table_creator_set_range_partition_columns(creator: *mut kudu_table_creator,
                                                          columns: kudu_slice_list);
    pub fn kudu_table_creator_num_replicas(creator: *mut kudu_table_creator, num_replicas: i32);
    pub fn kudu_table_creator_timeout(creator: *mut kudu_table_creator, timeout_ms: i64);
    pub fn kudu_table_creator_wait(creator: *mut kudu_table_creator, wait: /*bool*/i32);
    pub fn kudu_table_creator_create(creator: *mut kudu_table_creator) -> *const kudu_status;
}

#[cfg(test)]
mod test {

    use std::ptr;
    use std::ffi::CString;
    use super::*;

    #[test]
    fn test_unreachable_master() {
        unsafe {
            let builder = kudu_client_builder_create();
            let client = ptr::null_mut();
            kudu_client_builder_add_master_server_addr(builder, CString::new("kudu.example.com").unwrap().as_ptr());
            let status = kudu_client_builder_build(builder, &client);
            assert!(status != ptr::null());
        }
    }
}
