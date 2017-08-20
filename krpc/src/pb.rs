include!(concat!(env!("OUT_DIR"), "/kudu.rs"));

pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc.rs"));
}

pub mod security {
    include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
}
