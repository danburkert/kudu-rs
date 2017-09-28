mod kudu {
    include!(concat!(env!("OUT_DIR"), "/kudu.rs"));

    pub mod client {
        include!(concat!(env!("OUT_DIR"), "/kudu.client.rs"));
    }
    pub mod consensus {
        include!(concat!(env!("OUT_DIR"), "/kudu.consensus.rs"));
    }
    pub mod master {
        include!(concat!(env!("OUT_DIR"), "/kudu.master.rs"));
    }
    pub mod tablet {
        include!(concat!(env!("OUT_DIR"), "/kudu.tablet.rs"));
    }
    pub mod security {
        include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
    }
    pub mod tserver {
        include!(concat!(env!("OUT_DIR"), "/kudu.tserver.rs"));
    }
}

pub use pb::kudu::*;
