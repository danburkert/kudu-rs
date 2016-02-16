extern crate kudu_sys;

use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;
use std::result;
use std::slice;
use std::str;
use std::time::Duration;

pub struct Error {
    inner: *const kudu_sys::kudu_status_t,
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
            let mut len = 0;
            let msg = kudu_sys::kudu_status_message(self.inner, &mut len);
            // TODO: handle UTF8 errors
            str::from_utf8(slice::from_raw_parts(msg as *const _, len)).unwrap()
        }
    }
    fn from_status(status: *const kudu_sys::kudu_status_t) -> Result<()> {
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
    inner: *mut kudu_sys::kudu_client_builder_t,
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
        let addr = CString::new(addr).unwrap();
        unsafe {
            kudu_sys::kudu_client_builder_add_master_server_addr(self.inner, addr.as_ptr());
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

impl Drop for ClientBuilder {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_client_builder_destroy(self.inner);
        }
    }
}

pub struct Client {
    inner: *mut kudu_sys::kudu_client_t,
}

impl Client {
    pub fn list_tables(&self) -> Result<Vec<String>> {
        unsafe {
            let list = ptr::null_mut();
            try!(Error::from_status(kudu_sys::kudu_client_list_tables(self.inner, &list)));
            let size = kudu_sys::kudu_table_list_size(list);
            let mut tables = Vec::with_capacity(size);

            for i in 0..size {
                tables.push(CStr::from_ptr(kudu_sys::kudu_table_list_table_name(list, i))
                                .to_string_lossy()
                                .into_owned());
            }
            kudu_sys::kudu_table_list_destroy(list);
            Ok(tables)
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            kudu_sys::kudu_client_destroy(self.inner);
        }
    }
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}
