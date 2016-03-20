#![allow(non_camel_case_types)]

use std::os::raw::c_void;

pub enum kudu_client {}
pub enum kudu_client_builder {}
pub enum kudu_column_schema {}
pub enum kudu_column_schema_builder {}
pub enum kudu_delete {}
pub enum kudu_insert {}
pub enum kudu_partial_row {}
pub enum kudu_predicate {}
pub enum kudu_scan_batch {}
pub enum kudu_scanner {}
pub enum kudu_schema {}
pub enum kudu_schema_builder {}
pub enum kudu_session {}
pub enum kudu_status {}
pub enum kudu_table {}
pub enum kudu_table_creator {}
pub enum kudu_table_list {}
pub enum kudu_tablet_server {}
pub enum kudu_tablet_server_list {}
pub enum kudu_update {}

#[repr(C)]
pub struct kudu_scan_batch_row_ptr {
    schema: c_void,
    data: c_void,
}

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
  Lz4 = 3,
  Zlib = 4,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum FlushMode {
  AutoFlushSync,
  AutoFlushBackground,
  ManualFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum ExternalConsistencyMode {
  ClientPropogated,
  CommitWait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum ReadMode {
    Latest,
    Snapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum OrderMode {
    Unordered,
    Ordered,
}

#[link(name="kudu_client")]
extern "C" {

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Status
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_status_destroy(status: *mut kudu_status);
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
                                     client: *mut *mut kudu_client)
                                     -> *mut kudu_status;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Table List
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_table_list_destroy(list: *mut kudu_table_list);
    pub fn kudu_table_list_size(list: *const kudu_table_list) -> usize;
    pub fn kudu_table_list_table_name(list: *const kudu_table_list, index: usize) -> kudu_slice;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Schema
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_schema_destroy(schema: *mut kudu_schema);
    pub fn kudu_schema_num_columns(schema: *const kudu_schema) -> usize;
    pub fn kudu_schema_num_key_columns(schema: *const kudu_schema) -> usize;
    pub fn kudu_schema_column(schema: *const kudu_schema, index: usize) -> *mut kudu_column_schema;
    pub fn kudu_schema_find_column(schema: *const kudu_schema,
                                   column_name: kudu_slice,
                                   index: *mut usize)
                                   -> *mut kudu_status;
    pub fn kudu_schema_new_row(schema: *const kudu_schema) -> *mut kudu_partial_row;

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
                                     schema: *mut *mut kudu_schema) -> *mut kudu_status;

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
    pub fn kudu_client_new_table_creator(client: *mut kudu_client) -> *mut kudu_table_creator;
    pub fn kudu_client_delete_table(client: *mut kudu_client, table_name: kudu_slice) -> *mut kudu_status;
    pub fn kudu_client_get_table_schema(client: *mut kudu_client,
                                        table: kudu_slice,
                                        schema: *mut *mut kudu_schema)
                                        -> *mut kudu_status;
    pub fn kudu_client_list_tables(client: *mut kudu_client,
                                   filter: kudu_slice,
                                   tables: *mut *mut kudu_table_list)
                                   -> *mut kudu_status;
    pub fn kudu_client_list_tablet_servers(client: *mut kudu_client,
                                           tservers: *mut *mut kudu_tablet_server_list)
                                           -> *mut kudu_status;

    pub fn kudu_client_new_session(client: *mut kudu_client) -> *mut kudu_session;
    pub fn kudu_client_open_table(client: *mut kudu_client,
                                  table_name: kudu_slice,
                                  table: *mut *mut kudu_table)
                                  -> *mut kudu_status;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Tablet Server List
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_tablet_server_list_destroy(tservers: *mut kudu_tablet_server_list);
    pub fn kudu_tablet_server_list_size(tservers: *const kudu_tablet_server_list) -> usize;
    pub fn kudu_tablet_server_list_get(tservers: *const kudu_tablet_server_list,
                                       idx: usize)
                                       -> *const kudu_tablet_server;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Tablet Server
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_tablet_server_hostname(tserver: *const kudu_tablet_server) -> kudu_slice;
    pub fn kudu_tablet_server_uuid(tserver: *const kudu_tablet_server) -> kudu_slice;

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
    pub fn kudu_table_creator_add_split_row(creator: *mut kudu_table_creator, split_row: *mut kudu_partial_row);
    pub fn kudu_table_creator_num_replicas(creator: *mut kudu_table_creator, num_replicas: i32);
    pub fn kudu_table_creator_timeout(creator: *mut kudu_table_creator, timeout_ms: i64);
    pub fn kudu_table_creator_wait(creator: *mut kudu_table_creator, wait: /*bool*/i32);
    pub fn kudu_table_creator_create(creator: *mut kudu_table_creator) -> *mut kudu_status;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Session
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_session_destroy(session: *mut kudu_session);
    pub fn kudu_session_set_flush_mode(session: *mut kudu_session, mode: FlushMode) -> *mut kudu_status;
    pub fn kudu_session_set_external_consistency_mode(session: *mut kudu_session, mode: ExternalConsistencyMode) -> *mut kudu_status;
    pub fn kudu_session_set_timeout_millis(session: *mut kudu_session, millis: i32);
    pub fn kudu_session_insert(session: *mut kudu_session, insert: *mut kudu_insert) -> *mut kudu_status;
    pub fn kudu_session_update(session: *mut kudu_session, update: *mut kudu_update) -> *mut kudu_status;
    pub fn kudu_session_delete(session: *mut kudu_session, delete: *mut kudu_delete) -> *mut kudu_status;
    pub fn kudu_session_flush(session: *mut kudu_session) -> *mut kudu_status;
    pub fn kudu_session_close(session: *mut kudu_session) -> *mut kudu_status;
    pub fn kudu_session_has_pending_operations(session: *mut kudu_session) -> /*bool*/i32;
    pub fn kudu_session_count_buffered_operations(session: *mut kudu_session) -> i32;
    pub fn kudu_session_count_pending_errors(session: *mut kudu_session) -> i32;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Table
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_table_destroy(table: *mut kudu_table);
    pub fn kudu_table_name(table: *const kudu_table) -> kudu_slice;
    pub fn kudu_table_id(table: *const kudu_table) -> kudu_slice;
    pub fn kudu_table_new_insert(table: *mut kudu_table) -> *mut kudu_insert;
    pub fn kudu_table_new_update(table: *mut kudu_table) -> *mut kudu_update;
    pub fn kudu_table_new_delete(table: *mut kudu_table) -> *mut kudu_delete;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Insert
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_insert_destroy(insert: *mut kudu_insert);
    pub fn kudu_insert_row(insert: *const kudu_insert) -> *const kudu_partial_row;
    pub fn kudu_insert_mutable_row(insert: *mut kudu_insert) -> *mut kudu_partial_row;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Update
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_update_destroy(update: *mut kudu_update);
    pub fn kudu_update_row(update: *const kudu_update) -> *const kudu_partial_row;
    pub fn kudu_update_mutable_row(update: *mut kudu_update) -> *mut kudu_partial_row;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Delete
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_delete_destroy(delete: *mut kudu_delete);
    pub fn kudu_delete_row(delete: *const kudu_delete) -> *const kudu_partial_row;
    pub fn kudu_delete_mutable_row(delete: *mut kudu_delete) -> *mut kudu_partial_row;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Scanner
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_scanner_create(table: *mut kudu_table) -> *mut kudu_scanner;
    pub fn kudu_scanner_destroy(scanner: *mut kudu_scanner);
    pub fn kudu_scanner_set_projected_column_names(scanner: *mut kudu_scanner, col_names: kudu_slice_list) -> *mut kudu_status;
    pub fn kudu_scanner_set_projected_column_indexes(scanner: *mut kudu_scanner, indexes: *const usize, num_indexes: usize) -> *mut kudu_status;
    pub fn kudu_scanner_add_lower_bound(scanner: *mut kudu_scanner, bound: *const kudu_partial_row) -> *mut kudu_status;
    pub fn kudu_scanner_add_upper_bound(scanner: *mut kudu_scanner, bound: *const kudu_partial_row) -> *mut kudu_status;
    pub fn kudu_scanner_set_cache_blocks(scanner: *mut kudu_scanner, cache_blocks: /*bool*/i32) -> *mut kudu_status;
    pub fn kudu_scanner_open(scanner: *mut kudu_scanner) -> *mut kudu_status;
    pub fn kudu_scanner_close(scanner: *mut kudu_scanner);
    pub fn kudu_scanner_has_more_rows(scanner: *mut kudu_scanner) -> /*bool*/i32;
    pub fn kudu_scanner_next_batch(scanner: *mut kudu_scanner, batch: *mut kudu_scan_batch) -> *mut kudu_status;
    //pub fn kudu_scanner_get_current_server(scanner: *mut kudu_scanner, tserver: *mut *mut kudu_table_server) -> *mut kudu_status;
    pub fn kudu_scanner_set_batch_size_bytes(scanner: *mut kudu_scanner, batch_size: usize) -> *mut kudu_status;
    pub fn kudu_scanner_set_read_mode(scanner: *mut kudu_scanner, mode: ReadMode) -> *mut kudu_status;
    pub fn kudu_scanner_set_order_mode(scanner: *mut kudu_scanner, mode: OrderMode) -> *mut kudu_status;
    pub fn kudu_scanner_set_fault_tolerant(scanner: *mut kudu_scanner) -> *mut kudu_status;
    pub fn kudu_scanner_set_snapshot_micros(scanner: *mut kudu_scanner, timestamp: u64) -> *mut kudu_status;
    pub fn kudu_scanner_set_snapshot_raw(scanner: *mut kudu_scanner, timestamp: u64) -> *mut kudu_status;
    pub fn kudu_scanner_set_timeout_millis(scanner: *mut kudu_scanner, timeout: i32) -> *mut kudu_status;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Scan Batch
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_scan_batch_create() -> *mut kudu_scan_batch;
    pub fn kudu_scan_batch_destroy(batch: *mut kudu_scan_batch);
    pub fn kudu_scan_batch_num_rows(batch: *const kudu_scan_batch) -> usize;
    pub fn kudu_scan_batch_row(batch: *const kudu_scan_batch, idx: usize) -> kudu_scan_batch_row_ptr;

    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Scan Batch Row Ptr
    ////////////////////////////////////////////////////////////////////////////////


    pub fn kudu_scan_batch_row_ptr_get_bool_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int8_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i8) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int16_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i16) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int32_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i32) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int64_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_timestamp_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_float_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut f32) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_double_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut f64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_string_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_binary_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_is_null_by_name(row: *const kudu_scan_batch_row_ptr, column_name: kudu_slice) -> i32/*bool*/;

    pub fn kudu_scan_batch_row_ptr_get_bool(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int8(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i8) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int16(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i16) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int32(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i32) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_int64(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_timestamp(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_float(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut f32) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_double(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut f64) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_string(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_get_binary(row: *const kudu_scan_batch_row_ptr, column_idx: usize, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_scan_batch_row_ptr_is_null(row: *const kudu_scan_batch_row_ptr, column_idx: usize) -> i32/*bool*/;


    ////////////////////////////////////////////////////////////////////////////////
    // Kudu Partial Row
    ////////////////////////////////////////////////////////////////////////////////

    pub fn kudu_partial_row_destroy(row: *mut kudu_partial_row);

    pub fn kudu_partial_row_is_key_set(row: *const kudu_partial_row) -> i32/*bool*/ ;
    pub fn kudu_partial_row_all_columns_set(row: *const kudu_partial_row) -> i32/*bool*/;

    pub fn kudu_partial_row_set_bool_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int8_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i8) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int16_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i16) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int32_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i32) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int64_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_timestamp_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: i64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_float_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: f32) -> *mut kudu_status;
    pub fn kudu_partial_row_set_double_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: f64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_string_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_string_copy_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_binary_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_binary_copy_by_name(row: *mut kudu_partial_row, column_name: kudu_slice, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_null_by_name(row: *mut kudu_partial_row, column_name: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_unset_by_name(row: *mut kudu_partial_row, column_name: kudu_slice) -> *mut kudu_status;

    pub fn kudu_partial_row_set_bool(row: *mut kudu_partial_row, column_idx: usize, val: i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int8(row: *mut kudu_partial_row, column_idx: usize, val: i8) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int16(row: *mut kudu_partial_row, column_idx: usize, val: i16) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int32(row: *mut kudu_partial_row, column_idx: usize, val: i32) -> *mut kudu_status;
    pub fn kudu_partial_row_set_int64(row: *mut kudu_partial_row, column_idx: usize, val: i64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_timestamp(row: *mut kudu_partial_row, column_idx: usize, val: i64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_float(row: *mut kudu_partial_row, column_idx: usize, val: f32) -> *mut kudu_status;
    pub fn kudu_partial_row_set_double(row: *mut kudu_partial_row, column_idx: usize, val: f64) -> *mut kudu_status;
    pub fn kudu_partial_row_set_string(row: *mut kudu_partial_row, column_idx: usize, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_string_copy(row: *mut kudu_partial_row, column_idx: usize, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_binary(row: *mut kudu_partial_row, column_idx: usize, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_binary_copy(row: *mut kudu_partial_row, column_idx: usize, val: kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_set_null(row: *mut kudu_partial_row, column_idx: usize) -> *mut kudu_status;
    pub fn kudu_partial_row_unset(row: *mut kudu_partial_row, column_idx: usize) -> *mut kudu_status;

    pub fn kudu_partial_row_get_bool_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int8_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i8) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int16_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i16) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int32_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i32) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int64_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_timestamp_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_float_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut f32) -> *mut kudu_status;
    pub fn kudu_partial_row_get_double_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut f64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_string_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_get_binary_by_name(row: *const kudu_partial_row, column_name: kudu_slice, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_is_null_by_name(row: *const kudu_partial_row, column_name: kudu_slice) -> i32/*bool*/;
    pub fn kudu_partial_row_is_set_by_name(row: *const kudu_partial_row, column_name: kudu_slice) -> i32/*bool*/;

    pub fn kudu_partial_row_get_bool(row: *const kudu_partial_row, column_idx: usize, val: *mut i32/*bool*/) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int8(row: *const kudu_partial_row, column_idx: usize, val: *mut i8) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int16(row: *const kudu_partial_row, column_idx: usize, val: *mut i16) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int32(row: *const kudu_partial_row, column_idx: usize, val: *mut i32) -> *mut kudu_status;
    pub fn kudu_partial_row_get_int64(row: *const kudu_partial_row, column_idx: usize, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_timestamp(row: *const kudu_partial_row, column_idx: usize, val: *mut i64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_float(row: *const kudu_partial_row, column_idx: usize, val: *mut f32) -> *mut kudu_status;
    pub fn kudu_partial_row_get_double(row: *const kudu_partial_row, column_idx: usize, val: *mut f64) -> *mut kudu_status;
    pub fn kudu_partial_row_get_string(row: *const kudu_partial_row, column_idx: usize, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_get_binary(row: *const kudu_partial_row, column_idx: usize, val: *mut kudu_slice) -> *mut kudu_status;
    pub fn kudu_partial_row_is_null(row: *const kudu_partial_row, column_idx: usize) -> i32/*bool*/;
    pub fn kudu_partial_row_is_set(row: *const kudu_partial_row, column_idx: usize) -> i32/*bool*/;
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
