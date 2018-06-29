#![feature(nll)]

extern crate byteorder;
extern crate bytes;
extern crate ieee754;
extern crate ifaces;
extern crate krpc;
extern crate parking_lot;
extern crate prost;
extern crate prost_types;
extern crate rand;
extern crate tokio;
extern crate tokio_timer;
extern crate url;
extern crate uuid;
extern crate vec_map;

#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prost_derive;
#[cfg(test)]
extern crate tempdir;

#[cfg(any(feature = "proptest", test))]
#[macro_use]
extern crate proptest;

mod backoff;
mod bitmap;
mod bounds;
mod client;
mod error;
mod filter;
mod key;
mod meta_cache;
mod operation;
mod partition;
mod pb;
mod replica;
mod retry;
mod row;
mod scanner;
mod schema;
mod server;
mod table;
mod tablet;
mod timestamp;
mod util;
mod value;
mod writer;

#[cfg(test)]
mod mini_cluster;

#[cfg(any(feature = "proptest", test))]
pub mod prop;

pub use client::*;
pub use error::*;
pub use filter::*;
pub use operation::*;
pub use partition::*;
pub use row::Row;
pub use scanner::*;
pub use schema::*;
pub use server::*;
pub use table::*;
pub use tablet::*;
pub use writer::*;

use value::Value;

use std::fmt;
use std::str;
use std::time::Duration;

pub use krpc::HostPort;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Timestamp,
    Float,
    Double,
    Binary,
    String,
}

impl DataType {
    fn is_var_len(self) -> bool {
        match self {
            DataType::String | DataType::Binary => true,
            _ => false,
        }
    }

    fn size(self) -> usize {
        match self {
            DataType::Bool | DataType::Int8 => 1,
            DataType::Int16 => 2,
            DataType::Int32 | DataType::Float => 4,
            DataType::Int64 | DataType::Timestamp | DataType::Double => 8,
            DataType::Binary | DataType::String => 16,
        }
    }

    fn to_pb(self) -> i32 {
        let val = match self {
            DataType::Bool => pb::DataType::Bool,
            DataType::Int8 => pb::DataType::Int8,
            DataType::Int16 => pb::DataType::Int16,
            DataType::Int32 => pb::DataType::Int32,
            DataType::Int64 => pb::DataType::Int64,
            DataType::Timestamp => pb::DataType::UnixtimeMicros,
            DataType::Float => pb::DataType::Float,
            DataType::Double => pb::DataType::Double,
            DataType::Binary => pb::DataType::Binary,
            DataType::String => pb::DataType::String,
        };
        val as i32
    }

    fn from_pb(pb: pb::DataType) -> Result<DataType> {
        match pb {
            pb::DataType::Bool => Ok(DataType::Bool),
            pb::DataType::Int8 => Ok(DataType::Int8),
            pb::DataType::Int16 => Ok(DataType::Int16),
            pb::DataType::Int32 => Ok(DataType::Int32),
            pb::DataType::Int64 => Ok(DataType::Int64),
            pb::DataType::UnixtimeMicros => Ok(DataType::Timestamp),
            pb::DataType::Float => Ok(DataType::Float),
            pb::DataType::Double => Ok(DataType::Double),
            pb::DataType::Binary => Ok(DataType::Binary),
            pb::DataType::String => Ok(DataType::String),
            _ => Err(Error::Serialization("unknown data type".to_string())),
        }
    }

