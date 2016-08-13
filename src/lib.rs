#![allow(dead_code)]

extern crate byteorder;
extern crate chrono;
extern crate ieee754;
extern crate kudu_pb;
extern crate mio;
extern crate net2;
extern crate netbuf;
extern crate parking_lot;
extern crate protobuf;
extern crate rand;
extern crate slab;
extern crate uuid;
extern crate vec_map;

#[cfg(test)] extern crate env_logger;
#[cfg(test)] extern crate tempdir;

#[cfg(any(feature="quickcheck", test))] extern crate quickcheck;

#[macro_use] extern crate log;

mod backoff;
mod bit_set;
mod client;
mod dns;
mod error;
mod key;
mod master;
mod meta_cache;
mod partition;
mod queue_map;
mod row;
mod rpc;
mod schema;
mod table;
mod tablet;
mod tablet_proxy;
mod tablet_server;
mod util;
mod value;

#[cfg(test)]
mod mini_cluster;

pub use client::*;
pub use error::*;
pub use master::Master;
pub use partition::*;
pub use row::Row;
pub use schema::*;
pub use table::*;
pub use tablet::*;
pub use tablet_server::TabletServer;
pub use value::Value;

use std::fmt;
use std::str;

use uuid::Uuid;

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

    fn from_pb(pb: kudu_pb::common::DataType) -> Result<DataType> {
        match pb {
            kudu_pb::common::DataType::BOOL => Ok(DataType::Bool),
            kudu_pb::common::DataType::INT8 => Ok(DataType::Int8),
            kudu_pb::common::DataType::INT16 => Ok(DataType::Int16),
            kudu_pb::common::DataType::INT32 => Ok(DataType::Int32),
            kudu_pb::common::DataType::INT64 => Ok(DataType::Int64),
            kudu_pb::common::DataType::TIMESTAMP => Ok(DataType::Timestamp),
            kudu_pb::common::DataType::FLOAT => Ok(DataType::Float),
            kudu_pb::common::DataType::DOUBLE => Ok(DataType::Double),
            kudu_pb::common::DataType::BINARY => Ok(DataType::Binary),
            kudu_pb::common::DataType::STRING => Ok(DataType::String),
            _ => Err(Error::VersionMismatch("unknown data type".to_string())),
        }
    }

    #[cfg(any(feature="quickcheck", test))]
    pub fn arbitrary_primary_key<G>(g: &mut G) -> DataType where G: quickcheck::Gen {
        *g.choose(&[
                  DataType::Int8,
                  DataType::Int16,
                  DataType::Int32,
                  DataType::Int64,
                  DataType::Timestamp,
                  DataType::Binary,
                  DataType::String,
        ]).unwrap()
    }
}

