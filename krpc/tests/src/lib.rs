extern crate bytes;
extern crate krpc;
extern crate prost;

#[macro_use] extern crate prost_derive;

pub mod calculator_server;

pub mod rpc_tests {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test.rs"));
}

pub mod rpc_test_diff_package {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc_test_diff_package.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