    pub(crate) fn physical_type(self) -> PhysicalType {
        match self {
            DataType::Bool => PhysicalType::Bool,
            DataType::Int8 => PhysicalType::Int8,
            DataType::Int16 => PhysicalType::Int16,
            DataType::Int32 => PhysicalType::Int32,
            DataType::Int64 | DataType::Timestamp => PhysicalType::Int64,
            DataType::Float => PhysicalType::Float,
            DataType::Double => PhysicalType::Double,
            DataType::String | DataType::Binary => PhysicalType::Binary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodingType {
    Auto,
    Plain,
    Prefix,
    RunLength,
    Dictionary,
    BitShuffle,
}

impl EncodingType {
    fn to_pb(self) -> i32 {
        let val = match self {
            EncodingType::Auto => pb::EncodingType::AutoEncoding,
            EncodingType::Plain => pb::EncodingType::PlainEncoding,
            EncodingType::Prefix => pb::EncodingType::PrefixEncoding,
            EncodingType::RunLength => pb::EncodingType::Rle,
            EncodingType::Dictionary => pb::EncodingType::DictEncoding,
            EncodingType::BitShuffle => pb::EncodingType::BitShuffle,
        };
        val as i32
    }

    fn from_pb(pb: pb::EncodingType) -> Result<EncodingType> {
        match pb {
            pb::EncodingType::AutoEncoding => Ok(EncodingType::Auto),
            pb::EncodingType::PlainEncoding => Ok(EncodingType::Plain),
            pb::EncodingType::PrefixEncoding => Ok(EncodingType::Prefix),
            pb::EncodingType::Rle => Ok(EncodingType::RunLength),
            pb::EncodingType::DictEncoding => Ok(EncodingType::Dictionary),
            pb::EncodingType::BitShuffle => Ok(EncodingType::BitShuffle),
            _ => Err(Error::Serialization("unknown encoding type".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompressionType {
    Default,
    None,
    Snappy,
    Lz4,
    Zlib,
}

impl CompressionType {
    fn to_pb(self) -> i32 {
        let val = match self {
            CompressionType::Default => pb::CompressionType::DefaultCompression,
            CompressionType::None => pb::CompressionType::NoCompression,
            CompressionType::Snappy => pb::CompressionType::Snappy,
            CompressionType::Lz4 => pb::CompressionType::Lz4,
            CompressionType::Zlib => pb::CompressionType::Zlib,
        };
        val as i32
    }

    fn from_pb(pb: pb::CompressionType) -> Result<CompressionType> {
        match pb {
            pb::CompressionType::DefaultCompression => Ok(CompressionType::Default),
            pb::CompressionType::NoCompression => Ok(CompressionType::None),
            pb::CompressionType::Snappy => Ok(CompressionType::Snappy),
            pb::CompressionType::Lz4 => Ok(CompressionType::Lz4),
            pb::CompressionType::Zlib => Ok(CompressionType::Zlib),
            _ => Err(Error::Serialization("unknown compression type".to_string())),
        }
    }
}

pub use pb::consensus::raft_peer_pb::Role as RaftRole;

macro_rules! id {
    ($id:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $id {
            id: ::uuid::Uuid,
        }

        impl $id {
            pub fn as_bytes(&self) -> &[u8; 16] {
                self.id.as_bytes()
            }

            fn parse(input: &str) -> Result<$id> {
                ::uuid::Uuid::parse_str(input)
                    .map_err(|error| Error::Serialization(format!("{}", error)))
                    .map(|id| $id { id: id })
            }

            fn parse_bytes(input: &[u8]) -> Result<$id> {
                str::from_utf8(input)
                    .map_err(|error| ::error::Error::Serialization(format!("{}", error)))
                    .and_then($id::parse)
            }
        }

        impl ::std::fmt::Debug for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.id.simple())
            }
        }

        impl ::std::fmt::Display for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.id.simple())
            }
        }
    };
}

id!(MasterId);
id!(TableId);
id!(TabletId);
id!(TabletServerId);

// TODO: move this invocation to scanner.rs
id!(ScannerId);

#[derive(Clone)]
pub struct Options {
    rpc: krpc::Options,
    admin_timeout: Duration,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            rpc: krpc::Options::default(),
            admin_timeout: Duration::from_secs(60),
        }
    }
}

pub trait IntoMasterAddrs {
    fn into_master_addrs(self) -> Result<Vec<HostPort>>;
}

impl IntoMasterAddrs for Vec<HostPort> {
    fn into_master_addrs(self) -> Result<Vec<HostPort>> {
        Ok(self)
    }
}

impl IntoMasterAddrs for Vec<String> {
    fn into_master_addrs(self) -> Result<Vec<HostPort>> {
        let mut master_addrs = Vec::new();
        for master_addr in self {
            master_addrs.push(HostPort::parse(master_addr.as_ref(), 7051)?);
        }
        Ok(master_addrs)
    }
}

impl<'a> IntoMasterAddrs for &'a str {
    fn into_master_addrs(self) -> Result<Vec<HostPort>> {
        let mut master_addrs = Vec::new();
        for master_addr in self.split(',') {
            master_addrs.push(HostPort::parse(master_addr, 7051)?);
        }
        Ok(master_addrs)
    }
}
