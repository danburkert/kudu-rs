extern crate bytes;
extern crate tokio_io;
extern crate prost;

#[macro_use]
extern crate prost_derive;

mod codec;

include!(concat!(env!("OUT_DIR"), "/kudu.rs"));
pub mod rpc {
    include!(concat!(env!("OUT_DIR"), "/kudu.rpc.rs"));
}
pub mod security {
    include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
