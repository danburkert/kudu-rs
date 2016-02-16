#![allow(non_camel_case_types)]

use std::os::raw::c_char;

pub enum kudu_client_builder_t {}
pub enum kudu_client_t {}
pub enum kudu_status_t {}
pub enum kudu_table_list_t {}

#[link(name="kudu_client")]
extern "C" {

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Status
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_status_destroy(status: *const kudu_status_t);
    pub fn kudu_status_code(status: *const kudu_status_t) -> i8;
    pub fn kudu_status_posix_code(status: *const kudu_status_t) -> i16;
    pub fn kudu_status_message(status: *const kudu_status_t, len: *mut usize) -> *const c_char;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Client Builder
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_client_builder_create() -> *mut kudu_client_builder_t;
    pub fn kudu_client_builder_destroy(builder: *mut kudu_client_builder_t);
    pub fn kudu_client_builder_add_master_server_addr(builder: *mut kudu_client_builder_t,
                                                      addr: *const c_char);
    pub fn kudu_client_builder_clear_master_server_addrs(builder: *mut kudu_client_builder_t);
    pub fn kudu_client_builder_set_default_admin_operation_timeout(builder: *mut kudu_client_builder_t,
                                                                   timeout_millis: i64);
    pub fn kudu_client_builder_set_default_rpc_timeout(builder: *mut kudu_client_builder_t,
                                                       timeout_millis: i64);
    pub fn kudu_client_builder_build(builder: *mut kudu_client_builder_t,
                                     client: *const *mut kudu_client_t)
                                     -> *const kudu_status_t;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Table List
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_table_list_destroy(list: *mut kudu_table_list_t);
    pub fn kudu_table_list_size(list: *const kudu_table_list_t) -> usize;
    pub fn kudu_table_list_table_name(list: *const kudu_table_list_t,
                                      index: usize)
                                      -> *const c_char;

    ////////////////////////////////////////////////////////////////////////////
    // Kudu Client
    ////////////////////////////////////////////////////////////////////////////

    pub fn kudu_client_destroy(client: *mut kudu_client_t);
    pub fn kudu_client_list_tables(client: *const kudu_client_t,
                                   tables: *const *mut kudu_table_list_t)
                                   -> *const kudu_status_t;
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
