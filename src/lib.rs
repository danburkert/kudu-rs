#![cfg_attr(test, feature(static_mutex))]
extern crate kudu_pb;

extern crate byteorder;
extern crate eventual;
extern crate mio;
extern crate netbuf;
extern crate protobuf;
extern crate rand;
extern crate slab;

#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate env_logger;

#[macro_use]
extern crate log;

mod error;
mod rpc;

#[cfg(test)]
mod mini_cluster;

pub use error::*;
