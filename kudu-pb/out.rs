#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate bytes;
extern crate prost;
#[macro_use]
extern crate prost_derive;

mod kudu {


    //include!(concat!(env!("OUT_DIR"), "/master.rs"));
    //include!(concat!(env!("OUT_DIR"), "/rpc.rs"));
    //include!(concat!(env!("OUT_DIR"), "/tablet.rs"));
    //include!(concat!(env!("OUT_DIR"), "/security.rs"));
    //include!(concat!(env!("OUT_DIR"), "/tserver.rs"));


    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum CompressionType {
        UnknownCompression = 999,
        DefaultCompression = 0,
        NoCompression = 1,
        Snappy = 2,
        Lz4 = 3,
        Zlib = 4,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for CompressionType {
        #[inline]
        fn clone(&self) -> CompressionType { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for CompressionType { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for CompressionType {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&CompressionType::UnknownCompression,) => {
                    let mut builder =
                        __arg_0.debug_tuple("UnknownCompression");
                    builder.finish()
                }
                (&CompressionType::DefaultCompression,) => {
                    let mut builder =
                        __arg_0.debug_tuple("DefaultCompression");
                    builder.finish()
                }
                (&CompressionType::NoCompression,) => {
                    let mut builder = __arg_0.debug_tuple("NoCompression");
                    builder.finish()
                }
                (&CompressionType::Snappy,) => {
                    let mut builder = __arg_0.debug_tuple("Snappy");
                    builder.finish()
                }
                (&CompressionType::Lz4,) => {
                    let mut builder = __arg_0.debug_tuple("Lz4");
                    builder.finish()
                }
                (&CompressionType::Zlib,) => {
                    let mut builder = __arg_0.debug_tuple("Zlib");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for CompressionType {
        #[inline]
        fn eq(&self, __arg_0: &CompressionType) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for CompressionType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod CompressionType_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl CompressionType {
            #[doc =
                  "Returns `true` if `value` is a variant of `CompressionType`."]
            pub fn is_valid(value: i32) -> bool {
                match value {
                    999 => true,
                    0 => true,
                    1 => true,
                    2 => true,
                    3 => true,
                    4 => true,
                    _ => false,
                }
            }
            #[doc =
                  "Converts an `i32` to a `CompressionType`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32)
             -> ::std::option::Option<CompressionType> {
                match value {
                    999 =>
                    ::std::option::Option::Some(CompressionType::UnknownCompression),
                    0 =>
                    ::std::option::Option::Some(CompressionType::DefaultCompression),
                    1 =>
                    ::std::option::Option::Some(CompressionType::NoCompression),
                    2 => ::std::option::Option::Some(CompressionType::Snappy),
                    3 => ::std::option::Option::Some(CompressionType::Lz4),
                    4 => ::std::option::Option::Some(CompressionType::Zlib),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for CompressionType {
            fn default() -> CompressionType {
                CompressionType::UnknownCompression
            }
        }
        impl ::std::convert::From<CompressionType> for i32 {
            fn from(value: CompressionType) -> i32 { value as i32 }
        }
    }
    /// Supplemental protobuf container header, after the main header (see
    /// pb_util.h for details).
    pub struct ContainerSupHeaderPB {
        /// The protobuf schema for the messages expected in this container.
        ///
        /// This schema is complete, that is, it includes all of its dependencies
        /// (i.e. other schemas defined in .proto files imported by this schema's
        /// .proto file).
        #[prost(message, required, tag = "1")]
        pub protos: super::google::protobuf::FileDescriptorSet,
        /// The PB message type expected in each data entry in this container. Must
        /// be fully qualified (i.e. kudu.tablet.TabletSuperBlockPB).
        #[prost(string, required, tag = "2")]
        pub pb_type: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ContainerSupHeaderPB {
        #[inline]
        fn clone(&self) -> ContainerSupHeaderPB {
            match *self {
                ContainerSupHeaderPB {
                protos: ref __self_0_0, pb_type: ref __self_0_1 } =>
                ContainerSupHeaderPB{protos:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     pb_type:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ContainerSupHeaderPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ContainerSupHeaderPB {
                protos: ref __self_0_0, pb_type: ref __self_0_1 } => {
                    let mut builder =
                        __arg_0.debug_struct("ContainerSupHeaderPB");
                    let _ = builder.field("protos", &&(*__self_0_0));
                    let _ = builder.field("pb_type", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ContainerSupHeaderPB {
        #[inline]
        fn eq(&self, __arg_0: &ContainerSupHeaderPB) -> bool {
            match *__arg_0 {
                ContainerSupHeaderPB {
                protos: ref __self_1_0, pb_type: ref __self_1_1 } =>
                match *self {
                    ContainerSupHeaderPB {
                    protos: ref __self_0_0, pb_type: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ContainerSupHeaderPB) -> bool {
            match *__arg_0 {
                ContainerSupHeaderPB {
                protos: ref __self_1_0, pb_type: ref __self_1_1 } =>
                match *self {
                    ContainerSupHeaderPB {
                    protos: ref __self_0_0, pb_type: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ContainerSupHeaderPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ContainerSupHeaderPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::message::encode(1u32, &self.protos, buf);
                _prost::encoding::string::encode(2u32, &self.pb_type, buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge(wire_type,
                                                     &mut self.protos,
                                                     buf).map_err(|mut error|
                                                                      {
                                                                          error.push("ContainerSupHeaderPB",
                                                                                     "protos");
                                                                          error
                                                                      }),
                    2u32 =>
                    _prost::encoding::string::merge(wire_type,
                                                    &mut self.pb_type,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("ContainerSupHeaderPB",
                                                                                    "pb_type");
                                                                         error
                                                                     }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::message::encoded_len(1u32, &self.protos)
                    +
                    _prost::encoding::string::encoded_len(2u32, &self.pb_type)
            }
        }
        impl Default for ContainerSupHeaderPB {
            fn default() -> ContainerSupHeaderPB {
                ContainerSupHeaderPB{protos:
                                         ::std::default::Default::default(),
                                     pb_type: ::std::string::String::new(),}
            }
        }
    }
    /// TODO: Differentiate between the schema attributes
    /// that are only relevant to the server (e.g.,
    /// encoding and compression) and those that also
    /// matter to the client.
    pub struct ColumnSchemaPB {
        #[prost(uint32, optional, tag = "1")]
        pub id: Option<u32>,
        #[prost(string, required, tag = "2")]
        pub name: String,
        #[prost(enumeration = "DataType", required, tag = "3")]
        pub type_: i32,
        #[prost(bool, optional, tag = "4")]
        pub is_key: Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub is_nullable: Option<bool>,
        /// Default values.
        /// NOTE: as far as clients are concerned, there is only one
        /// "default value" of a column. The read/write defaults are used
        /// internally and should not be exposed by any public client APIs.
        ///
        /// When passing schemas to the master for create/alter table,
        /// specify the default in 'read_default_value'.
        ///
        /// Contrary to this, when the client opens a table, it will receive
        /// both the read and write defaults, but the *write* default is
        /// what should be exposed as the "current" default.
        #[prost(bytes, optional, tag = "6")]
        pub read_default_value: Option<Vec<u8>>,
        #[prost(bytes, optional, tag = "7")]
        pub write_default_value: Option<Vec<u8>>,
        /// The following attributes refer to the on-disk storage of the column.
        /// They won't always be set, depending on context.
        #[prost(enumeration = "EncodingType", optional, tag = "8")]
        pub encoding: Option<i32>,
        #[prost(enumeration = "CompressionType", optional, tag = "9")]
        pub compression: Option<i32>,
        #[prost(int32, optional, tag = "10")]
        pub cfile_block_size: Option<i32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ColumnSchemaPB {
        #[inline]
        fn clone(&self) -> ColumnSchemaPB {
            match *self {
                ColumnSchemaPB {
                id: ref __self_0_0,
                name: ref __self_0_1,
                type_: ref __self_0_2,
                is_key: ref __self_0_3,
                is_nullable: ref __self_0_4,
                read_default_value: ref __self_0_5,
                write_default_value: ref __self_0_6,
                encoding: ref __self_0_7,
                compression: ref __self_0_8,
                cfile_block_size: ref __self_0_9 } =>
                ColumnSchemaPB{id: ::std::clone::Clone::clone(&(*__self_0_0)),
                               name:
                                   ::std::clone::Clone::clone(&(*__self_0_1)),
                               type_:
                                   ::std::clone::Clone::clone(&(*__self_0_2)),
                               is_key:
                                   ::std::clone::Clone::clone(&(*__self_0_3)),
                               is_nullable:
                                   ::std::clone::Clone::clone(&(*__self_0_4)),
                               read_default_value:
                                   ::std::clone::Clone::clone(&(*__self_0_5)),
                               write_default_value:
                                   ::std::clone::Clone::clone(&(*__self_0_6)),
                               encoding:
                                   ::std::clone::Clone::clone(&(*__self_0_7)),
                               compression:
                                   ::std::clone::Clone::clone(&(*__self_0_8)),
                               cfile_block_size:
                                   ::std::clone::Clone::clone(&(*__self_0_9)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ColumnSchemaPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ColumnSchemaPB {
                id: ref __self_0_0,
                name: ref __self_0_1,
                type_: ref __self_0_2,
                is_key: ref __self_0_3,
                is_nullable: ref __self_0_4,
                read_default_value: ref __self_0_5,
                write_default_value: ref __self_0_6,
                encoding: ref __self_0_7,
                compression: ref __self_0_8,
                cfile_block_size: ref __self_0_9 } => {
                    let mut builder = __arg_0.debug_struct("ColumnSchemaPB");
                    let _ = builder.field("id", &&(*__self_0_0));
                    let _ = builder.field("name", &&(*__self_0_1));
                    let _ = builder.field("type_", &&(*__self_0_2));
                    let _ = builder.field("is_key", &&(*__self_0_3));
                    let _ = builder.field("is_nullable", &&(*__self_0_4));
                    let _ =
                        builder.field("read_default_value", &&(*__self_0_5));
                    let _ =
                        builder.field("write_default_value", &&(*__self_0_6));
                    let _ = builder.field("encoding", &&(*__self_0_7));
                    let _ = builder.field("compression", &&(*__self_0_8));
                    let _ =
                        builder.field("cfile_block_size", &&(*__self_0_9));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ColumnSchemaPB {
        #[inline]
        fn eq(&self, __arg_0: &ColumnSchemaPB) -> bool {
            match *__arg_0 {
                ColumnSchemaPB {
                id: ref __self_1_0,
                name: ref __self_1_1,
                type_: ref __self_1_2,
                is_key: ref __self_1_3,
                is_nullable: ref __self_1_4,
                read_default_value: ref __self_1_5,
                write_default_value: ref __self_1_6,
                encoding: ref __self_1_7,
                compression: ref __self_1_8,
                cfile_block_size: ref __self_1_9 } =>
                match *self {
                    ColumnSchemaPB {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    type_: ref __self_0_2,
                    is_key: ref __self_0_3,
                    is_nullable: ref __self_0_4,
                    read_default_value: ref __self_0_5,
                    write_default_value: ref __self_0_6,
                    encoding: ref __self_0_7,
                    compression: ref __self_0_8,
                    cfile_block_size: ref __self_0_9 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4) &&
                        (*__self_0_5) == (*__self_1_5) &&
                        (*__self_0_6) == (*__self_1_6) &&
                        (*__self_0_7) == (*__self_1_7) &&
                        (*__self_0_8) == (*__self_1_8) &&
                        (*__self_0_9) == (*__self_1_9),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ColumnSchemaPB) -> bool {
            match *__arg_0 {
                ColumnSchemaPB {
                id: ref __self_1_0,
                name: ref __self_1_1,
                type_: ref __self_1_2,
                is_key: ref __self_1_3,
                is_nullable: ref __self_1_4,
                read_default_value: ref __self_1_5,
                write_default_value: ref __self_1_6,
                encoding: ref __self_1_7,
                compression: ref __self_1_8,
                cfile_block_size: ref __self_1_9 } =>
                match *self {
                    ColumnSchemaPB {
                    id: ref __self_0_0,
                    name: ref __self_0_1,
                    type_: ref __self_0_2,
                    is_key: ref __self_0_3,
                    is_nullable: ref __self_0_4,
                    read_default_value: ref __self_0_5,
                    write_default_value: ref __self_0_6,
                    encoding: ref __self_0_7,
                    compression: ref __self_0_8,
                    cfile_block_size: ref __self_0_9 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3) ||
                        (*__self_0_4) != (*__self_1_4) ||
                        (*__self_0_5) != (*__self_1_5) ||
                        (*__self_0_6) != (*__self_1_6) ||
                        (*__self_0_7) != (*__self_1_7) ||
                        (*__self_0_8) != (*__self_1_8) ||
                        (*__self_0_9) != (*__self_1_9),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ColumnSchemaPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ColumnSchemaPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let ::std::option::Option::Some(ref value) = self.id {
                    _prost::encoding::uint32::encode(1u32, value, buf);
                }
                _prost::encoding::string::encode(2u32, &self.name, buf);
                _prost::encoding::int32::encode(3u32, &self.type_, buf);
                if let ::std::option::Option::Some(ref value) = self.is_key {
                    _prost::encoding::bool::encode(4u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.is_nullable {
                    _prost::encoding::bool::encode(5u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.read_default_value {
                    _prost::encoding::bytes::encode(6u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.write_default_value {
                    _prost::encoding::bytes::encode(7u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) = self.encoding
                       {
                    _prost::encoding::int32::encode(8u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.compression {
                    _prost::encoding::int32::encode(9u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.cfile_block_size {
                    _prost::encoding::int32::encode(10u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    {
                        if self.id.is_none() {
                            self.id = Some(Default::default());
                        }
                        match self.id {
                            Some(ref mut value) =>
                            _prost::encoding::uint32::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB", "id");
                                      error
                                  }),
                    2u32 =>
                    _prost::encoding::string::merge(wire_type, &mut self.name,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("ColumnSchemaPB",
                                                                                    "name");
                                                                         error
                                                                     }),
                    3u32 =>
                    _prost::encoding::int32::merge(wire_type, &mut self.type_,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("ColumnSchemaPB",
                                                                                   "type_");
                                                                        error
                                                                    }),
                    4u32 =>
                    {
                        if self.is_key.is_none() {
                            self.is_key = Some(Default::default());
                        }
                        match self.is_key {
                            Some(ref mut value) =>
                            _prost::encoding::bool::merge(wire_type, value,
                                                          buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB", "is_key");
                                      error
                                  }),
                    5u32 =>
                    {
                        if self.is_nullable.is_none() {
                            self.is_nullable = Some(Default::default());
                        }
                        match self.is_nullable {
                            Some(ref mut value) =>
                            _prost::encoding::bool::merge(wire_type, value,
                                                          buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "is_nullable");
                                      error
                                  }),
                    6u32 =>
                    {
                        if self.read_default_value.is_none() {
                            self.read_default_value =
                                Some(Default::default());
                        }
                        match self.read_default_value {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "read_default_value");
                                      error
                                  }),
                    7u32 =>
                    {
                        if self.write_default_value.is_none() {
                            self.write_default_value =
                                Some(Default::default());
                        }
                        match self.write_default_value {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "write_default_value");
                                      error
                                  }),
                    8u32 =>
                    {
                        if self.encoding.is_none() {
                            self.encoding = Some(Default::default());
                        }
                        match self.encoding {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "encoding");
                                      error
                                  }),
                    9u32 =>
                    {
                        if self.compression.is_none() {
                            self.compression = Some(Default::default());
                        }
                        match self.compression {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "compression");
                                      error
                                  }),
                    10u32 =>
                    {
                        if self.cfile_block_size.is_none() {
                            self.cfile_block_size = Some(Default::default());
                        }
                        match self.cfile_block_size {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   34u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaPB",
                                                 "cfile_block_size");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.id.as_ref().map_or(0,
                                            |value|
                                                _prost::encoding::uint32::encoded_len(1u32,
                                                                                      value))
                    + _prost::encoding::string::encoded_len(2u32, &self.name)
                    + _prost::encoding::int32::encoded_len(3u32, &self.type_)
                    +
                    self.is_key.as_ref().map_or(0,
                                                |value|
                                                    _prost::encoding::bool::encoded_len(4u32,
                                                                                        value))
                    +
                    self.is_nullable.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::bool::encoded_len(5u32,
                                                                                             value))
                    +
                    self.read_default_value.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::bytes::encoded_len(6u32,
                                                                                                     value))
                    +
                    self.write_default_value.as_ref().map_or(0,
                                                             |value|
                                                                 _prost::encoding::bytes::encoded_len(7u32,
                                                                                                      value))
                    +
                    self.encoding.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::int32::encoded_len(8u32,
                                                                                           value))
                    +
                    self.compression.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::int32::encoded_len(9u32,
                                                                                              value))
                    +
                    self.cfile_block_size.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::int32::encoded_len(10u32,
                                                                                                   value))
            }
        }
        impl Default for ColumnSchemaPB {
            fn default() -> ColumnSchemaPB {
                ColumnSchemaPB{id: ::std::option::Option::None,
                               name: ::std::string::String::new(),
                               type_: DataType::default() as i32,
                               is_key: ::std::option::Option::None,
                               is_nullable: ::std::option::Option::None,
                               read_default_value:
                                   ::std::option::Option::None,
                               write_default_value:
                                   ::std::option::Option::None,
                               encoding: ::std::option::Option::None,
                               compression: ::std::option::Option::None,
                               cfile_block_size: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl ColumnSchemaPB {
            pub fn id(&self) -> u32 {
                match self.id {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0u32,
                }
            }
            pub fn type_(&self) -> ::std::option::Option<DataType> {
                DataType::from_i32(self.type_)
            }
            pub fn set_type_(&mut self, value: DataType) {
                self.type_ = value as i32;
            }
            pub fn is_key(&self) -> bool {
                match self.is_key {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => false,
                }
            }
            pub fn is_nullable(&self) -> bool {
                match self.is_nullable {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => false,
                }
            }
            pub fn read_default_value(&self) -> &[u8] {
                match self.read_default_value {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
            pub fn write_default_value(&self) -> &[u8] {
                match self.write_default_value {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
            pub fn encoding(&self) -> ::std::option::Option<EncodingType> {
                self.encoding.and_then(EncodingType::from_i32)
            }
            pub fn set_encoding(&mut self, value: EncodingType) {
                self.encoding = ::std::option::Option::Some(value as i32);
            }
            pub fn compression(&self)
             -> ::std::option::Option<CompressionType> {
                self.compression.and_then(CompressionType::from_i32)
            }
            pub fn set_compression(&mut self, value: CompressionType) {
                self.compression = ::std::option::Option::Some(value as i32);
            }
            pub fn cfile_block_size(&self) -> i32 {
                match self.cfile_block_size {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
        }
    }
    pub struct ColumnSchemaDeltaPB {
        #[prost(string, optional, tag = "1")]
        pub name: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub new_name: Option<String>,
        #[prost(bytes, optional, tag = "4")]
        pub default_value: Option<Vec<u8>>,
        #[prost(bool, optional, tag = "5")]
        pub remove_default: Option<bool>,
        #[prost(enumeration = "EncodingType", optional, tag = "6")]
        pub encoding: Option<i32>,
        #[prost(enumeration = "CompressionType", optional, tag = "7")]
        pub compression: Option<i32>,
        #[prost(int32, optional, tag = "8")]
        pub block_size: Option<i32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ColumnSchemaDeltaPB {
        #[inline]
        fn clone(&self) -> ColumnSchemaDeltaPB {
            match *self {
                ColumnSchemaDeltaPB {
                name: ref __self_0_0,
                new_name: ref __self_0_1,
                default_value: ref __self_0_2,
                remove_default: ref __self_0_3,
                encoding: ref __self_0_4,
                compression: ref __self_0_5,
                block_size: ref __self_0_6 } =>
                ColumnSchemaDeltaPB{name:
                                        ::std::clone::Clone::clone(&(*__self_0_0)),
                                    new_name:
                                        ::std::clone::Clone::clone(&(*__self_0_1)),
                                    default_value:
                                        ::std::clone::Clone::clone(&(*__self_0_2)),
                                    remove_default:
                                        ::std::clone::Clone::clone(&(*__self_0_3)),
                                    encoding:
                                        ::std::clone::Clone::clone(&(*__self_0_4)),
                                    compression:
                                        ::std::clone::Clone::clone(&(*__self_0_5)),
                                    block_size:
                                        ::std::clone::Clone::clone(&(*__self_0_6)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ColumnSchemaDeltaPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ColumnSchemaDeltaPB {
                name: ref __self_0_0,
                new_name: ref __self_0_1,
                default_value: ref __self_0_2,
                remove_default: ref __self_0_3,
                encoding: ref __self_0_4,
                compression: ref __self_0_5,
                block_size: ref __self_0_6 } => {
                    let mut builder =
                        __arg_0.debug_struct("ColumnSchemaDeltaPB");
                    let _ = builder.field("name", &&(*__self_0_0));
                    let _ = builder.field("new_name", &&(*__self_0_1));
                    let _ = builder.field("default_value", &&(*__self_0_2));
                    let _ = builder.field("remove_default", &&(*__self_0_3));
                    let _ = builder.field("encoding", &&(*__self_0_4));
                    let _ = builder.field("compression", &&(*__self_0_5));
                    let _ = builder.field("block_size", &&(*__self_0_6));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ColumnSchemaDeltaPB {
        #[inline]
        fn eq(&self, __arg_0: &ColumnSchemaDeltaPB) -> bool {
            match *__arg_0 {
                ColumnSchemaDeltaPB {
                name: ref __self_1_0,
                new_name: ref __self_1_1,
                default_value: ref __self_1_2,
                remove_default: ref __self_1_3,
                encoding: ref __self_1_4,
                compression: ref __self_1_5,
                block_size: ref __self_1_6 } =>
                match *self {
                    ColumnSchemaDeltaPB {
                    name: ref __self_0_0,
                    new_name: ref __self_0_1,
                    default_value: ref __self_0_2,
                    remove_default: ref __self_0_3,
                    encoding: ref __self_0_4,
                    compression: ref __self_0_5,
                    block_size: ref __self_0_6 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4) &&
                        (*__self_0_5) == (*__self_1_5) &&
                        (*__self_0_6) == (*__self_1_6),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ColumnSchemaDeltaPB) -> bool {
            match *__arg_0 {
                ColumnSchemaDeltaPB {
                name: ref __self_1_0,
                new_name: ref __self_1_1,
                default_value: ref __self_1_2,
                remove_default: ref __self_1_3,
                encoding: ref __self_1_4,
                compression: ref __self_1_5,
                block_size: ref __self_1_6 } =>
                match *self {
                    ColumnSchemaDeltaPB {
                    name: ref __self_0_0,
                    new_name: ref __self_0_1,
                    default_value: ref __self_0_2,
                    remove_default: ref __self_0_3,
                    encoding: ref __self_0_4,
                    compression: ref __self_0_5,
                    block_size: ref __self_0_6 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3) ||
                        (*__self_0_4) != (*__self_1_4) ||
                        (*__self_0_5) != (*__self_1_5) ||
                        (*__self_0_6) != (*__self_1_6),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ColumnSchemaDeltaPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ColumnSchemaDeltaPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let ::std::option::Option::Some(ref value) = self.name {
                    _prost::encoding::string::encode(1u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) = self.new_name
                       {
                    _prost::encoding::string::encode(2u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.default_value {
                    _prost::encoding::bytes::encode(4u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.remove_default {
                    _prost::encoding::bool::encode(5u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) = self.encoding
                       {
                    _prost::encoding::int32::encode(6u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.compression {
                    _prost::encoding::int32::encode(7u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.block_size {
                    _prost::encoding::int32::encode(8u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    {
                        if self.name.is_none() {
                            self.name = Some(Default::default());
                        }
                        match self.name {
                            Some(ref mut value) =>
                            _prost::encoding::string::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "name");
                                      error
                                  }),
                    2u32 =>
                    {
                        if self.new_name.is_none() {
                            self.new_name = Some(Default::default());
                        }
                        match self.new_name {
                            Some(ref mut value) =>
                            _prost::encoding::string::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "new_name");
                                      error
                                  }),
                    4u32 =>
                    {
                        if self.default_value.is_none() {
                            self.default_value = Some(Default::default());
                        }
                        match self.default_value {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "default_value");
                                      error
                                  }),
                    5u32 =>
                    {
                        if self.remove_default.is_none() {
                            self.remove_default = Some(Default::default());
                        }
                        match self.remove_default {
                            Some(ref mut value) =>
                            _prost::encoding::bool::merge(wire_type, value,
                                                          buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "remove_default");
                                      error
                                  }),
                    6u32 =>
                    {
                        if self.encoding.is_none() {
                            self.encoding = Some(Default::default());
                        }
                        match self.encoding {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "encoding");
                                      error
                                  }),
                    7u32 =>
                    {
                        if self.compression.is_none() {
                            self.compression = Some(Default::default());
                        }
                        match self.compression {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "compression");
                                      error
                                  }),
                    8u32 =>
                    {
                        if self.block_size.is_none() {
                            self.block_size = Some(Default::default());
                        }
                        match self.block_size {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   70u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnSchemaDeltaPB",
                                                 "block_size");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.name.as_ref().map_or(0,
                                              |value|
                                                  _prost::encoding::string::encoded_len(1u32,
                                                                                        value))
                    +
                    self.new_name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(2u32,
                                                                                            value))
                    +
                    self.default_value.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::bytes::encoded_len(4u32,
                                                                                                value))
                    +
                    self.remove_default.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(5u32,
                                                                                                value))
                    +
                    self.encoding.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::int32::encoded_len(6u32,
                                                                                           value))
                    +
                    self.compression.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::int32::encoded_len(7u32,
                                                                                              value))
                    +
                    self.block_size.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::int32::encoded_len(8u32,
                                                                                             value))
            }
        }
        impl Default for ColumnSchemaDeltaPB {
            fn default() -> ColumnSchemaDeltaPB {
                ColumnSchemaDeltaPB{name: ::std::option::Option::None,
                                    new_name: ::std::option::Option::None,
                                    default_value:
                                        ::std::option::Option::None,
                                    remove_default:
                                        ::std::option::Option::None,
                                    encoding: ::std::option::Option::None,
                                    compression: ::std::option::Option::None,
                                    block_size: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl ColumnSchemaDeltaPB {
            pub fn name(&self) -> &str {
                match self.name {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => "",
                }
            }
            pub fn new_name(&self) -> &str {
                match self.new_name {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => "",
                }
            }
            pub fn default_value(&self) -> &[u8] {
                match self.default_value {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
            pub fn remove_default(&self) -> bool {
                match self.remove_default {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => false,
                }
            }
            pub fn encoding(&self) -> ::std::option::Option<EncodingType> {
                self.encoding.and_then(EncodingType::from_i32)
            }
            pub fn set_encoding(&mut self, value: EncodingType) {
                self.encoding = ::std::option::Option::Some(value as i32);
            }
            pub fn compression(&self)
             -> ::std::option::Option<CompressionType> {
                self.compression.and_then(CompressionType::from_i32)
            }
            pub fn set_compression(&mut self, value: CompressionType) {
                self.compression = ::std::option::Option::Some(value as i32);
            }
            pub fn block_size(&self) -> i32 {
                match self.block_size {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
        }
    }
    pub struct SchemaPB {
        #[prost(message, repeated, tag = "1")]
        pub columns: Vec<ColumnSchemaPB>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for SchemaPB {
        #[inline]
        fn clone(&self) -> SchemaPB {
            match *self {
                SchemaPB { columns: ref __self_0_0 } =>
                SchemaPB{columns:
                             ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for SchemaPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                SchemaPB { columns: ref __self_0_0 } => {
                    let mut builder = __arg_0.debug_struct("SchemaPB");
                    let _ = builder.field("columns", &&(*__self_0_0));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for SchemaPB {
        #[inline]
        fn eq(&self, __arg_0: &SchemaPB) -> bool {
            match *__arg_0 {
                SchemaPB { columns: ref __self_1_0 } =>
                match *self {
                    SchemaPB { columns: ref __self_0_0 } =>
                    true && (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &SchemaPB) -> bool {
            match *__arg_0 {
                SchemaPB { columns: ref __self_1_0 } =>
                match *self {
                    SchemaPB { columns: ref __self_0_0 } =>
                    false || (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod SchemaPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for SchemaPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                for msg in &self.columns {
                    _prost::encoding::message::encode(1u32, msg, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge_repeated(wire_type,
                                                              &mut self.columns,
                                                              buf).map_err(|mut error|
                                                                               {
                                                                                   error.push("SchemaPB",
                                                                                              "columns");
                                                                                   error
                                                                               }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::message::encoded_len_repeated(1u32,
                                                                    &self.columns)
            }
        }
        impl Default for SchemaPB {
            fn default() -> SchemaPB {
                SchemaPB{columns: ::std::default::Default::default(),}
            }
        }
    }
    pub struct HostPortPB {
        #[prost(string, required, tag = "1")]
        pub host: String,
        #[prost(uint32, required, tag = "2")]
        pub port: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for HostPortPB {
        #[inline]
        fn clone(&self) -> HostPortPB {
            match *self {
                HostPortPB { host: ref __self_0_0, port: ref __self_0_1 } =>
                HostPortPB{host: ::std::clone::Clone::clone(&(*__self_0_0)),
                           port: ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for HostPortPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                HostPortPB { host: ref __self_0_0, port: ref __self_0_1 } => {
                    let mut builder = __arg_0.debug_struct("HostPortPB");
                    let _ = builder.field("host", &&(*__self_0_0));
                    let _ = builder.field("port", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for HostPortPB {
        #[inline]
        fn eq(&self, __arg_0: &HostPortPB) -> bool {
            match *__arg_0 {
                HostPortPB { host: ref __self_1_0, port: ref __self_1_1 } =>
                match *self {
                    HostPortPB { host: ref __self_0_0, port: ref __self_0_1 }
                    =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &HostPortPB) -> bool {
            match *__arg_0 {
                HostPortPB { host: ref __self_1_0, port: ref __self_1_1 } =>
                match *self {
                    HostPortPB { host: ref __self_0_0, port: ref __self_0_1 }
                    =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod HostPortPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for HostPortPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::string::encode(1u32, &self.host, buf);
                _prost::encoding::uint32::encode(2u32, &self.port, buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::string::merge(wire_type, &mut self.host,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("HostPortPB",
                                                                                    "host");
                                                                         error
                                                                     }),
                    2u32 =>
                    _prost::encoding::uint32::merge(wire_type, &mut self.port,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("HostPortPB",
                                                                                    "port");
                                                                         error
                                                                     }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::string::encoded_len(1u32, &self.host) +
                    _prost::encoding::uint32::encoded_len(2u32, &self.port)
            }
        }
        impl Default for HostPortPB {
            fn default() -> HostPortPB {
                HostPortPB{host: ::std::string::String::new(), port: 0u32,}
            }
        }
    }
    /// The serialized format of a Kudu table partition schema.
    pub struct PartitionSchemaPB {
        #[prost(message, repeated, tag = "1")]
        pub hash_bucket_schemas: Vec<partition_schema_pb::HashBucketSchemaPB>,
        #[prost(message, optional, tag = "2")]
        pub range_schema: Option<partition_schema_pb::RangeSchemaPB>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PartitionSchemaPB {
        #[inline]
        fn clone(&self) -> PartitionSchemaPB {
            match *self {
                PartitionSchemaPB {
                hash_bucket_schemas: ref __self_0_0,
                range_schema: ref __self_0_1 } =>
                PartitionSchemaPB{hash_bucket_schemas:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),
                                  range_schema:
                                      ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PartitionSchemaPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                PartitionSchemaPB {
                hash_bucket_schemas: ref __self_0_0,
                range_schema: ref __self_0_1 } => {
                    let mut builder =
                        __arg_0.debug_struct("PartitionSchemaPB");
                    let _ =
                        builder.field("hash_bucket_schemas", &&(*__self_0_0));
                    let _ = builder.field("range_schema", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PartitionSchemaPB {
        #[inline]
        fn eq(&self, __arg_0: &PartitionSchemaPB) -> bool {
            match *__arg_0 {
                PartitionSchemaPB {
                hash_bucket_schemas: ref __self_1_0,
                range_schema: ref __self_1_1 } =>
                match *self {
                    PartitionSchemaPB {
                    hash_bucket_schemas: ref __self_0_0,
                    range_schema: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &PartitionSchemaPB) -> bool {
            match *__arg_0 {
                PartitionSchemaPB {
                hash_bucket_schemas: ref __self_1_0,
                range_schema: ref __self_1_1 } =>
                match *self {
                    PartitionSchemaPB {
                    hash_bucket_schemas: ref __self_0_0,
                    range_schema: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod PartitionSchemaPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for PartitionSchemaPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                for msg in &self.hash_bucket_schemas {
                    _prost::encoding::message::encode(1u32, msg, buf);
                }
                if let Some(ref msg) = self.range_schema {
                    _prost::encoding::message::encode(2u32, msg, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge_repeated(wire_type,
                                                              &mut self.hash_bucket_schemas,
                                                              buf).map_err(|mut error|
                                                                               {
                                                                                   error.push("PartitionSchemaPB",
                                                                                              "hash_bucket_schemas");
                                                                                   error
                                                                               }),
                    2u32 =>
                    {
                        if self.range_schema.is_none() {
                            self.range_schema = Some(Default::default());
                        }
                        match self.range_schema {
                            Some(ref mut msg) =>
                            _prost::encoding::message::merge(wire_type, msg,
                                                             buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   100u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("PartitionSchemaPB",
                                                 "range_schema");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::message::encoded_len_repeated(1u32,
                                                                    &self.hash_bucket_schemas)
                    +
                    self.range_schema.as_ref().map_or(0,
                                                      |msg|
                                                          _prost::encoding::message::encoded_len(2u32,
                                                                                                 msg))
            }
        }
        impl Default for PartitionSchemaPB {
            fn default() -> PartitionSchemaPB {
                PartitionSchemaPB{hash_bucket_schemas:
                                      ::std::default::Default::default(),
                                  range_schema:
                                      ::std::default::Default::default(),}
            }
        }
    }
    pub mod partition_schema_pb {
        /// A column identifier for partition schemas. In general, the name will be
        /// used when a client creates the table since column IDs are assigned by the
        /// master. All other uses of partition schemas will use the numeric column ID.
        pub struct ColumnIdentifierPB {
            #[prost(oneof = "column_identifier_pb::Identifier",
                    tags = "1, 2")]
            pub identifier: Option<column_identifier_pb::Identifier>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ColumnIdentifierPB {
            #[inline]
            fn clone(&self) -> ColumnIdentifierPB {
                match *self {
                    ColumnIdentifierPB { identifier: ref __self_0_0 } =>
                    ColumnIdentifierPB{identifier:
                                           ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ColumnIdentifierPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ColumnIdentifierPB { identifier: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("ColumnIdentifierPB");
                        let _ = builder.field("identifier", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ColumnIdentifierPB {
            #[inline]
            fn eq(&self, __arg_0: &ColumnIdentifierPB) -> bool {
                match *__arg_0 {
                    ColumnIdentifierPB { identifier: ref __self_1_0 } =>
                    match *self {
                        ColumnIdentifierPB { identifier: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ColumnIdentifierPB) -> bool {
                match *__arg_0 {
                    ColumnIdentifierPB { identifier: ref __self_1_0 } =>
                    match *self {
                        ColumnIdentifierPB { identifier: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ColumnIdentifierPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ColumnIdentifierPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let Some(ref oneof) = self.identifier {
                        oneof.encode(buf)
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 | 2u32 =>
                        column_identifier_pb::Identifier::merge(&mut self.identifier,
                                                                tag,
                                                                wire_type,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("ColumnIdentifierPB",
                                                                                                "identifier");
                                                                                     error
                                                                                 }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.identifier.as_ref().map_or(0,
                                                        column_identifier_pb::Identifier::encoded_len)
                }
            }
            impl Default for ColumnIdentifierPB {
                fn default() -> ColumnIdentifierPB {
                    ColumnIdentifierPB{identifier:
                                           ::std::default::Default::default(),}
                }
            }
        }
        pub mod column_identifier_pb {
            pub enum Identifier {

                #[prost(int32, tag = "1")]
                Id(i32),

                #[prost(string, tag = "2")]
                Name(String),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Identifier {
                #[inline]
                fn clone(&self) -> Identifier {
                    match (&*self,) {
                        (&Identifier::Id(ref __self_0),) =>
                        Identifier::Id(::std::clone::Clone::clone(&(*__self_0))),
                        (&Identifier::Name(ref __self_0),) =>
                        Identifier::Name(::std::clone::Clone::clone(&(*__self_0))),
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Identifier {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Identifier::Id(ref __self_0),) => {
                            let mut builder = __arg_0.debug_tuple("Id");
                            let _ = builder.field(&&(*__self_0));
                            builder.finish()
                        }
                        (&Identifier::Name(ref __self_0),) => {
                            let mut builder = __arg_0.debug_tuple("Name");
                            let _ = builder.field(&&(*__self_0));
                            builder.finish()
                        }
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Identifier_ONEOF {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl Identifier {
                    pub fn encode<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        match *self {
                            Identifier::Id(ref value) => {
                                _prost::encoding::int32::encode(1u32, &*value,
                                                                buf);
                            }
                            Identifier::Name(ref value) => {
                                _prost::encoding::string::encode(2u32,
                                                                 &*value,
                                                                 buf);
                            }
                        }
                    }
                    pub fn merge<B>(field:
                                        &mut ::std::option::Option<Identifier>,
                                    tag: u32,
                                    wire_type: _prost::encoding::WireType,
                                    buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        match tag {
                            1u32 => {
                                let mut value =
                                    ::std::default::Default::default();
                                _prost::encoding::int32::merge(wire_type,
                                                               &mut value,
                                                               buf).map(|_|
                                                                            *field
                                                                                =
                                                                                ::std::option::Option::Some(Identifier::Id(value)))
                            }
                            2u32 => {
                                let mut value =
                                    ::std::default::Default::default();
                                _prost::encoding::string::merge(wire_type,
                                                                &mut value,
                                                                buf).map(|_|
                                                                             *field
                                                                                 =
                                                                                 ::std::option::Option::Some(Identifier::Name(value)))
                            }
                            _ => {
                                {
                                    ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                             static __STATIC_FMTSTR:
                                                                                                    &'static [&'static str]
                                                                                                    =
                                                                                                 &["internal error: entered unreachable code: invalid Identifier tag: "];
                                                                                             __STATIC_FMTSTR
                                                                                         },
                                                                                         &match (&tag,)
                                                                                              {
                                                                                              (__arg0,)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }),
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   117u32,
                                                                   31u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }
                    #[inline]
                    pub fn encoded_len(&self) -> usize {
                        match *self {
                            Identifier::Id(ref value) =>
                            _prost::encoding::int32::encoded_len(1u32,
                                                                 &*value),
                            Identifier::Name(ref value) =>
                            _prost::encoding::string::encoded_len(2u32,
                                                                  &*value),
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Identifier {
                #[inline]
                fn eq(&self, __arg_0: &Identifier) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) {
                                (&Identifier::Id(ref __self_0),
                                 &Identifier::Id(ref __arg_1_0)) =>
                                true && (*__self_0) == (*__arg_1_0),
                                (&Identifier::Name(ref __self_0),
                                 &Identifier::Name(ref __arg_1_0)) =>
                                true && (*__self_0) == (*__arg_1_0),
                                _ => unsafe {
                                    ::std::intrinsics::unreachable()
                                }
                            }
                        } else { false }
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &Identifier) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) {
                                (&Identifier::Id(ref __self_0),
                                 &Identifier::Id(ref __arg_1_0)) =>
                                false || (*__self_0) != (*__arg_1_0),
                                (&Identifier::Name(ref __self_0),
                                 &Identifier::Name(ref __arg_1_0)) =>
                                false || (*__self_0) != (*__arg_1_0),
                                _ => unsafe {
                                    ::std::intrinsics::unreachable()
                                }
                            }
                        } else { true }
                    }
                }
            }
        }
        pub struct RangeSchemaPB {
            /// Column identifiers of columns included in the range. All columns must be
            /// a component of the primary key.
            #[prost(message, repeated, tag = "1")]
            pub columns: Vec<ColumnIdentifierPB>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for RangeSchemaPB {
            #[inline]
            fn clone(&self) -> RangeSchemaPB {
                match *self {
                    RangeSchemaPB { columns: ref __self_0_0 } =>
                    RangeSchemaPB{columns:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for RangeSchemaPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RangeSchemaPB { columns: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("RangeSchemaPB");
                        let _ = builder.field("columns", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for RangeSchemaPB {
            #[inline]
            fn eq(&self, __arg_0: &RangeSchemaPB) -> bool {
                match *__arg_0 {
                    RangeSchemaPB { columns: ref __self_1_0 } =>
                    match *self {
                        RangeSchemaPB { columns: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &RangeSchemaPB) -> bool {
                match *__arg_0 {
                    RangeSchemaPB { columns: ref __self_1_0 } =>
                    match *self {
                        RangeSchemaPB { columns: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod RangeSchemaPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for RangeSchemaPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.columns {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.columns,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("RangeSchemaPB",
                                                                                                  "columns");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(1u32,
                                                                        &self.columns)
                }
            }
            impl Default for RangeSchemaPB {
                fn default() -> RangeSchemaPB {
                    RangeSchemaPB{columns:
                                      ::std::default::Default::default(),}
                }
            }
        }
        pub struct HashBucketSchemaPB {
            /// Column identifiers of columns included in the hash. Every column must be
            /// a component of the primary key.
            #[prost(message, repeated, tag = "1")]
            pub columns: Vec<ColumnIdentifierPB>,
            /// Number of buckets into which columns will be hashed. Must be at least 2.
            #[prost(int32, required, tag = "2")]
            pub num_buckets: i32,
            /// Seed value for hash calculation. Administrators may set a seed value
            /// on a per-table basis in order to randomize the mapping of rows to
            /// buckets. Setting a seed provides some amount of protection against denial
            /// of service attacks when the hash bucket columns contain user provided
            /// input.
            #[prost(uint32, optional, tag = "3")]
            pub seed: Option<u32>,
            /// The hash algorithm to use for calculating the hash bucket.
            #[prost(enumeration = "hash_bucket_schema_pb::HashAlgorithm",
                    optional,
                    tag = "4")]
            pub hash_algorithm: Option<i32>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for HashBucketSchemaPB {
            #[inline]
            fn clone(&self) -> HashBucketSchemaPB {
                match *self {
                    HashBucketSchemaPB {
                    columns: ref __self_0_0,
                    num_buckets: ref __self_0_1,
                    seed: ref __self_0_2,
                    hash_algorithm: ref __self_0_3 } =>
                    HashBucketSchemaPB{columns:
                                           ::std::clone::Clone::clone(&(*__self_0_0)),
                                       num_buckets:
                                           ::std::clone::Clone::clone(&(*__self_0_1)),
                                       seed:
                                           ::std::clone::Clone::clone(&(*__self_0_2)),
                                       hash_algorithm:
                                           ::std::clone::Clone::clone(&(*__self_0_3)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for HashBucketSchemaPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    HashBucketSchemaPB {
                    columns: ref __self_0_0,
                    num_buckets: ref __self_0_1,
                    seed: ref __self_0_2,
                    hash_algorithm: ref __self_0_3 } => {
                        let mut builder =
                            __arg_0.debug_struct("HashBucketSchemaPB");
                        let _ = builder.field("columns", &&(*__self_0_0));
                        let _ = builder.field("num_buckets", &&(*__self_0_1));
                        let _ = builder.field("seed", &&(*__self_0_2));
                        let _ =
                            builder.field("hash_algorithm", &&(*__self_0_3));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for HashBucketSchemaPB {
            #[inline]
            fn eq(&self, __arg_0: &HashBucketSchemaPB) -> bool {
                match *__arg_0 {
                    HashBucketSchemaPB {
                    columns: ref __self_1_0,
                    num_buckets: ref __self_1_1,
                    seed: ref __self_1_2,
                    hash_algorithm: ref __self_1_3 } =>
                    match *self {
                        HashBucketSchemaPB {
                        columns: ref __self_0_0,
                        num_buckets: ref __self_0_1,
                        seed: ref __self_0_2,
                        hash_algorithm: ref __self_0_3 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &HashBucketSchemaPB) -> bool {
                match *__arg_0 {
                    HashBucketSchemaPB {
                    columns: ref __self_1_0,
                    num_buckets: ref __self_1_1,
                    seed: ref __self_1_2,
                    hash_algorithm: ref __self_1_3 } =>
                    match *self {
                        HashBucketSchemaPB {
                        columns: ref __self_0_0,
                        num_buckets: ref __self_0_1,
                        seed: ref __self_0_2,
                        hash_algorithm: ref __self_0_3 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod HashBucketSchemaPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for HashBucketSchemaPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.columns {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                    _prost::encoding::int32::encode(2u32, &self.num_buckets,
                                                    buf);
                    if let ::std::option::Option::Some(ref value) = self.seed
                           {
                        _prost::encoding::uint32::encode(3u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.hash_algorithm {
                        _prost::encoding::int32::encode(4u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.columns,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("HashBucketSchemaPB",
                                                                                                  "columns");
                                                                                       error
                                                                                   }),
                        2u32 =>
                        _prost::encoding::int32::merge(wire_type,
                                                       &mut self.num_buckets,
                                                       buf).map_err(|mut error|
                                                                        {
                                                                            error.push("HashBucketSchemaPB",
                                                                                       "num_buckets");
                                                                            error
                                                                        }),
                        3u32 =>
                        {
                            if self.seed.is_none() {
                                self.seed = Some(Default::default());
                            }
                            match self.seed {
                                Some(ref mut value) =>
                                _prost::encoding::uint32::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                       132u32,
                                                                       38u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("HashBucketSchemaPB",
                                                     "seed");
                                          error
                                      }),
                        4u32 =>
                        {
                            if self.hash_algorithm.is_none() {
                                self.hash_algorithm =
                                    Some(Default::default());
                            }
                            match self.hash_algorithm {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                       132u32,
                                                                       38u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("HashBucketSchemaPB",
                                                     "hash_algorithm");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(1u32,
                                                                        &self.columns)
                        +
                        _prost::encoding::int32::encoded_len(2u32,
                                                             &self.num_buckets)
                        +
                        self.seed.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::uint32::encoded_len(3u32,
                                                                                            value))
                        +
                        self.hash_algorithm.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::int32::encoded_len(4u32,
                                                                                                     value))
                }
            }
            impl Default for HashBucketSchemaPB {
                fn default() -> HashBucketSchemaPB {
                    HashBucketSchemaPB{columns:
                                           ::std::default::Default::default(),
                                       num_buckets: 0i32,
                                       seed: ::std::option::Option::None,
                                       hash_algorithm:
                                           ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl HashBucketSchemaPB {
                pub fn seed(&self) -> u32 {
                    match self.seed {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0u32,
                    }
                }
                pub fn hash_algorithm(&self)
                 ->
                     ::std::option::Option<hash_bucket_schema_pb::HashAlgorithm> {
                    self.hash_algorithm.and_then(hash_bucket_schema_pb::HashAlgorithm::from_i32)
                }
                pub fn set_hash_algorithm(&mut self,
                                          value:
                                              hash_bucket_schema_pb::HashAlgorithm) {
                    self.hash_algorithm =
                        ::std::option::Option::Some(value as i32);
                }
            }
        }
        pub mod hash_bucket_schema_pb {
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum HashAlgorithm { Unknown = 0, MurmurHash2 = 1, }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for HashAlgorithm {
                #[inline]
                fn clone(&self) -> HashAlgorithm { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for HashAlgorithm { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for HashAlgorithm {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&HashAlgorithm::Unknown,) => {
                            let mut builder = __arg_0.debug_tuple("Unknown");
                            builder.finish()
                        }
                        (&HashAlgorithm::MurmurHash2,) => {
                            let mut builder =
                                __arg_0.debug_tuple("MurmurHash2");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for HashAlgorithm {
                #[inline]
                fn eq(&self, __arg_0: &HashAlgorithm) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for HashAlgorithm {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod HashAlgorithm_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl HashAlgorithm {
                    #[doc =
                          "Returns `true` if `value` is a variant of `HashAlgorithm`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value { 0 => true, 1 => true, _ => false, }
                    }
                    #[doc =
                          "Converts an `i32` to a `HashAlgorithm`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<HashAlgorithm> {
                        match value {
                            0 =>
                            ::std::option::Option::Some(HashAlgorithm::Unknown),
                            1 =>
                            ::std::option::Option::Some(HashAlgorithm::MurmurHash2),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for HashAlgorithm {
                    fn default() -> HashAlgorithm { HashAlgorithm::Unknown }
                }
                impl ::std::convert::From<HashAlgorithm> for i32 {
                    fn from(value: HashAlgorithm) -> i32 { value as i32 }
                }
            }
        }
    }
    /// The serialized format of a Kudu table partition.
    pub struct PartitionPB {
        /// The hash buckets of the partition. The number of hash buckets must match
        /// the number of hash bucket components in the partition's schema.
        #[prost(int32, repeated, tag = "1")]
        pub hash_buckets: Vec<i32>,
        /// The encoded start partition key (inclusive).
        #[prost(bytes, optional, tag = "2")]
        pub partition_key_start: Option<Vec<u8>>,
        /// The encoded end partition key (exclusive).
        #[prost(bytes, optional, tag = "3")]
        pub partition_key_end: Option<Vec<u8>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PartitionPB {
        #[inline]
        fn clone(&self) -> PartitionPB {
            match *self {
                PartitionPB {
                hash_buckets: ref __self_0_0,
                partition_key_start: ref __self_0_1,
                partition_key_end: ref __self_0_2 } =>
                PartitionPB{hash_buckets:
                                ::std::clone::Clone::clone(&(*__self_0_0)),
                            partition_key_start:
                                ::std::clone::Clone::clone(&(*__self_0_1)),
                            partition_key_end:
                                ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PartitionPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                PartitionPB {
                hash_buckets: ref __self_0_0,
                partition_key_start: ref __self_0_1,
                partition_key_end: ref __self_0_2 } => {
                    let mut builder = __arg_0.debug_struct("PartitionPB");
                    let _ = builder.field("hash_buckets", &&(*__self_0_0));
                    let _ =
                        builder.field("partition_key_start", &&(*__self_0_1));
                    let _ =
                        builder.field("partition_key_end", &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PartitionPB {
        #[inline]
        fn eq(&self, __arg_0: &PartitionPB) -> bool {
            match *__arg_0 {
                PartitionPB {
                hash_buckets: ref __self_1_0,
                partition_key_start: ref __self_1_1,
                partition_key_end: ref __self_1_2 } =>
                match *self {
                    PartitionPB {
                    hash_buckets: ref __self_0_0,
                    partition_key_start: ref __self_0_1,
                    partition_key_end: ref __self_0_2 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &PartitionPB) -> bool {
            match *__arg_0 {
                PartitionPB {
                hash_buckets: ref __self_1_0,
                partition_key_start: ref __self_1_1,
                partition_key_end: ref __self_1_2 } =>
                match *self {
                    PartitionPB {
                    hash_buckets: ref __self_0_0,
                    partition_key_start: ref __self_0_1,
                    partition_key_end: ref __self_0_2 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod PartitionPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for PartitionPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::int32::encode_packed(1u32,
                                                       &self.hash_buckets,
                                                       buf);
                if let ::std::option::Option::Some(ref value) =
                       self.partition_key_start {
                    _prost::encoding::bytes::encode(2u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.partition_key_end {
                    _prost::encoding::bytes::encode(3u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::int32::merge_repeated(wire_type,
                                                            &mut self.hash_buckets,
                                                            buf).map_err(|mut error|
                                                                             {
                                                                                 error.push("PartitionPB",
                                                                                            "hash_buckets");
                                                                                 error
                                                                             }),
                    2u32 =>
                    {
                        if self.partition_key_start.is_none() {
                            self.partition_key_start =
                                Some(Default::default());
                        }
                        match self.partition_key_start {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   161u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("PartitionPB",
                                                 "partition_key_start");
                                      error
                                  }),
                    3u32 =>
                    {
                        if self.partition_key_end.is_none() {
                            self.partition_key_end = Some(Default::default());
                        }
                        match self.partition_key_end {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   161u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("PartitionPB",
                                                 "partition_key_end");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::int32::encoded_len_packed(1u32,
                                                                &self.hash_buckets)
                    +
                    self.partition_key_start.as_ref().map_or(0,
                                                             |value|
                                                                 _prost::encoding::bytes::encoded_len(2u32,
                                                                                                      value))
                    +
                    self.partition_key_end.as_ref().map_or(0,
                                                           |value|
                                                               _prost::encoding::bytes::encoded_len(3u32,
                                                                                                    value))
            }
        }
        impl Default for PartitionPB {
            fn default() -> PartitionPB {
                PartitionPB{hash_buckets: ::std::vec::Vec::new(),
                            partition_key_start: ::std::option::Option::None,
                            partition_key_end: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl PartitionPB {
            pub fn partition_key_start(&self) -> &[u8] {
                match self.partition_key_start {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
            pub fn partition_key_end(&self) -> &[u8] {
                match self.partition_key_end {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
        }
    }
    /// A predicate that can be applied on a Kudu column.
    pub struct ColumnPredicatePB {
        /// The predicate column name.
        #[prost(string, optional, tag = "1")]
        pub column: Option<String>,
        #[prost(oneof = "column_predicate_pb::Predicate",
                tags = "2, 3, 4, 5, 6")]
        pub predicate: Option<column_predicate_pb::Predicate>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ColumnPredicatePB {
        #[inline]
        fn clone(&self) -> ColumnPredicatePB {
            match *self {
                ColumnPredicatePB {
                column: ref __self_0_0, predicate: ref __self_0_1 } =>
                ColumnPredicatePB{column:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),
                                  predicate:
                                      ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ColumnPredicatePB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ColumnPredicatePB {
                column: ref __self_0_0, predicate: ref __self_0_1 } => {
                    let mut builder =
                        __arg_0.debug_struct("ColumnPredicatePB");
                    let _ = builder.field("column", &&(*__self_0_0));
                    let _ = builder.field("predicate", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ColumnPredicatePB {
        #[inline]
        fn eq(&self, __arg_0: &ColumnPredicatePB) -> bool {
            match *__arg_0 {
                ColumnPredicatePB {
                column: ref __self_1_0, predicate: ref __self_1_1 } =>
                match *self {
                    ColumnPredicatePB {
                    column: ref __self_0_0, predicate: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ColumnPredicatePB) -> bool {
            match *__arg_0 {
                ColumnPredicatePB {
                column: ref __self_1_0, predicate: ref __self_1_1 } =>
                match *self {
                    ColumnPredicatePB {
                    column: ref __self_0_0, predicate: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ColumnPredicatePB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ColumnPredicatePB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let ::std::option::Option::Some(ref value) = self.column {
                    _prost::encoding::string::encode(1u32, value, buf);
                }
                if let Some(ref oneof) = self.predicate { oneof.encode(buf) }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    {
                        if self.column.is_none() {
                            self.column = Some(Default::default());
                        }
                        match self.column {
                            Some(ref mut value) =>
                            _prost::encoding::string::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   175u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ColumnPredicatePB",
                                                 "column");
                                      error
                                  }),
                    2u32 | 3u32 | 4u32 | 5u32 | 6u32 =>
                    column_predicate_pb::Predicate::merge(&mut self.predicate,
                                                          tag, wire_type,
                                                          buf).map_err(|mut error|
                                                                           {
                                                                               error.push("ColumnPredicatePB",
                                                                                          "predicate");
                                                                               error
                                                                           }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.column.as_ref().map_or(0,
                                                |value|
                                                    _prost::encoding::string::encoded_len(1u32,
                                                                                          value))
                    +
                    self.predicate.as_ref().map_or(0,
                                                   column_predicate_pb::Predicate::encoded_len)
            }
        }
        impl Default for ColumnPredicatePB {
            fn default() -> ColumnPredicatePB {
                ColumnPredicatePB{column: ::std::option::Option::None,
                                  predicate:
                                      ::std::default::Default::default(),}
            }
        }
        #[allow(dead_code)]
        impl ColumnPredicatePB {
            pub fn column(&self) -> &str {
                match self.column {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => "",
                }
            }
        }
    }
    pub mod column_predicate_pb {
        pub struct Range {
            /// The inclusive lower bound.
            #[prost(bytes, optional, tag = "1")]
            pub lower: Option<Vec<u8>>,
            /// The exclusive upper bound.
            #[prost(bytes, optional, tag = "2")]
            pub upper: Option<Vec<u8>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Range {
            #[inline]
            fn clone(&self) -> Range {
                match *self {
                    Range { lower: ref __self_0_0, upper: ref __self_0_1 } =>
                    Range{lower: ::std::clone::Clone::clone(&(*__self_0_0)),
                          upper: ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Range {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Range { lower: ref __self_0_0, upper: ref __self_0_1 } =>
                    {
                        let mut builder = __arg_0.debug_struct("Range");
                        let _ = builder.field("lower", &&(*__self_0_0));
                        let _ = builder.field("upper", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Range {
            #[inline]
            fn eq(&self, __arg_0: &Range) -> bool {
                match *__arg_0 {
                    Range { lower: ref __self_1_0, upper: ref __self_1_1 } =>
                    match *self {
                        Range { lower: ref __self_0_0, upper: ref __self_0_1 }
                        =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &Range) -> bool {
                match *__arg_0 {
                    Range { lower: ref __self_1_0, upper: ref __self_1_1 } =>
                    match *self {
                        Range { lower: ref __self_0_0, upper: ref __self_0_1 }
                        =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod Range_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for Range {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.lower
                           {
                        _prost::encoding::bytes::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.upper
                           {
                        _prost::encoding::bytes::encode(2u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.lower.is_none() {
                                self.lower = Some(Default::default());
                            }
                            match self.lower {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                       184u32,
                                                                       38u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("Range", "lower");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.upper.is_none() {
                                self.upper = Some(Default::default());
                            }
                            match self.upper {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                       184u32,
                                                                       38u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("Range", "upper");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.lower.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::bytes::encoded_len(1u32,
                                                                                            value))
                        +
                        self.upper.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::bytes::encoded_len(2u32,
                                                                                            value))
                }
            }
            impl Default for Range {
                fn default() -> Range {
                    Range{lower: ::std::option::Option::None,
                          upper: ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl Range {
                pub fn lower(&self) -> &[u8] {
                    match self.lower {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn upper(&self) -> &[u8] {
                    match self.upper {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
            }
        }
        pub struct Equality {
            /// The inclusive lower bound. See comment in Range for notes on the
            /// encoding.
            #[prost(bytes, optional, tag = "1")]
            pub value: Option<Vec<u8>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Equality {
            #[inline]
            fn clone(&self) -> Equality {
                match *self {
                    Equality { value: ref __self_0_0 } =>
                    Equality{value:
                                 ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Equality {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Equality { value: ref __self_0_0 } => {
                        let mut builder = __arg_0.debug_struct("Equality");
                        let _ = builder.field("value", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Equality {
            #[inline]
            fn eq(&self, __arg_0: &Equality) -> bool {
                match *__arg_0 {
                    Equality { value: ref __self_1_0 } =>
                    match *self {
                        Equality { value: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &Equality) -> bool {
                match *__arg_0 {
                    Equality { value: ref __self_1_0 } =>
                    match *self {
                        Equality { value: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod Equality_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for Equality {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.value
                           {
                        _prost::encoding::bytes::encode(1u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.value.is_none() {
                                self.value = Some(Default::default());
                            }
                            match self.value {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                       203u32,
                                                                       38u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("Equality", "value");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.value.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::bytes::encoded_len(1u32,
                                                                                            value))
                }
            }
            impl Default for Equality {
                fn default() -> Equality {
                    Equality{value: ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl Equality {
                pub fn value(&self) -> &[u8] {
                    match self.value {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
            }
        }
        pub struct InList {
            /// A list of values for the field. See comment in Range for notes on
            /// the encoding.
            #[prost(bytes, repeated, tag = "1")]
            pub values: Vec<Vec<u8>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for InList {
            #[inline]
            fn clone(&self) -> InList {
                match *self {
                    InList { values: ref __self_0_0 } =>
                    InList{values:
                               ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for InList {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    InList { values: ref __self_0_0 } => {
                        let mut builder = __arg_0.debug_struct("InList");
                        let _ = builder.field("values", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for InList {
            #[inline]
            fn eq(&self, __arg_0: &InList) -> bool {
                match *__arg_0 {
                    InList { values: ref __self_1_0 } =>
                    match *self {
                        InList { values: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &InList) -> bool {
                match *__arg_0 {
                    InList { values: ref __self_1_0 } =>
                    match *self {
                        InList { values: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod InList_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for InList {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    _prost::encoding::bytes::encode_repeated(1u32,
                                                             &self.values,
                                                             buf);
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::bytes::merge_repeated(wire_type,
                                                                &mut self.values,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("InList",
                                                                                                "values");
                                                                                     error
                                                                                 }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::bytes::encoded_len_repeated(1u32,
                                                                      &self.values)
                }
            }
            impl Default for InList {
                fn default() -> InList {
                    InList{values: ::std::vec::Vec::new(),}
                }
            }
        }
        pub struct IsNotNull {
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for IsNotNull {
            #[inline]
            fn clone(&self) -> IsNotNull {
                match *self { IsNotNull {  } => IsNotNull{}, }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for IsNotNull {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    IsNotNull {  } => {
                        let mut builder = __arg_0.debug_struct("IsNotNull");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for IsNotNull {
            #[inline]
            fn eq(&self, __arg_0: &IsNotNull) -> bool {
                match *__arg_0 {
                    IsNotNull {  } => match *self { IsNotNull {  } => true, },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod IsNotNull_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for IsNotNull {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize { 0 }
            }
            impl Default for IsNotNull {
                fn default() -> IsNotNull { IsNotNull{} }
            }
        }
        pub struct IsNull {
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for IsNull {
            #[inline]
            fn clone(&self) -> IsNull {
                match *self { IsNull {  } => IsNull{}, }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for IsNull {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    IsNull {  } => {
                        let mut builder = __arg_0.debug_struct("IsNull");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for IsNull {
            #[inline]
            fn eq(&self, __arg_0: &IsNull) -> bool {
                match *__arg_0 {
                    IsNull {  } => match *self { IsNull {  } => true, },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod IsNull_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for IsNull {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize { 0 }
            }
            impl Default for IsNull {
                fn default() -> IsNull { IsNull{} }
            }
        }
        pub enum Predicate {

            #[prost(message, tag = "2")]
            Range(Range),

            #[prost(message, tag = "3")]
            Equality(Equality),

            #[prost(message, tag = "4")]
            IsNotNull(IsNotNull),

            #[prost(message, tag = "5")]
            InList(InList),

            #[prost(message, tag = "6")]
            IsNull(IsNull),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Predicate {
            #[inline]
            fn clone(&self) -> Predicate {
                match (&*self,) {
                    (&Predicate::Range(ref __self_0),) =>
                    Predicate::Range(::std::clone::Clone::clone(&(*__self_0))),
                    (&Predicate::Equality(ref __self_0),) =>
                    Predicate::Equality(::std::clone::Clone::clone(&(*__self_0))),
                    (&Predicate::IsNotNull(ref __self_0),) =>
                    Predicate::IsNotNull(::std::clone::Clone::clone(&(*__self_0))),
                    (&Predicate::InList(ref __self_0),) =>
                    Predicate::InList(::std::clone::Clone::clone(&(*__self_0))),
                    (&Predicate::IsNull(ref __self_0),) =>
                    Predicate::IsNull(::std::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Predicate {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&Predicate::Range(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Range");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Predicate::Equality(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Equality");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Predicate::IsNotNull(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("IsNotNull");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Predicate::InList(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("InList");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Predicate::IsNull(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("IsNull");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod Predicate_ONEOF {
            extern crate bytes as _bytes;
            extern crate prost as _prost;
            use super::*;
            impl Predicate {
                pub fn encode<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    match *self {
                        Predicate::Range(ref value) => {
                            _prost::encoding::message::encode(2u32, &*value,
                                                              buf);
                        }
                        Predicate::Equality(ref value) => {
                            _prost::encoding::message::encode(3u32, &*value,
                                                              buf);
                        }
                        Predicate::IsNotNull(ref value) => {
                            _prost::encoding::message::encode(4u32, &*value,
                                                              buf);
                        }
                        Predicate::InList(ref value) => {
                            _prost::encoding::message::encode(5u32, &*value,
                                                              buf);
                        }
                        Predicate::IsNull(ref value) => {
                            _prost::encoding::message::encode(6u32, &*value,
                                                              buf);
                        }
                    }
                }
                pub fn merge<B>(field: &mut ::std::option::Option<Predicate>,
                                tag: u32,
                                wire_type: _prost::encoding::WireType,
                                buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    match tag {
                        2u32 => {
                            let mut value =
                                ::std::default::Default::default();
                            _prost::encoding::message::merge(wire_type,
                                                             &mut value,
                                                             buf).map(|_|
                                                                          *field
                                                                              =
                                                                              ::std::option::Option::Some(Predicate::Range(value)))
                        }
                        3u32 => {
                            let mut value =
                                ::std::default::Default::default();
                            _prost::encoding::message::merge(wire_type,
                                                             &mut value,
                                                             buf).map(|_|
                                                                          *field
                                                                              =
                                                                              ::std::option::Option::Some(Predicate::Equality(value)))
                        }
                        4u32 => {
                            let mut value =
                                ::std::default::Default::default();
                            _prost::encoding::message::merge(wire_type,
                                                             &mut value,
                                                             buf).map(|_|
                                                                          *field
                                                                              =
                                                                              ::std::option::Option::Some(Predicate::IsNotNull(value)))
                        }
                        5u32 => {
                            let mut value =
                                ::std::default::Default::default();
                            _prost::encoding::message::merge(wire_type,
                                                             &mut value,
                                                             buf).map(|_|
                                                                          *field
                                                                              =
                                                                              ::std::option::Option::Some(Predicate::InList(value)))
                        }
                        6u32 => {
                            let mut value =
                                ::std::default::Default::default();
                            _prost::encoding::message::merge(wire_type,
                                                             &mut value,
                                                             buf).map(|_|
                                                                          *field
                                                                              =
                                                                              ::std::option::Option::Some(Predicate::IsNull(value)))
                        }
                        _ => {
                            {
                                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                         static __STATIC_FMTSTR:
                                                                                                &'static [&'static str]
                                                                                                =
                                                                                             &["internal error: entered unreachable code: invalid Predicate tag: "];
                                                                                         __STATIC_FMTSTR
                                                                                     },
                                                                                     &match (&tag,)
                                                                                          {
                                                                                          (__arg0,)
                                                                                          =>
                                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                      }),
                                                      {
                                                          static _FILE_LINE_COL:
                                                                 (&'static str,
                                                                  u32, u32) =
                                                              ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                               223u32, 27u32);
                                                          &_FILE_LINE_COL
                                                      })
                            }
                        }
                    }
                }
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Predicate::Range(ref value) =>
                        _prost::encoding::message::encoded_len(2u32, &*value),
                        Predicate::Equality(ref value) =>
                        _prost::encoding::message::encoded_len(3u32, &*value),
                        Predicate::IsNotNull(ref value) =>
                        _prost::encoding::message::encoded_len(4u32, &*value),
                        Predicate::InList(ref value) =>
                        _prost::encoding::message::encoded_len(5u32, &*value),
                        Predicate::IsNull(ref value) =>
                        _prost::encoding::message::encoded_len(6u32, &*value),
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Predicate {
            #[inline]
            fn eq(&self, __arg_0: &Predicate) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&Predicate::Range(ref __self_0),
                             &Predicate::Range(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&Predicate::Equality(ref __self_0),
                             &Predicate::Equality(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&Predicate::IsNotNull(ref __self_0),
                             &Predicate::IsNotNull(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&Predicate::InList(ref __self_0),
                             &Predicate::InList(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            (&Predicate::IsNull(ref __self_0),
                             &Predicate::IsNull(ref __arg_1_0)) =>
                            true && (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &Predicate) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&Predicate::Range(ref __self_0),
                             &Predicate::Range(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&Predicate::Equality(ref __self_0),
                             &Predicate::Equality(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&Predicate::IsNotNull(ref __self_0),
                             &Predicate::IsNotNull(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&Predicate::InList(ref __self_0),
                             &Predicate::InList(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            (&Predicate::IsNull(ref __self_0),
                             &Predicate::IsNull(ref __arg_1_0)) =>
                            false || (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
    }
    /// If you add a new type keep in mind to add it to the end
    /// or update AddMapping() functions like the one in key_encoder.cc
    /// that have a vector that maps the protobuf tag with the index.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum DataType {
        UnknownData = 999,
        Uint8 = 0,
        Int8 = 1,
        Uint16 = 2,
        Int16 = 3,
        Uint32 = 4,
        Int32 = 5,
        Uint64 = 6,
        Int64 = 7,
        String = 8,
        Bool = 9,
        Float = 10,
        Double = 11,
        Binary = 12,
        UnixtimeMicros = 13,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for DataType {
        #[inline]
        fn clone(&self) -> DataType { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for DataType { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for DataType {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&DataType::UnknownData,) => {
                    let mut builder = __arg_0.debug_tuple("UnknownData");
                    builder.finish()
                }
                (&DataType::Uint8,) => {
                    let mut builder = __arg_0.debug_tuple("Uint8");
                    builder.finish()
                }
                (&DataType::Int8,) => {
                    let mut builder = __arg_0.debug_tuple("Int8");
                    builder.finish()
                }
                (&DataType::Uint16,) => {
                    let mut builder = __arg_0.debug_tuple("Uint16");
                    builder.finish()
                }
                (&DataType::Int16,) => {
                    let mut builder = __arg_0.debug_tuple("Int16");
                    builder.finish()
                }
                (&DataType::Uint32,) => {
                    let mut builder = __arg_0.debug_tuple("Uint32");
                    builder.finish()
                }
                (&DataType::Int32,) => {
                    let mut builder = __arg_0.debug_tuple("Int32");
                    builder.finish()
                }
                (&DataType::Uint64,) => {
                    let mut builder = __arg_0.debug_tuple("Uint64");
                    builder.finish()
                }
                (&DataType::Int64,) => {
                    let mut builder = __arg_0.debug_tuple("Int64");
                    builder.finish()
                }
                (&DataType::String,) => {
                    let mut builder = __arg_0.debug_tuple("String");
                    builder.finish()
                }
                (&DataType::Bool,) => {
                    let mut builder = __arg_0.debug_tuple("Bool");
                    builder.finish()
                }
                (&DataType::Float,) => {
                    let mut builder = __arg_0.debug_tuple("Float");
                    builder.finish()
                }
                (&DataType::Double,) => {
                    let mut builder = __arg_0.debug_tuple("Double");
                    builder.finish()
                }
                (&DataType::Binary,) => {
                    let mut builder = __arg_0.debug_tuple("Binary");
                    builder.finish()
                }
                (&DataType::UnixtimeMicros,) => {
                    let mut builder = __arg_0.debug_tuple("UnixtimeMicros");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for DataType {
        #[inline]
        fn eq(&self, __arg_0: &DataType) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for DataType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod DataType_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl DataType {
            #[doc = "Returns `true` if `value` is a variant of `DataType`."]
            pub fn is_valid(value: i32) -> bool {
                match value {
                    999 => true,
                    0 => true,
                    1 => true,
                    2 => true,
                    3 => true,
                    4 => true,
                    5 => true,
                    6 => true,
                    7 => true,
                    8 => true,
                    9 => true,
                    10 => true,
                    11 => true,
                    12 => true,
                    13 => true,
                    _ => false,
                }
            }
            #[doc =
                  "Converts an `i32` to a `DataType`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32) -> ::std::option::Option<DataType> {
                match value {
                    999 => ::std::option::Option::Some(DataType::UnknownData),
                    0 => ::std::option::Option::Some(DataType::Uint8),
                    1 => ::std::option::Option::Some(DataType::Int8),
                    2 => ::std::option::Option::Some(DataType::Uint16),
                    3 => ::std::option::Option::Some(DataType::Int16),
                    4 => ::std::option::Option::Some(DataType::Uint32),
                    5 => ::std::option::Option::Some(DataType::Int32),
                    6 => ::std::option::Option::Some(DataType::Uint64),
                    7 => ::std::option::Option::Some(DataType::Int64),
                    8 => ::std::option::Option::Some(DataType::String),
                    9 => ::std::option::Option::Some(DataType::Bool),
                    10 => ::std::option::Option::Some(DataType::Float),
                    11 => ::std::option::Option::Some(DataType::Double),
                    12 => ::std::option::Option::Some(DataType::Binary),
                    13 =>
                    ::std::option::Option::Some(DataType::UnixtimeMicros),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for DataType {
            fn default() -> DataType { DataType::UnknownData }
        }
        impl ::std::convert::From<DataType> for i32 {
            fn from(value: DataType) -> i32 { value as i32 }
        }
    }
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum EncodingType {
        UnknownEncoding = 999,
        AutoEncoding = 0,
        PlainEncoding = 1,
        PrefixEncoding = 2,

        /// GROUP_VARINT encoding is deprecated and no longer implemented.
        GroupVarint = 3,
        Rle = 4,
        DictEncoding = 5,
        BitShuffle = 6,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for EncodingType {
        #[inline]
        fn clone(&self) -> EncodingType { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for EncodingType { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for EncodingType {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&EncodingType::UnknownEncoding,) => {
                    let mut builder = __arg_0.debug_tuple("UnknownEncoding");
                    builder.finish()
                }
                (&EncodingType::AutoEncoding,) => {
                    let mut builder = __arg_0.debug_tuple("AutoEncoding");
                    builder.finish()
                }
                (&EncodingType::PlainEncoding,) => {
                    let mut builder = __arg_0.debug_tuple("PlainEncoding");
                    builder.finish()
                }
                (&EncodingType::PrefixEncoding,) => {
                    let mut builder = __arg_0.debug_tuple("PrefixEncoding");
                    builder.finish()
                }
                (&EncodingType::GroupVarint,) => {
                    let mut builder = __arg_0.debug_tuple("GroupVarint");
                    builder.finish()
                }
                (&EncodingType::Rle,) => {
                    let mut builder = __arg_0.debug_tuple("Rle");
                    builder.finish()
                }
                (&EncodingType::DictEncoding,) => {
                    let mut builder = __arg_0.debug_tuple("DictEncoding");
                    builder.finish()
                }
                (&EncodingType::BitShuffle,) => {
                    let mut builder = __arg_0.debug_tuple("BitShuffle");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for EncodingType {
        #[inline]
        fn eq(&self, __arg_0: &EncodingType) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for EncodingType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod EncodingType_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl EncodingType {
            #[doc =
                  "Returns `true` if `value` is a variant of `EncodingType`."]
            pub fn is_valid(value: i32) -> bool {
                match value {
                    999 => true,
                    0 => true,
                    1 => true,
                    2 => true,
                    3 => true,
                    4 => true,
                    5 => true,
                    6 => true,
                    _ => false,
                }
            }
            #[doc =
                  "Converts an `i32` to a `EncodingType`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32)
             -> ::std::option::Option<EncodingType> {
                match value {
                    999 =>
                    ::std::option::Option::Some(EncodingType::UnknownEncoding),
                    0 =>
                    ::std::option::Option::Some(EncodingType::AutoEncoding),
                    1 =>
                    ::std::option::Option::Some(EncodingType::PlainEncoding),
                    2 =>
                    ::std::option::Option::Some(EncodingType::PrefixEncoding),
                    3 =>
                    ::std::option::Option::Some(EncodingType::GroupVarint),
                    4 => ::std::option::Option::Some(EncodingType::Rle),
                    5 =>
                    ::std::option::Option::Some(EncodingType::DictEncoding),
                    6 =>
                    ::std::option::Option::Some(EncodingType::BitShuffle),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for EncodingType {
            fn default() -> EncodingType { EncodingType::UnknownEncoding }
        }
        impl ::std::convert::From<EncodingType> for i32 {
            fn from(value: EncodingType) -> i32 { value as i32 }
        }
    }
    /// The external consistency mode for client requests.
    /// This defines how transactions and/or sequences of operations that touch
    /// several TabletServers, in different machines, can be observed by external
    /// clients.
    ///
    /// Note that ExternalConsistencyMode makes no guarantee on atomicity, i.e.
    /// no sequence of operations is made atomic (or transactional) just because
    /// an external consistency mode is set.
    /// Note also that ExternalConsistencyMode has no implication on the
    /// consistency between replicas of the same tablet.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum ExternalConsistencyMode {
        UnknownExternalConsistencyMode = 0,

        /// The response to any write will contain a timestamp.
        /// Any further calls from the same client to other servers will update
        /// those servers with that timestamp. The user will make sure that the
        /// timestamp is propagated through back-channels to other
        /// KuduClient's.
        ///
        /// WARNING: Failure to propagate timestamp information through
        /// back-channels will negate any external consistency guarantee under this
        /// mode.
        ///
        /// Example:
        /// 1 - Client A executes operation X in Tablet A
        /// 2 - Afterwards, Client A executes operation Y in Tablet B
        ///
        ///
        /// Client B may observe the following operation sequences:
        /// {}, {X}, {X Y}
        ///
        /// This is the default mode.
        ClientPropagated = 1,

        /// The server will guarantee that each transaction is externally
        /// consistent by making sure that none of its results are visible
        /// until every Kudu server agrees that the transaction is in the past.
        /// The client is not obligated to forward timestamp information
        /// through back-channels.
        ///
        /// WARNING: Depending on the clock synchronization state of TabletServers
        /// this may imply considerable latency. Moreover operations with
        /// COMMIT_WAIT requested external consistency will outright fail if
        /// TabletServer clocks are either unsynchronized or synchronized but
        /// with a maximum error which surpasses a pre-configured one.
        ///
        /// Example:
        /// - Client A executes operation X in Tablet A
        /// - Afterwards, Client A executes operation Y in Tablet B
        ///
        ///
        /// Client B may observe the following operation sequences:
        /// {}, {X}, {X Y}
        CommitWait = 2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExternalConsistencyMode {
        #[inline]
        fn clone(&self) -> ExternalConsistencyMode { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for ExternalConsistencyMode { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ExternalConsistencyMode {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&ExternalConsistencyMode::UnknownExternalConsistencyMode,) =>
                {
                    let mut builder =
                        __arg_0.debug_tuple("UnknownExternalConsistencyMode");
                    builder.finish()
                }
                (&ExternalConsistencyMode::ClientPropagated,) => {
                    let mut builder = __arg_0.debug_tuple("ClientPropagated");
                    builder.finish()
                }
                (&ExternalConsistencyMode::CommitWait,) => {
                    let mut builder = __arg_0.debug_tuple("CommitWait");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ExternalConsistencyMode {
        #[inline]
        fn eq(&self, __arg_0: &ExternalConsistencyMode) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ExternalConsistencyMode {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ExternalConsistencyMode_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl ExternalConsistencyMode {
            #[doc =
                  "Returns `true` if `value` is a variant of `ExternalConsistencyMode`."]
            pub fn is_valid(value: i32) -> bool {
                match value { 0 => true, 1 => true, 2 => true, _ => false, }
            }
            #[doc =
                  "Converts an `i32` to a `ExternalConsistencyMode`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32)
             -> ::std::option::Option<ExternalConsistencyMode> {
                match value {
                    0 =>
                    ::std::option::Option::Some(ExternalConsistencyMode::UnknownExternalConsistencyMode),
                    1 =>
                    ::std::option::Option::Some(ExternalConsistencyMode::ClientPropagated),
                    2 =>
                    ::std::option::Option::Some(ExternalConsistencyMode::CommitWait),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for ExternalConsistencyMode {
            fn default() -> ExternalConsistencyMode {
                ExternalConsistencyMode::UnknownExternalConsistencyMode
            }
        }
        impl ::std::convert::From<ExternalConsistencyMode> for i32 {
            fn from(value: ExternalConsistencyMode) -> i32 { value as i32 }
        }
    }
    /// The possible read modes for clients.
    /// Clients set these in Scan requests.
    /// The server keeps 2 snapshot boundaries:
    /// - The earliest snapshot: this corresponds to the earliest kept undo records
    ///   in the tablet, meaning the current state (Base) can be undone up to
    ///   this snapshot.
    /// - The latest snapshot: This corresponds to the instant beyond which no
    ///   no transaction will have an earlier timestamp. Usually this corresponds
    ///   to whatever clock->Now() returns, but can be higher if the client propagates
    ///   a timestamp (see below).
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum ReadMode {
        UnknownReadMode = 0,

        /// When READ_LATEST is specified the server will execute the read independently
        /// of the clock and will always return all visible writes at the time the request
        /// was received. This type of read does not return a snapshot timestamp since
        /// it might not be repeatable, i.e. a later read executed at the same snapshot
        /// timestamp might yield rows that were committed by in-flight transactions.
        ///
        /// This is the default mode.
        ReadLatest = 1,

        /// When READ_AT_SNAPSHOT is specified the server will attempt to perform a read
        /// at the required snapshot. If no snapshot is defined the server will take the
        /// current time as the snapshot timestamp. Snapshot reads are repeatable, i.e.
        /// all future reads at the same timestamp will yield the same rows. This is
        /// performed at the expense of waiting for in-flight transactions whose timestamp
        /// is lower than the snapshot's timestamp to complete.
        ///
        /// When mixing reads and writes clients that specify COMMIT_WAIT as their
        /// external consistency mode and then use the returned write_timestamp to
        /// to perform snapshot reads are guaranteed that that snapshot time is
        /// considered in the past by all servers and no additional action is
        /// necessary. Clients using CLIENT_PROPAGATED however must forcibly propagate
        /// the timestamps even at read time, so that the server will not generate
        /// any more transactions before the snapshot requested by the client.
        /// The latter option is implemented by allowing the client to specify one or
        /// two timestamps, the first one obtained from the previous CLIENT_PROPAGATED
        /// write, directly or through back-channels, must be signed and will be
        /// checked by the server. The second one, if defined, is the actual snapshot
        /// read time. When selecting both the latter must be lower than or equal to
        /// the former.
        /// TODO implement actually signing the propagated timestamp.
        ReadAtSnapshot = 2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ReadMode {
        #[inline]
        fn clone(&self) -> ReadMode { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for ReadMode { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ReadMode {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&ReadMode::UnknownReadMode,) => {
                    let mut builder = __arg_0.debug_tuple("UnknownReadMode");
                    builder.finish()
                }
                (&ReadMode::ReadLatest,) => {
                    let mut builder = __arg_0.debug_tuple("ReadLatest");
                    builder.finish()
                }
                (&ReadMode::ReadAtSnapshot,) => {
                    let mut builder = __arg_0.debug_tuple("ReadAtSnapshot");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ReadMode {
        #[inline]
        fn eq(&self, __arg_0: &ReadMode) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ReadMode {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ReadMode_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl ReadMode {
            #[doc = "Returns `true` if `value` is a variant of `ReadMode`."]
            pub fn is_valid(value: i32) -> bool {
                match value { 0 => true, 1 => true, 2 => true, _ => false, }
            }
            #[doc =
                  "Converts an `i32` to a `ReadMode`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32) -> ::std::option::Option<ReadMode> {
                match value {
                    0 =>
                    ::std::option::Option::Some(ReadMode::UnknownReadMode),
                    1 => ::std::option::Option::Some(ReadMode::ReadLatest),
                    2 =>
                    ::std::option::Option::Some(ReadMode::ReadAtSnapshot),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for ReadMode {
            fn default() -> ReadMode { ReadMode::UnknownReadMode }
        }
        impl ::std::convert::From<ReadMode> for i32 {
            fn from(value: ReadMode) -> i32 { value as i32 }
        }
    }
    /// The possible order modes for clients.
    /// Clients specify these in new scan requests.
    /// Ordered scans are fault-tolerant, and can be retried elsewhere in the case
    /// of tablet server failure. However, ordered scans impose additional overhead
    /// since the tablet server needs to sort the result rows.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum OrderMode {
        UnknownOrderMode = 0,

        /// This is the default order mode.
        Unordered = 1,
        Ordered = 2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for OrderMode {
        #[inline]
        fn clone(&self) -> OrderMode { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for OrderMode { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for OrderMode {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&OrderMode::UnknownOrderMode,) => {
                    let mut builder = __arg_0.debug_tuple("UnknownOrderMode");
                    builder.finish()
                }
                (&OrderMode::Unordered,) => {
                    let mut builder = __arg_0.debug_tuple("Unordered");
                    builder.finish()
                }
                (&OrderMode::Ordered,) => {
                    let mut builder = __arg_0.debug_tuple("Ordered");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for OrderMode {
        #[inline]
        fn eq(&self, __arg_0: &OrderMode) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for OrderMode {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod OrderMode_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl OrderMode {
            #[doc = "Returns `true` if `value` is a variant of `OrderMode`."]
            pub fn is_valid(value: i32) -> bool {
                match value { 0 => true, 1 => true, 2 => true, _ => false, }
            }
            #[doc =
                  "Converts an `i32` to a `OrderMode`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32) -> ::std::option::Option<OrderMode> {
                match value {
                    0 =>
                    ::std::option::Option::Some(OrderMode::UnknownOrderMode),
                    1 => ::std::option::Option::Some(OrderMode::Unordered),
                    2 => ::std::option::Option::Some(OrderMode::Ordered),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for OrderMode {
            fn default() -> OrderMode { OrderMode::UnknownOrderMode }
        }
        impl ::std::convert::From<OrderMode> for i32 {
            fn from(value: OrderMode) -> i32 { value as i32 }
        }
    }
    /// Error status returned by any RPC method.
    /// Every RPC method which could generate an application-level error
    /// should have this (or a more complex error result) as an optional field
    /// in its response.
    ///
    /// This maps to kudu::Status in C++ and org.apache.kudu.Status in Java.
    pub struct AppStatusPB {
        #[prost(enumeration = "app_status_pb::ErrorCode",
                required,
                tag = "1")]
        pub code: i32,
        #[prost(string, optional, tag = "2")]
        pub message: Option<String>,
        #[prost(int32, optional, tag = "4")]
        pub posix_code: Option<i32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for AppStatusPB {
        #[inline]
        fn clone(&self) -> AppStatusPB {
            match *self {
                AppStatusPB {
                code: ref __self_0_0,
                message: ref __self_0_1,
                posix_code: ref __self_0_2 } =>
                AppStatusPB{code: ::std::clone::Clone::clone(&(*__self_0_0)),
                            message:
                                ::std::clone::Clone::clone(&(*__self_0_1)),
                            posix_code:
                                ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for AppStatusPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                AppStatusPB {
                code: ref __self_0_0,
                message: ref __self_0_1,
                posix_code: ref __self_0_2 } => {
                    let mut builder = __arg_0.debug_struct("AppStatusPB");
                    let _ = builder.field("code", &&(*__self_0_0));
                    let _ = builder.field("message", &&(*__self_0_1));
                    let _ = builder.field("posix_code", &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for AppStatusPB {
        #[inline]
        fn eq(&self, __arg_0: &AppStatusPB) -> bool {
            match *__arg_0 {
                AppStatusPB {
                code: ref __self_1_0,
                message: ref __self_1_1,
                posix_code: ref __self_1_2 } =>
                match *self {
                    AppStatusPB {
                    code: ref __self_0_0,
                    message: ref __self_0_1,
                    posix_code: ref __self_0_2 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &AppStatusPB) -> bool {
            match *__arg_0 {
                AppStatusPB {
                code: ref __self_1_0,
                message: ref __self_1_1,
                posix_code: ref __self_1_2 } =>
                match *self {
                    AppStatusPB {
                    code: ref __self_0_0,
                    message: ref __self_0_1,
                    posix_code: ref __self_0_2 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod AppStatusPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for AppStatusPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::int32::encode(1u32, &self.code, buf);
                if let ::std::option::Option::Some(ref value) = self.message {
                    _prost::encoding::string::encode(2u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.posix_code {
                    _prost::encoding::int32::encode(4u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::int32::merge(wire_type, &mut self.code,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("AppStatusPB",
                                                                                   "code");
                                                                        error
                                                                    }),
                    2u32 =>
                    {
                        if self.message.is_none() {
                            self.message = Some(Default::default());
                        }
                        match self.message {
                            Some(ref mut value) =>
                            _prost::encoding::string::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   386u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("AppStatusPB", "message");
                                      error
                                  }),
                    4u32 =>
                    {
                        if self.posix_code.is_none() {
                            self.posix_code = Some(Default::default());
                        }
                        match self.posix_code {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   386u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("AppStatusPB", "posix_code");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::int32::encoded_len(1u32, &self.code) +
                    self.message.as_ref().map_or(0,
                                                 |value|
                                                     _prost::encoding::string::encoded_len(2u32,
                                                                                           value))
                    +
                    self.posix_code.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::int32::encoded_len(4u32,
                                                                                             value))
            }
        }
        impl Default for AppStatusPB {
            fn default() -> AppStatusPB {
                AppStatusPB{code: app_status_pb::ErrorCode::default() as i32,
                            message: ::std::option::Option::None,
                            posix_code: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl AppStatusPB {
            pub fn code(&self)
             -> ::std::option::Option<app_status_pb::ErrorCode> {
                app_status_pb::ErrorCode::from_i32(self.code)
            }
            pub fn set_code(&mut self, value: app_status_pb::ErrorCode) {
                self.code = value as i32;
            }
            pub fn message(&self) -> &str {
                match self.message {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => "",
                }
            }
            pub fn posix_code(&self) -> i32 {
                match self.posix_code {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
        }
    }
    pub mod app_status_pb {
        #[structural_match]
        #[rustc_copy_clone_marker]
        pub enum ErrorCode {
            UnknownError = 999,
            Ok = 0,
            NotFound = 1,
            Corruption = 2,
            NotSupported = 3,
            InvalidArgument = 4,
            IoError = 5,
            AlreadyPresent = 6,
            RuntimeError = 7,
            NetworkError = 8,
            IllegalState = 9,
            NotAuthorized = 10,
            Aborted = 11,
            RemoteError = 12,
            ServiceUnavailable = 13,
            TimedOut = 14,
            Uninitialized = 15,
            ConfigurationError = 16,
            Incomplete = 17,
            EndOfFile = 18,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ErrorCode {
            #[inline]
            fn clone(&self) -> ErrorCode { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for ErrorCode { }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ErrorCode {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&ErrorCode::UnknownError,) => {
                        let mut builder = __arg_0.debug_tuple("UnknownError");
                        builder.finish()
                    }
                    (&ErrorCode::Ok,) => {
                        let mut builder = __arg_0.debug_tuple("Ok");
                        builder.finish()
                    }
                    (&ErrorCode::NotFound,) => {
                        let mut builder = __arg_0.debug_tuple("NotFound");
                        builder.finish()
                    }
                    (&ErrorCode::Corruption,) => {
                        let mut builder = __arg_0.debug_tuple("Corruption");
                        builder.finish()
                    }
                    (&ErrorCode::NotSupported,) => {
                        let mut builder = __arg_0.debug_tuple("NotSupported");
                        builder.finish()
                    }
                    (&ErrorCode::InvalidArgument,) => {
                        let mut builder =
                            __arg_0.debug_tuple("InvalidArgument");
                        builder.finish()
                    }
                    (&ErrorCode::IoError,) => {
                        let mut builder = __arg_0.debug_tuple("IoError");
                        builder.finish()
                    }
                    (&ErrorCode::AlreadyPresent,) => {
                        let mut builder =
                            __arg_0.debug_tuple("AlreadyPresent");
                        builder.finish()
                    }
                    (&ErrorCode::RuntimeError,) => {
                        let mut builder = __arg_0.debug_tuple("RuntimeError");
                        builder.finish()
                    }
                    (&ErrorCode::NetworkError,) => {
                        let mut builder = __arg_0.debug_tuple("NetworkError");
                        builder.finish()
                    }
                    (&ErrorCode::IllegalState,) => {
                        let mut builder = __arg_0.debug_tuple("IllegalState");
                        builder.finish()
                    }
                    (&ErrorCode::NotAuthorized,) => {
                        let mut builder =
                            __arg_0.debug_tuple("NotAuthorized");
                        builder.finish()
                    }
                    (&ErrorCode::Aborted,) => {
                        let mut builder = __arg_0.debug_tuple("Aborted");
                        builder.finish()
                    }
                    (&ErrorCode::RemoteError,) => {
                        let mut builder = __arg_0.debug_tuple("RemoteError");
                        builder.finish()
                    }
                    (&ErrorCode::ServiceUnavailable,) => {
                        let mut builder =
                            __arg_0.debug_tuple("ServiceUnavailable");
                        builder.finish()
                    }
                    (&ErrorCode::TimedOut,) => {
                        let mut builder = __arg_0.debug_tuple("TimedOut");
                        builder.finish()
                    }
                    (&ErrorCode::Uninitialized,) => {
                        let mut builder =
                            __arg_0.debug_tuple("Uninitialized");
                        builder.finish()
                    }
                    (&ErrorCode::ConfigurationError,) => {
                        let mut builder =
                            __arg_0.debug_tuple("ConfigurationError");
                        builder.finish()
                    }
                    (&ErrorCode::Incomplete,) => {
                        let mut builder = __arg_0.debug_tuple("Incomplete");
                        builder.finish()
                    }
                    (&ErrorCode::EndOfFile,) => {
                        let mut builder = __arg_0.debug_tuple("EndOfFile");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ErrorCode {
            #[inline]
            fn eq(&self, __arg_0: &ErrorCode) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) { _ => true, }
                    } else { false }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for ErrorCode {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () { { } }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ErrorCode_ENUMERATION {
            extern crate bytes as _bytes;
            extern crate prost as _prost;
            use super::*;
            impl ErrorCode {
                #[doc =
                      "Returns `true` if `value` is a variant of `ErrorCode`."]
                pub fn is_valid(value: i32) -> bool {
                    match value {
                        999 => true,
                        0 => true,
                        1 => true,
                        2 => true,
                        3 => true,
                        4 => true,
                        5 => true,
                        6 => true,
                        7 => true,
                        8 => true,
                        9 => true,
                        10 => true,
                        11 => true,
                        12 => true,
                        13 => true,
                        14 => true,
                        15 => true,
                        16 => true,
                        17 => true,
                        18 => true,
                        _ => false,
                    }
                }
                #[doc =
                      "Converts an `i32` to a `ErrorCode`, or `None` if `value` is not a valid variant."]
                pub fn from_i32(value: i32)
                 -> ::std::option::Option<ErrorCode> {
                    match value {
                        999 =>
                        ::std::option::Option::Some(ErrorCode::UnknownError),
                        0 => ::std::option::Option::Some(ErrorCode::Ok),
                        1 => ::std::option::Option::Some(ErrorCode::NotFound),
                        2 =>
                        ::std::option::Option::Some(ErrorCode::Corruption),
                        3 =>
                        ::std::option::Option::Some(ErrorCode::NotSupported),
                        4 =>
                        ::std::option::Option::Some(ErrorCode::InvalidArgument),
                        5 => ::std::option::Option::Some(ErrorCode::IoError),
                        6 =>
                        ::std::option::Option::Some(ErrorCode::AlreadyPresent),
                        7 =>
                        ::std::option::Option::Some(ErrorCode::RuntimeError),
                        8 =>
                        ::std::option::Option::Some(ErrorCode::NetworkError),
                        9 =>
                        ::std::option::Option::Some(ErrorCode::IllegalState),
                        10 =>
                        ::std::option::Option::Some(ErrorCode::NotAuthorized),
                        11 => ::std::option::Option::Some(ErrorCode::Aborted),
                        12 =>
                        ::std::option::Option::Some(ErrorCode::RemoteError),
                        13 =>
                        ::std::option::Option::Some(ErrorCode::ServiceUnavailable),
                        14 =>
                        ::std::option::Option::Some(ErrorCode::TimedOut),
                        15 =>
                        ::std::option::Option::Some(ErrorCode::Uninitialized),
                        16 =>
                        ::std::option::Option::Some(ErrorCode::ConfigurationError),
                        17 =>
                        ::std::option::Option::Some(ErrorCode::Incomplete),
                        18 =>
                        ::std::option::Option::Some(ErrorCode::EndOfFile),
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl ::std::default::Default for ErrorCode {
                fn default() -> ErrorCode { ErrorCode::UnknownError }
            }
            impl ::std::convert::From<ErrorCode> for i32 {
                fn from(value: ErrorCode) -> i32 { value as i32 }
            }
        }
    }
    /// Uniquely identify a particular instance of a particular server in the
    /// cluster.
    pub struct NodeInstancePB {
        /// Unique ID which is created when the server is first started
        /// up. This is stored persistently on disk.
        #[prost(bytes, required, tag = "1")]
        pub permanent_uuid: Vec<u8>,
        /// Sequence number incremented on every start-up of the server.
        /// This makes it easy to detect when an instance has restarted (and
        /// thus can be assumed to have forgotten any soft state it had in
        /// memory).
        ///
        /// On a freshly initialized server, the first sequence number
        /// should be 0.
        #[prost(int64, required, tag = "2")]
        pub instance_seqno: i64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for NodeInstancePB {
        #[inline]
        fn clone(&self) -> NodeInstancePB {
            match *self {
                NodeInstancePB {
                permanent_uuid: ref __self_0_0, instance_seqno: ref __self_0_1
                } =>
                NodeInstancePB{permanent_uuid:
                                   ::std::clone::Clone::clone(&(*__self_0_0)),
                               instance_seqno:
                                   ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for NodeInstancePB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                NodeInstancePB {
                permanent_uuid: ref __self_0_0, instance_seqno: ref __self_0_1
                } => {
                    let mut builder = __arg_0.debug_struct("NodeInstancePB");
                    let _ = builder.field("permanent_uuid", &&(*__self_0_0));
                    let _ = builder.field("instance_seqno", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for NodeInstancePB {
        #[inline]
        fn eq(&self, __arg_0: &NodeInstancePB) -> bool {
            match *__arg_0 {
                NodeInstancePB {
                permanent_uuid: ref __self_1_0, instance_seqno: ref __self_1_1
                } =>
                match *self {
                    NodeInstancePB {
                    permanent_uuid: ref __self_0_0,
                    instance_seqno: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &NodeInstancePB) -> bool {
            match *__arg_0 {
                NodeInstancePB {
                permanent_uuid: ref __self_1_0, instance_seqno: ref __self_1_1
                } =>
                match *self {
                    NodeInstancePB {
                    permanent_uuid: ref __self_0_0,
                    instance_seqno: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod NodeInstancePB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for NodeInstancePB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::bytes::encode(1u32, &self.permanent_uuid,
                                                buf);
                _prost::encoding::int64::encode(2u32, &self.instance_seqno,
                                                buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::bytes::merge(wire_type,
                                                   &mut self.permanent_uuid,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("NodeInstancePB",
                                                                                   "permanent_uuid");
                                                                        error
                                                                    }),
                    2u32 =>
                    _prost::encoding::int64::merge(wire_type,
                                                   &mut self.instance_seqno,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("NodeInstancePB",
                                                                                   "instance_seqno");
                                                                        error
                                                                    }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::bytes::encoded_len(1u32,
                                                         &self.permanent_uuid)
                    +
                    _prost::encoding::int64::encoded_len(2u32,
                                                         &self.instance_seqno)
            }
        }
        impl Default for NodeInstancePB {
            fn default() -> NodeInstancePB {
                NodeInstancePB{permanent_uuid: ::std::vec::Vec::new(),
                               instance_seqno: 0i64,}
            }
        }
    }
    /// Some basic properties common to both masters and tservers.
    /// They are guaranteed not to change unless the server is restarted.
    pub struct ServerRegistrationPB {
        #[prost(message, repeated, tag = "1")]
        pub rpc_addresses: Vec<HostPortPB>,
        #[prost(message, repeated, tag = "2")]
        pub http_addresses: Vec<HostPortPB>,
        #[prost(string, optional, tag = "3")]
        pub software_version: Option<String>,
        /// True if HTTPS has been enabled for the web interface.
        /// In this case, https:// URLs should be generated for the above
        /// 'http_addresses' field.
        #[prost(bool, optional, tag = "4")]
        pub https_enabled: Option<bool>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ServerRegistrationPB {
        #[inline]
        fn clone(&self) -> ServerRegistrationPB {
            match *self {
                ServerRegistrationPB {
                rpc_addresses: ref __self_0_0,
                http_addresses: ref __self_0_1,
                software_version: ref __self_0_2,
                https_enabled: ref __self_0_3 } =>
                ServerRegistrationPB{rpc_addresses:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     http_addresses:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),
                                     software_version:
                                         ::std::clone::Clone::clone(&(*__self_0_2)),
                                     https_enabled:
                                         ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ServerRegistrationPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ServerRegistrationPB {
                rpc_addresses: ref __self_0_0,
                http_addresses: ref __self_0_1,
                software_version: ref __self_0_2,
                https_enabled: ref __self_0_3 } => {
                    let mut builder =
                        __arg_0.debug_struct("ServerRegistrationPB");
                    let _ = builder.field("rpc_addresses", &&(*__self_0_0));
                    let _ = builder.field("http_addresses", &&(*__self_0_1));
                    let _ =
                        builder.field("software_version", &&(*__self_0_2));
                    let _ = builder.field("https_enabled", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ServerRegistrationPB {
        #[inline]
        fn eq(&self, __arg_0: &ServerRegistrationPB) -> bool {
            match *__arg_0 {
                ServerRegistrationPB {
                rpc_addresses: ref __self_1_0,
                http_addresses: ref __self_1_1,
                software_version: ref __self_1_2,
                https_enabled: ref __self_1_3 } =>
                match *self {
                    ServerRegistrationPB {
                    rpc_addresses: ref __self_0_0,
                    http_addresses: ref __self_0_1,
                    software_version: ref __self_0_2,
                    https_enabled: ref __self_0_3 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ServerRegistrationPB) -> bool {
            match *__arg_0 {
                ServerRegistrationPB {
                rpc_addresses: ref __self_1_0,
                http_addresses: ref __self_1_1,
                software_version: ref __self_1_2,
                https_enabled: ref __self_1_3 } =>
                match *self {
                    ServerRegistrationPB {
                    rpc_addresses: ref __self_0_0,
                    http_addresses: ref __self_0_1,
                    software_version: ref __self_0_2,
                    https_enabled: ref __self_0_3 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ServerRegistrationPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ServerRegistrationPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                for msg in &self.rpc_addresses {
                    _prost::encoding::message::encode(1u32, msg, buf);
                }
                for msg in &self.http_addresses {
                    _prost::encoding::message::encode(2u32, msg, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.software_version {
                    _prost::encoding::string::encode(3u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.https_enabled {
                    _prost::encoding::bool::encode(4u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge_repeated(wire_type,
                                                              &mut self.rpc_addresses,
                                                              buf).map_err(|mut error|
                                                                               {
                                                                                   error.push("ServerRegistrationPB",
                                                                                              "rpc_addresses");
                                                                                   error
                                                                               }),
                    2u32 =>
                    _prost::encoding::message::merge_repeated(wire_type,
                                                              &mut self.http_addresses,
                                                              buf).map_err(|mut error|
                                                                               {
                                                                                   error.push("ServerRegistrationPB",
                                                                                              "http_addresses");
                                                                                   error
                                                                               }),
                    3u32 =>
                    {
                        if self.software_version.is_none() {
                            self.software_version = Some(Default::default());
                        }
                        match self.software_version {
                            Some(ref mut value) =>
                            _prost::encoding::string::merge(wire_type, value,
                                                            buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   440u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerRegistrationPB",
                                                 "software_version");
                                      error
                                  }),
                    4u32 =>
                    {
                        if self.https_enabled.is_none() {
                            self.https_enabled = Some(Default::default());
                        }
                        match self.https_enabled {
                            Some(ref mut value) =>
                            _prost::encoding::bool::merge(wire_type, value,
                                                          buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   440u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerRegistrationPB",
                                                 "https_enabled");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::message::encoded_len_repeated(1u32,
                                                                    &self.rpc_addresses)
                    +
                    _prost::encoding::message::encoded_len_repeated(2u32,
                                                                    &self.http_addresses)
                    +
                    self.software_version.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::string::encoded_len(3u32,
                                                                                                    value))
                    +
                    self.https_enabled.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::bool::encoded_len(4u32,
                                                                                               value))
            }
        }
        impl Default for ServerRegistrationPB {
            fn default() -> ServerRegistrationPB {
                ServerRegistrationPB{rpc_addresses:
                                         ::std::default::Default::default(),
                                     http_addresses:
                                         ::std::default::Default::default(),
                                     software_version:
                                         ::std::option::Option::None,
                                     https_enabled:
                                         ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl ServerRegistrationPB {
            pub fn software_version(&self) -> &str {
                match self.software_version {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => "",
                }
            }
            pub fn https_enabled(&self) -> bool {
                match self.https_enabled {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => false,
                }
            }
        }
    }
    pub struct ServerEntryPB {
        /// If there is an error communicating with the server (or retrieving
        /// the server registration on the server itself), this field will be
        /// set to contain the error.
        ///
        /// All subsequent fields are optional, as they may not be set if
        /// an error is encountered communicating with the individual server.
        #[prost(message, optional, tag = "1")]
        pub error: Option<AppStatusPB>,
        #[prost(message, optional, tag = "2")]
        pub instance_id: Option<NodeInstancePB>,
        #[prost(message, optional, tag = "3")]
        pub registration: Option<ServerRegistrationPB>,
        /// If an error has occured earlier in the RPC call, the role
        /// may be not be set.
        #[prost(enumeration = "consensus::raft_peer_pb::Role",
                optional,
                tag = "4")]
        pub role: Option<i32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ServerEntryPB {
        #[inline]
        fn clone(&self) -> ServerEntryPB {
            match *self {
                ServerEntryPB {
                error: ref __self_0_0,
                instance_id: ref __self_0_1,
                registration: ref __self_0_2,
                role: ref __self_0_3 } =>
                ServerEntryPB{error:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              instance_id:
                                  ::std::clone::Clone::clone(&(*__self_0_1)),
                              registration:
                                  ::std::clone::Clone::clone(&(*__self_0_2)),
                              role:
                                  ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ServerEntryPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                ServerEntryPB {
                error: ref __self_0_0,
                instance_id: ref __self_0_1,
                registration: ref __self_0_2,
                role: ref __self_0_3 } => {
                    let mut builder = __arg_0.debug_struct("ServerEntryPB");
                    let _ = builder.field("error", &&(*__self_0_0));
                    let _ = builder.field("instance_id", &&(*__self_0_1));
                    let _ = builder.field("registration", &&(*__self_0_2));
                    let _ = builder.field("role", &&(*__self_0_3));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ServerEntryPB {
        #[inline]
        fn eq(&self, __arg_0: &ServerEntryPB) -> bool {
            match *__arg_0 {
                ServerEntryPB {
                error: ref __self_1_0,
                instance_id: ref __self_1_1,
                registration: ref __self_1_2,
                role: ref __self_1_3 } =>
                match *self {
                    ServerEntryPB {
                    error: ref __self_0_0,
                    instance_id: ref __self_0_1,
                    registration: ref __self_0_2,
                    role: ref __self_0_3 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &ServerEntryPB) -> bool {
            match *__arg_0 {
                ServerEntryPB {
                error: ref __self_1_0,
                instance_id: ref __self_1_1,
                registration: ref __self_1_2,
                role: ref __self_1_3 } =>
                match *self {
                    ServerEntryPB {
                    error: ref __self_0_0,
                    instance_id: ref __self_0_1,
                    registration: ref __self_0_2,
                    role: ref __self_0_3 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod ServerEntryPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for ServerEntryPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let Some(ref msg) = self.error {
                    _prost::encoding::message::encode(1u32, msg, buf);
                }
                if let Some(ref msg) = self.instance_id {
                    _prost::encoding::message::encode(2u32, msg, buf);
                }
                if let Some(ref msg) = self.registration {
                    _prost::encoding::message::encode(3u32, msg, buf);
                }
                if let ::std::option::Option::Some(ref value) = self.role {
                    _prost::encoding::int32::encode(4u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    {
                        if self.error.is_none() {
                            self.error = Some(Default::default());
                        }
                        match self.error {
                            Some(ref mut msg) =>
                            _prost::encoding::message::merge(wire_type, msg,
                                                             buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   454u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerEntryPB", "error");
                                      error
                                  }),
                    2u32 =>
                    {
                        if self.instance_id.is_none() {
                            self.instance_id = Some(Default::default());
                        }
                        match self.instance_id {
                            Some(ref mut msg) =>
                            _prost::encoding::message::merge(wire_type, msg,
                                                             buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   454u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerEntryPB",
                                                 "instance_id");
                                      error
                                  }),
                    3u32 =>
                    {
                        if self.registration.is_none() {
                            self.registration = Some(Default::default());
                        }
                        match self.registration {
                            Some(ref mut msg) =>
                            _prost::encoding::message::merge(wire_type, msg,
                                                             buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   454u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerEntryPB",
                                                 "registration");
                                      error
                                  }),
                    4u32 =>
                    {
                        if self.role.is_none() {
                            self.role = Some(Default::default());
                        }
                        match self.role {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   454u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("ServerEntryPB", "role");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.error.as_ref().map_or(0,
                                               |msg|
                                                   _prost::encoding::message::encoded_len(1u32,
                                                                                          msg))
                    +
                    self.instance_id.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(2u32,
                                                                                                msg))
                    +
                    self.registration.as_ref().map_or(0,
                                                      |msg|
                                                          _prost::encoding::message::encoded_len(3u32,
                                                                                                 msg))
                    +
                    self.role.as_ref().map_or(0,
                                              |value|
                                                  _prost::encoding::int32::encoded_len(4u32,
                                                                                       value))
            }
        }
        impl Default for ServerEntryPB {
            fn default() -> ServerEntryPB {
                ServerEntryPB{error: ::std::default::Default::default(),
                              instance_id: ::std::default::Default::default(),
                              registration:
                                  ::std::default::Default::default(),
                              role: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl ServerEntryPB {
            pub fn role(&self)
             -> ::std::option::Option<consensus::raft_peer_pb::Role> {
                self.role.and_then(consensus::raft_peer_pb::Role::from_i32)
            }
            pub fn set_role(&mut self, value: consensus::raft_peer_pb::Role) {
                self.role = ::std::option::Option::Some(value as i32);
            }
        }
    }
    /// A row block in which each row is stored contiguously.
    pub struct RowwiseRowBlockPB {
        /// The number of rows in the block. This can typically be calculated
        /// by dividing rows.size() by the width of the row, but in the case that
        /// the client is scanning an empty projection (i.e a COUNT(*)), this
        /// field is the only way to determine how many rows were returned.
        #[prost(int32, optional, tag = "1")]
        pub num_rows: Option<i32>,
        /// Sidecar index for the row data.
        ///
        /// In the sidecar, each row is stored in the same in-memory format
        /// as kudu::ContiguousRow (i.e the raw unencoded data followed by
        /// a null bitmap).
        ///
        /// The data for NULL cells will be present with undefined contents --
        /// typically it will be filled with \x00s but this is not guaranteed,
        /// and clients may choose to initialize NULL cells with whatever they
        /// like. Setting to some constant improves RPC compression, though.
        ///
        /// Any pointers are made relative to the beginning of the indirect
        /// data sidecar.
        ///
        /// See rpc/rpc_sidecar.h for more information on where the data is
        /// actually stored.
        #[prost(int32, optional, tag = "2")]
        pub rows_sidecar: Option<i32>,
        /// Sidecar index for the indirect data.
        ///
        /// In the sidecar, "indirect" data types in the block are stored
        /// contiguously. For example, STRING values in the block will be
        /// stored using the normal Slice in-memory format, except that
        /// instead of being pointers in RAM, the pointer portion will be an
        /// offset into this protobuf field.
        #[prost(int32, optional, tag = "3")]
        pub indirect_data_sidecar: Option<i32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for RowwiseRowBlockPB {
        #[inline]
        fn clone(&self) -> RowwiseRowBlockPB {
            match *self {
                RowwiseRowBlockPB {
                num_rows: ref __self_0_0,
                rows_sidecar: ref __self_0_1,
                indirect_data_sidecar: ref __self_0_2 } =>
                RowwiseRowBlockPB{num_rows:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),
                                  rows_sidecar:
                                      ::std::clone::Clone::clone(&(*__self_0_1)),
                                  indirect_data_sidecar:
                                      ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RowwiseRowBlockPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                RowwiseRowBlockPB {
                num_rows: ref __self_0_0,
                rows_sidecar: ref __self_0_1,
                indirect_data_sidecar: ref __self_0_2 } => {
                    let mut builder =
                        __arg_0.debug_struct("RowwiseRowBlockPB");
                    let _ = builder.field("num_rows", &&(*__self_0_0));
                    let _ = builder.field("rows_sidecar", &&(*__self_0_1));
                    let _ =
                        builder.field("indirect_data_sidecar",
                                      &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for RowwiseRowBlockPB {
        #[inline]
        fn eq(&self, __arg_0: &RowwiseRowBlockPB) -> bool {
            match *__arg_0 {
                RowwiseRowBlockPB {
                num_rows: ref __self_1_0,
                rows_sidecar: ref __self_1_1,
                indirect_data_sidecar: ref __self_1_2 } =>
                match *self {
                    RowwiseRowBlockPB {
                    num_rows: ref __self_0_0,
                    rows_sidecar: ref __self_0_1,
                    indirect_data_sidecar: ref __self_0_2 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &RowwiseRowBlockPB) -> bool {
            match *__arg_0 {
                RowwiseRowBlockPB {
                num_rows: ref __self_1_0,
                rows_sidecar: ref __self_1_1,
                indirect_data_sidecar: ref __self_1_2 } =>
                match *self {
                    RowwiseRowBlockPB {
                    num_rows: ref __self_0_0,
                    rows_sidecar: ref __self_0_1,
                    indirect_data_sidecar: ref __self_0_2 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod RowwiseRowBlockPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for RowwiseRowBlockPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let ::std::option::Option::Some(ref value) = self.num_rows
                       {
                    _prost::encoding::int32::encode(1u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.rows_sidecar {
                    _prost::encoding::int32::encode(2u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.indirect_data_sidecar {
                    _prost::encoding::int32::encode(3u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    {
                        if self.num_rows.is_none() {
                            self.num_rows = Some(Default::default());
                        }
                        match self.num_rows {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   474u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("RowwiseRowBlockPB",
                                                 "num_rows");
                                      error
                                  }),
                    2u32 =>
                    {
                        if self.rows_sidecar.is_none() {
                            self.rows_sidecar = Some(Default::default());
                        }
                        match self.rows_sidecar {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   474u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("RowwiseRowBlockPB",
                                                 "rows_sidecar");
                                      error
                                  }),
                    3u32 =>
                    {
                        if self.indirect_data_sidecar.is_none() {
                            self.indirect_data_sidecar =
                                Some(Default::default());
                        }
                        match self.indirect_data_sidecar {
                            Some(ref mut value) =>
                            _prost::encoding::int32::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   474u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("RowwiseRowBlockPB",
                                                 "indirect_data_sidecar");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.num_rows.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::int32::encoded_len(1u32,
                                                                                           value))
                    +
                    self.rows_sidecar.as_ref().map_or(0,
                                                      |value|
                                                          _prost::encoding::int32::encoded_len(2u32,
                                                                                               value))
                    +
                    self.indirect_data_sidecar.as_ref().map_or(0,
                                                               |value|
                                                                   _prost::encoding::int32::encoded_len(3u32,
                                                                                                        value))
            }
        }
        impl Default for RowwiseRowBlockPB {
            fn default() -> RowwiseRowBlockPB {
                RowwiseRowBlockPB{num_rows: ::std::option::Option::None,
                                  rows_sidecar: ::std::option::Option::None,
                                  indirect_data_sidecar:
                                      ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl RowwiseRowBlockPB {
            pub fn num_rows(&self) -> i32 {
                match self.num_rows {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
            pub fn rows_sidecar(&self) -> i32 {
                match self.rows_sidecar {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
            pub fn indirect_data_sidecar(&self) -> i32 {
                match self.indirect_data_sidecar {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i32,
                }
            }
        }
    }
    /// A set of operations (INSERT, UPDATE, UPSERT, or DELETE) to apply to a table,
    /// or the set of split rows and range bounds when creating or altering table.
    /// Range bounds determine the boundaries of range partitions during table
    /// creation, split rows further subdivide the ranges into more partitions.
    pub struct RowOperationsPB {
        /// The row data for each operation is stored in the following format:
        ///
        /// [operation type] (one byte):
        ///   A single-byte field which determines the type of operation. The values are
        ///   based on the 'Type' enum above.
        /// [column isset bitmap]   (one bit for each column in the Schema, rounded to nearest byte)
        ///   A set bit in this bitmap indicates that the user has specified the given column
        ///   in the row. This indicates that the column will be present in the data to follow.
        /// [null bitmap]           (one bit for each Schema column, rounded to nearest byte)
        ///   A set bit in this bitmap indicates that the given column is NULL.
        ///   This is only present if there are any nullable columns.
        /// [column data]
        ///   For each column which is set and not NULL, the column's data follows. The data
        ///   format of each cell is the canonical in-memory format (eg little endian).
        ///   For string data, the pointers are relative to 'indirect_data'.
        ///
        /// The rows are concatenated end-to-end with no padding/alignment.
        #[prost(bytes, optional, tag = "2")]
        pub rows: Option<Vec<u8>>,
        #[prost(bytes, optional, tag = "3")]
        pub indirect_data: Option<Vec<u8>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for RowOperationsPB {
        #[inline]
        fn clone(&self) -> RowOperationsPB {
            match *self {
                RowOperationsPB {
                rows: ref __self_0_0, indirect_data: ref __self_0_1 } =>
                RowOperationsPB{rows:
                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                indirect_data:
                                    ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RowOperationsPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                RowOperationsPB {
                rows: ref __self_0_0, indirect_data: ref __self_0_1 } => {
                    let mut builder = __arg_0.debug_struct("RowOperationsPB");
                    let _ = builder.field("rows", &&(*__self_0_0));
                    let _ = builder.field("indirect_data", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for RowOperationsPB {
        #[inline]
        fn eq(&self, __arg_0: &RowOperationsPB) -> bool {
            match *__arg_0 {
                RowOperationsPB {
                rows: ref __self_1_0, indirect_data: ref __self_1_1 } =>
                match *self {
                    RowOperationsPB {
                    rows: ref __self_0_0, indirect_data: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &RowOperationsPB) -> bool {
            match *__arg_0 {
                RowOperationsPB {
                rows: ref __self_1_0, indirect_data: ref __self_1_1 } =>
                match *self {
                    RowOperationsPB {
                    rows: ref __self_0_0, indirect_data: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod RowOperationsPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for RowOperationsPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                if let ::std::option::Option::Some(ref value) = self.rows {
                    _prost::encoding::bytes::encode(2u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) =
                       self.indirect_data {
                    _prost::encoding::bytes::encode(3u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    2u32 =>
                    {
                        if self.rows.is_none() {
                            self.rows = Some(Default::default());
                        }
                        match self.rows {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   514u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("RowOperationsPB", "rows");
                                      error
                                  }),
                    3u32 =>
                    {
                        if self.indirect_data.is_none() {
                            self.indirect_data = Some(Default::default());
                        }
                        match self.indirect_data {
                            Some(ref mut value) =>
                            _prost::encoding::bytes::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   514u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("RowOperationsPB",
                                                 "indirect_data");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    self.rows.as_ref().map_or(0,
                                              |value|
                                                  _prost::encoding::bytes::encoded_len(2u32,
                                                                                       value))
                    +
                    self.indirect_data.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::bytes::encoded_len(3u32,
                                                                                                value))
            }
        }
        impl Default for RowOperationsPB {
            fn default() -> RowOperationsPB {
                RowOperationsPB{rows: ::std::option::Option::None,
                                indirect_data: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl RowOperationsPB {
            pub fn rows(&self) -> &[u8] {
                match self.rows {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
            pub fn indirect_data(&self) -> &[u8] {
                match self.indirect_data {
                    ::std::option::Option::Some(ref val) => &val[..],
                    ::std::option::Option::None => b"",
                }
            }
        }
    }
    pub mod row_operations_pb {
        #[structural_match]
        #[rustc_copy_clone_marker]
        pub enum Type {
            Unknown = 0,
            Insert = 1,
            Update = 2,
            Delete = 3,
            Upsert = 5,

            /// Used when specifying split rows on table creation.
            SplitRow = 4,

            /// Used when specifying an inclusive lower bound range on table creation.
            /// Should be followed by the associated upper bound. If all values are
            /// missing, then signifies unbounded.
            RangeLowerBound = 6,

            /// Used when specifying an exclusive upper bound range on table creation.
            /// Should be preceded by the associated lower bound. If all values are
            /// missing, then signifies unbounded.
            RangeUpperBound = 7,

            /// Used when specifying an exclusive lower bound range on table creation.
            /// Should be followed by the associated upper bound. If all values are
            /// missing, then signifies unbounded.
            ExclusiveRangeLowerBound = 8,

            /// Used when specifying an inclusive upper bound range on table creation.
            /// Should be preceded by the associated lower bound. If all values are
            /// missing, then signifies unbounded.
            InclusiveRangeUpperBound = 9,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Type {
            #[inline]
            fn clone(&self) -> Type { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for Type { }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Type {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&Type::Unknown,) => {
                        let mut builder = __arg_0.debug_tuple("Unknown");
                        builder.finish()
                    }
                    (&Type::Insert,) => {
                        let mut builder = __arg_0.debug_tuple("Insert");
                        builder.finish()
                    }
                    (&Type::Update,) => {
                        let mut builder = __arg_0.debug_tuple("Update");
                        builder.finish()
                    }
                    (&Type::Delete,) => {
                        let mut builder = __arg_0.debug_tuple("Delete");
                        builder.finish()
                    }
                    (&Type::Upsert,) => {
                        let mut builder = __arg_0.debug_tuple("Upsert");
                        builder.finish()
                    }
                    (&Type::SplitRow,) => {
                        let mut builder = __arg_0.debug_tuple("SplitRow");
                        builder.finish()
                    }
                    (&Type::RangeLowerBound,) => {
                        let mut builder =
                            __arg_0.debug_tuple("RangeLowerBound");
                        builder.finish()
                    }
                    (&Type::RangeUpperBound,) => {
                        let mut builder =
                            __arg_0.debug_tuple("RangeUpperBound");
                        builder.finish()
                    }
                    (&Type::ExclusiveRangeLowerBound,) => {
                        let mut builder =
                            __arg_0.debug_tuple("ExclusiveRangeLowerBound");
                        builder.finish()
                    }
                    (&Type::InclusiveRangeUpperBound,) => {
                        let mut builder =
                            __arg_0.debug_tuple("InclusiveRangeUpperBound");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Type {
            #[inline]
            fn eq(&self, __arg_0: &Type) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) { _ => true, }
                    } else { false }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for Type {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () { { } }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod Type_ENUMERATION {
            extern crate bytes as _bytes;
            extern crate prost as _prost;
            use super::*;
            impl Type {
                #[doc = "Returns `true` if `value` is a variant of `Type`."]
                pub fn is_valid(value: i32) -> bool {
                    match value {
                        0 => true,
                        1 => true,
                        2 => true,
                        3 => true,
                        5 => true,
                        4 => true,
                        6 => true,
                        7 => true,
                        8 => true,
                        9 => true,
                        _ => false,
                    }
                }
                #[doc =
                      "Converts an `i32` to a `Type`, or `None` if `value` is not a valid variant."]
                pub fn from_i32(value: i32) -> ::std::option::Option<Type> {
                    match value {
                        0 => ::std::option::Option::Some(Type::Unknown),
                        1 => ::std::option::Option::Some(Type::Insert),
                        2 => ::std::option::Option::Some(Type::Update),
                        3 => ::std::option::Option::Some(Type::Delete),
                        5 => ::std::option::Option::Some(Type::Upsert),
                        4 => ::std::option::Option::Some(Type::SplitRow),
                        6 =>
                        ::std::option::Option::Some(Type::RangeLowerBound),
                        7 =>
                        ::std::option::Option::Some(Type::RangeUpperBound),
                        8 =>
                        ::std::option::Option::Some(Type::ExclusiveRangeLowerBound),
                        9 =>
                        ::std::option::Option::Some(Type::InclusiveRangeUpperBound),
                        _ => ::std::option::Option::None,
                    }
                }
            }
            impl ::std::default::Default for Type {
                fn default() -> Type { Type::Unknown }
            }
            impl ::std::convert::From<Type> for i32 {
                fn from(value: Type) -> i32 { value as i32 }
            }
        }
    }
    /// When any server initializes a new filesystem (eg a new node is created in the
    /// cluster), it creates this structure and stores it persistently.
    pub struct InstanceMetadataPB {
        /// The UUID which is assigned when the instance is first formatted. This uniquely
        /// identifies the node in the cluster.
        #[prost(bytes, required, tag = "1")]
        pub uuid: Vec<u8>,
        /// Human-readable string indicating when and where the node was first
        /// initialized.
        #[prost(string, required, tag = "2")]
        pub format_stamp: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for InstanceMetadataPB {
        #[inline]
        fn clone(&self) -> InstanceMetadataPB {
            match *self {
                InstanceMetadataPB {
                uuid: ref __self_0_0, format_stamp: ref __self_0_1 } =>
                InstanceMetadataPB{uuid:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   format_stamp:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for InstanceMetadataPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                InstanceMetadataPB {
                uuid: ref __self_0_0, format_stamp: ref __self_0_1 } => {
                    let mut builder =
                        __arg_0.debug_struct("InstanceMetadataPB");
                    let _ = builder.field("uuid", &&(*__self_0_0));
                    let _ = builder.field("format_stamp", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for InstanceMetadataPB {
        #[inline]
        fn eq(&self, __arg_0: &InstanceMetadataPB) -> bool {
            match *__arg_0 {
                InstanceMetadataPB {
                uuid: ref __self_1_0, format_stamp: ref __self_1_1 } =>
                match *self {
                    InstanceMetadataPB {
                    uuid: ref __self_0_0, format_stamp: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &InstanceMetadataPB) -> bool {
            match *__arg_0 {
                InstanceMetadataPB {
                uuid: ref __self_1_0, format_stamp: ref __self_1_1 } =>
                match *self {
                    InstanceMetadataPB {
                    uuid: ref __self_0_0, format_stamp: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod InstanceMetadataPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for InstanceMetadataPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::bytes::encode(1u32, &self.uuid, buf);
                _prost::encoding::string::encode(2u32, &self.format_stamp,
                                                 buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::bytes::merge(wire_type, &mut self.uuid,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("InstanceMetadataPB",
                                                                                   "uuid");
                                                                        error
                                                                    }),
                    2u32 =>
                    _prost::encoding::string::merge(wire_type,
                                                    &mut self.format_stamp,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("InstanceMetadataPB",
                                                                                    "format_stamp");
                                                                         error
                                                                     }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::bytes::encoded_len(1u32, &self.uuid) +
                    _prost::encoding::string::encoded_len(2u32,
                                                          &self.format_stamp)
            }
        }
        impl Default for InstanceMetadataPB {
            fn default() -> InstanceMetadataPB {
                InstanceMetadataPB{uuid: ::std::vec::Vec::new(),
                                   format_stamp:
                                       ::std::string::String::new(),}
            }
        }
    }
    /// Describes a collection of filesystem path instances and the membership of a
    /// particular instance in the collection.
    ///
    /// In a healthy filesystem (see below), a path instance can be referred to via
    /// its UUID's position in all_uuids instead of via the UUID itself. This is
    /// useful when there are many such references, as the position is much shorter
    /// than the UUID.
    pub struct PathSetPB {
        /// Globally unique identifier for this path instance.
        #[prost(bytes, required, tag = "1")]
        pub uuid: Vec<u8>,
        /// All UUIDs in this path instance set. In a healthy filesystem:
        /// 1. There exists an on-disk PathInstanceMetadataPB for each listed UUID, and
        /// 2. Every PathSetPB contains an identical copy of all_uuids.
        #[prost(bytes, repeated, tag = "2")]
        pub all_uuids: Vec<Vec<u8>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PathSetPB {
        #[inline]
        fn clone(&self) -> PathSetPB {
            match *self {
                PathSetPB { uuid: ref __self_0_0, all_uuids: ref __self_0_1 }
                =>
                PathSetPB{uuid: ::std::clone::Clone::clone(&(*__self_0_0)),
                          all_uuids:
                              ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PathSetPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                PathSetPB { uuid: ref __self_0_0, all_uuids: ref __self_0_1 }
                => {
                    let mut builder = __arg_0.debug_struct("PathSetPB");
                    let _ = builder.field("uuid", &&(*__self_0_0));
                    let _ = builder.field("all_uuids", &&(*__self_0_1));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PathSetPB {
        #[inline]
        fn eq(&self, __arg_0: &PathSetPB) -> bool {
            match *__arg_0 {
                PathSetPB { uuid: ref __self_1_0, all_uuids: ref __self_1_1 }
                =>
                match *self {
                    PathSetPB {
                    uuid: ref __self_0_0, all_uuids: ref __self_0_1 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &PathSetPB) -> bool {
            match *__arg_0 {
                PathSetPB { uuid: ref __self_1_0, all_uuids: ref __self_1_1 }
                =>
                match *self {
                    PathSetPB {
                    uuid: ref __self_0_0, all_uuids: ref __self_0_1 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod PathSetPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for PathSetPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::bytes::encode(1u32, &self.uuid, buf);
                _prost::encoding::bytes::encode_repeated(2u32,
                                                         &self.all_uuids,
                                                         buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::bytes::merge(wire_type, &mut self.uuid,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("PathSetPB",
                                                                                   "uuid");
                                                                        error
                                                                    }),
                    2u32 =>
                    _prost::encoding::bytes::merge_repeated(wire_type,
                                                            &mut self.all_uuids,
                                                            buf).map_err(|mut error|
                                                                             {
                                                                                 error.push("PathSetPB",
                                                                                            "all_uuids");
                                                                                 error
                                                                             }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::bytes::encoded_len(1u32, &self.uuid) +
                    _prost::encoding::bytes::encoded_len_repeated(2u32,
                                                                  &self.all_uuids)
            }
        }
        impl Default for PathSetPB {
            fn default() -> PathSetPB {
                PathSetPB{uuid: ::std::vec::Vec::new(),
                          all_uuids: ::std::vec::Vec::new(),}
            }
        }
    }
    /// A filesystem instance can contain multiple paths. One of these structures
    /// is persisted in each path when the filesystem instance is created.
    pub struct PathInstanceMetadataPB {
        /// Describes this path instance as well as all of the other path instances
        /// that, taken together, describe a complete set.
        #[prost(message, required, tag = "1")]
        pub path_set: PathSetPB,
        /// Textual representation of the block manager that formatted this path.
        #[prost(string, required, tag = "2")]
        pub block_manager_type: String,
        /// Block size on the filesystem where this instance was created. If the
        /// instance (and its data) are ever copied to another location, the block
        /// size in that location must be the same.
        #[prost(uint64, required, tag = "3")]
        pub filesystem_block_size_bytes: u64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PathInstanceMetadataPB {
        #[inline]
        fn clone(&self) -> PathInstanceMetadataPB {
            match *self {
                PathInstanceMetadataPB {
                path_set: ref __self_0_0,
                block_manager_type: ref __self_0_1,
                filesystem_block_size_bytes: ref __self_0_2 } =>
                PathInstanceMetadataPB{path_set:
                                           ::std::clone::Clone::clone(&(*__self_0_0)),
                                       block_manager_type:
                                           ::std::clone::Clone::clone(&(*__self_0_1)),
                                       filesystem_block_size_bytes:
                                           ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PathInstanceMetadataPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                PathInstanceMetadataPB {
                path_set: ref __self_0_0,
                block_manager_type: ref __self_0_1,
                filesystem_block_size_bytes: ref __self_0_2 } => {
                    let mut builder =
                        __arg_0.debug_struct("PathInstanceMetadataPB");
                    let _ = builder.field("path_set", &&(*__self_0_0));
                    let _ =
                        builder.field("block_manager_type", &&(*__self_0_1));
                    let _ =
                        builder.field("filesystem_block_size_bytes",
                                      &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PathInstanceMetadataPB {
        #[inline]
        fn eq(&self, __arg_0: &PathInstanceMetadataPB) -> bool {
            match *__arg_0 {
                PathInstanceMetadataPB {
                path_set: ref __self_1_0,
                block_manager_type: ref __self_1_1,
                filesystem_block_size_bytes: ref __self_1_2 } =>
                match *self {
                    PathInstanceMetadataPB {
                    path_set: ref __self_0_0,
                    block_manager_type: ref __self_0_1,
                    filesystem_block_size_bytes: ref __self_0_2 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &PathInstanceMetadataPB) -> bool {
            match *__arg_0 {
                PathInstanceMetadataPB {
                path_set: ref __self_1_0,
                block_manager_type: ref __self_1_1,
                filesystem_block_size_bytes: ref __self_1_2 } =>
                match *self {
                    PathInstanceMetadataPB {
                    path_set: ref __self_0_0,
                    block_manager_type: ref __self_0_1,
                    filesystem_block_size_bytes: ref __self_0_2 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod PathInstanceMetadataPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for PathInstanceMetadataPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::message::encode(1u32, &self.path_set, buf);
                _prost::encoding::string::encode(2u32,
                                                 &self.block_manager_type,
                                                 buf);
                _prost::encoding::uint64::encode(3u32,
                                                 &self.filesystem_block_size_bytes,
                                                 buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge(wire_type,
                                                     &mut self.path_set,
                                                     buf).map_err(|mut error|
                                                                      {
                                                                          error.push("PathInstanceMetadataPB",
                                                                                     "path_set");
                                                                          error
                                                                      }),
                    2u32 =>
                    _prost::encoding::string::merge(wire_type,
                                                    &mut self.block_manager_type,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("PathInstanceMetadataPB",
                                                                                    "block_manager_type");
                                                                         error
                                                                     }),
                    3u32 =>
                    _prost::encoding::uint64::merge(wire_type,
                                                    &mut self.filesystem_block_size_bytes,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("PathInstanceMetadataPB",
                                                                                    "filesystem_block_size_bytes");
                                                                         error
                                                                     }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::message::encoded_len(1u32,
                                                           &self.path_set) +
                    _prost::encoding::string::encoded_len(2u32,
                                                          &self.block_manager_type)
                    +
                    _prost::encoding::uint64::encoded_len(3u32,
                                                          &self.filesystem_block_size_bytes)
            }
        }
        impl Default for PathInstanceMetadataPB {
            fn default() -> PathInstanceMetadataPB {
                PathInstanceMetadataPB{path_set:
                                           ::std::default::Default::default(),
                                       block_manager_type:
                                           ::std::string::String::new(),
                                       filesystem_block_size_bytes: 0u64,}
            }
        }
    }
    pub struct BlockIdPB {
        #[prost(fixed64, required, tag = "1")]
        pub id: u64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockIdPB {
        #[inline]
        fn clone(&self) -> BlockIdPB {
            match *self {
                BlockIdPB { id: ref __self_0_0 } =>
                BlockIdPB{id: ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockIdPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                BlockIdPB { id: ref __self_0_0 } => {
                    let mut builder = __arg_0.debug_struct("BlockIdPB");
                    let _ = builder.field("id", &&(*__self_0_0));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockIdPB {
        #[inline]
        fn eq(&self, __arg_0: &BlockIdPB) -> bool {
            match *__arg_0 {
                BlockIdPB { id: ref __self_1_0 } =>
                match *self {
                    BlockIdPB { id: ref __self_0_0 } =>
                    true && (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &BlockIdPB) -> bool {
            match *__arg_0 {
                BlockIdPB { id: ref __self_1_0 } =>
                match *self {
                    BlockIdPB { id: ref __self_0_0 } =>
                    false || (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod BlockIdPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for BlockIdPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::fixed64::encode(1u32, &self.id, buf);
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::fixed64::merge(wire_type, &mut self.id,
                                                     buf).map_err(|mut error|
                                                                      {
                                                                          error.push("BlockIdPB",
                                                                                     "id");
                                                                          error
                                                                      }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + _prost::encoding::fixed64::encoded_len(1u32, &self.id)
            }
        }
        impl Default for BlockIdPB {
            fn default() -> BlockIdPB { BlockIdPB{id: 0u64,} }
        }
    }
    /// An element found in a container metadata file of the log-backed block
    /// storage implementation.
    ///
    /// Each one tracks the existence (creation) or non-existence (deletion)
    /// of a particular block. They are written sequentially, with subsequent
    /// messages taking precedence over earlier ones (e.g. "CREATE foo" followed
    /// by "DELETE foo" means that block foo does not exist).
    pub struct BlockRecordPB {
        /// The unique identifier of the block.
        #[prost(message, required, tag = "1")]
        pub block_id: BlockIdPB,
        /// Whether this is a CREATE or a DELETE.
        #[prost(enumeration = "BlockRecordType", required, tag = "2")]
        pub op_type: i32,
        /// The time at which this block record was created, expressed in terms of
        /// microseconds since the epoch.
        #[prost(uint64, required, tag = "3")]
        pub timestamp_us: u64,
        /// The offset of the block in the container data file.
        ///
        /// Required for CREATE.
        #[prost(int64, optional, tag = "4")]
        pub offset: Option<i64>,
        /// The length of the block in the container data file.
        ///
        /// Required for CREATE.
        #[prost(int64, optional, tag = "5")]
        pub length: Option<i64>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockRecordPB {
        #[inline]
        fn clone(&self) -> BlockRecordPB {
            match *self {
                BlockRecordPB {
                block_id: ref __self_0_0,
                op_type: ref __self_0_1,
                timestamp_us: ref __self_0_2,
                offset: ref __self_0_3,
                length: ref __self_0_4 } =>
                BlockRecordPB{block_id:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              op_type:
                                  ::std::clone::Clone::clone(&(*__self_0_1)),
                              timestamp_us:
                                  ::std::clone::Clone::clone(&(*__self_0_2)),
                              offset:
                                  ::std::clone::Clone::clone(&(*__self_0_3)),
                              length:
                                  ::std::clone::Clone::clone(&(*__self_0_4)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockRecordPB {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                BlockRecordPB {
                block_id: ref __self_0_0,
                op_type: ref __self_0_1,
                timestamp_us: ref __self_0_2,
                offset: ref __self_0_3,
                length: ref __self_0_4 } => {
                    let mut builder = __arg_0.debug_struct("BlockRecordPB");
                    let _ = builder.field("block_id", &&(*__self_0_0));
                    let _ = builder.field("op_type", &&(*__self_0_1));
                    let _ = builder.field("timestamp_us", &&(*__self_0_2));
                    let _ = builder.field("offset", &&(*__self_0_3));
                    let _ = builder.field("length", &&(*__self_0_4));
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockRecordPB {
        #[inline]
        fn eq(&self, __arg_0: &BlockRecordPB) -> bool {
            match *__arg_0 {
                BlockRecordPB {
                block_id: ref __self_1_0,
                op_type: ref __self_1_1,
                timestamp_us: ref __self_1_2,
                offset: ref __self_1_3,
                length: ref __self_1_4 } =>
                match *self {
                    BlockRecordPB {
                    block_id: ref __self_0_0,
                    op_type: ref __self_0_1,
                    timestamp_us: ref __self_0_2,
                    offset: ref __self_0_3,
                    length: ref __self_0_4 } =>
                    true && (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4),
                },
            }
        }
        #[inline]
        fn ne(&self, __arg_0: &BlockRecordPB) -> bool {
            match *__arg_0 {
                BlockRecordPB {
                block_id: ref __self_1_0,
                op_type: ref __self_1_1,
                timestamp_us: ref __self_1_2,
                offset: ref __self_1_3,
                length: ref __self_1_4 } =>
                match *self {
                    BlockRecordPB {
                    block_id: ref __self_0_0,
                    op_type: ref __self_0_1,
                    timestamp_us: ref __self_0_2,
                    offset: ref __self_0_3,
                    length: ref __self_0_4 } =>
                    false || (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2) ||
                        (*__self_0_3) != (*__self_1_3) ||
                        (*__self_0_4) != (*__self_1_4),
                },
            }
        }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod BlockRecordPB_MESSAGE {
        extern crate prost as _prost;
        extern crate bytes as _bytes;
        use super::*;
        impl _prost::Message for BlockRecordPB {
            fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                _prost::encoding::message::encode(1u32, &self.block_id, buf);
                _prost::encoding::int32::encode(2u32, &self.op_type, buf);
                _prost::encoding::uint64::encode(3u32, &self.timestamp_us,
                                                 buf);
                if let ::std::option::Option::Some(ref value) = self.offset {
                    _prost::encoding::int64::encode(4u32, value, buf);
                }
                if let ::std::option::Option::Some(ref value) = self.length {
                    _prost::encoding::int64::encode(5u32, value, buf);
                }
            }
            fn merge_field<B>(&mut self, buf: &mut B)
             -> ::std::result::Result<(), _prost::DecodeError> where
             B: _bytes::Buf {
                let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                match tag {
                    1u32 =>
                    _prost::encoding::message::merge(wire_type,
                                                     &mut self.block_id,
                                                     buf).map_err(|mut error|
                                                                      {
                                                                          error.push("BlockRecordPB",
                                                                                     "block_id");
                                                                          error
                                                                      }),
                    2u32 =>
                    _prost::encoding::int32::merge(wire_type,
                                                   &mut self.op_type,
                                                   buf).map_err(|mut error|
                                                                    {
                                                                        error.push("BlockRecordPB",
                                                                                   "op_type");
                                                                        error
                                                                    }),
                    3u32 =>
                    _prost::encoding::uint64::merge(wire_type,
                                                    &mut self.timestamp_us,
                                                    buf).map_err(|mut error|
                                                                     {
                                                                         error.push("BlockRecordPB",
                                                                                    "timestamp_us");
                                                                         error
                                                                     }),
                    4u32 =>
                    {
                        if self.offset.is_none() {
                            self.offset = Some(Default::default());
                        }
                        match self.offset {
                            Some(ref mut value) =>
                            _prost::encoding::int64::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   630u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("BlockRecordPB", "offset");
                                      error
                                  }),
                    5u32 =>
                    {
                        if self.length.is_none() {
                            self.length = Some(Default::default());
                        }
                        match self.length {
                            Some(ref mut value) =>
                            _prost::encoding::int64::merge(wire_type, value,
                                                           buf),
                            _ => {
                                {
                                    ::rt::begin_panic_new("internal error: entered unreachable code",
                                                          {
                                                              static _FILE_LINE_COL:
                                                                     (&'static str,
                                                                      u32,
                                                                      u32) =
                                                                  ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/kudu.rs",
                                                                   630u32,
                                                                   34u32);
                                                              &_FILE_LINE_COL
                                                          })
                                }
                            }
                        }
                    }.map_err(|mut error|
                                  {
                                      error.push("BlockRecordPB", "length");
                                      error
                                  }),
                    _ => _prost::encoding::skip_field(wire_type, buf),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 +
                    _prost::encoding::message::encoded_len(1u32,
                                                           &self.block_id) +
                    _prost::encoding::int32::encoded_len(2u32, &self.op_type)
                    +
                    _prost::encoding::uint64::encoded_len(3u32,
                                                          &self.timestamp_us)
                    +
                    self.offset.as_ref().map_or(0,
                                                |value|
                                                    _prost::encoding::int64::encoded_len(4u32,
                                                                                         value))
                    +
                    self.length.as_ref().map_or(0,
                                                |value|
                                                    _prost::encoding::int64::encoded_len(5u32,
                                                                                         value))
            }
        }
        impl Default for BlockRecordPB {
            fn default() -> BlockRecordPB {
                BlockRecordPB{block_id: ::std::default::Default::default(),
                              op_type: BlockRecordType::default() as i32,
                              timestamp_us: 0u64,
                              offset: ::std::option::Option::None,
                              length: ::std::option::Option::None,}
            }
        }
        #[allow(dead_code)]
        impl BlockRecordPB {
            pub fn op_type(&self) -> ::std::option::Option<BlockRecordType> {
                BlockRecordType::from_i32(self.op_type)
            }
            pub fn set_op_type(&mut self, value: BlockRecordType) {
                self.op_type = value as i32;
            }
            pub fn offset(&self) -> i64 {
                match self.offset {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i64,
                }
            }
            pub fn length(&self) -> i64 {
                match self.length {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => 0i64,
                }
            }
        }
    }
    /// The kind of record.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum BlockRecordType { Unknown = 0, Create = 1, Delete = 2, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockRecordType {
        #[inline]
        fn clone(&self) -> BlockRecordType { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for BlockRecordType { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockRecordType {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockRecordType::Unknown,) => {
                    let mut builder = __arg_0.debug_tuple("Unknown");
                    builder.finish()
                }
                (&BlockRecordType::Create,) => {
                    let mut builder = __arg_0.debug_tuple("Create");
                    builder.finish()
                }
                (&BlockRecordType::Delete,) => {
                    let mut builder = __arg_0.debug_tuple("Delete");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockRecordType {
        #[inline]
        fn eq(&self, __arg_0: &BlockRecordType) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe {
                        ::std::intrinsics::discriminant_value(&*__arg_0)
                    } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*__arg_0) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for BlockRecordType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_snake_case, unused_attributes)]
    mod BlockRecordType_ENUMERATION {
        extern crate bytes as _bytes;
        extern crate prost as _prost;
        use super::*;
        impl BlockRecordType {
            #[doc =
                  "Returns `true` if `value` is a variant of `BlockRecordType`."]
            pub fn is_valid(value: i32) -> bool {
                match value { 0 => true, 1 => true, 2 => true, _ => false, }
            }
            #[doc =
                  "Converts an `i32` to a `BlockRecordType`, or `None` if `value` is not a valid variant."]
            pub fn from_i32(value: i32)
             -> ::std::option::Option<BlockRecordType> {
                match value {
                    0 =>
                    ::std::option::Option::Some(BlockRecordType::Unknown),
                    1 => ::std::option::Option::Some(BlockRecordType::Create),
                    2 => ::std::option::Option::Some(BlockRecordType::Delete),
                    _ => ::std::option::Option::None,
                }
            }
        }
        impl ::std::default::Default for BlockRecordType {
            fn default() -> BlockRecordType { BlockRecordType::Unknown }
        }
        impl ::std::convert::From<BlockRecordType> for i32 {
            fn from(value: BlockRecordType) -> i32 { value as i32 }
        }
    }
    fn foo(foo: ReadMode) { }
    pub mod client {
        /// Serialization format for client scan tokens. Scan tokens are serializable
        /// scan descriptors that are used by query engines to plan a set of parallizable
        /// scanners that are executed on remote task runners. The scan token protobuf
        /// format includes all of the information necessary to recreate a scanner in the
        /// remote task.
        pub struct ScanTokenPB {
            /// The feature set used by this scan token.
            #[prost(enumeration = "scan_token_pb::Feature",
                    repeated,
                    packed = "false",
                    tag = "1")]
            pub feature_flags: Vec<i32>,
            /// The table to scan.
            #[prost(string, optional, tag = "2")]
            pub table_name: Option<String>,
            /// Which columns to select.
            /// if this is an empty list, no data will be returned, but the num_rows
            /// field of the returned RowBlock will indicate how many rows passed
            /// the predicates. Note that in some cases, the scan may still require
            /// multiple round-trips, and the caller must aggregate the counts.
            #[prost(message, repeated, tag = "3")]
            pub projected_columns: Vec<super::ColumnSchemaPB>,
            /// Any column predicates to enforce.
            #[prost(message, repeated, tag = "4")]
            pub column_predicates: Vec<super::ColumnPredicatePB>,
            /// Encoded primary key to begin scanning at (inclusive).
            #[prost(bytes, optional, tag = "5")]
            pub lower_bound_primary_key: Option<Vec<u8>>,
            /// Encoded primary key to stop scanning at (exclusive).
            #[prost(bytes, optional, tag = "6")]
            pub upper_bound_primary_key: Option<Vec<u8>>,
            /// Encoded partition key to begin scanning at (inclusive).
            #[prost(bytes, optional, tag = "7")]
            pub lower_bound_partition_key: Option<Vec<u8>>,
            /// Encoded partition key to stop scanning at (exclusive).
            #[prost(bytes, optional, tag = "8")]
            pub upper_bound_partition_key: Option<Vec<u8>>,
            /// The maximum number of rows to scan.
            /// The scanner will automatically stop yielding results and close
            /// itself after reaching this number of result rows.
            #[prost(uint64, optional, tag = "9")]
            pub limit: Option<u64>,
            /// The read mode for this scan request.
            /// See common.proto for further information about read modes.
            #[prost(enumeration = "super::ReadMode", optional, tag = "10")]
            pub read_mode: Option<i32>,
            /// The requested snapshot timestamp. This is only used
            /// when the read mode is set to READ_AT_SNAPSHOT.
            #[prost(fixed64, optional, tag = "11")]
            pub snap_timestamp: Option<u64>,
            /// Sent by clients which previously executed CLIENT_PROPAGATED writes.
            /// This updates the server's time so that no transaction will be assigned
            /// a timestamp lower than or equal to 'previous_known_timestamp'
            #[prost(fixed64, optional, tag = "12")]
            pub propagated_timestamp: Option<u64>,
            /// Whether data blocks will be cached when read from the files or discarded after use.
            /// Disable this to lower cache churn when doing large scans.
            #[prost(bool, optional, tag = "13")]
            pub cache_blocks: Option<bool>,
            /// Whether the scan should be fault tolerant.
            #[prost(bool, optional, tag = "14")]
            pub fault_tolerant: Option<bool>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ScanTokenPB {
            #[inline]
            fn clone(&self) -> ScanTokenPB {
                match *self {
                    ScanTokenPB {
                    feature_flags: ref __self_0_0,
                    table_name: ref __self_0_1,
                    projected_columns: ref __self_0_2,
                    column_predicates: ref __self_0_3,
                    lower_bound_primary_key: ref __self_0_4,
                    upper_bound_primary_key: ref __self_0_5,
                    lower_bound_partition_key: ref __self_0_6,
                    upper_bound_partition_key: ref __self_0_7,
                    limit: ref __self_0_8,
                    read_mode: ref __self_0_9,
                    snap_timestamp: ref __self_0_10,
                    propagated_timestamp: ref __self_0_11,
                    cache_blocks: ref __self_0_12,
                    fault_tolerant: ref __self_0_13 } =>
                    ScanTokenPB{feature_flags:
                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                table_name:
                                    ::std::clone::Clone::clone(&(*__self_0_1)),
                                projected_columns:
                                    ::std::clone::Clone::clone(&(*__self_0_2)),
                                column_predicates:
                                    ::std::clone::Clone::clone(&(*__self_0_3)),
                                lower_bound_primary_key:
                                    ::std::clone::Clone::clone(&(*__self_0_4)),
                                upper_bound_primary_key:
                                    ::std::clone::Clone::clone(&(*__self_0_5)),
                                lower_bound_partition_key:
                                    ::std::clone::Clone::clone(&(*__self_0_6)),
                                upper_bound_partition_key:
                                    ::std::clone::Clone::clone(&(*__self_0_7)),
                                limit:
                                    ::std::clone::Clone::clone(&(*__self_0_8)),
                                read_mode:
                                    ::std::clone::Clone::clone(&(*__self_0_9)),
                                snap_timestamp:
                                    ::std::clone::Clone::clone(&(*__self_0_10)),
                                propagated_timestamp:
                                    ::std::clone::Clone::clone(&(*__self_0_11)),
                                cache_blocks:
                                    ::std::clone::Clone::clone(&(*__self_0_12)),
                                fault_tolerant:
                                    ::std::clone::Clone::clone(&(*__self_0_13)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ScanTokenPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ScanTokenPB {
                    feature_flags: ref __self_0_0,
                    table_name: ref __self_0_1,
                    projected_columns: ref __self_0_2,
                    column_predicates: ref __self_0_3,
                    lower_bound_primary_key: ref __self_0_4,
                    upper_bound_primary_key: ref __self_0_5,
                    lower_bound_partition_key: ref __self_0_6,
                    upper_bound_partition_key: ref __self_0_7,
                    limit: ref __self_0_8,
                    read_mode: ref __self_0_9,
                    snap_timestamp: ref __self_0_10,
                    propagated_timestamp: ref __self_0_11,
                    cache_blocks: ref __self_0_12,
                    fault_tolerant: ref __self_0_13 } => {
                        let mut builder = __arg_0.debug_struct("ScanTokenPB");
                        let _ =
                            builder.field("feature_flags", &&(*__self_0_0));
                        let _ = builder.field("table_name", &&(*__self_0_1));
                        let _ =
                            builder.field("projected_columns",
                                          &&(*__self_0_2));
                        let _ =
                            builder.field("column_predicates",
                                          &&(*__self_0_3));
                        let _ =
                            builder.field("lower_bound_primary_key",
                                          &&(*__self_0_4));
                        let _ =
                            builder.field("upper_bound_primary_key",
                                          &&(*__self_0_5));
                        let _ =
                            builder.field("lower_bound_partition_key",
                                          &&(*__self_0_6));
                        let _ =
                            builder.field("upper_bound_partition_key",
                                          &&(*__self_0_7));
                        let _ = builder.field("limit", &&(*__self_0_8));
                        let _ = builder.field("read_mode", &&(*__self_0_9));
                        let _ =
                            builder.field("snap_timestamp", &&(*__self_0_10));
                        let _ =
                            builder.field("propagated_timestamp",
                                          &&(*__self_0_11));
                        let _ =
                            builder.field("cache_blocks", &&(*__self_0_12));
                        let _ =
                            builder.field("fault_tolerant", &&(*__self_0_13));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ScanTokenPB {
            #[inline]
            fn eq(&self, __arg_0: &ScanTokenPB) -> bool {
                match *__arg_0 {
                    ScanTokenPB {
                    feature_flags: ref __self_1_0,
                    table_name: ref __self_1_1,
                    projected_columns: ref __self_1_2,
                    column_predicates: ref __self_1_3,
                    lower_bound_primary_key: ref __self_1_4,
                    upper_bound_primary_key: ref __self_1_5,
                    lower_bound_partition_key: ref __self_1_6,
                    upper_bound_partition_key: ref __self_1_7,
                    limit: ref __self_1_8,
                    read_mode: ref __self_1_9,
                    snap_timestamp: ref __self_1_10,
                    propagated_timestamp: ref __self_1_11,
                    cache_blocks: ref __self_1_12,
                    fault_tolerant: ref __self_1_13 } =>
                    match *self {
                        ScanTokenPB {
                        feature_flags: ref __self_0_0,
                        table_name: ref __self_0_1,
                        projected_columns: ref __self_0_2,
                        column_predicates: ref __self_0_3,
                        lower_bound_primary_key: ref __self_0_4,
                        upper_bound_primary_key: ref __self_0_5,
                        lower_bound_partition_key: ref __self_0_6,
                        upper_bound_partition_key: ref __self_0_7,
                        limit: ref __self_0_8,
                        read_mode: ref __self_0_9,
                        snap_timestamp: ref __self_0_10,
                        propagated_timestamp: ref __self_0_11,
                        cache_blocks: ref __self_0_12,
                        fault_tolerant: ref __self_0_13 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6) &&
                            (*__self_0_7) == (*__self_1_7) &&
                            (*__self_0_8) == (*__self_1_8) &&
                            (*__self_0_9) == (*__self_1_9) &&
                            (*__self_0_10) == (*__self_1_10) &&
                            (*__self_0_11) == (*__self_1_11) &&
                            (*__self_0_12) == (*__self_1_12) &&
                            (*__self_0_13) == (*__self_1_13),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ScanTokenPB) -> bool {
                match *__arg_0 {
                    ScanTokenPB {
                    feature_flags: ref __self_1_0,
                    table_name: ref __self_1_1,
                    projected_columns: ref __self_1_2,
                    column_predicates: ref __self_1_3,
                    lower_bound_primary_key: ref __self_1_4,
                    upper_bound_primary_key: ref __self_1_5,
                    lower_bound_partition_key: ref __self_1_6,
                    upper_bound_partition_key: ref __self_1_7,
                    limit: ref __self_1_8,
                    read_mode: ref __self_1_9,
                    snap_timestamp: ref __self_1_10,
                    propagated_timestamp: ref __self_1_11,
                    cache_blocks: ref __self_1_12,
                    fault_tolerant: ref __self_1_13 } =>
                    match *self {
                        ScanTokenPB {
                        feature_flags: ref __self_0_0,
                        table_name: ref __self_0_1,
                        projected_columns: ref __self_0_2,
                        column_predicates: ref __self_0_3,
                        lower_bound_primary_key: ref __self_0_4,
                        upper_bound_primary_key: ref __self_0_5,
                        lower_bound_partition_key: ref __self_0_6,
                        upper_bound_partition_key: ref __self_0_7,
                        limit: ref __self_0_8,
                        read_mode: ref __self_0_9,
                        snap_timestamp: ref __self_0_10,
                        propagated_timestamp: ref __self_0_11,
                        cache_blocks: ref __self_0_12,
                        fault_tolerant: ref __self_0_13 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6) ||
                            (*__self_0_7) != (*__self_1_7) ||
                            (*__self_0_8) != (*__self_1_8) ||
                            (*__self_0_9) != (*__self_1_9) ||
                            (*__self_0_10) != (*__self_1_10) ||
                            (*__self_0_11) != (*__self_1_11) ||
                            (*__self_0_12) != (*__self_1_12) ||
                            (*__self_0_13) != (*__self_1_13),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ScanTokenPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ScanTokenPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    _prost::encoding::int32::encode_repeated(1u32,
                                                             &self.feature_flags,
                                                             buf);
                    if let ::std::option::Option::Some(ref value) =
                           self.table_name {
                        _prost::encoding::string::encode(2u32, value, buf);
                    }
                    for msg in &self.projected_columns {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                    for msg in &self.column_predicates {
                        _prost::encoding::message::encode(4u32, msg, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.lower_bound_primary_key {
                        _prost::encoding::bytes::encode(5u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.upper_bound_primary_key {
                        _prost::encoding::bytes::encode(6u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.lower_bound_partition_key {
                        _prost::encoding::bytes::encode(7u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.upper_bound_partition_key {
                        _prost::encoding::bytes::encode(8u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.limit
                           {
                        _prost::encoding::uint64::encode(9u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.read_mode {
                        _prost::encoding::int32::encode(10u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.snap_timestamp {
                        _prost::encoding::fixed64::encode(11u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.propagated_timestamp {
                        _prost::encoding::fixed64::encode(12u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.cache_blocks {
                        _prost::encoding::bool::encode(13u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.fault_tolerant {
                        _prost::encoding::bool::encode(14u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::int32::merge_repeated(wire_type,
                                                                &mut self.feature_flags,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("ScanTokenPB",
                                                                                                "feature_flags");
                                                                                     error
                                                                                 }),
                        2u32 =>
                        {
                            if self.table_name.is_none() {
                                self.table_name = Some(Default::default());
                            }
                            match self.table_name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "table_name");
                                          error
                                      }),
                        3u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.projected_columns,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("ScanTokenPB",
                                                                                                  "projected_columns");
                                                                                       error
                                                                                   }),
                        4u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.column_predicates,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("ScanTokenPB",
                                                                                                  "column_predicates");
                                                                                       error
                                                                                   }),
                        5u32 =>
                        {
                            if self.lower_bound_primary_key.is_none() {
                                self.lower_bound_primary_key =
                                    Some(Default::default());
                            }
                            match self.lower_bound_primary_key {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "lower_bound_primary_key");
                                          error
                                      }),
                        6u32 =>
                        {
                            if self.upper_bound_primary_key.is_none() {
                                self.upper_bound_primary_key =
                                    Some(Default::default());
                            }
                            match self.upper_bound_primary_key {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "upper_bound_primary_key");
                                          error
                                      }),
                        7u32 =>
                        {
                            if self.lower_bound_partition_key.is_none() {
                                self.lower_bound_partition_key =
                                    Some(Default::default());
                            }
                            match self.lower_bound_partition_key {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "lower_bound_partition_key");
                                          error
                                      }),
                        8u32 =>
                        {
                            if self.upper_bound_partition_key.is_none() {
                                self.upper_bound_partition_key =
                                    Some(Default::default());
                            }
                            match self.upper_bound_partition_key {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "upper_bound_partition_key");
                                          error
                                      }),
                        9u32 =>
                        {
                            if self.limit.is_none() {
                                self.limit = Some(Default::default());
                            }
                            match self.limit {
                                Some(ref mut value) =>
                                _prost::encoding::uint64::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB", "limit");
                                          error
                                      }),
                        10u32 =>
                        {
                            if self.read_mode.is_none() {
                                self.read_mode = Some(Default::default());
                            }
                            match self.read_mode {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "read_mode");
                                          error
                                      }),
                        11u32 =>
                        {
                            if self.snap_timestamp.is_none() {
                                self.snap_timestamp =
                                    Some(Default::default());
                            }
                            match self.snap_timestamp {
                                Some(ref mut value) =>
                                _prost::encoding::fixed64::merge(wire_type,
                                                                 value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "snap_timestamp");
                                          error
                                      }),
                        12u32 =>
                        {
                            if self.propagated_timestamp.is_none() {
                                self.propagated_timestamp =
                                    Some(Default::default());
                            }
                            match self.propagated_timestamp {
                                Some(ref mut value) =>
                                _prost::encoding::fixed64::merge(wire_type,
                                                                 value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "propagated_timestamp");
                                          error
                                      }),
                        13u32 =>
                        {
                            if self.cache_blocks.is_none() {
                                self.cache_blocks = Some(Default::default());
                            }
                            match self.cache_blocks {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "cache_blocks");
                                          error
                                      }),
                        14u32 =>
                        {
                            if self.fault_tolerant.is_none() {
                                self.fault_tolerant =
                                    Some(Default::default());
                            }
                            match self.fault_tolerant {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ScanTokenPB",
                                                     "fault_tolerant");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::int32::encoded_len_repeated(1u32,
                                                                      &self.feature_flags)
                        +
                        self.table_name.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::string::encoded_len(2u32,
                                                                                                  value))
                        +
                        _prost::encoding::message::encoded_len_repeated(3u32,
                                                                        &self.projected_columns)
                        +
                        _prost::encoding::message::encoded_len_repeated(4u32,
                                                                        &self.column_predicates)
                        +
                        self.lower_bound_primary_key.as_ref().map_or(0,
                                                                     |value|
                                                                         _prost::encoding::bytes::encoded_len(5u32,
                                                                                                              value))
                        +
                        self.upper_bound_primary_key.as_ref().map_or(0,
                                                                     |value|
                                                                         _prost::encoding::bytes::encoded_len(6u32,
                                                                                                              value))
                        +
                        self.lower_bound_partition_key.as_ref().map_or(0,
                                                                       |value|
                                                                           _prost::encoding::bytes::encoded_len(7u32,
                                                                                                                value))
                        +
                        self.upper_bound_partition_key.as_ref().map_or(0,
                                                                       |value|
                                                                           _prost::encoding::bytes::encoded_len(8u32,
                                                                                                                value))
                        +
                        self.limit.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::uint64::encoded_len(9u32,
                                                                                             value))
                        +
                        self.read_mode.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::int32::encoded_len(10u32,
                                                                                                value))
                        +
                        self.snap_timestamp.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::fixed64::encoded_len(11u32,
                                                                                                       value))
                        +
                        self.propagated_timestamp.as_ref().map_or(0,
                                                                  |value|
                                                                      _prost::encoding::fixed64::encoded_len(12u32,
                                                                                                             value))
                        +
                        self.cache_blocks.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::bool::encoded_len(13u32,
                                                                                                  value))
                        +
                        self.fault_tolerant.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::bool::encoded_len(14u32,
                                                                                                    value))
                }
            }
            impl Default for ScanTokenPB {
                fn default() -> ScanTokenPB {
                    ScanTokenPB{feature_flags: ::std::vec::Vec::new(),
                                table_name: ::std::option::Option::None,
                                projected_columns:
                                    ::std::default::Default::default(),
                                column_predicates:
                                    ::std::default::Default::default(),
                                lower_bound_primary_key:
                                    ::std::option::Option::None,
                                upper_bound_primary_key:
                                    ::std::option::Option::None,
                                lower_bound_partition_key:
                                    ::std::option::Option::None,
                                upper_bound_partition_key:
                                    ::std::option::Option::None,
                                limit: ::std::option::Option::None,
                                read_mode: ::std::option::Option::None,
                                snap_timestamp: ::std::option::Option::None,
                                propagated_timestamp:
                                    ::std::option::Option::None,
                                cache_blocks: ::std::option::Option::None,
                                fault_tolerant: ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl ScanTokenPB {
                pub fn feature_flags(&self)
                 ->
                     ::std::iter::FilterMap<::std::iter::Cloned<::std::slice::Iter<i32>>,
                                            fn(i32)
                                                ->
                                                    Option<scan_token_pb::Feature>> {
                    self.feature_flags.iter().cloned().filter_map(scan_token_pb::Feature::from_i32)
                }
                pub fn push_feature_flags(&mut self,
                                          value: scan_token_pb::Feature) {
                    self.feature_flags.push(value as i32);
                }
                pub fn table_name(&self) -> &str {
                    match self.table_name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn lower_bound_primary_key(&self) -> &[u8] {
                    match self.lower_bound_primary_key {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn upper_bound_primary_key(&self) -> &[u8] {
                    match self.upper_bound_primary_key {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn lower_bound_partition_key(&self) -> &[u8] {
                    match self.lower_bound_partition_key {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn upper_bound_partition_key(&self) -> &[u8] {
                    match self.upper_bound_partition_key {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn limit(&self) -> u64 {
                    match self.limit {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0u64,
                    }
                }
                pub fn read_mode(&self)
                 -> ::std::option::Option<super::ReadMode> {
                    self.read_mode.and_then(super::ReadMode::from_i32)
                }
                pub fn set_read_mode(&mut self, value: super::ReadMode) {
                    self.read_mode =
                        ::std::option::Option::Some(value as i32);
                }
                pub fn snap_timestamp(&self) -> u64 {
                    match self.snap_timestamp {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0u64,
                    }
                }
                pub fn propagated_timestamp(&self) -> u64 {
                    match self.propagated_timestamp {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0u64,
                    }
                }
                pub fn cache_blocks(&self) -> bool {
                    match self.cache_blocks {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn fault_tolerant(&self) -> bool {
                    match self.fault_tolerant {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub mod scan_token_pb {
            /// Features used by the scan token message. Every time the ScanTokenPB message
            /// is updated with a new feature, the feature should be added to this enum, so
            /// that clients that lack the feature can recognize when they receive a token
            /// that uses unknown features.
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum Feature { Unknown = 0, }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Feature {
                #[inline]
                fn clone(&self) -> Feature { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for Feature { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Feature {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Feature::Unknown,) => {
                            let mut builder = __arg_0.debug_tuple("Unknown");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Feature {
                #[inline]
                fn eq(&self, __arg_0: &Feature) -> bool {
                    match (&*self, &*__arg_0) { _ => true, }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for Feature {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Feature_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl Feature {
                    #[doc =
                          "Returns `true` if `value` is a variant of `Feature`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value { 0 => true, _ => false, }
                    }
                    #[doc =
                          "Converts an `i32` to a `Feature`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<Feature> {
                        match value {
                            0 =>
                            ::std::option::Option::Some(Feature::Unknown),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for Feature {
                    fn default() -> Feature { Feature::Unknown }
                }
                impl ::std::convert::From<Feature> for i32 {
                    fn from(value: Feature) -> i32 { value as i32 }
                }
            }
        }
        /// All of the data necessary to authenticate to a cluster from a client with
        /// no Kerberos credentials.
        pub struct AuthenticationCredentialsPB {
            #[prost(message, optional, tag = "1")]
            pub authn_token: Option<super::security::SignedTokenPB>,
            #[prost(bytes, repeated, tag = "2")]
            pub ca_cert_ders: Vec<Vec<u8>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for AuthenticationCredentialsPB {
            #[inline]
            fn clone(&self) -> AuthenticationCredentialsPB {
                match *self {
                    AuthenticationCredentialsPB {
                    authn_token: ref __self_0_0, ca_cert_ders: ref __self_0_1
                    } =>
                    AuthenticationCredentialsPB{authn_token:
                                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                                ca_cert_ders:
                                                    ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for AuthenticationCredentialsPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    AuthenticationCredentialsPB {
                    authn_token: ref __self_0_0, ca_cert_ders: ref __self_0_1
                    } => {
                        let mut builder =
                            __arg_0.debug_struct("AuthenticationCredentialsPB");
                        let _ = builder.field("authn_token", &&(*__self_0_0));
                        let _ =
                            builder.field("ca_cert_ders", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for AuthenticationCredentialsPB {
            #[inline]
            fn eq(&self, __arg_0: &AuthenticationCredentialsPB) -> bool {
                match *__arg_0 {
                    AuthenticationCredentialsPB {
                    authn_token: ref __self_1_0, ca_cert_ders: ref __self_1_1
                    } =>
                    match *self {
                        AuthenticationCredentialsPB {
                        authn_token: ref __self_0_0,
                        ca_cert_ders: ref __self_0_1 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &AuthenticationCredentialsPB) -> bool {
                match *__arg_0 {
                    AuthenticationCredentialsPB {
                    authn_token: ref __self_1_0, ca_cert_ders: ref __self_1_1
                    } =>
                    match *self {
                        AuthenticationCredentialsPB {
                        authn_token: ref __self_0_0,
                        ca_cert_ders: ref __self_0_1 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod AuthenticationCredentialsPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for AuthenticationCredentialsPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let Some(ref msg) = self.authn_token {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                    _prost::encoding::bytes::encode_repeated(2u32,
                                                             &self.ca_cert_ders,
                                                             buf);
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.authn_token.is_none() {
                                self.authn_token = Some(Default::default());
                            }
                            match self.authn_token {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/client.rs",
                                                                       74u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("AuthenticationCredentialsPB",
                                                     "authn_token");
                                          error
                                      }),
                        2u32 =>
                        _prost::encoding::bytes::merge_repeated(wire_type,
                                                                &mut self.ca_cert_ders,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("AuthenticationCredentialsPB",
                                                                                                "ca_cert_ders");
                                                                                     error
                                                                                 }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.authn_token.as_ref().map_or(0,
                                                         |msg|
                                                             _prost::encoding::message::encoded_len(1u32,
                                                                                                    msg))
                        +
                        _prost::encoding::bytes::encoded_len_repeated(2u32,
                                                                      &self.ca_cert_ders)
                }
            }
            impl Default for AuthenticationCredentialsPB {
                fn default() -> AuthenticationCredentialsPB {
                    AuthenticationCredentialsPB{authn_token:
                                                    ::std::default::Default::default(),
                                                ca_cert_ders:
                                                    ::std::vec::Vec::new(),}
                }
            }
        }
    }
    pub mod consensus {
        /// A peer in a configuration.
        pub struct RaftPeerPB {
            /// Permanent uuid is optional: RaftPeerPB/RaftConfigPB instances may
            /// be created before the permanent uuid is known (e.g., when
            /// manually specifying a configuration for Master/CatalogManager);
            /// permament uuid can be retrieved at a later time through RPC.
            #[prost(bytes, optional, tag = "1")]
            pub permanent_uuid: Option<Vec<u8>>,
            #[prost(enumeration = "raft_peer_pb::MemberType",
                    optional,
                    tag = "2")]
            pub member_type: Option<i32>,
            #[prost(message, optional, tag = "3")]
            pub last_known_addr: Option<super::HostPortPB>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for RaftPeerPB {
            #[inline]
            fn clone(&self) -> RaftPeerPB {
                match *self {
                    RaftPeerPB {
                    permanent_uuid: ref __self_0_0,
                    member_type: ref __self_0_1,
                    last_known_addr: ref __self_0_2 } =>
                    RaftPeerPB{permanent_uuid:
                                   ::std::clone::Clone::clone(&(*__self_0_0)),
                               member_type:
                                   ::std::clone::Clone::clone(&(*__self_0_1)),
                               last_known_addr:
                                   ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for RaftPeerPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RaftPeerPB {
                    permanent_uuid: ref __self_0_0,
                    member_type: ref __self_0_1,
                    last_known_addr: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("RaftPeerPB");
                        let _ =
                            builder.field("permanent_uuid", &&(*__self_0_0));
                        let _ = builder.field("member_type", &&(*__self_0_1));
                        let _ =
                            builder.field("last_known_addr", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for RaftPeerPB {
            #[inline]
            fn eq(&self, __arg_0: &RaftPeerPB) -> bool {
                match *__arg_0 {
                    RaftPeerPB {
                    permanent_uuid: ref __self_1_0,
                    member_type: ref __self_1_1,
                    last_known_addr: ref __self_1_2 } =>
                    match *self {
                        RaftPeerPB {
                        permanent_uuid: ref __self_0_0,
                        member_type: ref __self_0_1,
                        last_known_addr: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &RaftPeerPB) -> bool {
                match *__arg_0 {
                    RaftPeerPB {
                    permanent_uuid: ref __self_1_0,
                    member_type: ref __self_1_1,
                    last_known_addr: ref __self_1_2 } =>
                    match *self {
                        RaftPeerPB {
                        permanent_uuid: ref __self_0_0,
                        member_type: ref __self_0_1,
                        last_known_addr: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod RaftPeerPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for RaftPeerPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.permanent_uuid {
                        _prost::encoding::bytes::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.member_type {
                        _prost::encoding::int32::encode(2u32, value, buf);
                    }
                    if let Some(ref msg) = self.last_known_addr {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.permanent_uuid.is_none() {
                                self.permanent_uuid =
                                    Some(Default::default());
                            }
                            match self.permanent_uuid {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftPeerPB",
                                                     "permanent_uuid");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.member_type.is_none() {
                                self.member_type = Some(Default::default());
                            }
                            match self.member_type {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftPeerPB",
                                                     "member_type");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.last_known_addr.is_none() {
                                self.last_known_addr =
                                    Some(Default::default());
                            }
                            match self.last_known_addr {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       6u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftPeerPB",
                                                     "last_known_addr");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.permanent_uuid.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::bytes::encoded_len(1u32,
                                                                                                     value))
                        +
                        self.member_type.as_ref().map_or(0,
                                                         |value|
                                                             _prost::encoding::int32::encoded_len(2u32,
                                                                                                  value))
                        +
                        self.last_known_addr.as_ref().map_or(0,
                                                             |msg|
                                                                 _prost::encoding::message::encoded_len(3u32,
                                                                                                        msg))
                }
            }
            impl Default for RaftPeerPB {
                fn default() -> RaftPeerPB {
                    RaftPeerPB{permanent_uuid: ::std::option::Option::None,
                               member_type: ::std::option::Option::None,
                               last_known_addr:
                                   ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl RaftPeerPB {
                pub fn permanent_uuid(&self) -> &[u8] {
                    match self.permanent_uuid {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn member_type(&self)
                 -> ::std::option::Option<raft_peer_pb::MemberType> {
                    self.member_type.and_then(raft_peer_pb::MemberType::from_i32)
                }
                pub fn set_member_type(&mut self,
                                       value: raft_peer_pb::MemberType) {
                    self.member_type =
                        ::std::option::Option::Some(value as i32);
                }
            }
        }
        pub mod raft_peer_pb {
            /// The possible roles for peers.
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum Role {
                UnknownRole = 999,

                /// Indicates this node is a follower in the configuration, i.e. that it participates
                /// in majorities and accepts Consensus::Update() calls.
                Follower = 0,

                /// Indicates this node is the current leader of the configuration, i.e. that it
                /// participates in majorities and accepts Consensus::Append() calls.
                Leader = 1,

                /// Indicates that this node participates in the configuration in a passive role,
                /// i.e. that it accepts Consensus::Update() calls but does not participate
                /// in elections or majorities.
                Learner = 2,

                /// Indicates that this node is not a participant of the configuration, i.e. does
                /// not accept Consensus::Update() or Consensus::Update() and cannot
                /// participate in elections or majorities. This is usually the role of a node
                /// that leaves the configuration.
                NonParticipant = 3,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Role {
                #[inline]
                fn clone(&self) -> Role { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for Role { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Role {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Role::UnknownRole,) => {
                            let mut builder =
                                __arg_0.debug_tuple("UnknownRole");
                            builder.finish()
                        }
                        (&Role::Follower,) => {
                            let mut builder = __arg_0.debug_tuple("Follower");
                            builder.finish()
                        }
                        (&Role::Leader,) => {
                            let mut builder = __arg_0.debug_tuple("Leader");
                            builder.finish()
                        }
                        (&Role::Learner,) => {
                            let mut builder = __arg_0.debug_tuple("Learner");
                            builder.finish()
                        }
                        (&Role::NonParticipant,) => {
                            let mut builder =
                                __arg_0.debug_tuple("NonParticipant");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Role {
                #[inline]
                fn eq(&self, __arg_0: &Role) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for Role {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Role_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl Role {
                    #[doc =
                          "Returns `true` if `value` is a variant of `Role`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            999 => true,
                            0 => true,
                            1 => true,
                            2 => true,
                            3 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `Role`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<Role> {
                        match value {
                            999 =>
                            ::std::option::Option::Some(Role::UnknownRole),
                            0 => ::std::option::Option::Some(Role::Follower),
                            1 => ::std::option::Option::Some(Role::Leader),
                            2 => ::std::option::Option::Some(Role::Learner),
                            3 =>
                            ::std::option::Option::Some(Role::NonParticipant),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for Role {
                    fn default() -> Role { Role::UnknownRole }
                }
                impl ::std::convert::From<Role> for i32 {
                    fn from(value: Role) -> i32 { value as i32 }
                }
            }
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum MemberType {
                UnknownMemberType = 999,
                NonVoter = 0,
                Voter = 1,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for MemberType {
                #[inline]
                fn clone(&self) -> MemberType { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for MemberType { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for MemberType {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&MemberType::UnknownMemberType,) => {
                            let mut builder =
                                __arg_0.debug_tuple("UnknownMemberType");
                            builder.finish()
                        }
                        (&MemberType::NonVoter,) => {
                            let mut builder = __arg_0.debug_tuple("NonVoter");
                            builder.finish()
                        }
                        (&MemberType::Voter,) => {
                            let mut builder = __arg_0.debug_tuple("Voter");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for MemberType {
                #[inline]
                fn eq(&self, __arg_0: &MemberType) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for MemberType {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod MemberType_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl MemberType {
                    #[doc =
                          "Returns `true` if `value` is a variant of `MemberType`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            999 => true,
                            0 => true,
                            1 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `MemberType`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<MemberType> {
                        match value {
                            999 =>
                            ::std::option::Option::Some(MemberType::UnknownMemberType),
                            0 =>
                            ::std::option::Option::Some(MemberType::NonVoter),
                            1 =>
                            ::std::option::Option::Some(MemberType::Voter),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for MemberType {
                    fn default() -> MemberType {
                        MemberType::UnknownMemberType
                    }
                }
                impl ::std::convert::From<MemberType> for i32 {
                    fn from(value: MemberType) -> i32 { value as i32 }
                }
            }
        }
        /// A set of peers, serving a single tablet.
        pub struct RaftConfigPB {
            /// The index of the operation which serialized this RaftConfigPB through
            /// consensus. It is set when the operation is consensus-committed (replicated
            /// to a majority of voters) and before the consensus metadata is updated.
            /// It is left undefined if the operation isn't committed.
            #[prost(int64, optional, tag = "1")]
            pub opid_index: Option<i64>,
            /// Obsolete. This parameter has been retired.
            #[prost(bool, optional, tag = "2")]
            pub obsolete_local: Option<bool>,
            /// Flag to allow unsafe config change operations.
            #[prost(bool, optional, tag = "4")]
            pub unsafe_config_change: Option<bool>,
            /// The set of peers in the configuration.
            #[prost(message, repeated, tag = "3")]
            pub peers: Vec<RaftPeerPB>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for RaftConfigPB {
            #[inline]
            fn clone(&self) -> RaftConfigPB {
                match *self {
                    RaftConfigPB {
                    opid_index: ref __self_0_0,
                    obsolete_local: ref __self_0_1,
                    unsafe_config_change: ref __self_0_2,
                    peers: ref __self_0_3 } =>
                    RaftConfigPB{opid_index:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 obsolete_local:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),
                                 unsafe_config_change:
                                     ::std::clone::Clone::clone(&(*__self_0_2)),
                                 peers:
                                     ::std::clone::Clone::clone(&(*__self_0_3)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for RaftConfigPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RaftConfigPB {
                    opid_index: ref __self_0_0,
                    obsolete_local: ref __self_0_1,
                    unsafe_config_change: ref __self_0_2,
                    peers: ref __self_0_3 } => {
                        let mut builder =
                            __arg_0.debug_struct("RaftConfigPB");
                        let _ = builder.field("opid_index", &&(*__self_0_0));
                        let _ =
                            builder.field("obsolete_local", &&(*__self_0_1));
                        let _ =
                            builder.field("unsafe_config_change",
                                          &&(*__self_0_2));
                        let _ = builder.field("peers", &&(*__self_0_3));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for RaftConfigPB {
            #[inline]
            fn eq(&self, __arg_0: &RaftConfigPB) -> bool {
                match *__arg_0 {
                    RaftConfigPB {
                    opid_index: ref __self_1_0,
                    obsolete_local: ref __self_1_1,
                    unsafe_config_change: ref __self_1_2,
                    peers: ref __self_1_3 } =>
                    match *self {
                        RaftConfigPB {
                        opid_index: ref __self_0_0,
                        obsolete_local: ref __self_0_1,
                        unsafe_config_change: ref __self_0_2,
                        peers: ref __self_0_3 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &RaftConfigPB) -> bool {
                match *__arg_0 {
                    RaftConfigPB {
                    opid_index: ref __self_1_0,
                    obsolete_local: ref __self_1_1,
                    unsafe_config_change: ref __self_1_2,
                    peers: ref __self_1_3 } =>
                    match *self {
                        RaftConfigPB {
                        opid_index: ref __self_0_0,
                        obsolete_local: ref __self_0_1,
                        unsafe_config_change: ref __self_0_2,
                        peers: ref __self_0_3 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod RaftConfigPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for RaftConfigPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.opid_index {
                        _prost::encoding::int64::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.obsolete_local {
                        _prost::encoding::bool::encode(2u32, value, buf);
                    }
                    for msg in &self.peers {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.unsafe_config_change {
                        _prost::encoding::bool::encode(4u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.opid_index.is_none() {
                                self.opid_index = Some(Default::default());
                            }
                            match self.opid_index {
                                Some(ref mut value) =>
                                _prost::encoding::int64::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       48u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftConfigPB",
                                                     "opid_index");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.obsolete_local.is_none() {
                                self.obsolete_local =
                                    Some(Default::default());
                            }
                            match self.obsolete_local {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       48u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftConfigPB",
                                                     "obsolete_local");
                                          error
                                      }),
                        3u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.peers,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("RaftConfigPB",
                                                                                                  "peers");
                                                                                       error
                                                                                   }),
                        4u32 =>
                        {
                            if self.unsafe_config_change.is_none() {
                                self.unsafe_config_change =
                                    Some(Default::default());
                            }
                            match self.unsafe_config_change {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       48u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("RaftConfigPB",
                                                     "unsafe_config_change");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.opid_index.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::int64::encoded_len(1u32,
                                                                                                 value))
                        +
                        self.obsolete_local.as_ref().map_or(0,
                                                            |value|
                                                                _prost::encoding::bool::encoded_len(2u32,
                                                                                                    value))
                        +
                        _prost::encoding::message::encoded_len_repeated(3u32,
                                                                        &self.peers)
                        +
                        self.unsafe_config_change.as_ref().map_or(0,
                                                                  |value|
                                                                      _prost::encoding::bool::encoded_len(4u32,
                                                                                                          value))
                }
            }
            impl Default for RaftConfigPB {
                fn default() -> RaftConfigPB {
                    RaftConfigPB{opid_index: ::std::option::Option::None,
                                 obsolete_local: ::std::option::Option::None,
                                 peers: ::std::default::Default::default(),
                                 unsafe_config_change:
                                     ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl RaftConfigPB {
                pub fn opid_index(&self) -> i64 {
                    match self.opid_index {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0i64,
                    }
                }
                pub fn obsolete_local(&self) -> bool {
                    match self.obsolete_local {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn unsafe_config_change(&self) -> bool {
                    match self.unsafe_config_change {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        /// Represents a snapshot of a configuration at a given moment in time.
        pub struct ConsensusStatePB {
            /// A configuration is always guaranteed to have a known term.
            #[prost(int64, required, tag = "1")]
            pub current_term: i64,
            /// There may not always be a leader of a configuration at any given time.
            ///
            /// The node that the local peer considers to be leader changes based on rules
            /// defined in the Raft specification. Roughly, this corresponds either to
            /// being elected leader (in the case that the local peer is the leader), or
            /// when an update is accepted from another node, which basically just amounts
            /// to a term check on the UpdateConsensus() RPC request.
            ///
            /// Whenever the local peer sees a new term, the leader flag is cleared until
            /// a new leader is acknowledged based on the above critera. Simply casting a
            /// vote for a peer is not sufficient to assume that that peer has won the
            /// election, so we do not update this field based on our vote.
            ///
            /// The leader may be a part of the committed or the pending configuration (or both).
            #[prost(string, optional, tag = "2")]
            pub leader_uuid: Option<String>,
            /// The committed peers. Initial peership is set on tablet start, so this
            /// field should always be present.
            #[prost(message, required, tag = "3")]
            pub committed_config: RaftConfigPB,
            /// The peers in the pending configuration, if there is one.
            #[prost(message, optional, tag = "4")]
            pub pending_config: Option<RaftConfigPB>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ConsensusStatePB {
            #[inline]
            fn clone(&self) -> ConsensusStatePB {
                match *self {
                    ConsensusStatePB {
                    current_term: ref __self_0_0,
                    leader_uuid: ref __self_0_1,
                    committed_config: ref __self_0_2,
                    pending_config: ref __self_0_3 } =>
                    ConsensusStatePB{current_term:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     leader_uuid:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),
                                     committed_config:
                                         ::std::clone::Clone::clone(&(*__self_0_2)),
                                     pending_config:
                                         ::std::clone::Clone::clone(&(*__self_0_3)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ConsensusStatePB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ConsensusStatePB {
                    current_term: ref __self_0_0,
                    leader_uuid: ref __self_0_1,
                    committed_config: ref __self_0_2,
                    pending_config: ref __self_0_3 } => {
                        let mut builder =
                            __arg_0.debug_struct("ConsensusStatePB");
                        let _ =
                            builder.field("current_term", &&(*__self_0_0));
                        let _ = builder.field("leader_uuid", &&(*__self_0_1));
                        let _ =
                            builder.field("committed_config",
                                          &&(*__self_0_2));
                        let _ =
                            builder.field("pending_config", &&(*__self_0_3));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ConsensusStatePB {
            #[inline]
            fn eq(&self, __arg_0: &ConsensusStatePB) -> bool {
                match *__arg_0 {
                    ConsensusStatePB {
                    current_term: ref __self_1_0,
                    leader_uuid: ref __self_1_1,
                    committed_config: ref __self_1_2,
                    pending_config: ref __self_1_3 } =>
                    match *self {
                        ConsensusStatePB {
                        current_term: ref __self_0_0,
                        leader_uuid: ref __self_0_1,
                        committed_config: ref __self_0_2,
                        pending_config: ref __self_0_3 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ConsensusStatePB) -> bool {
                match *__arg_0 {
                    ConsensusStatePB {
                    current_term: ref __self_1_0,
                    leader_uuid: ref __self_1_1,
                    committed_config: ref __self_1_2,
                    pending_config: ref __self_1_3 } =>
                    match *self {
                        ConsensusStatePB {
                        current_term: ref __self_0_0,
                        leader_uuid: ref __self_0_1,
                        committed_config: ref __self_0_2,
                        pending_config: ref __self_0_3 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ConsensusStatePB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ConsensusStatePB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    _prost::encoding::int64::encode(1u32, &self.current_term,
                                                    buf);
                    if let ::std::option::Option::Some(ref value) =
                           self.leader_uuid {
                        _prost::encoding::string::encode(2u32, value, buf);
                    }
                    _prost::encoding::message::encode(3u32,
                                                      &self.committed_config,
                                                      buf);
                    if let Some(ref msg) = self.pending_config {
                        _prost::encoding::message::encode(4u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::int64::merge(wire_type,
                                                       &mut self.current_term,
                                                       buf).map_err(|mut error|
                                                                        {
                                                                            error.push("ConsensusStatePB",
                                                                                       "current_term");
                                                                            error
                                                                        }),
                        2u32 =>
                        {
                            if self.leader_uuid.is_none() {
                                self.leader_uuid = Some(Default::default());
                            }
                            match self.leader_uuid {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       67u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ConsensusStatePB",
                                                     "leader_uuid");
                                          error
                                      }),
                        3u32 =>
                        _prost::encoding::message::merge(wire_type,
                                                         &mut self.committed_config,
                                                         buf).map_err(|mut error|
                                                                          {
                                                                              error.push("ConsensusStatePB",
                                                                                         "committed_config");
                                                                              error
                                                                          }),
                        4u32 =>
                        {
                            if self.pending_config.is_none() {
                                self.pending_config =
                                    Some(Default::default());
                            }
                            match self.pending_config {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       67u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ConsensusStatePB",
                                                     "pending_config");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::int64::encoded_len(1u32,
                                                             &self.current_term)
                        +
                        self.leader_uuid.as_ref().map_or(0,
                                                         |value|
                                                             _prost::encoding::string::encoded_len(2u32,
                                                                                                   value))
                        +
                        _prost::encoding::message::encoded_len(3u32,
                                                               &self.committed_config)
                        +
                        self.pending_config.as_ref().map_or(0,
                                                            |msg|
                                                                _prost::encoding::message::encoded_len(4u32,
                                                                                                       msg))
                }
            }
            impl Default for ConsensusStatePB {
                fn default() -> ConsensusStatePB {
                    ConsensusStatePB{current_term: 0i64,
                                     leader_uuid: ::std::option::Option::None,
                                     committed_config:
                                         ::std::default::Default::default(),
                                     pending_config:
                                         ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl ConsensusStatePB {
                pub fn leader_uuid(&self) -> &str {
                    match self.leader_uuid {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// This PB is used to serialize all of the persistent state needed for
        /// Consensus that is not in the WAL, such as leader election and
        /// communication on startup.
        pub struct ConsensusMetadataPB {
            /// Last-committed peership.
            #[prost(message, required, tag = "1")]
            pub committed_config: RaftConfigPB,
            /// Latest term this server has seen.
            /// When a configuration is first created, initialized to 0.
            ///
            /// Whenever a new election is started, the candidate increments this by one
            /// and requests votes from peers.
            ///
            /// If any RPC or RPC response is received from another node containing a term higher
            /// than this one, the server should step down to FOLLOWER and set its current_term to
            /// match the caller's term.
            ///
            /// If a follower receives an UpdateConsensus RPC with a term lower than this
            /// term, then that implies that the RPC is coming from a former LEADER who has
            /// not realized yet that its term is over. In that case, we will reject the
            /// UpdateConsensus() call with ConsensusErrorPB::INVALID_TERM.
            ///
            /// If a follower receives a RequestConsensusVote() RPC with an earlier term,
            /// the vote is denied.
            #[prost(int64, required, tag = "2")]
            pub current_term: i64,
            /// Permanent UUID of the candidate voted for in 'current_term', or not present
            /// if no vote was made in the current term.
            #[prost(string, optional, tag = "3")]
            pub voted_for: Option<String>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ConsensusMetadataPB {
            #[inline]
            fn clone(&self) -> ConsensusMetadataPB {
                match *self {
                    ConsensusMetadataPB {
                    committed_config: ref __self_0_0,
                    current_term: ref __self_0_1,
                    voted_for: ref __self_0_2 } =>
                    ConsensusMetadataPB{committed_config:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        current_term:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        voted_for:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ConsensusMetadataPB {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ConsensusMetadataPB {
                    committed_config: ref __self_0_0,
                    current_term: ref __self_0_1,
                    voted_for: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("ConsensusMetadataPB");
                        let _ =
                            builder.field("committed_config",
                                          &&(*__self_0_0));
                        let _ =
                            builder.field("current_term", &&(*__self_0_1));
                        let _ = builder.field("voted_for", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ConsensusMetadataPB {
            #[inline]
            fn eq(&self, __arg_0: &ConsensusMetadataPB) -> bool {
                match *__arg_0 {
                    ConsensusMetadataPB {
                    committed_config: ref __self_1_0,
                    current_term: ref __self_1_1,
                    voted_for: ref __self_1_2 } =>
                    match *self {
                        ConsensusMetadataPB {
                        committed_config: ref __self_0_0,
                        current_term: ref __self_0_1,
                        voted_for: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ConsensusMetadataPB) -> bool {
                match *__arg_0 {
                    ConsensusMetadataPB {
                    committed_config: ref __self_1_0,
                    current_term: ref __self_1_1,
                    voted_for: ref __self_1_2 } =>
                    match *self {
                        ConsensusMetadataPB {
                        committed_config: ref __self_0_0,
                        current_term: ref __self_0_1,
                        voted_for: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ConsensusMetadataPB_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ConsensusMetadataPB {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    _prost::encoding::message::encode(1u32,
                                                      &self.committed_config,
                                                      buf);
                    _prost::encoding::int64::encode(2u32, &self.current_term,
                                                    buf);
                    if let ::std::option::Option::Some(ref value) =
                           self.voted_for {
                        _prost::encoding::string::encode(3u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge(wire_type,
                                                         &mut self.committed_config,
                                                         buf).map_err(|mut error|
                                                                          {
                                                                              error.push("ConsensusMetadataPB",
                                                                                         "committed_config");
                                                                              error
                                                                          }),
                        2u32 =>
                        _prost::encoding::int64::merge(wire_type,
                                                       &mut self.current_term,
                                                       buf).map_err(|mut error|
                                                                        {
                                                                            error.push("ConsensusMetadataPB",
                                                                                       "current_term");
                                                                            error
                                                                        }),
                        3u32 =>
                        {
                            if self.voted_for.is_none() {
                                self.voted_for = Some(Default::default());
                            }
                            match self.voted_for {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/consensus.rs",
                                                                       99u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ConsensusMetadataPB",
                                                     "voted_for");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len(1u32,
                                                               &self.committed_config)
                        +
                        _prost::encoding::int64::encoded_len(2u32,
                                                             &self.current_term)
                        +
                        self.voted_for.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::string::encoded_len(3u32,
                                                                                                 value))
                }
            }
            impl Default for ConsensusMetadataPB {
                fn default() -> ConsensusMetadataPB {
                    ConsensusMetadataPB{committed_config:
                                            ::std::default::Default::default(),
                                        current_term: 0i64,
                                        voted_for:
                                            ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl ConsensusMetadataPB {
                pub fn voted_for(&self) -> &str {
                    match self.voted_for {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// An id for a generic state machine operation. Composed of the leaders' term
        /// plus the index of the operation in that term, e.g., the <index>th operation
        /// of the <term>th leader.
        pub struct OpId {
            /// The term of an operation or the leader's sequence id.
            #[prost(int64, required, tag = "1")]
            pub term: i64,
            #[prost(int64, required, tag = "2")]
            pub index: i64,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for OpId {
            #[inline]
            fn clone(&self) -> OpId {
                match *self {
                    OpId { term: ref __self_0_0, index: ref __self_0_1 } =>
                    OpId{term: ::std::clone::Clone::clone(&(*__self_0_0)),
                         index: ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for OpId {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    OpId { term: ref __self_0_0, index: ref __self_0_1 } => {
                        let mut builder = __arg_0.debug_struct("OpId");
                        let _ = builder.field("term", &&(*__self_0_0));
                        let _ = builder.field("index", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for OpId {
            #[inline]
            fn eq(&self, __arg_0: &OpId) -> bool {
                match *__arg_0 {
                    OpId { term: ref __self_1_0, index: ref __self_1_1 } =>
                    match *self {
                        OpId { term: ref __self_0_0, index: ref __self_0_1 }
                        =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &OpId) -> bool {
                match *__arg_0 {
                    OpId { term: ref __self_1_0, index: ref __self_1_1 } =>
                    match *self {
                        OpId { term: ref __self_0_0, index: ref __self_0_1 }
                        =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod OpId_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for OpId {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    _prost::encoding::int64::encode(1u32, &self.term, buf);
                    _prost::encoding::int64::encode(2u32, &self.index, buf);
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::int64::merge(wire_type,
                                                       &mut self.term,
                                                       buf).map_err(|mut error|
                                                                        {
                                                                            error.push("OpId",
                                                                                       "term");
                                                                            error
                                                                        }),
                        2u32 =>
                        _prost::encoding::int64::merge(wire_type,
                                                       &mut self.index,
                                                       buf).map_err(|mut error|
                                                                        {
                                                                            error.push("OpId",
                                                                                       "index");
                                                                            error
                                                                        }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + _prost::encoding::int64::encoded_len(1u32, &self.term)
                        +
                        _prost::encoding::int64::encoded_len(2u32,
                                                             &self.index)
                }
            }
            impl Default for OpId {
                fn default() -> OpId { OpId{term: 0i64, index: 0i64,} }
            }
        }
    }
    pub mod master { }
    pub mod rpc { }
    pub mod tablet { }
    pub mod security { }
    pub mod tserver { }
}
pub mod google {
    pub mod protobuf {
        /// The protocol compiler can output a FileDescriptorSet containing the .proto
        /// files it parses.
        pub struct FileDescriptorSet {
            #[prost(message, repeated, tag = "1")]
            pub file: Vec<FileDescriptorProto>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for FileDescriptorSet {
            #[inline]
            fn clone(&self) -> FileDescriptorSet {
                match *self {
                    FileDescriptorSet { file: ref __self_0_0 } =>
                    FileDescriptorSet{file:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for FileDescriptorSet {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FileDescriptorSet { file: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("FileDescriptorSet");
                        let _ = builder.field("file", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for FileDescriptorSet {
            #[inline]
            fn eq(&self, __arg_0: &FileDescriptorSet) -> bool {
                match *__arg_0 {
                    FileDescriptorSet { file: ref __self_1_0 } =>
                    match *self {
                        FileDescriptorSet { file: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &FileDescriptorSet) -> bool {
                match *__arg_0 {
                    FileDescriptorSet { file: ref __self_1_0 } =>
                    match *self {
                        FileDescriptorSet { file: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod FileDescriptorSet_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for FileDescriptorSet {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.file {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.file,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileDescriptorSet",
                                                                                                  "file");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(1u32,
                                                                        &self.file)
                }
            }
            impl Default for FileDescriptorSet {
                fn default() -> FileDescriptorSet {
                    FileDescriptorSet{file:
                                          ::std::default::Default::default(),}
                }
            }
        }
        /// Describes a complete .proto file.
        pub struct FileDescriptorProto {
            /// file name, relative to root of source tree
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            /// e.g. "foo", "foo.bar", etc.
            #[prost(string, optional, tag = "2")]
            pub package: Option<String>,
            /// Names of files imported by this file.
            #[prost(string, repeated, tag = "3")]
            pub dependency: Vec<String>,
            /// Indexes of the public imported files in the dependency list above.
            #[prost(int32, repeated, packed = "false", tag = "10")]
            pub public_dependency: Vec<i32>,
            /// Indexes of the weak imported files in the dependency list.
            /// For Google-internal migration only. Do not use.
            #[prost(int32, repeated, packed = "false", tag = "11")]
            pub weak_dependency: Vec<i32>,
            /// All top-level definitions in this file.
            #[prost(message, repeated, tag = "4")]
            pub message_type: Vec<DescriptorProto>,
            #[prost(message, repeated, tag = "5")]
            pub enum_type: Vec<EnumDescriptorProto>,
            #[prost(message, repeated, tag = "6")]
            pub service: Vec<ServiceDescriptorProto>,
            #[prost(message, repeated, tag = "7")]
            pub extension: Vec<FieldDescriptorProto>,
            #[prost(message, optional, tag = "8")]
            pub options: Option<FileOptions>,
            /// This field contains optional information about the original source code.
            /// You may safely remove this entire field without harming runtime
            /// functionality of the descriptors -- the information is needed only by
            /// development tools.
            #[prost(message, optional, tag = "9")]
            pub source_code_info: Option<SourceCodeInfo>,
            /// The syntax of the proto file.
            /// The supported values are "proto2" and "proto3".
            #[prost(string, optional, tag = "12")]
            pub syntax: Option<String>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for FileDescriptorProto {
            #[inline]
            fn clone(&self) -> FileDescriptorProto {
                match *self {
                    FileDescriptorProto {
                    name: ref __self_0_0,
                    package: ref __self_0_1,
                    dependency: ref __self_0_2,
                    public_dependency: ref __self_0_3,
                    weak_dependency: ref __self_0_4,
                    message_type: ref __self_0_5,
                    enum_type: ref __self_0_6,
                    service: ref __self_0_7,
                    extension: ref __self_0_8,
                    options: ref __self_0_9,
                    source_code_info: ref __self_0_10,
                    syntax: ref __self_0_11 } =>
                    FileDescriptorProto{name:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        package:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        dependency:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),
                                        public_dependency:
                                            ::std::clone::Clone::clone(&(*__self_0_3)),
                                        weak_dependency:
                                            ::std::clone::Clone::clone(&(*__self_0_4)),
                                        message_type:
                                            ::std::clone::Clone::clone(&(*__self_0_5)),
                                        enum_type:
                                            ::std::clone::Clone::clone(&(*__self_0_6)),
                                        service:
                                            ::std::clone::Clone::clone(&(*__self_0_7)),
                                        extension:
                                            ::std::clone::Clone::clone(&(*__self_0_8)),
                                        options:
                                            ::std::clone::Clone::clone(&(*__self_0_9)),
                                        source_code_info:
                                            ::std::clone::Clone::clone(&(*__self_0_10)),
                                        syntax:
                                            ::std::clone::Clone::clone(&(*__self_0_11)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for FileDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FileDescriptorProto {
                    name: ref __self_0_0,
                    package: ref __self_0_1,
                    dependency: ref __self_0_2,
                    public_dependency: ref __self_0_3,
                    weak_dependency: ref __self_0_4,
                    message_type: ref __self_0_5,
                    enum_type: ref __self_0_6,
                    service: ref __self_0_7,
                    extension: ref __self_0_8,
                    options: ref __self_0_9,
                    source_code_info: ref __self_0_10,
                    syntax: ref __self_0_11 } => {
                        let mut builder =
                            __arg_0.debug_struct("FileDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("package", &&(*__self_0_1));
                        let _ = builder.field("dependency", &&(*__self_0_2));
                        let _ =
                            builder.field("public_dependency",
                                          &&(*__self_0_3));
                        let _ =
                            builder.field("weak_dependency", &&(*__self_0_4));
                        let _ =
                            builder.field("message_type", &&(*__self_0_5));
                        let _ = builder.field("enum_type", &&(*__self_0_6));
                        let _ = builder.field("service", &&(*__self_0_7));
                        let _ = builder.field("extension", &&(*__self_0_8));
                        let _ = builder.field("options", &&(*__self_0_9));
                        let _ =
                            builder.field("source_code_info",
                                          &&(*__self_0_10));
                        let _ = builder.field("syntax", &&(*__self_0_11));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for FileDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &FileDescriptorProto) -> bool {
                match *__arg_0 {
                    FileDescriptorProto {
                    name: ref __self_1_0,
                    package: ref __self_1_1,
                    dependency: ref __self_1_2,
                    public_dependency: ref __self_1_3,
                    weak_dependency: ref __self_1_4,
                    message_type: ref __self_1_5,
                    enum_type: ref __self_1_6,
                    service: ref __self_1_7,
                    extension: ref __self_1_8,
                    options: ref __self_1_9,
                    source_code_info: ref __self_1_10,
                    syntax: ref __self_1_11 } =>
                    match *self {
                        FileDescriptorProto {
                        name: ref __self_0_0,
                        package: ref __self_0_1,
                        dependency: ref __self_0_2,
                        public_dependency: ref __self_0_3,
                        weak_dependency: ref __self_0_4,
                        message_type: ref __self_0_5,
                        enum_type: ref __self_0_6,
                        service: ref __self_0_7,
                        extension: ref __self_0_8,
                        options: ref __self_0_9,
                        source_code_info: ref __self_0_10,
                        syntax: ref __self_0_11 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6) &&
                            (*__self_0_7) == (*__self_1_7) &&
                            (*__self_0_8) == (*__self_1_8) &&
                            (*__self_0_9) == (*__self_1_9) &&
                            (*__self_0_10) == (*__self_1_10) &&
                            (*__self_0_11) == (*__self_1_11),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &FileDescriptorProto) -> bool {
                match *__arg_0 {
                    FileDescriptorProto {
                    name: ref __self_1_0,
                    package: ref __self_1_1,
                    dependency: ref __self_1_2,
                    public_dependency: ref __self_1_3,
                    weak_dependency: ref __self_1_4,
                    message_type: ref __self_1_5,
                    enum_type: ref __self_1_6,
                    service: ref __self_1_7,
                    extension: ref __self_1_8,
                    options: ref __self_1_9,
                    source_code_info: ref __self_1_10,
                    syntax: ref __self_1_11 } =>
                    match *self {
                        FileDescriptorProto {
                        name: ref __self_0_0,
                        package: ref __self_0_1,
                        dependency: ref __self_0_2,
                        public_dependency: ref __self_0_3,
                        weak_dependency: ref __self_0_4,
                        message_type: ref __self_0_5,
                        enum_type: ref __self_0_6,
                        service: ref __self_0_7,
                        extension: ref __self_0_8,
                        options: ref __self_0_9,
                        source_code_info: ref __self_0_10,
                        syntax: ref __self_0_11 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6) ||
                            (*__self_0_7) != (*__self_1_7) ||
                            (*__self_0_8) != (*__self_1_8) ||
                            (*__self_0_9) != (*__self_1_9) ||
                            (*__self_0_10) != (*__self_1_10) ||
                            (*__self_0_11) != (*__self_1_11),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod FileDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for FileDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.package {
                        _prost::encoding::string::encode(2u32, value, buf);
                    }
                    _prost::encoding::string::encode_repeated(3u32,
                                                              &self.dependency,
                                                              buf);
                    for msg in &self.message_type {
                        _prost::encoding::message::encode(4u32, msg, buf);
                    }
                    for msg in &self.enum_type {
                        _prost::encoding::message::encode(5u32, msg, buf);
                    }
                    for msg in &self.service {
                        _prost::encoding::message::encode(6u32, msg, buf);
                    }
                    for msg in &self.extension {
                        _prost::encoding::message::encode(7u32, msg, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(8u32, msg, buf);
                    }
                    if let Some(ref msg) = self.source_code_info {
                        _prost::encoding::message::encode(9u32, msg, buf);
                    }
                    _prost::encoding::int32::encode_repeated(10u32,
                                                             &self.public_dependency,
                                                             buf);
                    _prost::encoding::int32::encode_repeated(11u32,
                                                             &self.weak_dependency,
                                                             buf);
                    if let ::std::option::Option::Some(ref value) =
                           self.syntax {
                        _prost::encoding::string::encode(12u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       9u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.package.is_none() {
                                self.package = Some(Default::default());
                            }
                            match self.package {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       9u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileDescriptorProto",
                                                     "package");
                                          error
                                      }),
                        3u32 =>
                        _prost::encoding::string::merge_repeated(wire_type,
                                                                 &mut self.dependency,
                                                                 buf).map_err(|mut error|
                                                                                  {
                                                                                      error.push("FileDescriptorProto",
                                                                                                 "dependency");
                                                                                      error
                                                                                  }),
                        4u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.message_type,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileDescriptorProto",
                                                                                                  "message_type");
                                                                                       error
                                                                                   }),
                        5u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.enum_type,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileDescriptorProto",
                                                                                                  "enum_type");
                                                                                       error
                                                                                   }),
                        6u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.service,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileDescriptorProto",
                                                                                                  "service");
                                                                                       error
                                                                                   }),
                        7u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.extension,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileDescriptorProto",
                                                                                                  "extension");
                                                                                       error
                                                                                   }),
                        8u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       9u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        9u32 =>
                        {
                            if self.source_code_info.is_none() {
                                self.source_code_info =
                                    Some(Default::default());
                            }
                            match self.source_code_info {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       9u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileDescriptorProto",
                                                     "source_code_info");
                                          error
                                      }),
                        10u32 =>
                        _prost::encoding::int32::merge_repeated(wire_type,
                                                                &mut self.public_dependency,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("FileDescriptorProto",
                                                                                                "public_dependency");
                                                                                     error
                                                                                 }),
                        11u32 =>
                        _prost::encoding::int32::merge_repeated(wire_type,
                                                                &mut self.weak_dependency,
                                                                buf).map_err(|mut error|
                                                                                 {
                                                                                     error.push("FileDescriptorProto",
                                                                                                "weak_dependency");
                                                                                     error
                                                                                 }),
                        12u32 =>
                        {
                            if self.syntax.is_none() {
                                self.syntax = Some(Default::default());
                            }
                            match self.syntax {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       9u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileDescriptorProto",
                                                     "syntax");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        self.package.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::string::encoded_len(2u32,
                                                                                               value))
                        +
                        _prost::encoding::string::encoded_len_repeated(3u32,
                                                                       &self.dependency)
                        +
                        _prost::encoding::message::encoded_len_repeated(4u32,
                                                                        &self.message_type)
                        +
                        _prost::encoding::message::encoded_len_repeated(5u32,
                                                                        &self.enum_type)
                        +
                        _prost::encoding::message::encoded_len_repeated(6u32,
                                                                        &self.service)
                        +
                        _prost::encoding::message::encoded_len_repeated(7u32,
                                                                        &self.extension)
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(8u32,
                                                                                                msg))
                        +
                        self.source_code_info.as_ref().map_or(0,
                                                              |msg|
                                                                  _prost::encoding::message::encoded_len(9u32,
                                                                                                         msg))
                        +
                        _prost::encoding::int32::encoded_len_repeated(10u32,
                                                                      &self.public_dependency)
                        +
                        _prost::encoding::int32::encoded_len_repeated(11u32,
                                                                      &self.weak_dependency)
                        +
                        self.syntax.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::string::encoded_len(12u32,
                                                                                              value))
                }
            }
            impl Default for FileDescriptorProto {
                fn default() -> FileDescriptorProto {
                    FileDescriptorProto{name: ::std::option::Option::None,
                                        package: ::std::option::Option::None,
                                        dependency: ::std::vec::Vec::new(),
                                        message_type:
                                            ::std::default::Default::default(),
                                        enum_type:
                                            ::std::default::Default::default(),
                                        service:
                                            ::std::default::Default::default(),
                                        extension:
                                            ::std::default::Default::default(),
                                        options:
                                            ::std::default::Default::default(),
                                        source_code_info:
                                            ::std::default::Default::default(),
                                        public_dependency:
                                            ::std::vec::Vec::new(),
                                        weak_dependency:
                                            ::std::vec::Vec::new(),
                                        syntax: ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl FileDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn package(&self) -> &str {
                    match self.package {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn syntax(&self) -> &str {
                    match self.syntax {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// Describes a message type.
        pub struct DescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(message, repeated, tag = "2")]
            pub field: Vec<FieldDescriptorProto>,
            #[prost(message, repeated, tag = "6")]
            pub extension: Vec<FieldDescriptorProto>,
            #[prost(message, repeated, tag = "3")]
            pub nested_type: Vec<DescriptorProto>,
            #[prost(message, repeated, tag = "4")]
            pub enum_type: Vec<EnumDescriptorProto>,
            #[prost(message, repeated, tag = "5")]
            pub extension_range: Vec<descriptor_proto::ExtensionRange>,
            #[prost(message, repeated, tag = "8")]
            pub oneof_decl: Vec<OneofDescriptorProto>,
            #[prost(message, optional, tag = "7")]
            pub options: Option<MessageOptions>,
            #[prost(message, repeated, tag = "9")]
            pub reserved_range: Vec<descriptor_proto::ReservedRange>,
            /// Reserved field names, which may not be used by fields in the same message.
            /// A given name may only be reserved once.
            #[prost(string, repeated, tag = "10")]
            pub reserved_name: Vec<String>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for DescriptorProto {
            #[inline]
            fn clone(&self) -> DescriptorProto {
                match *self {
                    DescriptorProto {
                    name: ref __self_0_0,
                    field: ref __self_0_1,
                    extension: ref __self_0_2,
                    nested_type: ref __self_0_3,
                    enum_type: ref __self_0_4,
                    extension_range: ref __self_0_5,
                    oneof_decl: ref __self_0_6,
                    options: ref __self_0_7,
                    reserved_range: ref __self_0_8,
                    reserved_name: ref __self_0_9 } =>
                    DescriptorProto{name:
                                        ::std::clone::Clone::clone(&(*__self_0_0)),
                                    field:
                                        ::std::clone::Clone::clone(&(*__self_0_1)),
                                    extension:
                                        ::std::clone::Clone::clone(&(*__self_0_2)),
                                    nested_type:
                                        ::std::clone::Clone::clone(&(*__self_0_3)),
                                    enum_type:
                                        ::std::clone::Clone::clone(&(*__self_0_4)),
                                    extension_range:
                                        ::std::clone::Clone::clone(&(*__self_0_5)),
                                    oneof_decl:
                                        ::std::clone::Clone::clone(&(*__self_0_6)),
                                    options:
                                        ::std::clone::Clone::clone(&(*__self_0_7)),
                                    reserved_range:
                                        ::std::clone::Clone::clone(&(*__self_0_8)),
                                    reserved_name:
                                        ::std::clone::Clone::clone(&(*__self_0_9)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for DescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    DescriptorProto {
                    name: ref __self_0_0,
                    field: ref __self_0_1,
                    extension: ref __self_0_2,
                    nested_type: ref __self_0_3,
                    enum_type: ref __self_0_4,
                    extension_range: ref __self_0_5,
                    oneof_decl: ref __self_0_6,
                    options: ref __self_0_7,
                    reserved_range: ref __self_0_8,
                    reserved_name: ref __self_0_9 } => {
                        let mut builder =
                            __arg_0.debug_struct("DescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("field", &&(*__self_0_1));
                        let _ = builder.field("extension", &&(*__self_0_2));
                        let _ = builder.field("nested_type", &&(*__self_0_3));
                        let _ = builder.field("enum_type", &&(*__self_0_4));
                        let _ =
                            builder.field("extension_range", &&(*__self_0_5));
                        let _ = builder.field("oneof_decl", &&(*__self_0_6));
                        let _ = builder.field("options", &&(*__self_0_7));
                        let _ =
                            builder.field("reserved_range", &&(*__self_0_8));
                        let _ =
                            builder.field("reserved_name", &&(*__self_0_9));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for DescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &DescriptorProto) -> bool {
                match *__arg_0 {
                    DescriptorProto {
                    name: ref __self_1_0,
                    field: ref __self_1_1,
                    extension: ref __self_1_2,
                    nested_type: ref __self_1_3,
                    enum_type: ref __self_1_4,
                    extension_range: ref __self_1_5,
                    oneof_decl: ref __self_1_6,
                    options: ref __self_1_7,
                    reserved_range: ref __self_1_8,
                    reserved_name: ref __self_1_9 } =>
                    match *self {
                        DescriptorProto {
                        name: ref __self_0_0,
                        field: ref __self_0_1,
                        extension: ref __self_0_2,
                        nested_type: ref __self_0_3,
                        enum_type: ref __self_0_4,
                        extension_range: ref __self_0_5,
                        oneof_decl: ref __self_0_6,
                        options: ref __self_0_7,
                        reserved_range: ref __self_0_8,
                        reserved_name: ref __self_0_9 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6) &&
                            (*__self_0_7) == (*__self_1_7) &&
                            (*__self_0_8) == (*__self_1_8) &&
                            (*__self_0_9) == (*__self_1_9),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &DescriptorProto) -> bool {
                match *__arg_0 {
                    DescriptorProto {
                    name: ref __self_1_0,
                    field: ref __self_1_1,
                    extension: ref __self_1_2,
                    nested_type: ref __self_1_3,
                    enum_type: ref __self_1_4,
                    extension_range: ref __self_1_5,
                    oneof_decl: ref __self_1_6,
                    options: ref __self_1_7,
                    reserved_range: ref __self_1_8,
                    reserved_name: ref __self_1_9 } =>
                    match *self {
                        DescriptorProto {
                        name: ref __self_0_0,
                        field: ref __self_0_1,
                        extension: ref __self_0_2,
                        nested_type: ref __self_0_3,
                        enum_type: ref __self_0_4,
                        extension_range: ref __self_0_5,
                        oneof_decl: ref __self_0_6,
                        options: ref __self_0_7,
                        reserved_range: ref __self_0_8,
                        reserved_name: ref __self_0_9 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6) ||
                            (*__self_0_7) != (*__self_1_7) ||
                            (*__self_0_8) != (*__self_1_8) ||
                            (*__self_0_9) != (*__self_1_9),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod DescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for DescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    for msg in &self.field {
                        _prost::encoding::message::encode(2u32, msg, buf);
                    }
                    for msg in &self.nested_type {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                    for msg in &self.enum_type {
                        _prost::encoding::message::encode(4u32, msg, buf);
                    }
                    for msg in &self.extension_range {
                        _prost::encoding::message::encode(5u32, msg, buf);
                    }
                    for msg in &self.extension {
                        _prost::encoding::message::encode(6u32, msg, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(7u32, msg, buf);
                    }
                    for msg in &self.oneof_decl {
                        _prost::encoding::message::encode(8u32, msg, buf);
                    }
                    for msg in &self.reserved_range {
                        _prost::encoding::message::encode(9u32, msg, buf);
                    }
                    _prost::encoding::string::encode_repeated(10u32,
                                                              &self.reserved_name,
                                                              buf);
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       50u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("DescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.field,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "field");
                                                                                       error
                                                                                   }),
                        3u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.nested_type,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "nested_type");
                                                                                       error
                                                                                   }),
                        4u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.enum_type,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "enum_type");
                                                                                       error
                                                                                   }),
                        5u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.extension_range,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "extension_range");
                                                                                       error
                                                                                   }),
                        6u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.extension,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "extension");
                                                                                       error
                                                                                   }),
                        7u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       50u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("DescriptorProto",
                                                     "options");
                                          error
                                      }),
                        8u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.oneof_decl,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "oneof_decl");
                                                                                       error
                                                                                   }),
                        9u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.reserved_range,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("DescriptorProto",
                                                                                                  "reserved_range");
                                                                                       error
                                                                                   }),
                        10u32 =>
                        _prost::encoding::string::merge_repeated(wire_type,
                                                                 &mut self.reserved_name,
                                                                 buf).map_err(|mut error|
                                                                                  {
                                                                                      error.push("DescriptorProto",
                                                                                                 "reserved_name");
                                                                                      error
                                                                                  }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        _prost::encoding::message::encoded_len_repeated(2u32,
                                                                        &self.field)
                        +
                        _prost::encoding::message::encoded_len_repeated(3u32,
                                                                        &self.nested_type)
                        +
                        _prost::encoding::message::encoded_len_repeated(4u32,
                                                                        &self.enum_type)
                        +
                        _prost::encoding::message::encoded_len_repeated(5u32,
                                                                        &self.extension_range)
                        +
                        _prost::encoding::message::encoded_len_repeated(6u32,
                                                                        &self.extension)
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(7u32,
                                                                                                msg))
                        +
                        _prost::encoding::message::encoded_len_repeated(8u32,
                                                                        &self.oneof_decl)
                        +
                        _prost::encoding::message::encoded_len_repeated(9u32,
                                                                        &self.reserved_range)
                        +
                        _prost::encoding::string::encoded_len_repeated(10u32,
                                                                       &self.reserved_name)
                }
            }
            impl Default for DescriptorProto {
                fn default() -> DescriptorProto {
                    DescriptorProto{name: ::std::option::Option::None,
                                    field: ::std::default::Default::default(),
                                    nested_type:
                                        ::std::default::Default::default(),
                                    enum_type:
                                        ::std::default::Default::default(),
                                    extension_range:
                                        ::std::default::Default::default(),
                                    extension:
                                        ::std::default::Default::default(),
                                    options:
                                        ::std::default::Default::default(),
                                    oneof_decl:
                                        ::std::default::Default::default(),
                                    reserved_range:
                                        ::std::default::Default::default(),
                                    reserved_name: ::std::vec::Vec::new(),}
                }
            }
            #[allow(dead_code)]
            impl DescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        pub mod descriptor_proto {
            pub struct ExtensionRange {
                #[prost(int32, optional, tag = "1")]
                pub start: Option<i32>,
                #[prost(int32, optional, tag = "2")]
                pub end: Option<i32>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for ExtensionRange {
                #[inline]
                fn clone(&self) -> ExtensionRange {
                    match *self {
                        ExtensionRange {
                        start: ref __self_0_0, end: ref __self_0_1 } =>
                        ExtensionRange{start:
                                           ::std::clone::Clone::clone(&(*__self_0_0)),
                                       end:
                                           ::std::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for ExtensionRange {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match *self {
                        ExtensionRange {
                        start: ref __self_0_0, end: ref __self_0_1 } => {
                            let mut builder =
                                __arg_0.debug_struct("ExtensionRange");
                            let _ = builder.field("start", &&(*__self_0_0));
                            let _ = builder.field("end", &&(*__self_0_1));
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for ExtensionRange {
                #[inline]
                fn eq(&self, __arg_0: &ExtensionRange) -> bool {
                    match *__arg_0 {
                        ExtensionRange {
                        start: ref __self_1_0, end: ref __self_1_1 } =>
                        match *self {
                            ExtensionRange {
                            start: ref __self_0_0, end: ref __self_0_1 } =>
                            true && (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &ExtensionRange) -> bool {
                    match *__arg_0 {
                        ExtensionRange {
                        start: ref __self_1_0, end: ref __self_1_1 } =>
                        match *self {
                            ExtensionRange {
                            start: ref __self_0_0, end: ref __self_0_1 } =>
                            false || (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod ExtensionRange_MESSAGE {
                extern crate prost as _prost;
                extern crate bytes as _bytes;
                use super::*;
                impl _prost::Message for ExtensionRange {
                    fn encode_raw<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        if let ::std::option::Option::Some(ref value) =
                               self.start {
                            _prost::encoding::int32::encode(1u32, value, buf);
                        }
                        if let ::std::option::Option::Some(ref value) =
                               self.end {
                            _prost::encoding::int32::encode(2u32, value, buf);
                        }
                    }
                    fn merge_field<B>(&mut self, buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        let (tag, wire_type) =
                            _prost::encoding::decode_key(buf)?;
                        match tag {
                            1u32 =>
                            {
                                if self.start.is_none() {
                                    self.start = Some(Default::default());
                                }
                                match self.start {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           76u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("ExtensionRange",
                                                         "start");
                                              error
                                          }),
                            2u32 =>
                            {
                                if self.end.is_none() {
                                    self.end = Some(Default::default());
                                }
                                match self.end {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           76u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("ExtensionRange",
                                                         "end");
                                              error
                                          }),
                            _ => _prost::encoding::skip_field(wire_type, buf),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 +
                            self.start.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::int32::encoded_len(1u32,
                                                                                                value))
                            +
                            self.end.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::int32::encoded_len(2u32,
                                                                                              value))
                    }
                }
                impl Default for ExtensionRange {
                    fn default() -> ExtensionRange {
                        ExtensionRange{start: ::std::option::Option::None,
                                       end: ::std::option::Option::None,}
                    }
                }
                #[allow(dead_code)]
                impl ExtensionRange {
                    pub fn start(&self) -> i32 {
                        match self.start {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                    pub fn end(&self) -> i32 {
                        match self.end {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                }
            }
            /// Range of reserved tag numbers. Reserved tag numbers may not be used by
            /// fields or extension ranges in the same message. Reserved ranges may
            /// not overlap.
            pub struct ReservedRange {
                /// Inclusive.
                #[prost(int32, optional, tag = "1")]
                pub start: Option<i32>,
                /// Exclusive.
                #[prost(int32, optional, tag = "2")]
                pub end: Option<i32>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for ReservedRange {
                #[inline]
                fn clone(&self) -> ReservedRange {
                    match *self {
                        ReservedRange {
                        start: ref __self_0_0, end: ref __self_0_1 } =>
                        ReservedRange{start:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),
                                      end:
                                          ::std::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for ReservedRange {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match *self {
                        ReservedRange {
                        start: ref __self_0_0, end: ref __self_0_1 } => {
                            let mut builder =
                                __arg_0.debug_struct("ReservedRange");
                            let _ = builder.field("start", &&(*__self_0_0));
                            let _ = builder.field("end", &&(*__self_0_1));
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for ReservedRange {
                #[inline]
                fn eq(&self, __arg_0: &ReservedRange) -> bool {
                    match *__arg_0 {
                        ReservedRange {
                        start: ref __self_1_0, end: ref __self_1_1 } =>
                        match *self {
                            ReservedRange {
                            start: ref __self_0_0, end: ref __self_0_1 } =>
                            true && (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &ReservedRange) -> bool {
                    match *__arg_0 {
                        ReservedRange {
                        start: ref __self_1_0, end: ref __self_1_1 } =>
                        match *self {
                            ReservedRange {
                            start: ref __self_0_0, end: ref __self_0_1 } =>
                            false || (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod ReservedRange_MESSAGE {
                extern crate prost as _prost;
                extern crate bytes as _bytes;
                use super::*;
                impl _prost::Message for ReservedRange {
                    fn encode_raw<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        if let ::std::option::Option::Some(ref value) =
                               self.start {
                            _prost::encoding::int32::encode(1u32, value, buf);
                        }
                        if let ::std::option::Option::Some(ref value) =
                               self.end {
                            _prost::encoding::int32::encode(2u32, value, buf);
                        }
                    }
                    fn merge_field<B>(&mut self, buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        let (tag, wire_type) =
                            _prost::encoding::decode_key(buf)?;
                        match tag {
                            1u32 =>
                            {
                                if self.start.is_none() {
                                    self.start = Some(Default::default());
                                }
                                match self.start {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           86u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("ReservedRange",
                                                         "start");
                                              error
                                          }),
                            2u32 =>
                            {
                                if self.end.is_none() {
                                    self.end = Some(Default::default());
                                }
                                match self.end {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           86u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("ReservedRange",
                                                         "end");
                                              error
                                          }),
                            _ => _prost::encoding::skip_field(wire_type, buf),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 +
                            self.start.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::int32::encoded_len(1u32,
                                                                                                value))
                            +
                            self.end.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::int32::encoded_len(2u32,
                                                                                              value))
                    }
                }
                impl Default for ReservedRange {
                    fn default() -> ReservedRange {
                        ReservedRange{start: ::std::option::Option::None,
                                      end: ::std::option::Option::None,}
                    }
                }
                #[allow(dead_code)]
                impl ReservedRange {
                    pub fn start(&self) -> i32 {
                        match self.start {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                    pub fn end(&self) -> i32 {
                        match self.end {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                }
            }
        }
        /// Describes a field within a message.
        pub struct FieldDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(int32, optional, tag = "3")]
            pub number: Option<i32>,
            #[prost(enumeration = "field_descriptor_proto::Label",
                    optional,
                    tag = "4")]
            pub label: Option<i32>,
            /// If type_name is set, this need not be set.  If both this and type_name
            /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
            #[prost(enumeration = "field_descriptor_proto::Type",
                    optional,
                    tag = "5")]
            pub type_: Option<i32>,
            /// For message and enum types, this is the name of the type.  If the name
            /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
            /// rules are used to find the type (i.e. first the nested types within this
            /// message are searched, then within the parent, on up to the root
            /// namespace).
            #[prost(string, optional, tag = "6")]
            pub type_name: Option<String>,
            /// For extensions, this is the name of the type being extended.  It is
            /// resolved in the same manner as type_name.
            #[prost(string, optional, tag = "2")]
            pub extendee: Option<String>,
            /// For numeric types, contains the original text representation of the value.
            /// For booleans, "true" or "false".
            /// For strings, contains the default text contents (not escaped in any way).
            /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
            /// TODO(kenton):  Base-64 encode?
            #[prost(string, optional, tag = "7")]
            pub default_value: Option<String>,
            /// If set, gives the index of a oneof in the containing type's oneof_decl
            /// list.  This field is a member of that oneof.
            #[prost(int32, optional, tag = "9")]
            pub oneof_index: Option<i32>,
            /// JSON name of this field. The value is set by protocol compiler. If the
            /// user has set a "json_name" option on this field, that option's value
            /// will be used. Otherwise, it's deduced from the field's name by converting
            /// it to camelCase.
            #[prost(string, optional, tag = "10")]
            pub json_name: Option<String>,
            #[prost(message, optional, tag = "8")]
            pub options: Option<FieldOptions>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for FieldDescriptorProto {
            #[inline]
            fn clone(&self) -> FieldDescriptorProto {
                match *self {
                    FieldDescriptorProto {
                    name: ref __self_0_0,
                    number: ref __self_0_1,
                    label: ref __self_0_2,
                    type_: ref __self_0_3,
                    type_name: ref __self_0_4,
                    extendee: ref __self_0_5,
                    default_value: ref __self_0_6,
                    oneof_index: ref __self_0_7,
                    json_name: ref __self_0_8,
                    options: ref __self_0_9 } =>
                    FieldDescriptorProto{name:
                                             ::std::clone::Clone::clone(&(*__self_0_0)),
                                         number:
                                             ::std::clone::Clone::clone(&(*__self_0_1)),
                                         label:
                                             ::std::clone::Clone::clone(&(*__self_0_2)),
                                         type_:
                                             ::std::clone::Clone::clone(&(*__self_0_3)),
                                         type_name:
                                             ::std::clone::Clone::clone(&(*__self_0_4)),
                                         extendee:
                                             ::std::clone::Clone::clone(&(*__self_0_5)),
                                         default_value:
                                             ::std::clone::Clone::clone(&(*__self_0_6)),
                                         oneof_index:
                                             ::std::clone::Clone::clone(&(*__self_0_7)),
                                         json_name:
                                             ::std::clone::Clone::clone(&(*__self_0_8)),
                                         options:
                                             ::std::clone::Clone::clone(&(*__self_0_9)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for FieldDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FieldDescriptorProto {
                    name: ref __self_0_0,
                    number: ref __self_0_1,
                    label: ref __self_0_2,
                    type_: ref __self_0_3,
                    type_name: ref __self_0_4,
                    extendee: ref __self_0_5,
                    default_value: ref __self_0_6,
                    oneof_index: ref __self_0_7,
                    json_name: ref __self_0_8,
                    options: ref __self_0_9 } => {
                        let mut builder =
                            __arg_0.debug_struct("FieldDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("number", &&(*__self_0_1));
                        let _ = builder.field("label", &&(*__self_0_2));
                        let _ = builder.field("type_", &&(*__self_0_3));
                        let _ = builder.field("type_name", &&(*__self_0_4));
                        let _ = builder.field("extendee", &&(*__self_0_5));
                        let _ =
                            builder.field("default_value", &&(*__self_0_6));
                        let _ = builder.field("oneof_index", &&(*__self_0_7));
                        let _ = builder.field("json_name", &&(*__self_0_8));
                        let _ = builder.field("options", &&(*__self_0_9));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for FieldDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &FieldDescriptorProto) -> bool {
                match *__arg_0 {
                    FieldDescriptorProto {
                    name: ref __self_1_0,
                    number: ref __self_1_1,
                    label: ref __self_1_2,
                    type_: ref __self_1_3,
                    type_name: ref __self_1_4,
                    extendee: ref __self_1_5,
                    default_value: ref __self_1_6,
                    oneof_index: ref __self_1_7,
                    json_name: ref __self_1_8,
                    options: ref __self_1_9 } =>
                    match *self {
                        FieldDescriptorProto {
                        name: ref __self_0_0,
                        number: ref __self_0_1,
                        label: ref __self_0_2,
                        type_: ref __self_0_3,
                        type_name: ref __self_0_4,
                        extendee: ref __self_0_5,
                        default_value: ref __self_0_6,
                        oneof_index: ref __self_0_7,
                        json_name: ref __self_0_8,
                        options: ref __self_0_9 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6) &&
                            (*__self_0_7) == (*__self_1_7) &&
                            (*__self_0_8) == (*__self_1_8) &&
                            (*__self_0_9) == (*__self_1_9),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &FieldDescriptorProto) -> bool {
                match *__arg_0 {
                    FieldDescriptorProto {
                    name: ref __self_1_0,
                    number: ref __self_1_1,
                    label: ref __self_1_2,
                    type_: ref __self_1_3,
                    type_name: ref __self_1_4,
                    extendee: ref __self_1_5,
                    default_value: ref __self_1_6,
                    oneof_index: ref __self_1_7,
                    json_name: ref __self_1_8,
                    options: ref __self_1_9 } =>
                    match *self {
                        FieldDescriptorProto {
                        name: ref __self_0_0,
                        number: ref __self_0_1,
                        label: ref __self_0_2,
                        type_: ref __self_0_3,
                        type_name: ref __self_0_4,
                        extendee: ref __self_0_5,
                        default_value: ref __self_0_6,
                        oneof_index: ref __self_0_7,
                        json_name: ref __self_0_8,
                        options: ref __self_0_9 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6) ||
                            (*__self_0_7) != (*__self_1_7) ||
                            (*__self_0_8) != (*__self_1_8) ||
                            (*__self_0_9) != (*__self_1_9),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod FieldDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for FieldDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.extendee {
                        _prost::encoding::string::encode(2u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.number {
                        _prost::encoding::int32::encode(3u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.label
                           {
                        _prost::encoding::int32::encode(4u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.type_
                           {
                        _prost::encoding::int32::encode(5u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.type_name {
                        _prost::encoding::string::encode(6u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.default_value {
                        _prost::encoding::string::encode(7u32, value, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(8u32, msg, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.oneof_index {
                        _prost::encoding::int32::encode(9u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.json_name {
                        _prost::encoding::string::encode(10u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.extendee.is_none() {
                                self.extendee = Some(Default::default());
                            }
                            match self.extendee {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "extendee");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.number.is_none() {
                                self.number = Some(Default::default());
                            }
                            match self.number {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "number");
                                          error
                                      }),
                        4u32 =>
                        {
                            if self.label.is_none() {
                                self.label = Some(Default::default());
                            }
                            match self.label {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "label");
                                          error
                                      }),
                        5u32 =>
                        {
                            if self.type_.is_none() {
                                self.type_ = Some(Default::default());
                            }
                            match self.type_ {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "type_");
                                          error
                                      }),
                        6u32 =>
                        {
                            if self.type_name.is_none() {
                                self.type_name = Some(Default::default());
                            }
                            match self.type_name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "type_name");
                                          error
                                      }),
                        7u32 =>
                        {
                            if self.default_value.is_none() {
                                self.default_value = Some(Default::default());
                            }
                            match self.default_value {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "default_value");
                                          error
                                      }),
                        8u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        9u32 =>
                        {
                            if self.oneof_index.is_none() {
                                self.oneof_index = Some(Default::default());
                            }
                            match self.oneof_index {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "oneof_index");
                                          error
                                      }),
                        10u32 =>
                        {
                            if self.json_name.is_none() {
                                self.json_name = Some(Default::default());
                            }
                            match self.json_name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       97u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldDescriptorProto",
                                                     "json_name");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        self.extendee.as_ref().map_or(0,
                                                      |value|
                                                          _prost::encoding::string::encoded_len(2u32,
                                                                                                value))
                        +
                        self.number.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::int32::encoded_len(3u32,
                                                                                             value))
                        +
                        self.label.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::int32::encoded_len(4u32,
                                                                                            value))
                        +
                        self.type_.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::int32::encoded_len(5u32,
                                                                                            value))
                        +
                        self.type_name.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::string::encoded_len(6u32,
                                                                                                 value))
                        +
                        self.default_value.as_ref().map_or(0,
                                                           |value|
                                                               _prost::encoding::string::encoded_len(7u32,
                                                                                                     value))
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(8u32,
                                                                                                msg))
                        +
                        self.oneof_index.as_ref().map_or(0,
                                                         |value|
                                                             _prost::encoding::int32::encoded_len(9u32,
                                                                                                  value))
                        +
                        self.json_name.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::string::encoded_len(10u32,
                                                                                                 value))
                }
            }
            impl Default for FieldDescriptorProto {
                fn default() -> FieldDescriptorProto {
                    FieldDescriptorProto{name: ::std::option::Option::None,
                                         extendee:
                                             ::std::option::Option::None,
                                         number: ::std::option::Option::None,
                                         label: ::std::option::Option::None,
                                         type_: ::std::option::Option::None,
                                         type_name:
                                             ::std::option::Option::None,
                                         default_value:
                                             ::std::option::Option::None,
                                         options:
                                             ::std::default::Default::default(),
                                         oneof_index:
                                             ::std::option::Option::None,
                                         json_name:
                                             ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl FieldDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn extendee(&self) -> &str {
                    match self.extendee {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn number(&self) -> i32 {
                    match self.number {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0i32,
                    }
                }
                pub fn label(&self)
                 -> ::std::option::Option<field_descriptor_proto::Label> {
                    self.label.and_then(field_descriptor_proto::Label::from_i32)
                }
                pub fn set_label(&mut self,
                                 value: field_descriptor_proto::Label) {
                    self.label = ::std::option::Option::Some(value as i32);
                }
                pub fn type_(&self)
                 -> ::std::option::Option<field_descriptor_proto::Type> {
                    self.type_.and_then(field_descriptor_proto::Type::from_i32)
                }
                pub fn set_type_(&mut self,
                                 value: field_descriptor_proto::Type) {
                    self.type_ = ::std::option::Option::Some(value as i32);
                }
                pub fn type_name(&self) -> &str {
                    match self.type_name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn default_value(&self) -> &str {
                    match self.default_value {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn oneof_index(&self) -> i32 {
                    match self.oneof_index {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0i32,
                    }
                }
                pub fn json_name(&self) -> &str {
                    match self.json_name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        pub mod field_descriptor_proto {
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum Type {

                /// 0 is reserved for errors.
                /// Order is weird for historical reasons.
                TypeDouble = 1,
                TypeFloat = 2,

                /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
                /// negative values are likely.
                TypeInt64 = 3,
                TypeUint64 = 4,

                /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
                /// negative values are likely.
                TypeInt32 = 5,
                TypeFixed64 = 6,
                TypeFixed32 = 7,
                TypeBool = 8,
                TypeString = 9,

                /// Tag-delimited aggregate.
                /// Group type is deprecated and not supported in proto3. However, Proto3
                /// implementations should still be able to parse the group wire format and
                /// treat group fields as unknown fields.
                TypeGroup = 10,

                /// Length-delimited aggregate.
                TypeMessage = 11,

                /// New in version 2.
                TypeBytes = 12,
                TypeUint32 = 13,
                TypeEnum = 14,
                TypeSfixed32 = 15,
                TypeSfixed64 = 16,

                /// Uses ZigZag encoding.
                TypeSint32 = 17,

                /// Uses ZigZag encoding.
                TypeSint64 = 18,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Type {
                #[inline]
                fn clone(&self) -> Type { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for Type { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Type {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Type::TypeDouble,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeDouble");
                            builder.finish()
                        }
                        (&Type::TypeFloat,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeFloat");
                            builder.finish()
                        }
                        (&Type::TypeInt64,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeInt64");
                            builder.finish()
                        }
                        (&Type::TypeUint64,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeUint64");
                            builder.finish()
                        }
                        (&Type::TypeInt32,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeInt32");
                            builder.finish()
                        }
                        (&Type::TypeFixed64,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeFixed64");
                            builder.finish()
                        }
                        (&Type::TypeFixed32,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeFixed32");
                            builder.finish()
                        }
                        (&Type::TypeBool,) => {
                            let mut builder = __arg_0.debug_tuple("TypeBool");
                            builder.finish()
                        }
                        (&Type::TypeString,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeString");
                            builder.finish()
                        }
                        (&Type::TypeGroup,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeGroup");
                            builder.finish()
                        }
                        (&Type::TypeMessage,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeMessage");
                            builder.finish()
                        }
                        (&Type::TypeBytes,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeBytes");
                            builder.finish()
                        }
                        (&Type::TypeUint32,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeUint32");
                            builder.finish()
                        }
                        (&Type::TypeEnum,) => {
                            let mut builder = __arg_0.debug_tuple("TypeEnum");
                            builder.finish()
                        }
                        (&Type::TypeSfixed32,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeSfixed32");
                            builder.finish()
                        }
                        (&Type::TypeSfixed64,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeSfixed64");
                            builder.finish()
                        }
                        (&Type::TypeSint32,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeSint32");
                            builder.finish()
                        }
                        (&Type::TypeSint64,) => {
                            let mut builder =
                                __arg_0.debug_tuple("TypeSint64");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Type {
                #[inline]
                fn eq(&self, __arg_0: &Type) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for Type {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Type_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl Type {
                    #[doc =
                          "Returns `true` if `value` is a variant of `Type`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            1 => true,
                            2 => true,
                            3 => true,
                            4 => true,
                            5 => true,
                            6 => true,
                            7 => true,
                            8 => true,
                            9 => true,
                            10 => true,
                            11 => true,
                            12 => true,
                            13 => true,
                            14 => true,
                            15 => true,
                            16 => true,
                            17 => true,
                            18 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `Type`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<Type> {
                        match value {
                            1 =>
                            ::std::option::Option::Some(Type::TypeDouble),
                            2 => ::std::option::Option::Some(Type::TypeFloat),
                            3 => ::std::option::Option::Some(Type::TypeInt64),
                            4 =>
                            ::std::option::Option::Some(Type::TypeUint64),
                            5 => ::std::option::Option::Some(Type::TypeInt32),
                            6 =>
                            ::std::option::Option::Some(Type::TypeFixed64),
                            7 =>
                            ::std::option::Option::Some(Type::TypeFixed32),
                            8 => ::std::option::Option::Some(Type::TypeBool),
                            9 =>
                            ::std::option::Option::Some(Type::TypeString),
                            10 =>
                            ::std::option::Option::Some(Type::TypeGroup),
                            11 =>
                            ::std::option::Option::Some(Type::TypeMessage),
                            12 =>
                            ::std::option::Option::Some(Type::TypeBytes),
                            13 =>
                            ::std::option::Option::Some(Type::TypeUint32),
                            14 => ::std::option::Option::Some(Type::TypeEnum),
                            15 =>
                            ::std::option::Option::Some(Type::TypeSfixed32),
                            16 =>
                            ::std::option::Option::Some(Type::TypeSfixed64),
                            17 =>
                            ::std::option::Option::Some(Type::TypeSint32),
                            18 =>
                            ::std::option::Option::Some(Type::TypeSint64),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for Type {
                    fn default() -> Type { Type::TypeDouble }
                }
                impl ::std::convert::From<Type> for i32 {
                    fn from(value: Type) -> i32 { value as i32 }
                }
            }
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum Label {

                /// 0 is reserved for errors
                LabelOptional = 1,
                LabelRequired = 2,
                LabelRepeated = 3,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Label {
                #[inline]
                fn clone(&self) -> Label { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for Label { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Label {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Label::LabelOptional,) => {
                            let mut builder =
                                __arg_0.debug_tuple("LabelOptional");
                            builder.finish()
                        }
                        (&Label::LabelRequired,) => {
                            let mut builder =
                                __arg_0.debug_tuple("LabelRequired");
                            builder.finish()
                        }
                        (&Label::LabelRepeated,) => {
                            let mut builder =
                                __arg_0.debug_tuple("LabelRepeated");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Label {
                #[inline]
                fn eq(&self, __arg_0: &Label) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for Label {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Label_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl Label {
                    #[doc =
                          "Returns `true` if `value` is a variant of `Label`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            1 => true,
                            2 => true,
                            3 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `Label`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<Label> {
                        match value {
                            1 =>
                            ::std::option::Option::Some(Label::LabelOptional),
                            2 =>
                            ::std::option::Option::Some(Label::LabelRequired),
                            3 =>
                            ::std::option::Option::Some(Label::LabelRepeated),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for Label {
                    fn default() -> Label { Label::LabelOptional }
                }
                impl ::std::convert::From<Label> for i32 {
                    fn from(value: Label) -> i32 { value as i32 }
                }
            }
        }
        /// Describes a oneof.
        pub struct OneofDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(message, optional, tag = "2")]
            pub options: Option<OneofOptions>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for OneofDescriptorProto {
            #[inline]
            fn clone(&self) -> OneofDescriptorProto {
                match *self {
                    OneofDescriptorProto {
                    name: ref __self_0_0, options: ref __self_0_1 } =>
                    OneofDescriptorProto{name:
                                             ::std::clone::Clone::clone(&(*__self_0_0)),
                                         options:
                                             ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for OneofDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    OneofDescriptorProto {
                    name: ref __self_0_0, options: ref __self_0_1 } => {
                        let mut builder =
                            __arg_0.debug_struct("OneofDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("options", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for OneofDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &OneofDescriptorProto) -> bool {
                match *__arg_0 {
                    OneofDescriptorProto {
                    name: ref __self_1_0, options: ref __self_1_1 } =>
                    match *self {
                        OneofDescriptorProto {
                        name: ref __self_0_0, options: ref __self_0_1 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &OneofDescriptorProto) -> bool {
                match *__arg_0 {
                    OneofDescriptorProto {
                    name: ref __self_1_0, options: ref __self_1_1 } =>
                    match *self {
                        OneofDescriptorProto {
                        name: ref __self_0_0, options: ref __self_0_1 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod OneofDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for OneofDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(2u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       185u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("OneofDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       185u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("OneofDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(2u32,
                                                                                                msg))
                }
            }
            impl Default for OneofDescriptorProto {
                fn default() -> OneofDescriptorProto {
                    OneofDescriptorProto{name: ::std::option::Option::None,
                                         options:
                                             ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl OneofDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// Describes an enum type.
        pub struct EnumDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(message, repeated, tag = "2")]
            pub value: Vec<EnumValueDescriptorProto>,
            #[prost(message, optional, tag = "3")]
            pub options: Option<EnumOptions>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for EnumDescriptorProto {
            #[inline]
            fn clone(&self) -> EnumDescriptorProto {
                match *self {
                    EnumDescriptorProto {
                    name: ref __self_0_0,
                    value: ref __self_0_1,
                    options: ref __self_0_2 } =>
                    EnumDescriptorProto{name:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        value:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        options:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for EnumDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    EnumDescriptorProto {
                    name: ref __self_0_0,
                    value: ref __self_0_1,
                    options: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("EnumDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("value", &&(*__self_0_1));
                        let _ = builder.field("options", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for EnumDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &EnumDescriptorProto) -> bool {
                match *__arg_0 {
                    EnumDescriptorProto {
                    name: ref __self_1_0,
                    value: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        EnumDescriptorProto {
                        name: ref __self_0_0,
                        value: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &EnumDescriptorProto) -> bool {
                match *__arg_0 {
                    EnumDescriptorProto {
                    name: ref __self_1_0,
                    value: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        EnumDescriptorProto {
                        name: ref __self_0_0,
                        value: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod EnumDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for EnumDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    for msg in &self.value {
                        _prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       193u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.value,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("EnumDescriptorProto",
                                                                                                  "value");
                                                                                       error
                                                                                   }),
                        3u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       193u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        _prost::encoding::message::encoded_len_repeated(2u32,
                                                                        &self.value)
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(3u32,
                                                                                                msg))
                }
            }
            impl Default for EnumDescriptorProto {
                fn default() -> EnumDescriptorProto {
                    EnumDescriptorProto{name: ::std::option::Option::None,
                                        value:
                                            ::std::default::Default::default(),
                                        options:
                                            ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl EnumDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// Describes a value within an enum.
        pub struct EnumValueDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(int32, optional, tag = "2")]
            pub number: Option<i32>,
            #[prost(message, optional, tag = "3")]
            pub options: Option<EnumValueOptions>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for EnumValueDescriptorProto {
            #[inline]
            fn clone(&self) -> EnumValueDescriptorProto {
                match *self {
                    EnumValueDescriptorProto {
                    name: ref __self_0_0,
                    number: ref __self_0_1,
                    options: ref __self_0_2 } =>
                    EnumValueDescriptorProto{name:
                                                 ::std::clone::Clone::clone(&(*__self_0_0)),
                                             number:
                                                 ::std::clone::Clone::clone(&(*__self_0_1)),
                                             options:
                                                 ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for EnumValueDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    EnumValueDescriptorProto {
                    name: ref __self_0_0,
                    number: ref __self_0_1,
                    options: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("EnumValueDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("number", &&(*__self_0_1));
                        let _ = builder.field("options", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for EnumValueDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &EnumValueDescriptorProto) -> bool {
                match *__arg_0 {
                    EnumValueDescriptorProto {
                    name: ref __self_1_0,
                    number: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        EnumValueDescriptorProto {
                        name: ref __self_0_0,
                        number: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &EnumValueDescriptorProto) -> bool {
                match *__arg_0 {
                    EnumValueDescriptorProto {
                    name: ref __self_1_0,
                    number: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        EnumValueDescriptorProto {
                        name: ref __self_0_0,
                        number: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod EnumValueDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for EnumValueDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.number {
                        _prost::encoding::int32::encode(2u32, value, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       203u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumValueDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.number.is_none() {
                                self.number = Some(Default::default());
                            }
                            match self.number {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       203u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumValueDescriptorProto",
                                                     "number");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       203u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumValueDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        self.number.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::int32::encoded_len(2u32,
                                                                                             value))
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(3u32,
                                                                                                msg))
                }
            }
            impl Default for EnumValueDescriptorProto {
                fn default() -> EnumValueDescriptorProto {
                    EnumValueDescriptorProto{name:
                                                 ::std::option::Option::None,
                                             number:
                                                 ::std::option::Option::None,
                                             options:
                                                 ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl EnumValueDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn number(&self) -> i32 {
                    match self.number {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0i32,
                    }
                }
            }
        }
        /// Describes a service.
        pub struct ServiceDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            #[prost(message, repeated, tag = "2")]
            pub method: Vec<MethodDescriptorProto>,
            #[prost(message, optional, tag = "3")]
            pub options: Option<ServiceOptions>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ServiceDescriptorProto {
            #[inline]
            fn clone(&self) -> ServiceDescriptorProto {
                match *self {
                    ServiceDescriptorProto {
                    name: ref __self_0_0,
                    method: ref __self_0_1,
                    options: ref __self_0_2 } =>
                    ServiceDescriptorProto{name:
                                               ::std::clone::Clone::clone(&(*__self_0_0)),
                                           method:
                                               ::std::clone::Clone::clone(&(*__self_0_1)),
                                           options:
                                               ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ServiceDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ServiceDescriptorProto {
                    name: ref __self_0_0,
                    method: ref __self_0_1,
                    options: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("ServiceDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("method", &&(*__self_0_1));
                        let _ = builder.field("options", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ServiceDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &ServiceDescriptorProto) -> bool {
                match *__arg_0 {
                    ServiceDescriptorProto {
                    name: ref __self_1_0,
                    method: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        ServiceDescriptorProto {
                        name: ref __self_0_0,
                        method: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ServiceDescriptorProto) -> bool {
                match *__arg_0 {
                    ServiceDescriptorProto {
                    name: ref __self_1_0,
                    method: ref __self_1_1,
                    options: ref __self_1_2 } =>
                    match *self {
                        ServiceDescriptorProto {
                        name: ref __self_0_0,
                        method: ref __self_0_1,
                        options: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ServiceDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ServiceDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    for msg in &self.method {
                        _prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(3u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       213u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ServiceDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.method,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("ServiceDescriptorProto",
                                                                                                  "method");
                                                                                       error
                                                                                   }),
                        3u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       213u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ServiceDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        _prost::encoding::message::encoded_len_repeated(2u32,
                                                                        &self.method)
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(3u32,
                                                                                                msg))
                }
            }
            impl Default for ServiceDescriptorProto {
                fn default() -> ServiceDescriptorProto {
                    ServiceDescriptorProto{name: ::std::option::Option::None,
                                           method:
                                               ::std::default::Default::default(),
                                           options:
                                               ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl ServiceDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        /// Describes a method of a service.
        pub struct MethodDescriptorProto {
            #[prost(string, optional, tag = "1")]
            pub name: Option<String>,
            /// Input and output type names.  These are resolved in the same way as
            /// FieldDescriptorProto.type_name, but must refer to a message type.
            #[prost(string, optional, tag = "2")]
            pub input_type: Option<String>,
            #[prost(string, optional, tag = "3")]
            pub output_type: Option<String>,
            #[prost(message, optional, tag = "4")]
            pub options: Option<MethodOptions>,
            /// Identifies if client streams multiple client messages
            #[prost(bool, optional, tag = "5")]
            pub client_streaming: Option<bool>,
            /// Identifies if server streams multiple server messages
            #[prost(bool, optional, tag = "6")]
            pub server_streaming: Option<bool>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for MethodDescriptorProto {
            #[inline]
            fn clone(&self) -> MethodDescriptorProto {
                match *self {
                    MethodDescriptorProto {
                    name: ref __self_0_0,
                    input_type: ref __self_0_1,
                    output_type: ref __self_0_2,
                    options: ref __self_0_3,
                    client_streaming: ref __self_0_4,
                    server_streaming: ref __self_0_5 } =>
                    MethodDescriptorProto{name:
                                              ::std::clone::Clone::clone(&(*__self_0_0)),
                                          input_type:
                                              ::std::clone::Clone::clone(&(*__self_0_1)),
                                          output_type:
                                              ::std::clone::Clone::clone(&(*__self_0_2)),
                                          options:
                                              ::std::clone::Clone::clone(&(*__self_0_3)),
                                          client_streaming:
                                              ::std::clone::Clone::clone(&(*__self_0_4)),
                                          server_streaming:
                                              ::std::clone::Clone::clone(&(*__self_0_5)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for MethodDescriptorProto {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    MethodDescriptorProto {
                    name: ref __self_0_0,
                    input_type: ref __self_0_1,
                    output_type: ref __self_0_2,
                    options: ref __self_0_3,
                    client_streaming: ref __self_0_4,
                    server_streaming: ref __self_0_5 } => {
                        let mut builder =
                            __arg_0.debug_struct("MethodDescriptorProto");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ = builder.field("input_type", &&(*__self_0_1));
                        let _ = builder.field("output_type", &&(*__self_0_2));
                        let _ = builder.field("options", &&(*__self_0_3));
                        let _ =
                            builder.field("client_streaming",
                                          &&(*__self_0_4));
                        let _ =
                            builder.field("server_streaming",
                                          &&(*__self_0_5));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for MethodDescriptorProto {
            #[inline]
            fn eq(&self, __arg_0: &MethodDescriptorProto) -> bool {
                match *__arg_0 {
                    MethodDescriptorProto {
                    name: ref __self_1_0,
                    input_type: ref __self_1_1,
                    output_type: ref __self_1_2,
                    options: ref __self_1_3,
                    client_streaming: ref __self_1_4,
                    server_streaming: ref __self_1_5 } =>
                    match *self {
                        MethodDescriptorProto {
                        name: ref __self_0_0,
                        input_type: ref __self_0_1,
                        output_type: ref __self_0_2,
                        options: ref __self_0_3,
                        client_streaming: ref __self_0_4,
                        server_streaming: ref __self_0_5 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &MethodDescriptorProto) -> bool {
                match *__arg_0 {
                    MethodDescriptorProto {
                    name: ref __self_1_0,
                    input_type: ref __self_1_1,
                    output_type: ref __self_1_2,
                    options: ref __self_1_3,
                    client_streaming: ref __self_1_4,
                    server_streaming: ref __self_1_5 } =>
                    match *self {
                        MethodDescriptorProto {
                        name: ref __self_0_0,
                        input_type: ref __self_0_1,
                        output_type: ref __self_0_2,
                        options: ref __self_0_3,
                        client_streaming: ref __self_0_4,
                        server_streaming: ref __self_0_5 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod MethodDescriptorProto_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for MethodDescriptorProto {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.name
                           {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.input_type {
                        _prost::encoding::string::encode(2u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.output_type {
                        _prost::encoding::string::encode(3u32, value, buf);
                    }
                    if let Some(ref msg) = self.options {
                        _prost::encoding::message::encode(4u32, msg, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.client_streaming {
                        _prost::encoding::bool::encode(5u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.server_streaming {
                        _prost::encoding::bool::encode(6u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.name.is_none() {
                                self.name = Some(Default::default());
                            }
                            match self.name {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "name");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.input_type.is_none() {
                                self.input_type = Some(Default::default());
                            }
                            match self.input_type {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "input_type");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.output_type.is_none() {
                                self.output_type = Some(Default::default());
                            }
                            match self.output_type {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "output_type");
                                          error
                                      }),
                        4u32 =>
                        {
                            if self.options.is_none() {
                                self.options = Some(Default::default());
                            }
                            match self.options {
                                Some(ref mut msg) =>
                                _prost::encoding::message::merge(wire_type,
                                                                 msg, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "options");
                                          error
                                      }),
                        5u32 =>
                        {
                            if self.client_streaming.is_none() {
                                self.client_streaming =
                                    Some(Default::default());
                            }
                            match self.client_streaming {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "client_streaming");
                                          error
                                      }),
                        6u32 =>
                        {
                            if self.server_streaming.is_none() {
                                self.server_streaming =
                                    Some(Default::default());
                            }
                            match self.server_streaming {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       223u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodDescriptorProto",
                                                     "server_streaming");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.name.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::string::encoded_len(1u32,
                                                                                            value))
                        +
                        self.input_type.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::string::encoded_len(2u32,
                                                                                                  value))
                        +
                        self.output_type.as_ref().map_or(0,
                                                         |value|
                                                             _prost::encoding::string::encoded_len(3u32,
                                                                                                   value))
                        +
                        self.options.as_ref().map_or(0,
                                                     |msg|
                                                         _prost::encoding::message::encoded_len(4u32,
                                                                                                msg))
                        +
                        self.client_streaming.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::bool::encoded_len(5u32,
                                                                                                      value))
                        +
                        self.server_streaming.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::bool::encoded_len(6u32,
                                                                                                      value))
                }
            }
            impl Default for MethodDescriptorProto {
                fn default() -> MethodDescriptorProto {
                    MethodDescriptorProto{name: ::std::option::Option::None,
                                          input_type:
                                              ::std::option::Option::None,
                                          output_type:
                                              ::std::option::Option::None,
                                          options:
                                              ::std::default::Default::default(),
                                          client_streaming:
                                              ::std::option::Option::None,
                                          server_streaming:
                                              ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl MethodDescriptorProto {
                pub fn name(&self) -> &str {
                    match self.name {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn input_type(&self) -> &str {
                    match self.input_type {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn output_type(&self) -> &str {
                    match self.output_type {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn client_streaming(&self) -> bool {
                    match self.client_streaming {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn server_streaming(&self) -> bool {
                    match self.server_streaming {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub struct FileOptions {
            /// Sets the Java package where classes generated from this .proto will be
            /// placed.  By default, the proto package is used, but this is often
            /// inappropriate because proto packages do not normally start with backwards
            /// domain names.
            #[prost(string, optional, tag = "1")]
            pub java_package: Option<String>,
            /// If set, all the classes from the .proto file are wrapped in a single
            /// outer class with the given name.  This applies to both Proto1
            /// (equivalent to the old "--one_java_file" option) and Proto2 (where
            /// a .proto always translates to a single class, but you may want to
            /// explicitly choose the class name).
            #[prost(string, optional, tag = "8")]
            pub java_outer_classname: Option<String>,
            /// If set true, then the Java code generator will generate a separate .java
            /// file for each top-level message, enum, and service defined in the .proto
            /// file.  Thus, these types will *not* be nested inside the outer class
            /// named by java_outer_classname.  However, the outer class will still be
            /// generated to contain the file's getDescriptor() method as well as any
            /// top-level extensions defined in the file.
            #[prost(bool, optional, tag = "10")]
            pub java_multiple_files: Option<bool>,
            /// This option does nothing.
            #[prost(bool, optional, tag = "20")]
            pub java_generate_equals_and_hash: Option<bool>,
            /// If set true, then the Java2 code generator will generate code that
            /// throws an exception whenever an attempt is made to assign a non-UTF-8
            /// byte sequence to a string field.
            /// Message reflection will do the same.
            /// However, an extension field still accepts non-UTF-8 byte sequences.
            /// This option has no effect on when used with the lite runtime.
            #[prost(bool, optional, tag = "27")]
            pub java_string_check_utf8: Option<bool>,
            #[prost(enumeration = "file_options::OptimizeMode",
                    optional,
                    tag = "9")]
            pub optimize_for: Option<i32>,
            /// Sets the Go package where structs generated from this .proto will be
            /// placed. If omitted, the Go package will be derived from the following:
            ///   - The basename of the package import path, if provided.
            ///   - Otherwise, the package statement in the .proto file, if present.
            ///   - Otherwise, the basename of the .proto file, without extension.
            #[prost(string, optional, tag = "11")]
            pub go_package: Option<String>,
            /// Should generic services be generated in each language?  "Generic" services
            /// are not specific to any particular RPC system.  They are generated by the
            /// main code generators in each language (without additional plugins).
            /// Generic services were the only kind of service generation supported by
            /// early versions of google.protobuf.
            ///
            /// Generic services are now considered deprecated in favor of using plugins
            /// that generate code specific to your particular RPC system.  Therefore,
            /// these default to false.  Old code which depends on generic services should
            /// explicitly set them to true.
            #[prost(bool, optional, tag = "16")]
            pub cc_generic_services: Option<bool>,
            #[prost(bool, optional, tag = "17")]
            pub java_generic_services: Option<bool>,
            #[prost(bool, optional, tag = "18")]
            pub py_generic_services: Option<bool>,
            /// Is this file deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for everything in the file, or it will be completely ignored; in the very
            /// least, this is a formalization for deprecating files.
            #[prost(bool, optional, tag = "23")]
            pub deprecated: Option<bool>,
            /// Enables the use of arenas for the proto messages in this file. This applies
            /// only to generated classes for C++.
            #[prost(bool, optional, tag = "31")]
            pub cc_enable_arenas: Option<bool>,
            /// Sets the objective c class prefix which is prepended to all objective c
            /// generated classes from this .proto. There is no default.
            #[prost(string, optional, tag = "36")]
            pub objc_class_prefix: Option<String>,
            /// Namespace for generated classes; defaults to the package.
            #[prost(string, optional, tag = "37")]
            pub csharp_namespace: Option<String>,
            /// By default Swift generators will take the proto package and CamelCase it
            /// replacing '.' with underscore and use that to prefix the types/symbols
            /// defined. When this options is provided, they will use this value instead
            /// to prefix the types/symbols defined.
            #[prost(string, optional, tag = "39")]
            pub swift_prefix: Option<String>,
            /// Sets the php class prefix which is prepended to all php generated classes
            /// from this .proto. Default is empty.
            #[prost(string, optional, tag = "40")]
            pub php_class_prefix: Option<String>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for FileOptions {
            #[inline]
            fn clone(&self) -> FileOptions {
                match *self {
                    FileOptions {
                    java_package: ref __self_0_0,
                    java_outer_classname: ref __self_0_1,
                    java_multiple_files: ref __self_0_2,
                    java_generate_equals_and_hash: ref __self_0_3,
                    java_string_check_utf8: ref __self_0_4,
                    optimize_for: ref __self_0_5,
                    go_package: ref __self_0_6,
                    cc_generic_services: ref __self_0_7,
                    java_generic_services: ref __self_0_8,
                    py_generic_services: ref __self_0_9,
                    deprecated: ref __self_0_10,
                    cc_enable_arenas: ref __self_0_11,
                    objc_class_prefix: ref __self_0_12,
                    csharp_namespace: ref __self_0_13,
                    swift_prefix: ref __self_0_14,
                    php_class_prefix: ref __self_0_15,
                    uninterpreted_option: ref __self_0_16 } =>
                    FileOptions{java_package:
                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                java_outer_classname:
                                    ::std::clone::Clone::clone(&(*__self_0_1)),
                                java_multiple_files:
                                    ::std::clone::Clone::clone(&(*__self_0_2)),
                                java_generate_equals_and_hash:
                                    ::std::clone::Clone::clone(&(*__self_0_3)),
                                java_string_check_utf8:
                                    ::std::clone::Clone::clone(&(*__self_0_4)),
                                optimize_for:
                                    ::std::clone::Clone::clone(&(*__self_0_5)),
                                go_package:
                                    ::std::clone::Clone::clone(&(*__self_0_6)),
                                cc_generic_services:
                                    ::std::clone::Clone::clone(&(*__self_0_7)),
                                java_generic_services:
                                    ::std::clone::Clone::clone(&(*__self_0_8)),
                                py_generic_services:
                                    ::std::clone::Clone::clone(&(*__self_0_9)),
                                deprecated:
                                    ::std::clone::Clone::clone(&(*__self_0_10)),
                                cc_enable_arenas:
                                    ::std::clone::Clone::clone(&(*__self_0_11)),
                                objc_class_prefix:
                                    ::std::clone::Clone::clone(&(*__self_0_12)),
                                csharp_namespace:
                                    ::std::clone::Clone::clone(&(*__self_0_13)),
                                swift_prefix:
                                    ::std::clone::Clone::clone(&(*__self_0_14)),
                                php_class_prefix:
                                    ::std::clone::Clone::clone(&(*__self_0_15)),
                                uninterpreted_option:
                                    ::std::clone::Clone::clone(&(*__self_0_16)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for FileOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FileOptions {
                    java_package: ref __self_0_0,
                    java_outer_classname: ref __self_0_1,
                    java_multiple_files: ref __self_0_2,
                    java_generate_equals_and_hash: ref __self_0_3,
                    java_string_check_utf8: ref __self_0_4,
                    optimize_for: ref __self_0_5,
                    go_package: ref __self_0_6,
                    cc_generic_services: ref __self_0_7,
                    java_generic_services: ref __self_0_8,
                    py_generic_services: ref __self_0_9,
                    deprecated: ref __self_0_10,
                    cc_enable_arenas: ref __self_0_11,
                    objc_class_prefix: ref __self_0_12,
                    csharp_namespace: ref __self_0_13,
                    swift_prefix: ref __self_0_14,
                    php_class_prefix: ref __self_0_15,
                    uninterpreted_option: ref __self_0_16 } => {
                        let mut builder = __arg_0.debug_struct("FileOptions");
                        let _ =
                            builder.field("java_package", &&(*__self_0_0));
                        let _ =
                            builder.field("java_outer_classname",
                                          &&(*__self_0_1));
                        let _ =
                            builder.field("java_multiple_files",
                                          &&(*__self_0_2));
                        let _ =
                            builder.field("java_generate_equals_and_hash",
                                          &&(*__self_0_3));
                        let _ =
                            builder.field("java_string_check_utf8",
                                          &&(*__self_0_4));
                        let _ =
                            builder.field("optimize_for", &&(*__self_0_5));
                        let _ = builder.field("go_package", &&(*__self_0_6));
                        let _ =
                            builder.field("cc_generic_services",
                                          &&(*__self_0_7));
                        let _ =
                            builder.field("java_generic_services",
                                          &&(*__self_0_8));
                        let _ =
                            builder.field("py_generic_services",
                                          &&(*__self_0_9));
                        let _ = builder.field("deprecated", &&(*__self_0_10));
                        let _ =
                            builder.field("cc_enable_arenas",
                                          &&(*__self_0_11));
                        let _ =
                            builder.field("objc_class_prefix",
                                          &&(*__self_0_12));
                        let _ =
                            builder.field("csharp_namespace",
                                          &&(*__self_0_13));
                        let _ =
                            builder.field("swift_prefix", &&(*__self_0_14));
                        let _ =
                            builder.field("php_class_prefix",
                                          &&(*__self_0_15));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_16));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for FileOptions {
            #[inline]
            fn eq(&self, __arg_0: &FileOptions) -> bool {
                match *__arg_0 {
                    FileOptions {
                    java_package: ref __self_1_0,
                    java_outer_classname: ref __self_1_1,
                    java_multiple_files: ref __self_1_2,
                    java_generate_equals_and_hash: ref __self_1_3,
                    java_string_check_utf8: ref __self_1_4,
                    optimize_for: ref __self_1_5,
                    go_package: ref __self_1_6,
                    cc_generic_services: ref __self_1_7,
                    java_generic_services: ref __self_1_8,
                    py_generic_services: ref __self_1_9,
                    deprecated: ref __self_1_10,
                    cc_enable_arenas: ref __self_1_11,
                    objc_class_prefix: ref __self_1_12,
                    csharp_namespace: ref __self_1_13,
                    swift_prefix: ref __self_1_14,
                    php_class_prefix: ref __self_1_15,
                    uninterpreted_option: ref __self_1_16 } =>
                    match *self {
                        FileOptions {
                        java_package: ref __self_0_0,
                        java_outer_classname: ref __self_0_1,
                        java_multiple_files: ref __self_0_2,
                        java_generate_equals_and_hash: ref __self_0_3,
                        java_string_check_utf8: ref __self_0_4,
                        optimize_for: ref __self_0_5,
                        go_package: ref __self_0_6,
                        cc_generic_services: ref __self_0_7,
                        java_generic_services: ref __self_0_8,
                        py_generic_services: ref __self_0_9,
                        deprecated: ref __self_0_10,
                        cc_enable_arenas: ref __self_0_11,
                        objc_class_prefix: ref __self_0_12,
                        csharp_namespace: ref __self_0_13,
                        swift_prefix: ref __self_0_14,
                        php_class_prefix: ref __self_0_15,
                        uninterpreted_option: ref __self_0_16 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6) &&
                            (*__self_0_7) == (*__self_1_7) &&
                            (*__self_0_8) == (*__self_1_8) &&
                            (*__self_0_9) == (*__self_1_9) &&
                            (*__self_0_10) == (*__self_1_10) &&
                            (*__self_0_11) == (*__self_1_11) &&
                            (*__self_0_12) == (*__self_1_12) &&
                            (*__self_0_13) == (*__self_1_13) &&
                            (*__self_0_14) == (*__self_1_14) &&
                            (*__self_0_15) == (*__self_1_15) &&
                            (*__self_0_16) == (*__self_1_16),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &FileOptions) -> bool {
                match *__arg_0 {
                    FileOptions {
                    java_package: ref __self_1_0,
                    java_outer_classname: ref __self_1_1,
                    java_multiple_files: ref __self_1_2,
                    java_generate_equals_and_hash: ref __self_1_3,
                    java_string_check_utf8: ref __self_1_4,
                    optimize_for: ref __self_1_5,
                    go_package: ref __self_1_6,
                    cc_generic_services: ref __self_1_7,
                    java_generic_services: ref __self_1_8,
                    py_generic_services: ref __self_1_9,
                    deprecated: ref __self_1_10,
                    cc_enable_arenas: ref __self_1_11,
                    objc_class_prefix: ref __self_1_12,
                    csharp_namespace: ref __self_1_13,
                    swift_prefix: ref __self_1_14,
                    php_class_prefix: ref __self_1_15,
                    uninterpreted_option: ref __self_1_16 } =>
                    match *self {
                        FileOptions {
                        java_package: ref __self_0_0,
                        java_outer_classname: ref __self_0_1,
                        java_multiple_files: ref __self_0_2,
                        java_generate_equals_and_hash: ref __self_0_3,
                        java_string_check_utf8: ref __self_0_4,
                        optimize_for: ref __self_0_5,
                        go_package: ref __self_0_6,
                        cc_generic_services: ref __self_0_7,
                        java_generic_services: ref __self_0_8,
                        py_generic_services: ref __self_0_9,
                        deprecated: ref __self_0_10,
                        cc_enable_arenas: ref __self_0_11,
                        objc_class_prefix: ref __self_0_12,
                        csharp_namespace: ref __self_0_13,
                        swift_prefix: ref __self_0_14,
                        php_class_prefix: ref __self_0_15,
                        uninterpreted_option: ref __self_0_16 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6) ||
                            (*__self_0_7) != (*__self_1_7) ||
                            (*__self_0_8) != (*__self_1_8) ||
                            (*__self_0_9) != (*__self_1_9) ||
                            (*__self_0_10) != (*__self_1_10) ||
                            (*__self_0_11) != (*__self_1_11) ||
                            (*__self_0_12) != (*__self_1_12) ||
                            (*__self_0_13) != (*__self_1_13) ||
                            (*__self_0_14) != (*__self_1_14) ||
                            (*__self_0_15) != (*__self_1_15) ||
                            (*__self_0_16) != (*__self_1_16),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod FileOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for FileOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.java_package {
                        _prost::encoding::string::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.java_outer_classname {
                        _prost::encoding::string::encode(8u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.optimize_for {
                        _prost::encoding::int32::encode(9u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.java_multiple_files {
                        _prost::encoding::bool::encode(10u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.go_package {
                        _prost::encoding::string::encode(11u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.cc_generic_services {
                        _prost::encoding::bool::encode(16u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.java_generic_services {
                        _prost::encoding::bool::encode(17u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.py_generic_services {
                        _prost::encoding::bool::encode(18u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.java_generate_equals_and_hash {
                        _prost::encoding::bool::encode(20u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(23u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.java_string_check_utf8 {
                        _prost::encoding::bool::encode(27u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.cc_enable_arenas {
                        _prost::encoding::bool::encode(31u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.objc_class_prefix {
                        _prost::encoding::string::encode(36u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.csharp_namespace {
                        _prost::encoding::string::encode(37u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.swift_prefix {
                        _prost::encoding::string::encode(39u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.php_class_prefix {
                        _prost::encoding::string::encode(40u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.java_package.is_none() {
                                self.java_package = Some(Default::default());
                            }
                            match self.java_package {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_package");
                                          error
                                      }),
                        8u32 =>
                        {
                            if self.java_outer_classname.is_none() {
                                self.java_outer_classname =
                                    Some(Default::default());
                            }
                            match self.java_outer_classname {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_outer_classname");
                                          error
                                      }),
                        9u32 =>
                        {
                            if self.optimize_for.is_none() {
                                self.optimize_for = Some(Default::default());
                            }
                            match self.optimize_for {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "optimize_for");
                                          error
                                      }),
                        10u32 =>
                        {
                            if self.java_multiple_files.is_none() {
                                self.java_multiple_files =
                                    Some(Default::default());
                            }
                            match self.java_multiple_files {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_multiple_files");
                                          error
                                      }),
                        11u32 =>
                        {
                            if self.go_package.is_none() {
                                self.go_package = Some(Default::default());
                            }
                            match self.go_package {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "go_package");
                                          error
                                      }),
                        16u32 =>
                        {
                            if self.cc_generic_services.is_none() {
                                self.cc_generic_services =
                                    Some(Default::default());
                            }
                            match self.cc_generic_services {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "cc_generic_services");
                                          error
                                      }),
                        17u32 =>
                        {
                            if self.java_generic_services.is_none() {
                                self.java_generic_services =
                                    Some(Default::default());
                            }
                            match self.java_generic_services {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_generic_services");
                                          error
                                      }),
                        18u32 =>
                        {
                            if self.py_generic_services.is_none() {
                                self.py_generic_services =
                                    Some(Default::default());
                            }
                            match self.py_generic_services {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "py_generic_services");
                                          error
                                      }),
                        20u32 =>
                        {
                            if self.java_generate_equals_and_hash.is_none() {
                                self.java_generate_equals_and_hash =
                                    Some(Default::default());
                            }
                            match self.java_generate_equals_and_hash {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_generate_equals_and_hash");
                                          error
                                      }),
                        23u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "deprecated");
                                          error
                                      }),
                        27u32 =>
                        {
                            if self.java_string_check_utf8.is_none() {
                                self.java_string_check_utf8 =
                                    Some(Default::default());
                            }
                            match self.java_string_check_utf8 {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "java_string_check_utf8");
                                          error
                                      }),
                        31u32 =>
                        {
                            if self.cc_enable_arenas.is_none() {
                                self.cc_enable_arenas =
                                    Some(Default::default());
                            }
                            match self.cc_enable_arenas {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "cc_enable_arenas");
                                          error
                                      }),
                        36u32 =>
                        {
                            if self.objc_class_prefix.is_none() {
                                self.objc_class_prefix =
                                    Some(Default::default());
                            }
                            match self.objc_class_prefix {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "objc_class_prefix");
                                          error
                                      }),
                        37u32 =>
                        {
                            if self.csharp_namespace.is_none() {
                                self.csharp_namespace =
                                    Some(Default::default());
                            }
                            match self.csharp_namespace {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "csharp_namespace");
                                          error
                                      }),
                        39u32 =>
                        {
                            if self.swift_prefix.is_none() {
                                self.swift_prefix = Some(Default::default());
                            }
                            match self.swift_prefix {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "swift_prefix");
                                          error
                                      }),
                        40u32 =>
                        {
                            if self.php_class_prefix.is_none() {
                                self.php_class_prefix =
                                    Some(Default::default());
                            }
                            match self.php_class_prefix {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       274u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FileOptions",
                                                     "php_class_prefix");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FileOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.java_package.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::string::encoded_len(1u32,
                                                                                                    value))
                        +
                        self.java_outer_classname.as_ref().map_or(0,
                                                                  |value|
                                                                      _prost::encoding::string::encoded_len(8u32,
                                                                                                            value))
                        +
                        self.optimize_for.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::int32::encoded_len(9u32,
                                                                                                   value))
                        +
                        self.java_multiple_files.as_ref().map_or(0,
                                                                 |value|
                                                                     _prost::encoding::bool::encoded_len(10u32,
                                                                                                         value))
                        +
                        self.go_package.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::string::encoded_len(11u32,
                                                                                                  value))
                        +
                        self.cc_generic_services.as_ref().map_or(0,
                                                                 |value|
                                                                     _prost::encoding::bool::encoded_len(16u32,
                                                                                                         value))
                        +
                        self.java_generic_services.as_ref().map_or(0,
                                                                   |value|
                                                                       _prost::encoding::bool::encoded_len(17u32,
                                                                                                           value))
                        +
                        self.py_generic_services.as_ref().map_or(0,
                                                                 |value|
                                                                     _prost::encoding::bool::encoded_len(18u32,
                                                                                                         value))
                        +
                        self.java_generate_equals_and_hash.as_ref().map_or(0,
                                                                           |value|
                                                                               _prost::encoding::bool::encoded_len(20u32,
                                                                                                                   value))
                        +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(23u32,
                                                                                                value))
                        +
                        self.java_string_check_utf8.as_ref().map_or(0,
                                                                    |value|
                                                                        _prost::encoding::bool::encoded_len(27u32,
                                                                                                            value))
                        +
                        self.cc_enable_arenas.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::bool::encoded_len(31u32,
                                                                                                      value))
                        +
                        self.objc_class_prefix.as_ref().map_or(0,
                                                               |value|
                                                                   _prost::encoding::string::encoded_len(36u32,
                                                                                                         value))
                        +
                        self.csharp_namespace.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::string::encoded_len(37u32,
                                                                                                        value))
                        +
                        self.swift_prefix.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::string::encoded_len(39u32,
                                                                                                    value))
                        +
                        self.php_class_prefix.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::string::encoded_len(40u32,
                                                                                                        value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for FileOptions {
                fn default() -> FileOptions {
                    FileOptions{java_package: ::std::option::Option::None,
                                java_outer_classname:
                                    ::std::option::Option::None,
                                optimize_for: ::std::option::Option::None,
                                java_multiple_files:
                                    ::std::option::Option::None,
                                go_package: ::std::option::Option::None,
                                cc_generic_services:
                                    ::std::option::Option::None,
                                java_generic_services:
                                    ::std::option::Option::None,
                                py_generic_services:
                                    ::std::option::Option::None,
                                java_generate_equals_and_hash:
                                    ::std::option::Option::None,
                                deprecated: ::std::option::Option::None,
                                java_string_check_utf8:
                                    ::std::option::Option::None,
                                cc_enable_arenas: ::std::option::Option::None,
                                objc_class_prefix:
                                    ::std::option::Option::None,
                                csharp_namespace: ::std::option::Option::None,
                                swift_prefix: ::std::option::Option::None,
                                php_class_prefix: ::std::option::Option::None,
                                uninterpreted_option:
                                    ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl FileOptions {
                pub fn java_package(&self) -> &str {
                    match self.java_package {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn java_outer_classname(&self) -> &str {
                    match self.java_outer_classname {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn optimize_for(&self)
                 -> ::std::option::Option<file_options::OptimizeMode> {
                    self.optimize_for.and_then(file_options::OptimizeMode::from_i32)
                }
                pub fn set_optimize_for(&mut self,
                                        value: file_options::OptimizeMode) {
                    self.optimize_for =
                        ::std::option::Option::Some(value as i32);
                }
                pub fn java_multiple_files(&self) -> bool {
                    match self.java_multiple_files {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn go_package(&self) -> &str {
                    match self.go_package {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn cc_generic_services(&self) -> bool {
                    match self.cc_generic_services {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn java_generic_services(&self) -> bool {
                    match self.java_generic_services {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn py_generic_services(&self) -> bool {
                    match self.py_generic_services {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn java_generate_equals_and_hash(&self) -> bool {
                    match self.java_generate_equals_and_hash {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn java_string_check_utf8(&self) -> bool {
                    match self.java_string_check_utf8 {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn cc_enable_arenas(&self) -> bool {
                    match self.cc_enable_arenas {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn objc_class_prefix(&self) -> &str {
                    match self.objc_class_prefix {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn csharp_namespace(&self) -> &str {
                    match self.csharp_namespace {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn swift_prefix(&self) -> &str {
                    match self.swift_prefix {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn php_class_prefix(&self) -> &str {
                    match self.php_class_prefix {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        pub mod file_options {
            /// Generated classes can be optimized for speed or code size.
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum OptimizeMode {

                /// Generate complete code for parsing, serialization,
                Speed = 1,

                /// etc.
                /// Use ReflectionOps to implement these methods.
                CodeSize = 2,

                /// Generate code using MessageLite and the lite runtime.
                LiteRuntime = 3,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for OptimizeMode {
                #[inline]
                fn clone(&self) -> OptimizeMode { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for OptimizeMode { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for OptimizeMode {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&OptimizeMode::Speed,) => {
                            let mut builder = __arg_0.debug_tuple("Speed");
                            builder.finish()
                        }
                        (&OptimizeMode::CodeSize,) => {
                            let mut builder = __arg_0.debug_tuple("CodeSize");
                            builder.finish()
                        }
                        (&OptimizeMode::LiteRuntime,) => {
                            let mut builder =
                                __arg_0.debug_tuple("LiteRuntime");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for OptimizeMode {
                #[inline]
                fn eq(&self, __arg_0: &OptimizeMode) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for OptimizeMode {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod OptimizeMode_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl OptimizeMode {
                    #[doc =
                          "Returns `true` if `value` is a variant of `OptimizeMode`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            1 => true,
                            2 => true,
                            3 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `OptimizeMode`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<OptimizeMode> {
                        match value {
                            1 =>
                            ::std::option::Option::Some(OptimizeMode::Speed),
                            2 =>
                            ::std::option::Option::Some(OptimizeMode::CodeSize),
                            3 =>
                            ::std::option::Option::Some(OptimizeMode::LiteRuntime),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for OptimizeMode {
                    fn default() -> OptimizeMode { OptimizeMode::Speed }
                }
                impl ::std::convert::From<OptimizeMode> for i32 {
                    fn from(value: OptimizeMode) -> i32 { value as i32 }
                }
            }
        }
        pub struct MessageOptions {
            /// Set true to use the old proto1 MessageSet wire format for extensions.
            /// This is provided for backwards-compatibility with the MessageSet wire
            /// format.  You should not use this for any other reason:  It's less
            /// efficient, has fewer features, and is more complicated.
            ///
            /// The message must be defined exactly as follows:
            ///   message Foo {
            ///     option message_set_wire_format = true;
            ///     extensions 4 to max;
            ///   }
            /// Note that the message cannot have any defined fields; MessageSets only
            /// have extensions.
            ///
            /// All extensions of your type must be singular messages; e.g. they cannot
            /// be int32s, enums, or repeated messages.
            ///
            /// Because this is an option, the above two restrictions are not enforced by
            /// the protocol compiler.
            #[prost(bool, optional, tag = "1")]
            pub message_set_wire_format: Option<bool>,
            /// Disables the generation of the standard "descriptor()" accessor, which can
            /// conflict with a field of the same name.  This is meant to make migration
            /// from proto1 easier; new code should avoid fields named "descriptor".
            #[prost(bool, optional, tag = "2")]
            pub no_standard_descriptor_accessor: Option<bool>,
            /// Is this message deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for the message, or it will be completely ignored; in the very least,
            /// this is a formalization for deprecating messages.
            #[prost(bool, optional, tag = "3")]
            pub deprecated: Option<bool>,
            /// Whether the message is an automatically generated map entry type for the
            /// maps field.
            ///
            /// For maps fields:
            ///     map<KeyType, ValueType> map_field = 1;
            /// The parsed descriptor looks like:
            ///     message MapFieldEntry {
            ///         option map_entry = true;
            ///         optional KeyType key = 1;
            ///         optional ValueType value = 2;
            ///     }
            ///     repeated MapFieldEntry map_field = 1;
            ///
            /// Implementations may choose not to generate the map_entry=true message, but
            /// use a native map in the target language to hold the keys and values.
            /// The reflection APIs in such implementions still need to work as
            /// if the field is a repeated message field.
            ///
            /// NOTE: Do not set the option in .proto files. Always use the maps syntax
            /// instead. The option should only be implicitly set by the proto compiler
            /// parser.
            #[prost(bool, optional, tag = "7")]
            pub map_entry: Option<bool>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for MessageOptions {
            #[inline]
            fn clone(&self) -> MessageOptions {
                match *self {
                    MessageOptions {
                    message_set_wire_format: ref __self_0_0,
                    no_standard_descriptor_accessor: ref __self_0_1,
                    deprecated: ref __self_0_2,
                    map_entry: ref __self_0_3,
                    uninterpreted_option: ref __self_0_4 } =>
                    MessageOptions{message_set_wire_format:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   no_standard_descriptor_accessor:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),
                                   deprecated:
                                       ::std::clone::Clone::clone(&(*__self_0_2)),
                                   map_entry:
                                       ::std::clone::Clone::clone(&(*__self_0_3)),
                                   uninterpreted_option:
                                       ::std::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for MessageOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    MessageOptions {
                    message_set_wire_format: ref __self_0_0,
                    no_standard_descriptor_accessor: ref __self_0_1,
                    deprecated: ref __self_0_2,
                    map_entry: ref __self_0_3,
                    uninterpreted_option: ref __self_0_4 } => {
                        let mut builder =
                            __arg_0.debug_struct("MessageOptions");
                        let _ =
                            builder.field("message_set_wire_format",
                                          &&(*__self_0_0));
                        let _ =
                            builder.field("no_standard_descriptor_accessor",
                                          &&(*__self_0_1));
                        let _ = builder.field("deprecated", &&(*__self_0_2));
                        let _ = builder.field("map_entry", &&(*__self_0_3));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_4));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for MessageOptions {
            #[inline]
            fn eq(&self, __arg_0: &MessageOptions) -> bool {
                match *__arg_0 {
                    MessageOptions {
                    message_set_wire_format: ref __self_1_0,
                    no_standard_descriptor_accessor: ref __self_1_1,
                    deprecated: ref __self_1_2,
                    map_entry: ref __self_1_3,
                    uninterpreted_option: ref __self_1_4 } =>
                    match *self {
                        MessageOptions {
                        message_set_wire_format: ref __self_0_0,
                        no_standard_descriptor_accessor: ref __self_0_1,
                        deprecated: ref __self_0_2,
                        map_entry: ref __self_0_3,
                        uninterpreted_option: ref __self_0_4 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &MessageOptions) -> bool {
                match *__arg_0 {
                    MessageOptions {
                    message_set_wire_format: ref __self_1_0,
                    no_standard_descriptor_accessor: ref __self_1_1,
                    deprecated: ref __self_1_2,
                    map_entry: ref __self_1_3,
                    uninterpreted_option: ref __self_1_4 } =>
                    match *self {
                        MessageOptions {
                        message_set_wire_format: ref __self_0_0,
                        no_standard_descriptor_accessor: ref __self_0_1,
                        deprecated: ref __self_0_2,
                        map_entry: ref __self_0_3,
                        uninterpreted_option: ref __self_0_4 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod MessageOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for MessageOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.message_set_wire_format {
                        _prost::encoding::bool::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.no_standard_descriptor_accessor {
                        _prost::encoding::bool::encode(2u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(3u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.map_entry {
                        _prost::encoding::bool::encode(7u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.message_set_wire_format.is_none() {
                                self.message_set_wire_format =
                                    Some(Default::default());
                            }
                            match self.message_set_wire_format {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       377u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MessageOptions",
                                                     "message_set_wire_format");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.no_standard_descriptor_accessor.is_none()
                               {
                                self.no_standard_descriptor_accessor =
                                    Some(Default::default());
                            }
                            match self.no_standard_descriptor_accessor {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       377u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MessageOptions",
                                                     "no_standard_descriptor_accessor");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       377u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MessageOptions",
                                                     "deprecated");
                                          error
                                      }),
                        7u32 =>
                        {
                            if self.map_entry.is_none() {
                                self.map_entry = Some(Default::default());
                            }
                            match self.map_entry {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       377u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MessageOptions",
                                                     "map_entry");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("MessageOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.message_set_wire_format.as_ref().map_or(0,
                                                                     |value|
                                                                         _prost::encoding::bool::encoded_len(1u32,
                                                                                                             value))
                        +
                        self.no_standard_descriptor_accessor.as_ref().map_or(0,
                                                                             |value|
                                                                                 _prost::encoding::bool::encoded_len(2u32,
                                                                                                                     value))
                        +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(3u32,
                                                                                                value))
                        +
                        self.map_entry.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::bool::encoded_len(7u32,
                                                                                               value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for MessageOptions {
                fn default() -> MessageOptions {
                    MessageOptions{message_set_wire_format:
                                       ::std::option::Option::None,
                                   no_standard_descriptor_accessor:
                                       ::std::option::Option::None,
                                   deprecated: ::std::option::Option::None,
                                   map_entry: ::std::option::Option::None,
                                   uninterpreted_option:
                                       ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl MessageOptions {
                pub fn message_set_wire_format(&self) -> bool {
                    match self.message_set_wire_format {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn no_standard_descriptor_accessor(&self) -> bool {
                    match self.no_standard_descriptor_accessor {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn map_entry(&self) -> bool {
                    match self.map_entry {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub struct FieldOptions {
            /// The ctype option instructs the C++ code generator to use a different
            /// representation of the field than it normally would.  See the specific
            /// options below.  This option is not yet implemented in the open source
            /// release -- sorry, we'll try to include it in a future version!
            #[prost(enumeration = "field_options::CType",
                    optional,
                    tag = "1")]
            pub ctype: Option<i32>,
            /// The packed option can be enabled for repeated primitive fields to enable
            /// a more efficient representation on the wire. Rather than repeatedly
            /// writing the tag and type for each element, the entire array is encoded as
            /// a single length-delimited blob. In proto3, only explicit setting it to
            /// false will avoid using packed encoding.
            #[prost(bool, optional, tag = "2")]
            pub packed: Option<bool>,
            /// The jstype option determines the JavaScript type used for values of the
            /// field.  The option is permitted only for 64 bit integral and fixed types
            /// (int64, uint64, sint64, fixed64, sfixed64).  By default these types are
            /// represented as JavaScript strings.  This avoids loss of precision that can
            /// happen when a large value is converted to a floating point JavaScript
            /// numbers.  Specifying JS_NUMBER for the jstype causes the generated
            /// JavaScript code to use the JavaScript "number" type instead of strings.
            /// This option is an enum to permit additional types to be added,
            /// e.g. goog.math.Integer.
            #[prost(enumeration = "field_options::JSType",
                    optional,
                    tag = "6")]
            pub jstype: Option<i32>,
            /// Should this field be parsed lazily?  Lazy applies only to message-type
            /// fields.  It means that when the outer message is initially parsed, the
            /// inner message's contents will not be parsed but instead stored in encoded
            /// form.  The inner message will actually be parsed when it is first accessed.
            ///
            /// This is only a hint.  Implementations are free to choose whether to use
            /// eager or lazy parsing regardless of the value of this option.  However,
            /// setting this option true suggests that the protocol author believes that
            /// using lazy parsing on this field is worth the additional bookkeeping
            /// overhead typically needed to implement it.
            ///
            /// This option does not affect the public interface of any generated code;
            /// all method signatures remain the same.  Furthermore, thread-safety of the
            /// interface is not affected by this option; const methods remain safe to
            /// call from multiple threads concurrently, while non-const methods continue
            /// to require exclusive access.
            ///
            ///
            /// Note that implementations may choose not to check required fields within
            /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
            /// may return true even if the inner message has missing required fields.
            /// This is necessary because otherwise the inner message would have to be
            /// parsed in order to perform the check, defeating the purpose of lazy
            /// parsing.  An implementation which chooses not to check required fields
            /// must be consistent about it.  That is, for any particular sub-message, the
            /// implementation must either *always* check its required fields, or *never*
            /// check its required fields, regardless of whether or not the message has
            /// been parsed.
            #[prost(bool, optional, tag = "5")]
            pub lazy: Option<bool>,
            /// Is this field deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for accessors, or it will be completely ignored; in the very least, this
            /// is a formalization for deprecating fields.
            #[prost(bool, optional, tag = "3")]
            pub deprecated: Option<bool>,
            /// For Google-internal migration only. Do not use.
            #[prost(bool, optional, tag = "10")]
            pub weak: Option<bool>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for FieldOptions {
            #[inline]
            fn clone(&self) -> FieldOptions {
                match *self {
                    FieldOptions {
                    ctype: ref __self_0_0,
                    packed: ref __self_0_1,
                    jstype: ref __self_0_2,
                    lazy: ref __self_0_3,
                    deprecated: ref __self_0_4,
                    weak: ref __self_0_5,
                    uninterpreted_option: ref __self_0_6 } =>
                    FieldOptions{ctype:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 packed:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),
                                 jstype:
                                     ::std::clone::Clone::clone(&(*__self_0_2)),
                                 lazy:
                                     ::std::clone::Clone::clone(&(*__self_0_3)),
                                 deprecated:
                                     ::std::clone::Clone::clone(&(*__self_0_4)),
                                 weak:
                                     ::std::clone::Clone::clone(&(*__self_0_5)),
                                 uninterpreted_option:
                                     ::std::clone::Clone::clone(&(*__self_0_6)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for FieldOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FieldOptions {
                    ctype: ref __self_0_0,
                    packed: ref __self_0_1,
                    jstype: ref __self_0_2,
                    lazy: ref __self_0_3,
                    deprecated: ref __self_0_4,
                    weak: ref __self_0_5,
                    uninterpreted_option: ref __self_0_6 } => {
                        let mut builder =
                            __arg_0.debug_struct("FieldOptions");
                        let _ = builder.field("ctype", &&(*__self_0_0));
                        let _ = builder.field("packed", &&(*__self_0_1));
                        let _ = builder.field("jstype", &&(*__self_0_2));
                        let _ = builder.field("lazy", &&(*__self_0_3));
                        let _ = builder.field("deprecated", &&(*__self_0_4));
                        let _ = builder.field("weak", &&(*__self_0_5));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_6));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for FieldOptions {
            #[inline]
            fn eq(&self, __arg_0: &FieldOptions) -> bool {
                match *__arg_0 {
                    FieldOptions {
                    ctype: ref __self_1_0,
                    packed: ref __self_1_1,
                    jstype: ref __self_1_2,
                    lazy: ref __self_1_3,
                    deprecated: ref __self_1_4,
                    weak: ref __self_1_5,
                    uninterpreted_option: ref __self_1_6 } =>
                    match *self {
                        FieldOptions {
                        ctype: ref __self_0_0,
                        packed: ref __self_0_1,
                        jstype: ref __self_0_2,
                        lazy: ref __self_0_3,
                        deprecated: ref __self_0_4,
                        weak: ref __self_0_5,
                        uninterpreted_option: ref __self_0_6 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &FieldOptions) -> bool {
                match *__arg_0 {
                    FieldOptions {
                    ctype: ref __self_1_0,
                    packed: ref __self_1_1,
                    jstype: ref __self_1_2,
                    lazy: ref __self_1_3,
                    deprecated: ref __self_1_4,
                    weak: ref __self_1_5,
                    uninterpreted_option: ref __self_1_6 } =>
                    match *self {
                        FieldOptions {
                        ctype: ref __self_0_0,
                        packed: ref __self_0_1,
                        jstype: ref __self_0_2,
                        lazy: ref __self_0_3,
                        deprecated: ref __self_0_4,
                        weak: ref __self_0_5,
                        uninterpreted_option: ref __self_0_6 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod FieldOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for FieldOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) = self.ctype
                           {
                        _prost::encoding::int32::encode(1u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.packed {
                        _prost::encoding::bool::encode(2u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(3u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.lazy
                           {
                        _prost::encoding::bool::encode(5u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.jstype {
                        _prost::encoding::int32::encode(6u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) = self.weak
                           {
                        _prost::encoding::bool::encode(10u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.ctype.is_none() {
                                self.ctype = Some(Default::default());
                            }
                            match self.ctype {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions", "ctype");
                                          error
                                      }),
                        2u32 =>
                        {
                            if self.packed.is_none() {
                                self.packed = Some(Default::default());
                            }
                            match self.packed {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions",
                                                     "packed");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions",
                                                     "deprecated");
                                          error
                                      }),
                        5u32 =>
                        {
                            if self.lazy.is_none() {
                                self.lazy = Some(Default::default());
                            }
                            match self.lazy {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions", "lazy");
                                          error
                                      }),
                        6u32 =>
                        {
                            if self.jstype.is_none() {
                                self.jstype = Some(Default::default());
                            }
                            match self.jstype {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions",
                                                     "jstype");
                                          error
                                      }),
                        10u32 =>
                        {
                            if self.weak.is_none() {
                                self.weak = Some(Default::default());
                            }
                            match self.weak {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       437u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("FieldOptions", "weak");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("FieldOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.ctype.as_ref().map_or(0,
                                                   |value|
                                                       _prost::encoding::int32::encoded_len(1u32,
                                                                                            value))
                        +
                        self.packed.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::bool::encoded_len(2u32,
                                                                                            value))
                        +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(3u32,
                                                                                                value))
                        +
                        self.lazy.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::bool::encoded_len(5u32,
                                                                                          value))
                        +
                        self.jstype.as_ref().map_or(0,
                                                    |value|
                                                        _prost::encoding::int32::encoded_len(6u32,
                                                                                             value))
                        +
                        self.weak.as_ref().map_or(0,
                                                  |value|
                                                      _prost::encoding::bool::encoded_len(10u32,
                                                                                          value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for FieldOptions {
                fn default() -> FieldOptions {
                    FieldOptions{ctype: ::std::option::Option::None,
                                 packed: ::std::option::Option::None,
                                 deprecated: ::std::option::Option::None,
                                 lazy: ::std::option::Option::None,
                                 jstype: ::std::option::Option::None,
                                 weak: ::std::option::Option::None,
                                 uninterpreted_option:
                                     ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl FieldOptions {
                pub fn ctype(&self)
                 -> ::std::option::Option<field_options::CType> {
                    self.ctype.and_then(field_options::CType::from_i32)
                }
                pub fn set_ctype(&mut self, value: field_options::CType) {
                    self.ctype = ::std::option::Option::Some(value as i32);
                }
                pub fn packed(&self) -> bool {
                    match self.packed {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn lazy(&self) -> bool {
                    match self.lazy {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn jstype(&self)
                 -> ::std::option::Option<field_options::JSType> {
                    self.jstype.and_then(field_options::JSType::from_i32)
                }
                pub fn set_jstype(&mut self, value: field_options::JSType) {
                    self.jstype = ::std::option::Option::Some(value as i32);
                }
                pub fn weak(&self) -> bool {
                    match self.weak {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub mod field_options {
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum CType {

                /// Default mode.
                String = 0,
                Cord = 1,
                StringPiece = 2,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for CType {
                #[inline]
                fn clone(&self) -> CType { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for CType { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for CType {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&CType::String,) => {
                            let mut builder = __arg_0.debug_tuple("String");
                            builder.finish()
                        }
                        (&CType::Cord,) => {
                            let mut builder = __arg_0.debug_tuple("Cord");
                            builder.finish()
                        }
                        (&CType::StringPiece,) => {
                            let mut builder =
                                __arg_0.debug_tuple("StringPiece");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for CType {
                #[inline]
                fn eq(&self, __arg_0: &CType) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for CType {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod CType_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl CType {
                    #[doc =
                          "Returns `true` if `value` is a variant of `CType`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            0 => true,
                            1 => true,
                            2 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `CType`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<CType> {
                        match value {
                            0 => ::std::option::Option::Some(CType::String),
                            1 => ::std::option::Option::Some(CType::Cord),
                            2 =>
                            ::std::option::Option::Some(CType::StringPiece),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for CType {
                    fn default() -> CType { CType::String }
                }
                impl ::std::convert::From<CType> for i32 {
                    fn from(value: CType) -> i32 { value as i32 }
                }
            }
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum JSType {

                /// Use the default type.
                JsNormal = 0,

                /// Use JavaScript strings.
                JsString = 1,

                /// Use JavaScript numbers.
                JsNumber = 2,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for JSType {
                #[inline]
                fn clone(&self) -> JSType { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for JSType { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for JSType {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&JSType::JsNormal,) => {
                            let mut builder = __arg_0.debug_tuple("JsNormal");
                            builder.finish()
                        }
                        (&JSType::JsString,) => {
                            let mut builder = __arg_0.debug_tuple("JsString");
                            builder.finish()
                        }
                        (&JSType::JsNumber,) => {
                            let mut builder = __arg_0.debug_tuple("JsNumber");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for JSType {
                #[inline]
                fn eq(&self, __arg_0: &JSType) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for JSType {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod JSType_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl JSType {
                    #[doc =
                          "Returns `true` if `value` is a variant of `JSType`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            0 => true,
                            1 => true,
                            2 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `JSType`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<JSType> {
                        match value {
                            0 =>
                            ::std::option::Option::Some(JSType::JsNormal),
                            1 =>
                            ::std::option::Option::Some(JSType::JsString),
                            2 =>
                            ::std::option::Option::Some(JSType::JsNumber),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for JSType {
                    fn default() -> JSType { JSType::JsNormal }
                }
                impl ::std::convert::From<JSType> for i32 {
                    fn from(value: JSType) -> i32 { value as i32 }
                }
            }
        }
        pub struct OneofOptions {
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for OneofOptions {
            #[inline]
            fn clone(&self) -> OneofOptions {
                match *self {
                    OneofOptions { uninterpreted_option: ref __self_0_0 } =>
                    OneofOptions{uninterpreted_option:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for OneofOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    OneofOptions { uninterpreted_option: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("OneofOptions");
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for OneofOptions {
            #[inline]
            fn eq(&self, __arg_0: &OneofOptions) -> bool {
                match *__arg_0 {
                    OneofOptions { uninterpreted_option: ref __self_1_0 } =>
                    match *self {
                        OneofOptions { uninterpreted_option: ref __self_0_0 }
                        => true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &OneofOptions) -> bool {
                match *__arg_0 {
                    OneofOptions { uninterpreted_option: ref __self_1_0 } =>
                    match *self {
                        OneofOptions { uninterpreted_option: ref __self_0_0 }
                        => false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod OneofOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for OneofOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("OneofOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for OneofOptions {
                fn default() -> OneofOptions {
                    OneofOptions{uninterpreted_option:
                                     ::std::default::Default::default(),}
                }
            }
        }
        pub struct EnumOptions {
            /// Set this option to true to allow mapping different tag names to the same
            /// value.
            #[prost(bool, optional, tag = "2")]
            pub allow_alias: Option<bool>,
            /// Is this enum deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for the enum, or it will be completely ignored; in the very least, this
            /// is a formalization for deprecating enums.
            #[prost(bool, optional, tag = "3")]
            pub deprecated: Option<bool>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for EnumOptions {
            #[inline]
            fn clone(&self) -> EnumOptions {
                match *self {
                    EnumOptions {
                    allow_alias: ref __self_0_0,
                    deprecated: ref __self_0_1,
                    uninterpreted_option: ref __self_0_2 } =>
                    EnumOptions{allow_alias:
                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                deprecated:
                                    ::std::clone::Clone::clone(&(*__self_0_1)),
                                uninterpreted_option:
                                    ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for EnumOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    EnumOptions {
                    allow_alias: ref __self_0_0,
                    deprecated: ref __self_0_1,
                    uninterpreted_option: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("EnumOptions");
                        let _ = builder.field("allow_alias", &&(*__self_0_0));
                        let _ = builder.field("deprecated", &&(*__self_0_1));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for EnumOptions {
            #[inline]
            fn eq(&self, __arg_0: &EnumOptions) -> bool {
                match *__arg_0 {
                    EnumOptions {
                    allow_alias: ref __self_1_0,
                    deprecated: ref __self_1_1,
                    uninterpreted_option: ref __self_1_2 } =>
                    match *self {
                        EnumOptions {
                        allow_alias: ref __self_0_0,
                        deprecated: ref __self_0_1,
                        uninterpreted_option: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &EnumOptions) -> bool {
                match *__arg_0 {
                    EnumOptions {
                    allow_alias: ref __self_1_0,
                    deprecated: ref __self_1_1,
                    uninterpreted_option: ref __self_1_2 } =>
                    match *self {
                        EnumOptions {
                        allow_alias: ref __self_0_0,
                        deprecated: ref __self_0_1,
                        uninterpreted_option: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod EnumOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for EnumOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.allow_alias {
                        _prost::encoding::bool::encode(2u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(3u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        2u32 =>
                        {
                            if self.allow_alias.is_none() {
                                self.allow_alias = Some(Default::default());
                            }
                            match self.allow_alias {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       530u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumOptions",
                                                     "allow_alias");
                                          error
                                      }),
                        3u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       530u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumOptions",
                                                     "deprecated");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("EnumOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.allow_alias.as_ref().map_or(0,
                                                         |value|
                                                             _prost::encoding::bool::encoded_len(2u32,
                                                                                                 value))
                        +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(3u32,
                                                                                                value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for EnumOptions {
                fn default() -> EnumOptions {
                    EnumOptions{allow_alias: ::std::option::Option::None,
                                deprecated: ::std::option::Option::None,
                                uninterpreted_option:
                                    ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl EnumOptions {
                pub fn allow_alias(&self) -> bool {
                    match self.allow_alias {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub struct EnumValueOptions {
            /// Is this enum value deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for the enum value, or it will be completely ignored; in the very least,
            /// this is a formalization for deprecating enum values.
            #[prost(bool, optional, tag = "1")]
            pub deprecated: Option<bool>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for EnumValueOptions {
            #[inline]
            fn clone(&self) -> EnumValueOptions {
                match *self {
                    EnumValueOptions {
                    deprecated: ref __self_0_0,
                    uninterpreted_option: ref __self_0_1 } =>
                    EnumValueOptions{deprecated:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     uninterpreted_option:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for EnumValueOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    EnumValueOptions {
                    deprecated: ref __self_0_0,
                    uninterpreted_option: ref __self_0_1 } => {
                        let mut builder =
                            __arg_0.debug_struct("EnumValueOptions");
                        let _ = builder.field("deprecated", &&(*__self_0_0));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for EnumValueOptions {
            #[inline]
            fn eq(&self, __arg_0: &EnumValueOptions) -> bool {
                match *__arg_0 {
                    EnumValueOptions {
                    deprecated: ref __self_1_0,
                    uninterpreted_option: ref __self_1_1 } =>
                    match *self {
                        EnumValueOptions {
                        deprecated: ref __self_0_0,
                        uninterpreted_option: ref __self_0_1 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &EnumValueOptions) -> bool {
                match *__arg_0 {
                    EnumValueOptions {
                    deprecated: ref __self_1_0,
                    uninterpreted_option: ref __self_1_1 } =>
                    match *self {
                        EnumValueOptions {
                        deprecated: ref __self_0_0,
                        uninterpreted_option: ref __self_0_1 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod EnumValueOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for EnumValueOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(1u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       546u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("EnumValueOptions",
                                                     "deprecated");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("EnumValueOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(1u32,
                                                                                                value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for EnumValueOptions {
                fn default() -> EnumValueOptions {
                    EnumValueOptions{deprecated: ::std::option::Option::None,
                                     uninterpreted_option:
                                         ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl EnumValueOptions {
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub struct ServiceOptions {
            /// Is this service deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for the service, or it will be completely ignored; in the very least,
            /// this is a formalization for deprecating services.
            #[prost(bool, optional, tag = "33")]
            pub deprecated: Option<bool>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for ServiceOptions {
            #[inline]
            fn clone(&self) -> ServiceOptions {
                match *self {
                    ServiceOptions {
                    deprecated: ref __self_0_0,
                    uninterpreted_option: ref __self_0_1 } =>
                    ServiceOptions{deprecated:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   uninterpreted_option:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for ServiceOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ServiceOptions {
                    deprecated: ref __self_0_0,
                    uninterpreted_option: ref __self_0_1 } => {
                        let mut builder =
                            __arg_0.debug_struct("ServiceOptions");
                        let _ = builder.field("deprecated", &&(*__self_0_0));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for ServiceOptions {
            #[inline]
            fn eq(&self, __arg_0: &ServiceOptions) -> bool {
                match *__arg_0 {
                    ServiceOptions {
                    deprecated: ref __self_1_0,
                    uninterpreted_option: ref __self_1_1 } =>
                    match *self {
                        ServiceOptions {
                        deprecated: ref __self_0_0,
                        uninterpreted_option: ref __self_0_1 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &ServiceOptions) -> bool {
                match *__arg_0 {
                    ServiceOptions {
                    deprecated: ref __self_1_0,
                    uninterpreted_option: ref __self_1_1 } =>
                    match *self {
                        ServiceOptions {
                        deprecated: ref __self_0_0,
                        uninterpreted_option: ref __self_0_1 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod ServiceOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for ServiceOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(33u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        33u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       558u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("ServiceOptions",
                                                     "deprecated");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("ServiceOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(33u32,
                                                                                                value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for ServiceOptions {
                fn default() -> ServiceOptions {
                    ServiceOptions{deprecated: ::std::option::Option::None,
                                   uninterpreted_option:
                                       ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl ServiceOptions {
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
            }
        }
        pub struct MethodOptions {
            /// Is this method deprecated?
            /// Depending on the target platform, this can emit Deprecated annotations
            /// for the method, or it will be completely ignored; in the very least,
            /// this is a formalization for deprecating methods.
            #[prost(bool, optional, tag = "33")]
            pub deprecated: Option<bool>,
            #[prost(enumeration = "method_options::IdempotencyLevel",
                    optional,
                    tag = "34")]
            pub idempotency_level: Option<i32>,
            /// The parser stores options it doesn't recognize here. See above.
            #[prost(message, repeated, tag = "999")]
            pub uninterpreted_option: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for MethodOptions {
            #[inline]
            fn clone(&self) -> MethodOptions {
                match *self {
                    MethodOptions {
                    deprecated: ref __self_0_0,
                    idempotency_level: ref __self_0_1,
                    uninterpreted_option: ref __self_0_2 } =>
                    MethodOptions{deprecated:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),
                                  idempotency_level:
                                      ::std::clone::Clone::clone(&(*__self_0_1)),
                                  uninterpreted_option:
                                      ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for MethodOptions {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    MethodOptions {
                    deprecated: ref __self_0_0,
                    idempotency_level: ref __self_0_1,
                    uninterpreted_option: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("MethodOptions");
                        let _ = builder.field("deprecated", &&(*__self_0_0));
                        let _ =
                            builder.field("idempotency_level",
                                          &&(*__self_0_1));
                        let _ =
                            builder.field("uninterpreted_option",
                                          &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for MethodOptions {
            #[inline]
            fn eq(&self, __arg_0: &MethodOptions) -> bool {
                match *__arg_0 {
                    MethodOptions {
                    deprecated: ref __self_1_0,
                    idempotency_level: ref __self_1_1,
                    uninterpreted_option: ref __self_1_2 } =>
                    match *self {
                        MethodOptions {
                        deprecated: ref __self_0_0,
                        idempotency_level: ref __self_0_1,
                        uninterpreted_option: ref __self_0_2 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &MethodOptions) -> bool {
                match *__arg_0 {
                    MethodOptions {
                    deprecated: ref __self_1_0,
                    idempotency_level: ref __self_1_1,
                    uninterpreted_option: ref __self_1_2 } =>
                    match *self {
                        MethodOptions {
                        deprecated: ref __self_0_0,
                        idempotency_level: ref __self_0_1,
                        uninterpreted_option: ref __self_0_2 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod MethodOptions_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for MethodOptions {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    if let ::std::option::Option::Some(ref value) =
                           self.deprecated {
                        _prost::encoding::bool::encode(33u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.idempotency_level {
                        _prost::encoding::int32::encode(34u32, value, buf);
                    }
                    for msg in &self.uninterpreted_option {
                        _prost::encoding::message::encode(999u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        33u32 =>
                        {
                            if self.deprecated.is_none() {
                                self.deprecated = Some(Default::default());
                            }
                            match self.deprecated {
                                Some(ref mut value) =>
                                _prost::encoding::bool::merge(wire_type,
                                                              value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       575u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodOptions",
                                                     "deprecated");
                                          error
                                      }),
                        34u32 =>
                        {
                            if self.idempotency_level.is_none() {
                                self.idempotency_level =
                                    Some(Default::default());
                            }
                            match self.idempotency_level {
                                Some(ref mut value) =>
                                _prost::encoding::int32::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       575u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("MethodOptions",
                                                     "idempotency_level");
                                          error
                                      }),
                        999u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.uninterpreted_option,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("MethodOptions",
                                                                                                  "uninterpreted_option");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        self.deprecated.as_ref().map_or(0,
                                                        |value|
                                                            _prost::encoding::bool::encoded_len(33u32,
                                                                                                value))
                        +
                        self.idempotency_level.as_ref().map_or(0,
                                                               |value|
                                                                   _prost::encoding::int32::encoded_len(34u32,
                                                                                                        value))
                        +
                        _prost::encoding::message::encoded_len_repeated(999u32,
                                                                        &self.uninterpreted_option)
                }
            }
            impl Default for MethodOptions {
                fn default() -> MethodOptions {
                    MethodOptions{deprecated: ::std::option::Option::None,
                                  idempotency_level:
                                      ::std::option::Option::None,
                                  uninterpreted_option:
                                      ::std::default::Default::default(),}
                }
            }
            #[allow(dead_code)]
            impl MethodOptions {
                pub fn deprecated(&self) -> bool {
                    match self.deprecated {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => false,
                    }
                }
                pub fn idempotency_level(&self)
                 -> ::std::option::Option<method_options::IdempotencyLevel> {
                    self.idempotency_level.and_then(method_options::IdempotencyLevel::from_i32)
                }
                pub fn set_idempotency_level(&mut self,
                                             value:
                                                 method_options::IdempotencyLevel) {
                    self.idempotency_level =
                        ::std::option::Option::Some(value as i32);
                }
            }
        }
        pub mod method_options {
            /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
            /// or neither? HTTP based RPC implementation may choose GET verb for safe
            /// methods, and PUT verb for idempotent methods instead of the default POST.
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub enum IdempotencyLevel {
                IdempotencyUnknown = 0,

                /// implies idempotent
                NoSideEffects = 1,

                /// idempotent, but may have side effects
                Idempotent = 2,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for IdempotencyLevel {
                #[inline]
                fn clone(&self) -> IdempotencyLevel { { *self } }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for IdempotencyLevel { }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for IdempotencyLevel {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match (&*self,) {
                        (&IdempotencyLevel::IdempotencyUnknown,) => {
                            let mut builder =
                                __arg_0.debug_tuple("IdempotencyUnknown");
                            builder.finish()
                        }
                        (&IdempotencyLevel::NoSideEffects,) => {
                            let mut builder =
                                __arg_0.debug_tuple("NoSideEffects");
                            builder.finish()
                        }
                        (&IdempotencyLevel::Idempotent,) => {
                            let mut builder =
                                __arg_0.debug_tuple("Idempotent");
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for IdempotencyLevel {
                #[inline]
                fn eq(&self, __arg_0: &IdempotencyLevel) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::std::intrinsics::discriminant_value(&*__arg_0)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*__arg_0) { _ => true, }
                        } else { false }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::Eq for IdempotencyLevel {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () { { } }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod IdempotencyLevel_ENUMERATION {
                extern crate bytes as _bytes;
                extern crate prost as _prost;
                use super::*;
                impl IdempotencyLevel {
                    #[doc =
                          "Returns `true` if `value` is a variant of `IdempotencyLevel`."]
                    pub fn is_valid(value: i32) -> bool {
                        match value {
                            0 => true,
                            1 => true,
                            2 => true,
                            _ => false,
                        }
                    }
                    #[doc =
                          "Converts an `i32` to a `IdempotencyLevel`, or `None` if `value` is not a valid variant."]
                    pub fn from_i32(value: i32)
                     -> ::std::option::Option<IdempotencyLevel> {
                        match value {
                            0 =>
                            ::std::option::Option::Some(IdempotencyLevel::IdempotencyUnknown),
                            1 =>
                            ::std::option::Option::Some(IdempotencyLevel::NoSideEffects),
                            2 =>
                            ::std::option::Option::Some(IdempotencyLevel::Idempotent),
                            _ => ::std::option::Option::None,
                        }
                    }
                }
                impl ::std::default::Default for IdempotencyLevel {
                    fn default() -> IdempotencyLevel {
                        IdempotencyLevel::IdempotencyUnknown
                    }
                }
                impl ::std::convert::From<IdempotencyLevel> for i32 {
                    fn from(value: IdempotencyLevel) -> i32 { value as i32 }
                }
            }
        }
        /// A message representing a option the parser does not recognize. This only
        /// appears in options protos created by the compiler::Parser class.
        /// DescriptorPool resolves these when building Descriptor objects. Therefore,
        /// options protos in descriptor objects (e.g. returned by Descriptor::options(),
        /// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
        /// in them.
        pub struct UninterpretedOption {
            #[prost(message, repeated, tag = "2")]
            pub name: Vec<uninterpreted_option::NamePart>,
            /// The value of the uninterpreted option, in whatever type the tokenizer
            /// identified it as during parsing. Exactly one of these should be set.
            #[prost(string, optional, tag = "3")]
            pub identifier_value: Option<String>,
            #[prost(uint64, optional, tag = "4")]
            pub positive_int_value: Option<u64>,
            #[prost(int64, optional, tag = "5")]
            pub negative_int_value: Option<i64>,
            #[prost(double, optional, tag = "6")]
            pub double_value: Option<f64>,
            #[prost(bytes, optional, tag = "7")]
            pub string_value: Option<Vec<u8>>,
            #[prost(string, optional, tag = "8")]
            pub aggregate_value: Option<String>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for UninterpretedOption {
            #[inline]
            fn clone(&self) -> UninterpretedOption {
                match *self {
                    UninterpretedOption {
                    name: ref __self_0_0,
                    identifier_value: ref __self_0_1,
                    positive_int_value: ref __self_0_2,
                    negative_int_value: ref __self_0_3,
                    double_value: ref __self_0_4,
                    string_value: ref __self_0_5,
                    aggregate_value: ref __self_0_6 } =>
                    UninterpretedOption{name:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        identifier_value:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        positive_int_value:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),
                                        negative_int_value:
                                            ::std::clone::Clone::clone(&(*__self_0_3)),
                                        double_value:
                                            ::std::clone::Clone::clone(&(*__self_0_4)),
                                        string_value:
                                            ::std::clone::Clone::clone(&(*__self_0_5)),
                                        aggregate_value:
                                            ::std::clone::Clone::clone(&(*__self_0_6)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for UninterpretedOption {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    UninterpretedOption {
                    name: ref __self_0_0,
                    identifier_value: ref __self_0_1,
                    positive_int_value: ref __self_0_2,
                    negative_int_value: ref __self_0_3,
                    double_value: ref __self_0_4,
                    string_value: ref __self_0_5,
                    aggregate_value: ref __self_0_6 } => {
                        let mut builder =
                            __arg_0.debug_struct("UninterpretedOption");
                        let _ = builder.field("name", &&(*__self_0_0));
                        let _ =
                            builder.field("identifier_value",
                                          &&(*__self_0_1));
                        let _ =
                            builder.field("positive_int_value",
                                          &&(*__self_0_2));
                        let _ =
                            builder.field("negative_int_value",
                                          &&(*__self_0_3));
                        let _ =
                            builder.field("double_value", &&(*__self_0_4));
                        let _ =
                            builder.field("string_value", &&(*__self_0_5));
                        let _ =
                            builder.field("aggregate_value", &&(*__self_0_6));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for UninterpretedOption {
            #[inline]
            fn eq(&self, __arg_0: &UninterpretedOption) -> bool {
                match *__arg_0 {
                    UninterpretedOption {
                    name: ref __self_1_0,
                    identifier_value: ref __self_1_1,
                    positive_int_value: ref __self_1_2,
                    negative_int_value: ref __self_1_3,
                    double_value: ref __self_1_4,
                    string_value: ref __self_1_5,
                    aggregate_value: ref __self_1_6 } =>
                    match *self {
                        UninterpretedOption {
                        name: ref __self_0_0,
                        identifier_value: ref __self_0_1,
                        positive_int_value: ref __self_0_2,
                        negative_int_value: ref __self_0_3,
                        double_value: ref __self_0_4,
                        string_value: ref __self_0_5,
                        aggregate_value: ref __self_0_6 } =>
                        true && (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &UninterpretedOption) -> bool {
                match *__arg_0 {
                    UninterpretedOption {
                    name: ref __self_1_0,
                    identifier_value: ref __self_1_1,
                    positive_int_value: ref __self_1_2,
                    negative_int_value: ref __self_1_3,
                    double_value: ref __self_1_4,
                    string_value: ref __self_1_5,
                    aggregate_value: ref __self_1_6 } =>
                    match *self {
                        UninterpretedOption {
                        name: ref __self_0_0,
                        identifier_value: ref __self_0_1,
                        positive_int_value: ref __self_0_2,
                        negative_int_value: ref __self_0_3,
                        double_value: ref __self_0_4,
                        string_value: ref __self_0_5,
                        aggregate_value: ref __self_0_6 } =>
                        false || (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod UninterpretedOption_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for UninterpretedOption {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.name {
                        _prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.identifier_value {
                        _prost::encoding::string::encode(3u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.positive_int_value {
                        _prost::encoding::uint64::encode(4u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.negative_int_value {
                        _prost::encoding::int64::encode(5u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.double_value {
                        _prost::encoding::double::encode(6u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.string_value {
                        _prost::encoding::bytes::encode(7u32, value, buf);
                    }
                    if let ::std::option::Option::Some(ref value) =
                           self.aggregate_value {
                        _prost::encoding::string::encode(8u32, value, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        2u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.name,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("UninterpretedOption",
                                                                                                  "name");
                                                                                       error
                                                                                   }),
                        3u32 =>
                        {
                            if self.identifier_value.is_none() {
                                self.identifier_value =
                                    Some(Default::default());
                            }
                            match self.identifier_value {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "identifier_value");
                                          error
                                      }),
                        4u32 =>
                        {
                            if self.positive_int_value.is_none() {
                                self.positive_int_value =
                                    Some(Default::default());
                            }
                            match self.positive_int_value {
                                Some(ref mut value) =>
                                _prost::encoding::uint64::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "positive_int_value");
                                          error
                                      }),
                        5u32 =>
                        {
                            if self.negative_int_value.is_none() {
                                self.negative_int_value =
                                    Some(Default::default());
                            }
                            match self.negative_int_value {
                                Some(ref mut value) =>
                                _prost::encoding::int64::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "negative_int_value");
                                          error
                                      }),
                        6u32 =>
                        {
                            if self.double_value.is_none() {
                                self.double_value = Some(Default::default());
                            }
                            match self.double_value {
                                Some(ref mut value) =>
                                _prost::encoding::double::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "double_value");
                                          error
                                      }),
                        7u32 =>
                        {
                            if self.string_value.is_none() {
                                self.string_value = Some(Default::default());
                            }
                            match self.string_value {
                                Some(ref mut value) =>
                                _prost::encoding::bytes::merge(wire_type,
                                                               value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "string_value");
                                          error
                                      }),
                        8u32 =>
                        {
                            if self.aggregate_value.is_none() {
                                self.aggregate_value =
                                    Some(Default::default());
                            }
                            match self.aggregate_value {
                                Some(ref mut value) =>
                                _prost::encoding::string::merge(wire_type,
                                                                value, buf),
                                _ => {
                                    {
                                        ::rt::begin_panic_new("internal error: entered unreachable code",
                                                              {
                                                                  static _FILE_LINE_COL:
                                                                         (&'static str,
                                                                          u32,
                                                                          u32)
                                                                         =
                                                                      ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                       613u32,
                                                                       34u32);
                                                                  &_FILE_LINE_COL
                                                              })
                                    }
                                }
                            }
                        }.map_err(|mut error|
                                      {
                                          error.push("UninterpretedOption",
                                                     "aggregate_value");
                                          error
                                      }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(2u32,
                                                                        &self.name)
                        +
                        self.identifier_value.as_ref().map_or(0,
                                                              |value|
                                                                  _prost::encoding::string::encoded_len(3u32,
                                                                                                        value))
                        +
                        self.positive_int_value.as_ref().map_or(0,
                                                                |value|
                                                                    _prost::encoding::uint64::encoded_len(4u32,
                                                                                                          value))
                        +
                        self.negative_int_value.as_ref().map_or(0,
                                                                |value|
                                                                    _prost::encoding::int64::encoded_len(5u32,
                                                                                                         value))
                        +
                        self.double_value.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::double::encoded_len(6u32,
                                                                                                    value))
                        +
                        self.string_value.as_ref().map_or(0,
                                                          |value|
                                                              _prost::encoding::bytes::encoded_len(7u32,
                                                                                                   value))
                        +
                        self.aggregate_value.as_ref().map_or(0,
                                                             |value|
                                                                 _prost::encoding::string::encoded_len(8u32,
                                                                                                       value))
                }
            }
            impl Default for UninterpretedOption {
                fn default() -> UninterpretedOption {
                    UninterpretedOption{name:
                                            ::std::default::Default::default(),
                                        identifier_value:
                                            ::std::option::Option::None,
                                        positive_int_value:
                                            ::std::option::Option::None,
                                        negative_int_value:
                                            ::std::option::Option::None,
                                        double_value:
                                            ::std::option::Option::None,
                                        string_value:
                                            ::std::option::Option::None,
                                        aggregate_value:
                                            ::std::option::Option::None,}
                }
            }
            #[allow(dead_code)]
            impl UninterpretedOption {
                pub fn identifier_value(&self) -> &str {
                    match self.identifier_value {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
                pub fn positive_int_value(&self) -> u64 {
                    match self.positive_int_value {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0u64,
                    }
                }
                pub fn negative_int_value(&self) -> i64 {
                    match self.negative_int_value {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0i64,
                    }
                }
                pub fn double_value(&self) -> f64 {
                    match self.double_value {
                        ::std::option::Option::Some(val) => val,
                        ::std::option::Option::None => 0f64,
                    }
                }
                pub fn string_value(&self) -> &[u8] {
                    match self.string_value {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => b"",
                    }
                }
                pub fn aggregate_value(&self) -> &str {
                    match self.aggregate_value {
                        ::std::option::Option::Some(ref val) => &val[..],
                        ::std::option::Option::None => "",
                    }
                }
            }
        }
        pub mod uninterpreted_option {
            /// The name of the uninterpreted option.  Each string represents a segment in
            /// a dot-separated name.  is_extension is true iff a segment represents an
            /// extension (denoted with parentheses in options specs in .proto files).
            /// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
            /// "foo.(bar.baz).qux".
            pub struct NamePart {
                #[prost(string, required, tag = "1")]
                pub name_part: String,
                #[prost(bool, required, tag = "2")]
                pub is_extension: bool,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for NamePart {
                #[inline]
                fn clone(&self) -> NamePart {
                    match *self {
                        NamePart {
                        name_part: ref __self_0_0,
                        is_extension: ref __self_0_1 } =>
                        NamePart{name_part:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 is_extension:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for NamePart {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match *self {
                        NamePart {
                        name_part: ref __self_0_0,
                        is_extension: ref __self_0_1 } => {
                            let mut builder =
                                __arg_0.debug_struct("NamePart");
                            let _ =
                                builder.field("name_part", &&(*__self_0_0));
                            let _ =
                                builder.field("is_extension",
                                              &&(*__self_0_1));
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for NamePart {
                #[inline]
                fn eq(&self, __arg_0: &NamePart) -> bool {
                    match *__arg_0 {
                        NamePart {
                        name_part: ref __self_1_0,
                        is_extension: ref __self_1_1 } =>
                        match *self {
                            NamePart {
                            name_part: ref __self_0_0,
                            is_extension: ref __self_0_1 } =>
                            true && (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &NamePart) -> bool {
                    match *__arg_0 {
                        NamePart {
                        name_part: ref __self_1_0,
                        is_extension: ref __self_1_1 } =>
                        match *self {
                            NamePart {
                            name_part: ref __self_0_0,
                            is_extension: ref __self_0_1 } =>
                            false || (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod NamePart_MESSAGE {
                extern crate prost as _prost;
                extern crate bytes as _bytes;
                use super::*;
                impl _prost::Message for NamePart {
                    fn encode_raw<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        _prost::encoding::string::encode(1u32,
                                                         &self.name_part,
                                                         buf);
                        _prost::encoding::bool::encode(2u32,
                                                       &self.is_extension,
                                                       buf);
                    }
                    fn merge_field<B>(&mut self, buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        let (tag, wire_type) =
                            _prost::encoding::decode_key(buf)?;
                        match tag {
                            1u32 =>
                            _prost::encoding::string::merge(wire_type,
                                                            &mut self.name_part,
                                                            buf).map_err(|mut error|
                                                                             {
                                                                                 error.push("NamePart",
                                                                                            "name_part");
                                                                                 error
                                                                             }),
                            2u32 =>
                            _prost::encoding::bool::merge(wire_type,
                                                          &mut self.is_extension,
                                                          buf).map_err(|mut error|
                                                                           {
                                                                               error.push("NamePart",
                                                                                          "is_extension");
                                                                               error
                                                                           }),
                            _ => _prost::encoding::skip_field(wire_type, buf),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 +
                            _prost::encoding::string::encoded_len(1u32,
                                                                  &self.name_part)
                            +
                            _prost::encoding::bool::encoded_len(2u32,
                                                                &self.is_extension)
                    }
                }
                impl Default for NamePart {
                    fn default() -> NamePart {
                        NamePart{name_part: ::std::string::String::new(),
                                 is_extension: false,}
                    }
                }
            }
        }
        /// Encapsulates information about the original source file from which a
        /// FileDescriptorProto was generated.
        pub struct SourceCodeInfo {
            /// A Location identifies a piece of source code in a .proto file which
            /// corresponds to a particular definition.  This information is intended
            /// to be useful to IDEs, code indexers, documentation generators, and similar
            /// tools.
            ///
            /// For example, say we have a file like:
            ///   message Foo {
            ///     optional string foo = 1;
            ///   }
            /// Let's look at just the field definition:
            ///   optional string foo = 1;
            ///   ^       ^^     ^^  ^  ^^^
            ///   a       bc     de  f  ghi
            /// We have the following locations:
            ///   span   path               represents
            ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
            ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
            ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
            ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
            ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).
            ///
            /// Notes:
            /// - A location may refer to a repeated field itself (i.e. not to any
            ///   particular index within it).  This is used whenever a set of elements are
            ///   logically enclosed in a single code segment.  For example, an entire
            ///   extend block (possibly containing multiple extension definitions) will
            ///   have an outer location whose path refers to the "extensions" repeated
            ///   field without an index.
            /// - Multiple locations may have the same path.  This happens when a single
            ///   logical declaration is spread out across multiple places.  The most
            ///   obvious example is the "extend" block again -- there may be multiple
            ///   extend blocks in the same scope, each of which will have the same path.
            /// - A location's span is not always a subset of its parent's span.  For
            ///   example, the "extendee" of an extension declaration appears at the
            ///   beginning of the "extend" block and is shared by all extensions within
            ///   the block.
            /// - Just because a location's span is a subset of some other location's span
            ///   does not mean that it is a descendent.  For example, a "group" defines
            ///   both a type and a field in a single declaration.  Thus, the locations
            ///   corresponding to the type and field and their components will overlap.
            /// - Code which tries to interpret locations should probably be designed to
            ///   ignore those that it doesn't understand, as more types of locations could
            ///   be recorded in the future.
            #[prost(message, repeated, tag = "1")]
            pub location: Vec<source_code_info::Location>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for SourceCodeInfo {
            #[inline]
            fn clone(&self) -> SourceCodeInfo {
                match *self {
                    SourceCodeInfo { location: ref __self_0_0 } =>
                    SourceCodeInfo{location:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for SourceCodeInfo {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    SourceCodeInfo { location: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("SourceCodeInfo");
                        let _ = builder.field("location", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for SourceCodeInfo {
            #[inline]
            fn eq(&self, __arg_0: &SourceCodeInfo) -> bool {
                match *__arg_0 {
                    SourceCodeInfo { location: ref __self_1_0 } =>
                    match *self {
                        SourceCodeInfo { location: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &SourceCodeInfo) -> bool {
                match *__arg_0 {
                    SourceCodeInfo { location: ref __self_1_0 } =>
                    match *self {
                        SourceCodeInfo { location: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod SourceCodeInfo_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for SourceCodeInfo {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.location {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.location,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("SourceCodeInfo",
                                                                                                  "location");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(1u32,
                                                                        &self.location)
                }
            }
            impl Default for SourceCodeInfo {
                fn default() -> SourceCodeInfo {
                    SourceCodeInfo{location:
                                       ::std::default::Default::default(),}
                }
            }
        }
        pub mod source_code_info {
            pub struct Location {
                /// Identifies which part of the FileDescriptorProto was defined at this
                /// location.
                ///
                /// Each element is a field number or an index.  They form a path from
                /// the root FileDescriptorProto to the place where the definition.  For
                /// example, this path:
                ///   [ 4, 3, 2, 7, 1 ]
                /// refers to:
                ///   file.message_type(3)  // 4, 3
                ///       .field(7)         // 2, 7
                ///       .name()           // 1
                /// This is because FileDescriptorProto.message_type has field number 4:
                ///   repeated DescriptorProto message_type = 4;
                /// and DescriptorProto.field has field number 2:
                ///   repeated FieldDescriptorProto field = 2;
                /// and FieldDescriptorProto.name has field number 1:
                ///   optional string name = 1;
                ///
                /// Thus, the above path gives the location of a field name.  If we removed
                /// the last element:
                ///   [ 4, 3, 2, 7 ]
                /// this path refers to the whole field declaration (from the beginning
                /// of the label to the terminating semicolon).
                #[prost(int32, repeated, tag = "1")]
                pub path: Vec<i32>,
                /// Always has exactly three or four elements: start line, start column,
                /// end line (optional, otherwise assumed same as start line), end column.
                /// These are packed into a single field for efficiency.  Note that line
                /// and column numbers are zero-based -- typically you will want to add
                /// 1 to each before displaying to a user.
                #[prost(int32, repeated, tag = "2")]
                pub span: Vec<i32>,
                /// If this SourceCodeInfo represents a complete declaration, these are any
                /// comments appearing before and after the declaration which appear to be
                /// attached to the declaration.
                ///
                /// A series of line comments appearing on consecutive lines, with no other
                /// tokens appearing on those lines, will be treated as a single comment.
                ///
                /// leading_detached_comments will keep paragraphs of comments that appear
                /// before (but not connected to) the current element. Each paragraph,
                /// separated by empty lines, will be one comment element in the repeated
                /// field.
                ///
                /// Only the comment content is provided; comment markers (e.g. //) are
                /// stripped out.  For block comments, leading whitespace and an asterisk
                /// will be stripped from the beginning of each line other than the first.
                /// Newlines are included in the output.
                ///
                /// Examples:
                ///
                ///   optional int32 foo = 1;  // Comment attached to foo.
                ///   // Comment attached to bar.
                ///   optional int32 bar = 2;
                ///
                ///   optional string baz = 3;
                ///   // Comment attached to baz.
                ///   // Another line attached to baz.
                ///
                ///   // Comment attached to qux.
                ///   //
                ///   // Another line attached to qux.
                ///   optional double qux = 4;
                ///
                ///   // Detached comment for corge. This is not leading or trailing comments
                ///   // to qux or corge because there are blank lines separating it from
                ///   // both.
                ///
                ///   // Detached comment for corge paragraph 2.
                ///
                ///   optional string corge = 5;
                ///   /* Block comment attached
                ///    * to corge.  Leading asterisks
                ///    * will be removed. */
                ///   /* Block comment attached to
                ///    * grault. */
                ///   optional int32 grault = 6;
                ///
                ///   // ignored detached comments.
                #[prost(string, optional, tag = "3")]
                pub leading_comments: Option<String>,
                #[prost(string, optional, tag = "4")]
                pub trailing_comments: Option<String>,
                #[prost(string, repeated, tag = "6")]
                pub leading_detached_comments: Vec<String>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Location {
                #[inline]
                fn clone(&self) -> Location {
                    match *self {
                        Location {
                        path: ref __self_0_0,
                        span: ref __self_0_1,
                        leading_comments: ref __self_0_2,
                        trailing_comments: ref __self_0_3,
                        leading_detached_comments: ref __self_0_4 } =>
                        Location{path:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 span:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),
                                 leading_comments:
                                     ::std::clone::Clone::clone(&(*__self_0_2)),
                                 trailing_comments:
                                     ::std::clone::Clone::clone(&(*__self_0_3)),
                                 leading_detached_comments:
                                     ::std::clone::Clone::clone(&(*__self_0_4)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Location {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match *self {
                        Location {
                        path: ref __self_0_0,
                        span: ref __self_0_1,
                        leading_comments: ref __self_0_2,
                        trailing_comments: ref __self_0_3,
                        leading_detached_comments: ref __self_0_4 } => {
                            let mut builder =
                                __arg_0.debug_struct("Location");
                            let _ = builder.field("path", &&(*__self_0_0));
                            let _ = builder.field("span", &&(*__self_0_1));
                            let _ =
                                builder.field("leading_comments",
                                              &&(*__self_0_2));
                            let _ =
                                builder.field("trailing_comments",
                                              &&(*__self_0_3));
                            let _ =
                                builder.field("leading_detached_comments",
                                              &&(*__self_0_4));
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Location {
                #[inline]
                fn eq(&self, __arg_0: &Location) -> bool {
                    match *__arg_0 {
                        Location {
                        path: ref __self_1_0,
                        span: ref __self_1_1,
                        leading_comments: ref __self_1_2,
                        trailing_comments: ref __self_1_3,
                        leading_detached_comments: ref __self_1_4 } =>
                        match *self {
                            Location {
                            path: ref __self_0_0,
                            span: ref __self_0_1,
                            leading_comments: ref __self_0_2,
                            trailing_comments: ref __self_0_3,
                            leading_detached_comments: ref __self_0_4 } =>
                            true && (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1) &&
                                (*__self_0_2) == (*__self_1_2) &&
                                (*__self_0_3) == (*__self_1_3) &&
                                (*__self_0_4) == (*__self_1_4),
                        },
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &Location) -> bool {
                    match *__arg_0 {
                        Location {
                        path: ref __self_1_0,
                        span: ref __self_1_1,
                        leading_comments: ref __self_1_2,
                        trailing_comments: ref __self_1_3,
                        leading_detached_comments: ref __self_1_4 } =>
                        match *self {
                            Location {
                            path: ref __self_0_0,
                            span: ref __self_0_1,
                            leading_comments: ref __self_0_2,
                            trailing_comments: ref __self_0_3,
                            leading_detached_comments: ref __self_0_4 } =>
                            false || (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1) ||
                                (*__self_0_2) != (*__self_1_2) ||
                                (*__self_0_3) != (*__self_1_3) ||
                                (*__self_0_4) != (*__self_1_4),
                        },
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Location_MESSAGE {
                extern crate prost as _prost;
                extern crate bytes as _bytes;
                use super::*;
                impl _prost::Message for Location {
                    fn encode_raw<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        _prost::encoding::int32::encode_packed(1u32,
                                                               &self.path,
                                                               buf);
                        _prost::encoding::int32::encode_packed(2u32,
                                                               &self.span,
                                                               buf);
                        if let ::std::option::Option::Some(ref value) =
                               self.leading_comments {
                            _prost::encoding::string::encode(3u32, value,
                                                             buf);
                        }
                        if let ::std::option::Option::Some(ref value) =
                               self.trailing_comments {
                            _prost::encoding::string::encode(4u32, value,
                                                             buf);
                        }
                        _prost::encoding::string::encode_repeated(6u32,
                                                                  &self.leading_detached_comments,
                                                                  buf);
                    }
                    fn merge_field<B>(&mut self, buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        let (tag, wire_type) =
                            _prost::encoding::decode_key(buf)?;
                        match tag {
                            1u32 =>
                            _prost::encoding::int32::merge_repeated(wire_type,
                                                                    &mut self.path,
                                                                    buf).map_err(|mut error|
                                                                                     {
                                                                                         error.push("Location",
                                                                                                    "path");
                                                                                         error
                                                                                     }),
                            2u32 =>
                            _prost::encoding::int32::merge_repeated(wire_type,
                                                                    &mut self.span,
                                                                    buf).map_err(|mut error|
                                                                                     {
                                                                                         error.push("Location",
                                                                                                    "span");
                                                                                         error
                                                                                     }),
                            3u32 =>
                            {
                                if self.leading_comments.is_none() {
                                    self.leading_comments =
                                        Some(Default::default());
                                }
                                match self.leading_comments {
                                    Some(ref mut value) =>
                                    _prost::encoding::string::merge(wire_type,
                                                                    value,
                                                                    buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           700u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("Location",
                                                         "leading_comments");
                                              error
                                          }),
                            4u32 =>
                            {
                                if self.trailing_comments.is_none() {
                                    self.trailing_comments =
                                        Some(Default::default());
                                }
                                match self.trailing_comments {
                                    Some(ref mut value) =>
                                    _prost::encoding::string::merge(wire_type,
                                                                    value,
                                                                    buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           700u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("Location",
                                                         "trailing_comments");
                                              error
                                          }),
                            6u32 =>
                            _prost::encoding::string::merge_repeated(wire_type,
                                                                     &mut self.leading_detached_comments,
                                                                     buf).map_err(|mut error|
                                                                                      {
                                                                                          error.push("Location",
                                                                                                     "leading_detached_comments");
                                                                                          error
                                                                                      }),
                            _ => _prost::encoding::skip_field(wire_type, buf),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 +
                            _prost::encoding::int32::encoded_len_packed(1u32,
                                                                        &self.path)
                            +
                            _prost::encoding::int32::encoded_len_packed(2u32,
                                                                        &self.span)
                            +
                            self.leading_comments.as_ref().map_or(0,
                                                                  |value|
                                                                      _prost::encoding::string::encoded_len(3u32,
                                                                                                            value))
                            +
                            self.trailing_comments.as_ref().map_or(0,
                                                                   |value|
                                                                       _prost::encoding::string::encoded_len(4u32,
                                                                                                             value))
                            +
                            _prost::encoding::string::encoded_len_repeated(6u32,
                                                                           &self.leading_detached_comments)
                    }
                }
                impl Default for Location {
                    fn default() -> Location {
                        Location{path: ::std::vec::Vec::new(),
                                 span: ::std::vec::Vec::new(),
                                 leading_comments:
                                     ::std::option::Option::None,
                                 trailing_comments:
                                     ::std::option::Option::None,
                                 leading_detached_comments:
                                     ::std::vec::Vec::new(),}
                    }
                }
                #[allow(dead_code)]
                impl Location {
                    pub fn leading_comments(&self) -> &str {
                        match self.leading_comments {
                            ::std::option::Option::Some(ref val) => &val[..],
                            ::std::option::Option::None => "",
                        }
                    }
                    pub fn trailing_comments(&self) -> &str {
                        match self.trailing_comments {
                            ::std::option::Option::Some(ref val) => &val[..],
                            ::std::option::Option::None => "",
                        }
                    }
                }
            }
        }
        /// Describes the relationship between generated code and its original source
        /// file. A GeneratedCodeInfo message is associated with only one generated
        /// source file, but may contain references to different source .proto files.
        pub struct GeneratedCodeInfo {
            /// An Annotation connects some span of text in generated code to an element
            /// of its generating .proto file.
            #[prost(message, repeated, tag = "1")]
            pub annotation: Vec<generated_code_info::Annotation>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for GeneratedCodeInfo {
            #[inline]
            fn clone(&self) -> GeneratedCodeInfo {
                match *self {
                    GeneratedCodeInfo { annotation: ref __self_0_0 } =>
                    GeneratedCodeInfo{annotation:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for GeneratedCodeInfo {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    GeneratedCodeInfo { annotation: ref __self_0_0 } => {
                        let mut builder =
                            __arg_0.debug_struct("GeneratedCodeInfo");
                        let _ = builder.field("annotation", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for GeneratedCodeInfo {
            #[inline]
            fn eq(&self, __arg_0: &GeneratedCodeInfo) -> bool {
                match *__arg_0 {
                    GeneratedCodeInfo { annotation: ref __self_1_0 } =>
                    match *self {
                        GeneratedCodeInfo { annotation: ref __self_0_0 } =>
                        true && (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &GeneratedCodeInfo) -> bool {
                match *__arg_0 {
                    GeneratedCodeInfo { annotation: ref __self_1_0 } =>
                    match *self {
                        GeneratedCodeInfo { annotation: ref __self_0_0 } =>
                        false || (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[allow(non_snake_case, unused_attributes)]
        mod GeneratedCodeInfo_MESSAGE {
            extern crate prost as _prost;
            extern crate bytes as _bytes;
            use super::*;
            impl _prost::Message for GeneratedCodeInfo {
                fn encode_raw<B>(&self, buf: &mut B) where B: _bytes::BufMut {
                    for msg in &self.annotation {
                        _prost::encoding::message::encode(1u32, msg, buf);
                    }
                }
                fn merge_field<B>(&mut self, buf: &mut B)
                 -> ::std::result::Result<(), _prost::DecodeError> where
                 B: _bytes::Buf {
                    let (tag, wire_type) = _prost::encoding::decode_key(buf)?;
                    match tag {
                        1u32 =>
                        _prost::encoding::message::merge_repeated(wire_type,
                                                                  &mut self.annotation,
                                                                  buf).map_err(|mut error|
                                                                                   {
                                                                                       error.push("GeneratedCodeInfo",
                                                                                                  "annotation");
                                                                                       error
                                                                                   }),
                        _ => _prost::encoding::skip_field(wire_type, buf),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 +
                        _prost::encoding::message::encoded_len_repeated(1u32,
                                                                        &self.annotation)
                }
            }
            impl Default for GeneratedCodeInfo {
                fn default() -> GeneratedCodeInfo {
                    GeneratedCodeInfo{annotation:
                                          ::std::default::Default::default(),}
                }
            }
        }
        pub mod generated_code_info {
            pub struct Annotation {
                /// Identifies the element in the original source .proto file. This field
                /// is formatted the same as SourceCodeInfo.Location.path.
                #[prost(int32, repeated, tag = "1")]
                pub path: Vec<i32>,
                /// Identifies the filesystem path to the original source .proto.
                #[prost(string, optional, tag = "2")]
                pub source_file: Option<String>,
                /// Identifies the starting offset in bytes in the generated code
                /// that relates to the identified object.
                #[prost(int32, optional, tag = "3")]
                pub begin: Option<i32>,
                /// Identifies the ending offset in bytes in the generated code that
                /// relates to the identified offset. The end offset should be one past
                /// the last relevant byte (so the length of the text = end - begin).
                #[prost(int32, optional, tag = "4")]
                pub end: Option<i32>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Annotation {
                #[inline]
                fn clone(&self) -> Annotation {
                    match *self {
                        Annotation {
                        path: ref __self_0_0,
                        source_file: ref __self_0_1,
                        begin: ref __self_0_2,
                        end: ref __self_0_3 } =>
                        Annotation{path:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   source_file:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),
                                   begin:
                                       ::std::clone::Clone::clone(&(*__self_0_2)),
                                   end:
                                       ::std::clone::Clone::clone(&(*__self_0_3)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Annotation {
                fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
                 -> ::std::fmt::Result {
                    match *self {
                        Annotation {
                        path: ref __self_0_0,
                        source_file: ref __self_0_1,
                        begin: ref __self_0_2,
                        end: ref __self_0_3 } => {
                            let mut builder =
                                __arg_0.debug_struct("Annotation");
                            let _ = builder.field("path", &&(*__self_0_0));
                            let _ =
                                builder.field("source_file", &&(*__self_0_1));
                            let _ = builder.field("begin", &&(*__self_0_2));
                            let _ = builder.field("end", &&(*__self_0_3));
                            builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::cmp::PartialEq for Annotation {
                #[inline]
                fn eq(&self, __arg_0: &Annotation) -> bool {
                    match *__arg_0 {
                        Annotation {
                        path: ref __self_1_0,
                        source_file: ref __self_1_1,
                        begin: ref __self_1_2,
                        end: ref __self_1_3 } =>
                        match *self {
                            Annotation {
                            path: ref __self_0_0,
                            source_file: ref __self_0_1,
                            begin: ref __self_0_2,
                            end: ref __self_0_3 } =>
                            true && (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1) &&
                                (*__self_0_2) == (*__self_1_2) &&
                                (*__self_0_3) == (*__self_1_3),
                        },
                    }
                }
                #[inline]
                fn ne(&self, __arg_0: &Annotation) -> bool {
                    match *__arg_0 {
                        Annotation {
                        path: ref __self_1_0,
                        source_file: ref __self_1_1,
                        begin: ref __self_1_2,
                        end: ref __self_1_3 } =>
                        match *self {
                            Annotation {
                            path: ref __self_0_0,
                            source_file: ref __self_0_1,
                            begin: ref __self_0_2,
                            end: ref __self_0_3 } =>
                            false || (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1) ||
                                (*__self_0_2) != (*__self_1_2) ||
                                (*__self_0_3) != (*__self_1_3),
                        },
                    }
                }
            }
            #[allow(non_snake_case, unused_attributes)]
            mod Annotation_MESSAGE {
                extern crate prost as _prost;
                extern crate bytes as _bytes;
                use super::*;
                impl _prost::Message for Annotation {
                    fn encode_raw<B>(&self, buf: &mut B) where
                     B: _bytes::BufMut {
                        _prost::encoding::int32::encode_packed(1u32,
                                                               &self.path,
                                                               buf);
                        if let ::std::option::Option::Some(ref value) =
                               self.source_file {
                            _prost::encoding::string::encode(2u32, value,
                                                             buf);
                        }
                        if let ::std::option::Option::Some(ref value) =
                               self.begin {
                            _prost::encoding::int32::encode(3u32, value, buf);
                        }
                        if let ::std::option::Option::Some(ref value) =
                               self.end {
                            _prost::encoding::int32::encode(4u32, value, buf);
                        }
                    }
                    fn merge_field<B>(&mut self, buf: &mut B)
                     -> ::std::result::Result<(), _prost::DecodeError> where
                     B: _bytes::Buf {
                        let (tag, wire_type) =
                            _prost::encoding::decode_key(buf)?;
                        match tag {
                            1u32 =>
                            _prost::encoding::int32::merge_repeated(wire_type,
                                                                    &mut self.path,
                                                                    buf).map_err(|mut error|
                                                                                     {
                                                                                         error.push("Annotation",
                                                                                                    "path");
                                                                                         error
                                                                                     }),
                            2u32 =>
                            {
                                if self.source_file.is_none() {
                                    self.source_file =
                                        Some(Default::default());
                                }
                                match self.source_file {
                                    Some(ref mut value) =>
                                    _prost::encoding::string::merge(wire_type,
                                                                    value,
                                                                    buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           800u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("Annotation",
                                                         "source_file");
                                              error
                                          }),
                            3u32 =>
                            {
                                if self.begin.is_none() {
                                    self.begin = Some(Default::default());
                                }
                                match self.begin {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           800u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("Annotation",
                                                         "begin");
                                              error
                                          }),
                            4u32 =>
                            {
                                if self.end.is_none() {
                                    self.end = Some(Default::default());
                                }
                                match self.end {
                                    Some(ref mut value) =>
                                    _prost::encoding::int32::merge(wire_type,
                                                                   value,
                                                                   buf),
                                    _ => {
                                        {
                                            ::rt::begin_panic_new("internal error: entered unreachable code",
                                                                  {
                                                                      static _FILE_LINE_COL:
                                                                             (&'static str,
                                                                              u32,
                                                                              u32)
                                                                             =
                                                                          ("/Users/dan/src/rust/kudu-rs/target/debug/build/kudu-pb-872f006efd159fef/out/protobuf.rs",
                                                                           800u32,
                                                                           38u32);
                                                                      &_FILE_LINE_COL
                                                                  })
                                        }
                                    }
                                }
                            }.map_err(|mut error|
                                          {
                                              error.push("Annotation", "end");
                                              error
                                          }),
                            _ => _prost::encoding::skip_field(wire_type, buf),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 +
                            _prost::encoding::int32::encoded_len_packed(1u32,
                                                                        &self.path)
                            +
                            self.source_file.as_ref().map_or(0,
                                                             |value|
                                                                 _prost::encoding::string::encoded_len(2u32,
                                                                                                       value))
                            +
                            self.begin.as_ref().map_or(0,
                                                       |value|
                                                           _prost::encoding::int32::encoded_len(3u32,
                                                                                                value))
                            +
                            self.end.as_ref().map_or(0,
                                                     |value|
                                                         _prost::encoding::int32::encoded_len(4u32,
                                                                                              value))
                    }
                }
                impl Default for Annotation {
                    fn default() -> Annotation {
                        Annotation{path: ::std::vec::Vec::new(),
                                   source_file: ::std::option::Option::None,
                                   begin: ::std::option::Option::None,
                                   end: ::std::option::Option::None,}
                    }
                }
                #[allow(dead_code)]
                impl Annotation {
                    pub fn source_file(&self) -> &str {
                        match self.source_file {
                            ::std::option::Option::Some(ref val) => &val[..],
                            ::std::option::Option::None => "",
                        }
                    }
                    pub fn begin(&self) -> i32 {
                        match self.begin {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                    pub fn end(&self) -> i32 {
                        match self.end {
                            ::std::option::Option::Some(val) => val,
                            ::std::option::Option::None => 0i32,
                        }
                    }
                }
            }
        }
    }
}
pub use kudu::*;
