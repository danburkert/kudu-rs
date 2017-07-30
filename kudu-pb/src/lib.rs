extern crate bytes;
extern crate prost;
#[macro_use] extern crate prost_derive;

mod kudu {
    include!(concat!(env!("OUT_DIR"), "/kudu.rs"));

    fn foo(foo: ReadMode) {
    }

    pub mod client {
        include!(concat!(env!("OUT_DIR"), "/client.rs"));
    }
    pub mod consensus {
        include!(concat!(env!("OUT_DIR"), "/consensus.rs"));
    }
    pub mod master {
        //include!(concat!(env!("OUT_DIR"), "/master.rs"));
    }
    pub mod rpc {
        //include!(concat!(env!("OUT_DIR"), "/rpc.rs"));
    }
    pub mod tablet {
        //include!(concat!(env!("OUT_DIR"), "/tablet.rs"));
    }
    pub mod security {
        //include!(concat!(env!("OUT_DIR"), "/security.rs"));
    }
    pub mod tserver {
        //include!(concat!(env!("OUT_DIR"), "/tserver.rs"));
    }
}

pub mod google {
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/protobuf.rs"));
    }
}

pub use kudu::*;
