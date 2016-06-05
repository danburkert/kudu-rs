#![allow(dead_code)]

extern crate byteorder;
extern crate eventual;
extern crate kudu_pb;
extern crate mio;
extern crate netbuf;
extern crate parking_lot;
extern crate protobuf;
extern crate rand;
extern crate slab;
extern crate vec_map;

#[cfg(test)] extern crate tempdir;
#[cfg(test)] extern crate env_logger;

#[macro_use] extern crate log;

mod bit_set;
mod error;
mod master;
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

    fn from_pb(pb: kudu_pb::common::EncodingType) -> Option<EncodingType> {
        match pb {
            kudu_pb::common::EncodingType::AUTO_ENCODING => Some(EncodingType::Auto),
            kudu_pb::common::EncodingType::PLAIN_ENCODING => Some(EncodingType::Plain),
            kudu_pb::common::EncodingType::PREFIX_ENCODING => Some(EncodingType::Prefix),
            kudu_pb::common::EncodingType::GROUP_VARINT => Some(EncodingType::GroupVarint),
            kudu_pb::common::EncodingType::RLE => Some(EncodingType::RunLength),
            kudu_pb::common::EncodingType::DICT_ENCODING => Some(EncodingType::Dictionary),
            kudu_pb::common::EncodingType::BIT_SHUFFLE => Some(EncodingType::BitShuffle),
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

    fn from_pb(pb: kudu_pb::common::CompressionType) -> Option<CompressionType> {
        match pb {
            kudu_pb::common::CompressionType::DEFAULT_COMPRESSION => Some(CompressionType::Default),
            kudu_pb::common::CompressionType::NO_COMPRESSION => Some(CompressionType::None),
            kudu_pb::common::CompressionType::SNAPPY => Some(CompressionType::Snappy),
            kudu_pb::common::CompressionType::LZ4 => Some(CompressionType::Lz4),
            kudu_pb::common::CompressionType::ZLIB => Some(CompressionType::Zlib),
            _ => None,
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MasterState {
    /// The permanent UUID of the master server.
    uuid: Vec<u8>, // TODO: parse into proper UUID

    /// Sequence number incremented on every start-up of the master.
    ///
    /// This makes it easy to detect when an instance has restarted.
    ///
    /// On a freshly initialized server, the first sequence number should be 0.
    seqno: i64,

    /// The Raft role of the master.
    role: RaftRole,

    rpc_addresses: Vec<(String, u32)>,

    http_addresses: Vec<(String, u32)>,
}
