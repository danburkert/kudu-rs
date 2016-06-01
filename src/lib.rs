#![cfg_attr(test, feature(static_mutex))]
extern crate kudu_pb;

extern crate byteorder;
extern crate eventual;
extern crate mio;
extern crate netbuf;
extern crate parking_lot;
extern crate protobuf;
extern crate rand;
extern crate slab;
extern crate vec_map;

#[cfg(test)]
extern crate tempdir;
#[cfg(test)]
extern crate env_logger;

#[macro_use]
extern crate log;

mod bit_set;
mod error;
mod row;
mod rpc;
mod schema;
mod table;
mod value;

#[cfg(test)]
mod mini_cluster;

pub use error::*;
pub use schema::*;

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

    fn to_pb(self) -> kudu_pb::common::DataType {
        match self {
            DataType::Bool => kudu_pb::common::DataType::BOOL,
            DataType::Int8 => kudu_pb::common::DataType::INT8,
            DataType::Int16 => kudu_pb::common::DataType::INT16,
            DataType::Int32 => kudu_pb::common::DataType::INT32,
            DataType::Int64 => kudu_pb::common::DataType::INT64,
            DataType::Timestamp => kudu_pb::common::DataType::TIMESTAMP,
            DataType::Float => kudu_pb::common::DataType::FLOAT,
            DataType::Double => kudu_pb::common::DataType::DOUBLE,
            DataType::Binary => kudu_pb::common::DataType::BINARY,
            DataType::String => kudu_pb::common::DataType::STRING,
        }
    }

    fn from_pb(pb: kudu_pb::common::DataType) -> Option<DataType> {
        match pb {
            kudu_pb::common::DataType::BOOL => Some(DataType::Bool),
            kudu_pb::common::DataType::INT8 => Some(DataType::Int8),
            kudu_pb::common::DataType::INT16 => Some(DataType::Int16),
            kudu_pb::common::DataType::INT32 => Some(DataType::Int32),
            kudu_pb::common::DataType::INT64 => Some(DataType::Int64),
            kudu_pb::common::DataType::TIMESTAMP => Some(DataType::Timestamp),
            kudu_pb::common::DataType::FLOAT => Some(DataType::Float),
            kudu_pb::common::DataType::DOUBLE => Some(DataType::Double),
            kudu_pb::common::DataType::BINARY => Some(DataType::Binary),
            kudu_pb::common::DataType::STRING => Some(DataType::String),
            _ => None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodingType {
    Default,
    Plain,
    Prefix,
    GroupVarint,
    RunLength,
    Dictionary,
    BitShuffle,
}