#[cfg(any(feature="quickcheck", test))]
impl quickcheck::Arbitrary for DataType {
    fn arbitrary<G>(g: &mut G) -> DataType where G: quickcheck::Gen {
        *g.choose(&[
                  DataType::Bool,
                  DataType::Int8,
                  DataType::Int16,
                  DataType::Int32,
                  DataType::Int64,
                  DataType::Timestamp,
                  DataType::Float,
                  DataType::Double,
                  DataType::Binary,
                  DataType::String,
        ]).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodingType {
    Auto,
    Plain,
    Prefix,
    GroupVarint,
    RunLength,
    Dictionary,
    BitShuffle,
}

impl EncodingType {
    fn to_pb(self) -> kudu_pb::common::EncodingType {
        match self {
            EncodingType::Auto => kudu_pb::common::EncodingType::AUTO_ENCODING,
            EncodingType::Plain => kudu_pb::common::EncodingType::PLAIN_ENCODING,
            EncodingType::Prefix => kudu_pb::common::EncodingType::PREFIX_ENCODING,
            EncodingType::GroupVarint => kudu_pb::common::EncodingType::GROUP_VARINT,
            EncodingType::RunLength => kudu_pb::common::EncodingType::RLE,
            EncodingType::Dictionary => kudu_pb::common::EncodingType::DICT_ENCODING,
            EncodingType::BitShuffle => kudu_pb::common::EncodingType::BIT_SHUFFLE,
        }
    }

    fn from_pb(pb: kudu_pb::common::EncodingType) -> Result<EncodingType> {
        match pb {
            kudu_pb::common::EncodingType::AUTO_ENCODING => Ok(EncodingType::Auto),
            kudu_pb::common::EncodingType::PLAIN_ENCODING => Ok(EncodingType::Plain),
            kudu_pb::common::EncodingType::PREFIX_ENCODING => Ok(EncodingType::Prefix),
            kudu_pb::common::EncodingType::GROUP_VARINT => Ok(EncodingType::GroupVarint),
            kudu_pb::common::EncodingType::RLE => Ok(EncodingType::RunLength),
            kudu_pb::common::EncodingType::DICT_ENCODING => Ok(EncodingType::Dictionary),
            kudu_pb::common::EncodingType::BIT_SHUFFLE => Ok(EncodingType::BitShuffle),
            _ => Err(Error::VersionMismatch("unknown encoding type".to_string())),
        }
    }

    #[cfg(any(feature="quickcheck", test))]
    pub fn arbitrary<G>(g: &mut G, data_type: DataType) -> EncodingType where G: quickcheck::Gen {
        match data_type {
            DataType::Bool => *g.choose(&[
                EncodingType::Auto,
                EncodingType::Plain,
                EncodingType::RunLength
            ]).unwrap(),
            DataType::Int8 | DataType::Int16 |
            DataType::Int32 | DataType::Int64 | DataType::Timestamp => *g.choose(&[
                EncodingType::Auto,
                EncodingType::Plain,
                EncodingType::RunLength,
                EncodingType::BitShuffle
            ]).unwrap(),
            DataType::Float | DataType::Double => *g.choose(&[
                EncodingType::Auto,
                EncodingType::Plain,
                EncodingType::BitShuffle
            ]).unwrap(),
            DataType::Binary | DataType::String => *g.choose(&[
                EncodingType::Auto,
                EncodingType::Plain,
                EncodingType::Prefix,
                EncodingType::Dictionary
            ]).unwrap(),
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
    fn to_pb(self) -> kudu_pb::common::CompressionType {
        match self {
            CompressionType::Default => kudu_pb::common::CompressionType::DEFAULT_COMPRESSION,
            CompressionType::None => kudu_pb::common::CompressionType::NO_COMPRESSION,
            CompressionType::Snappy => kudu_pb::common::CompressionType::SNAPPY,
            CompressionType::Lz4 => kudu_pb::common::CompressionType::LZ4,
            CompressionType::Zlib => kudu_pb::common::CompressionType::ZLIB,
        }
    }

    fn from_pb(pb: kudu_pb::common::CompressionType) -> Result<CompressionType> {
        match pb {
            kudu_pb::common::CompressionType::DEFAULT_COMPRESSION => Ok(CompressionType::Default),
            kudu_pb::common::CompressionType::NO_COMPRESSION => Ok(CompressionType::None),
            kudu_pb::common::CompressionType::SNAPPY => Ok(CompressionType::Snappy),
            kudu_pb::common::CompressionType::LZ4 => Ok(CompressionType::Lz4),
            kudu_pb::common::CompressionType::ZLIB => Ok(CompressionType::Zlib),
            _ => Err(Error::VersionMismatch("unknown compression type".to_string())),
        }
    }
}

#[cfg(any(feature="quickcheck", test))]
impl quickcheck::Arbitrary for CompressionType {
    fn arbitrary<G>(g: &mut G) -> CompressionType where G: quickcheck::Gen {
        *g.choose(&[
                  CompressionType::Default,
                  CompressionType::None,
                  CompressionType::Snappy,
                  CompressionType::Lz4,
                  CompressionType::Zlib,
        ]).unwrap()
    }
}

/// The role of a Kudu master or tserver in a consensus group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RaftRole {
    Follower,
    Leader,
    Learner,
    NonParticipant,
    Unknown,
}

impl RaftRole {
    fn from_pb(pb: kudu_pb::consensus_metadata::RaftPeerPB_Role) -> RaftRole {
        match pb {
            kudu_pb::consensus_metadata::RaftPeerPB_Role::UNKNOWN_ROLE => RaftRole::Unknown,
            kudu_pb::consensus_metadata::RaftPeerPB_Role::FOLLOWER => RaftRole::Follower,
            kudu_pb::consensus_metadata::RaftPeerPB_Role::LEADER => RaftRole::Leader,
            kudu_pb::consensus_metadata::RaftPeerPB_Role::LEARNER => RaftRole::Learner,
            kudu_pb::consensus_metadata::RaftPeerPB_Role::NON_PARTICIPANT => RaftRole::NonParticipant,
        }
    }
}

macro_rules! id {
    ($id:ident) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $id {
            id: Uuid,
        }

        impl $id {
            pub fn as_bytes(&self) -> &[u8; 16] {
                self.id.as_bytes()
            }

            fn parse(input: &str) -> Result<$id> {
                Uuid::parse_str(input).map_err(|error| Error::Serialization(format!("{}", error)))
                                      .map(|id| $id { id: id })
            }

            fn parse_bytes(input: &[u8]) -> Result<$id> {
                str::from_utf8(input)
                    .map_err(|error| Error::Serialization(format!("{}", error)))
                    .and_then($id::parse)
            }
        }

        impl fmt::Debug for $id {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.id.simple())
            }
        }

        impl fmt::Display for $id {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.id.simple())
            }
        }
    }
}

id!(MasterId);
id!(ReplicaId);
id!(TableId);
id!(TabletId);
id!(TabletServerId);
