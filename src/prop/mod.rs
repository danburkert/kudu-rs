pub mod binary;
pub mod bool;
mod row;
mod schema;
pub mod string;

use CompressionType;
use DataType;
use EncodingType;

use proptest::prelude::*;
use proptest::sample;

impl DataType {
    pub fn arbitrary() -> impl Strategy<Value = DataType> {
        sample::select(
            &[
                DataType::Bool,
                DataType::Int8,
                DataType::Int16,
                DataType::Int32,
                DataType::Int64,
                DataType::Timestamp,
                DataType::Float,
                DataType::Double,
                DataType::String,
                DataType::Binary,
            ][..],
        )
    }

    pub fn arbitrary_primary_key() -> impl Strategy<Value = DataType> {
        sample::select(
            &[
                DataType::Int8,
                DataType::Int16,
                DataType::Int32,
                DataType::Int64,
                DataType::Timestamp,
                DataType::String,
                DataType::Binary,
            ][..],
        )
    }
}

impl EncodingType {
    pub fn arbitrary(data_type: DataType) -> impl Strategy<Value = EncodingType> {
        match data_type {
            DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::Timestamp => sample::select(
                &[
                    EncodingType::Auto,
                    EncodingType::Plain,
                    EncodingType::BitShuffle,
                    EncodingType::RunLength,
                ][..],
            ),
            DataType::Bool | DataType::Float | DataType::Double => sample::select(
                &[
                    EncodingType::Auto,
                    EncodingType::Plain,
                    EncodingType::RunLength,
                ][..],
            ),
            DataType::String | DataType::Binary => sample::select(
                &[
                    EncodingType::Auto,
                    EncodingType::Plain,
                    EncodingType::Dictionary,
                    EncodingType::RunLength,
                ][..],
            ),
        }
    }
}

impl CompressionType {
    pub fn arbitrary() -> impl Strategy<Value = CompressionType> {
        sample::select(
            &[
                CompressionType::Default,
                CompressionType::None,
                CompressionType::Lz4,
                CompressionType::Snappy,
                CompressionType::Zlib,
            ][..],
        )
    }
}
