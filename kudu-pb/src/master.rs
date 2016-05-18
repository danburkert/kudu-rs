// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::CodedOutputStream;
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct MasterErrorPB {
    // message fields
    code: ::std::option::Option<MasterErrorPB_Code>,
    status: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MasterErrorPB {}

impl MasterErrorPB {
    pub fn new() -> MasterErrorPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MasterErrorPB {
        static mut instance: ::protobuf::lazy::Lazy<MasterErrorPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MasterErrorPB,
        };
        unsafe {
            instance.get(|| {
                MasterErrorPB {
                    code: ::std::option::Option::None,
                    status: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.MasterErrorPB.Code code = 1;

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: MasterErrorPB_Code) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code(&self) -> MasterErrorPB_Code {
        self.code.unwrap_or(MasterErrorPB_Code::UNKNOWN_ERROR)
    }

    // required .kudu.AppStatusPB status = 2;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::wire_protocol::AppStatusPB) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::wire_protocol::AppStatusPB {
        if self.status.is_none() {
            self.status.set_default();
        };
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::wire_protocol::AppStatusPB {
        self.status.take().unwrap_or_else(|| super::wire_protocol::AppStatusPB::new())
    }

    pub fn get_status(&self) -> &super::wire_protocol::AppStatusPB {
        self.status.as_ref().unwrap_or_else(|| super::wire_protocol::AppStatusPB::default_instance())
    }
}

impl ::protobuf::Message for MasterErrorPB {
    fn is_initialized(&self) -> bool {
        if self.code.is_none() {
            return false;
        };
        if self.status.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.code.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.status.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.code {
            try!(w.write_enum(1, v.value()));
        };
        if let Some(v) = self.status.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<MasterErrorPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MasterErrorPB {
    fn new() -> MasterErrorPB {
        MasterErrorPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<MasterErrorPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "code",
                    MasterErrorPB::has_code,
                    MasterErrorPB::get_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "status",
                    MasterErrorPB::has_status,
                    MasterErrorPB::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MasterErrorPB>(
                    "MasterErrorPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MasterErrorPB {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MasterErrorPB {
    fn eq(&self, other: &MasterErrorPB) -> bool {
        self.code == other.code &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MasterErrorPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MasterErrorPB_Code {
    UNKNOWN_ERROR = 1,
    INVALID_SCHEMA = 2,
    TABLE_NOT_FOUND = 3,
    TABLE_ALREADY_PRESENT = 4,
    TOO_MANY_TABLETS = 5,
    CATALOG_MANAGER_NOT_INITIALIZED = 6,
    NOT_THE_LEADER = 7,
    REPLICATION_FACTOR_TOO_HIGH = 8,
}

impl ::protobuf::ProtobufEnum for MasterErrorPB_Code {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MasterErrorPB_Code> {
        match value {
            1 => ::std::option::Option::Some(MasterErrorPB_Code::UNKNOWN_ERROR),
            2 => ::std::option::Option::Some(MasterErrorPB_Code::INVALID_SCHEMA),
            3 => ::std::option::Option::Some(MasterErrorPB_Code::TABLE_NOT_FOUND),
            4 => ::std::option::Option::Some(MasterErrorPB_Code::TABLE_ALREADY_PRESENT),
            5 => ::std::option::Option::Some(MasterErrorPB_Code::TOO_MANY_TABLETS),
            6 => ::std::option::Option::Some(MasterErrorPB_Code::CATALOG_MANAGER_NOT_INITIALIZED),
            7 => ::std::option::Option::Some(MasterErrorPB_Code::NOT_THE_LEADER),
            8 => ::std::option::Option::Some(MasterErrorPB_Code::REPLICATION_FACTOR_TOO_HIGH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MasterErrorPB_Code] = &[
            MasterErrorPB_Code::UNKNOWN_ERROR,
            MasterErrorPB_Code::INVALID_SCHEMA,
            MasterErrorPB_Code::TABLE_NOT_FOUND,
            MasterErrorPB_Code::TABLE_ALREADY_PRESENT,
            MasterErrorPB_Code::TOO_MANY_TABLETS,
            MasterErrorPB_Code::CATALOG_MANAGER_NOT_INITIALIZED,
            MasterErrorPB_Code::NOT_THE_LEADER,
            MasterErrorPB_Code::REPLICATION_FACTOR_TOO_HIGH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MasterErrorPB_Code>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MasterErrorPB_Code", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MasterErrorPB_Code {
}

#[derive(Clone,Default)]
pub struct TSToMasterCommonPB {
    // message fields
    ts_instance: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TSToMasterCommonPB {}

impl TSToMasterCommonPB {
    pub fn new() -> TSToMasterCommonPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSToMasterCommonPB {
        static mut instance: ::protobuf::lazy::Lazy<TSToMasterCommonPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSToMasterCommonPB,
        };
        unsafe {
            instance.get(|| {
                TSToMasterCommonPB {
                    ts_instance: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.NodeInstancePB ts_instance = 1;

    pub fn clear_ts_instance(&mut self) {
        self.ts_instance.clear();
    }

    pub fn has_ts_instance(&self) -> bool {
        self.ts_instance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts_instance(&mut self, v: super::wire_protocol::NodeInstancePB) {
        self.ts_instance = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ts_instance(&mut self) -> &mut super::wire_protocol::NodeInstancePB {
        if self.ts_instance.is_none() {
            self.ts_instance.set_default();
        };
        self.ts_instance.as_mut().unwrap()
    }

    // Take field
    pub fn take_ts_instance(&mut self) -> super::wire_protocol::NodeInstancePB {
        self.ts_instance.take().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::new())
    }

    pub fn get_ts_instance(&self) -> &super::wire_protocol::NodeInstancePB {
        self.ts_instance.as_ref().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::default_instance())
    }
}

impl ::protobuf::Message for TSToMasterCommonPB {
    fn is_initialized(&self) -> bool {
        if self.ts_instance.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ts_instance));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ts_instance.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.ts_instance.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSToMasterCommonPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSToMasterCommonPB {
    fn new() -> TSToMasterCommonPB {
        TSToMasterCommonPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSToMasterCommonPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ts_instance",
                    TSToMasterCommonPB::has_ts_instance,
                    TSToMasterCommonPB::get_ts_instance,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSToMasterCommonPB>(
                    "TSToMasterCommonPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSToMasterCommonPB {
    fn clear(&mut self) {
        self.clear_ts_instance();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSToMasterCommonPB {
    fn eq(&self, other: &TSToMasterCommonPB) -> bool {
        self.ts_instance == other.ts_instance &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSToMasterCommonPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TableIdentifierPB {
    // message fields
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TableIdentifierPB {}

impl TableIdentifierPB {
    pub fn new() -> TableIdentifierPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TableIdentifierPB {
        static mut instance: ::protobuf::lazy::Lazy<TableIdentifierPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TableIdentifierPB,
        };
        unsafe {
            instance.get(|| {
                TableIdentifierPB {
                    table_id: ::protobuf::SingularField::none(),
                    table_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id.clear();
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.table_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table_id.is_none() {
            self.table_id.set_default();
        };
        self.table_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_id(&mut self) -> ::std::vec::Vec<u8> {
        self.table_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table_id(&self) -> &[u8] {
        match self.table_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string table_name = 2;

    pub fn clear_table_name(&mut self) {
        self.table_name.clear();
    }

    pub fn has_table_name(&self) -> bool {
        self.table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_name(&mut self, v: ::std::string::String) {
        self.table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_name(&mut self) -> &mut ::std::string::String {
        if self.table_name.is_none() {
            self.table_name.set_default();
        };
        self.table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_name(&mut self) -> ::std::string::String {
        self.table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table_name(&self) -> &str {
        match self.table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for TableIdentifierPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.table_name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table_id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.table_name.as_ref() {
            try!(w.write_string(2, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TableIdentifierPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TableIdentifierPB {
    fn new() -> TableIdentifierPB {
        TableIdentifierPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TableIdentifierPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table_id",
                    TableIdentifierPB::has_table_id,
                    TableIdentifierPB::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "table_name",
                    TableIdentifierPB::has_table_name,
                    TableIdentifierPB::get_table_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TableIdentifierPB>(
                    "TableIdentifierPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TableIdentifierPB {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_table_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TableIdentifierPB {
    fn eq(&self, other: &TableIdentifierPB) -> bool {
        self.table_id == other.table_id &&
        self.table_name == other.table_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TableIdentifierPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SysTabletsEntryPB {
    // message fields
    DEPRECATED_start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    DEPRECATED_end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition: ::protobuf::SingularPtrField<super::common::PartitionPB>,
    committed_consensus_state: ::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB>,
    state: ::std::option::Option<SysTabletsEntryPB_State>,
    state_msg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SysTabletsEntryPB {}

impl SysTabletsEntryPB {
    pub fn new() -> SysTabletsEntryPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SysTabletsEntryPB {
        static mut instance: ::protobuf::lazy::Lazy<SysTabletsEntryPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SysTabletsEntryPB,
        };
        unsafe {
            instance.get(|| {
                SysTabletsEntryPB {
                    DEPRECATED_start_key: ::protobuf::SingularField::none(),
                    DEPRECATED_end_key: ::protobuf::SingularField::none(),
                    partition: ::protobuf::SingularPtrField::none(),
                    committed_consensus_state: ::protobuf::SingularPtrField::none(),
                    state: ::std::option::Option::None,
                    state_msg: ::protobuf::SingularField::none(),
                    table_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes DEPRECATED_start_key = 1;

    pub fn clear_DEPRECATED_start_key(&mut self) {
        self.DEPRECATED_start_key.clear();
    }

    pub fn has_DEPRECATED_start_key(&self) -> bool {
        self.DEPRECATED_start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_DEPRECATED_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.DEPRECATED_start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_DEPRECATED_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.DEPRECATED_start_key.is_none() {
            self.DEPRECATED_start_key.set_default();
        };
        self.DEPRECATED_start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_DEPRECATED_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.DEPRECATED_start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_DEPRECATED_start_key(&self) -> &[u8] {
        match self.DEPRECATED_start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes DEPRECATED_end_key = 2;

    pub fn clear_DEPRECATED_end_key(&mut self) {
        self.DEPRECATED_end_key.clear();
    }

    pub fn has_DEPRECATED_end_key(&self) -> bool {
        self.DEPRECATED_end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_DEPRECATED_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.DEPRECATED_end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_DEPRECATED_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.DEPRECATED_end_key.is_none() {
            self.DEPRECATED_end_key.set_default();
        };
        self.DEPRECATED_end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_DEPRECATED_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.DEPRECATED_end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_DEPRECATED_end_key(&self) -> &[u8] {
        match self.DEPRECATED_end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .kudu.PartitionPB partition = 7;

    pub fn clear_partition(&mut self) {
        self.partition.clear();
    }

    pub fn has_partition(&self) -> bool {
        self.partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition(&mut self, v: super::common::PartitionPB) {
        self.partition = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition(&mut self) -> &mut super::common::PartitionPB {
        if self.partition.is_none() {
            self.partition.set_default();
        };
        self.partition.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition(&mut self) -> super::common::PartitionPB {
        self.partition.take().unwrap_or_else(|| super::common::PartitionPB::new())
    }

    pub fn get_partition(&self) -> &super::common::PartitionPB {
        self.partition.as_ref().unwrap_or_else(|| super::common::PartitionPB::default_instance())
    }

    // optional .kudu.consensus.ConsensusStatePB committed_consensus_state = 3;

    pub fn clear_committed_consensus_state(&mut self) {
        self.committed_consensus_state.clear();
    }

    pub fn has_committed_consensus_state(&self) -> bool {
        self.committed_consensus_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_committed_consensus_state(&mut self, v: super::consensus_metadata::ConsensusStatePB) {
        self.committed_consensus_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_committed_consensus_state(&mut self) -> &mut super::consensus_metadata::ConsensusStatePB {
        if self.committed_consensus_state.is_none() {
            self.committed_consensus_state.set_default();
        };
        self.committed_consensus_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_committed_consensus_state(&mut self) -> super::consensus_metadata::ConsensusStatePB {
        self.committed_consensus_state.take().unwrap_or_else(|| super::consensus_metadata::ConsensusStatePB::new())
    }

    pub fn get_committed_consensus_state(&self) -> &super::consensus_metadata::ConsensusStatePB {
        self.committed_consensus_state.as_ref().unwrap_or_else(|| super::consensus_metadata::ConsensusStatePB::default_instance())
    }

    // optional .kudu.master.SysTabletsEntryPB.State state = 4;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: SysTabletsEntryPB_State) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> SysTabletsEntryPB_State {
        self.state.unwrap_or(SysTabletsEntryPB_State::UNKNOWN)
    }

    // optional bytes state_msg = 5;

    pub fn clear_state_msg(&mut self) {
        self.state_msg.clear();
    }

    pub fn has_state_msg(&self) -> bool {
        self.state_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.state_msg.is_none() {
            self.state_msg.set_default();
        };
        self.state_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_state_msg(&mut self) -> ::std::vec::Vec<u8> {
        self.state_msg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_state_msg(&self) -> &[u8] {
        match self.state_msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes table_id = 6;

    pub fn clear_table_id(&mut self) {
        self.table_id.clear();
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.table_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table_id.is_none() {
            self.table_id.set_default();
        };
        self.table_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_id(&mut self) -> ::std::vec::Vec<u8> {
        self.table_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table_id(&self) -> &[u8] {
        match self.table_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SysTabletsEntryPB {
    fn is_initialized(&self) -> bool {
        if self.table_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.DEPRECATED_start_key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.DEPRECATED_end_key));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.committed_consensus_state));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.state_msg));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.DEPRECATED_start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.DEPRECATED_end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.partition.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.committed_consensus_state.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in self.state_msg.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.DEPRECATED_start_key.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.DEPRECATED_end_key.as_ref() {
            try!(w.write_bytes(2, &v));
        };
        if let Some(v) = self.partition.as_ref() {
            try!(w.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.state {
            try!(w.write_enum(4, v.value()));
        };
        if let Some(v) = self.state_msg.as_ref() {
            try!(w.write_bytes(5, &v));
        };
        if let Some(v) = self.table_id.as_ref() {
            try!(w.write_bytes(6, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SysTabletsEntryPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SysTabletsEntryPB {
    fn new() -> SysTabletsEntryPB {
        SysTabletsEntryPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<SysTabletsEntryPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "DEPRECATED_start_key",
                    SysTabletsEntryPB::has_DEPRECATED_start_key,
                    SysTabletsEntryPB::get_DEPRECATED_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "DEPRECATED_end_key",
                    SysTabletsEntryPB::has_DEPRECATED_end_key,
                    SysTabletsEntryPB::get_DEPRECATED_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition",
                    SysTabletsEntryPB::has_partition,
                    SysTabletsEntryPB::get_partition,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "committed_consensus_state",
                    SysTabletsEntryPB::has_committed_consensus_state,
                    SysTabletsEntryPB::get_committed_consensus_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    SysTabletsEntryPB::has_state,
                    SysTabletsEntryPB::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "state_msg",
                    SysTabletsEntryPB::has_state_msg,
                    SysTabletsEntryPB::get_state_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table_id",
                    SysTabletsEntryPB::has_table_id,
                    SysTabletsEntryPB::get_table_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SysTabletsEntryPB>(
                    "SysTabletsEntryPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SysTabletsEntryPB {
    fn clear(&mut self) {
        self.clear_DEPRECATED_start_key();
        self.clear_DEPRECATED_end_key();
        self.clear_partition();
        self.clear_committed_consensus_state();
        self.clear_state();
        self.clear_state_msg();
        self.clear_table_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SysTabletsEntryPB {
    fn eq(&self, other: &SysTabletsEntryPB) -> bool {
        self.DEPRECATED_start_key == other.DEPRECATED_start_key &&
        self.DEPRECATED_end_key == other.DEPRECATED_end_key &&
        self.partition == other.partition &&
        self.committed_consensus_state == other.committed_consensus_state &&
        self.state == other.state &&
        self.state_msg == other.state_msg &&
        self.table_id == other.table_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SysTabletsEntryPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SysTabletsEntryPB_State {
    UNKNOWN = 999,
    PREPARING = 0,
    CREATING = 1,
    RUNNING = 2,
    REPLACED = 3,
    DELETED = 4,
}

impl ::protobuf::ProtobufEnum for SysTabletsEntryPB_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SysTabletsEntryPB_State> {
        match value {
            999 => ::std::option::Option::Some(SysTabletsEntryPB_State::UNKNOWN),
            0 => ::std::option::Option::Some(SysTabletsEntryPB_State::PREPARING),
            1 => ::std::option::Option::Some(SysTabletsEntryPB_State::CREATING),
            2 => ::std::option::Option::Some(SysTabletsEntryPB_State::RUNNING),
            3 => ::std::option::Option::Some(SysTabletsEntryPB_State::REPLACED),
            4 => ::std::option::Option::Some(SysTabletsEntryPB_State::DELETED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SysTabletsEntryPB_State] = &[
            SysTabletsEntryPB_State::UNKNOWN,
            SysTabletsEntryPB_State::PREPARING,
            SysTabletsEntryPB_State::CREATING,
            SysTabletsEntryPB_State::RUNNING,
            SysTabletsEntryPB_State::REPLACED,
            SysTabletsEntryPB_State::DELETED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SysTabletsEntryPB_State>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SysTabletsEntryPB_State", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SysTabletsEntryPB_State {
}

#[derive(Clone,Default)]
pub struct SysTablesEntryPB {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u32>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    fully_applied_schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    partition_schema: ::protobuf::SingularPtrField<super::common::PartitionSchemaPB>,
    next_column_id: ::std::option::Option<i32>,
    num_replicas: ::std::option::Option<i32>,
    state: ::std::option::Option<SysTablesEntryPB_State>,
    state_msg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SysTablesEntryPB {}

impl SysTablesEntryPB {
    pub fn new() -> SysTablesEntryPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SysTablesEntryPB {
        static mut instance: ::protobuf::lazy::Lazy<SysTablesEntryPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SysTablesEntryPB,
        };
        unsafe {
            instance.get(|| {
                SysTablesEntryPB {
                    name: ::protobuf::SingularField::none(),
                    version: ::std::option::Option::None,
                    schema: ::protobuf::SingularPtrField::none(),
                    fully_applied_schema: ::protobuf::SingularPtrField::none(),
                    partition_schema: ::protobuf::SingularPtrField::none(),
                    next_column_id: ::std::option::Option::None,
                    num_replicas: ::std::option::Option::None,
                    state: ::std::option::Option::None,
                    state_msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    // required .kudu.SchemaPB schema = 3;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: super::common::SchemaPB) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut super::common::SchemaPB {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> super::common::SchemaPB {
        self.schema.take().unwrap_or_else(|| super::common::SchemaPB::new())
    }

    pub fn get_schema(&self) -> &super::common::SchemaPB {
        self.schema.as_ref().unwrap_or_else(|| super::common::SchemaPB::default_instance())
    }

    // optional .kudu.SchemaPB fully_applied_schema = 4;

    pub fn clear_fully_applied_schema(&mut self) {
        self.fully_applied_schema.clear();
    }

    pub fn has_fully_applied_schema(&self) -> bool {
        self.fully_applied_schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fully_applied_schema(&mut self, v: super::common::SchemaPB) {
        self.fully_applied_schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fully_applied_schema(&mut self) -> &mut super::common::SchemaPB {
        if self.fully_applied_schema.is_none() {
            self.fully_applied_schema.set_default();
        };
        self.fully_applied_schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_fully_applied_schema(&mut self) -> super::common::SchemaPB {
        self.fully_applied_schema.take().unwrap_or_else(|| super::common::SchemaPB::new())
    }

    pub fn get_fully_applied_schema(&self) -> &super::common::SchemaPB {
        self.fully_applied_schema.as_ref().unwrap_or_else(|| super::common::SchemaPB::default_instance())
    }

    // optional .kudu.PartitionSchemaPB partition_schema = 9;

    pub fn clear_partition_schema(&mut self) {
        self.partition_schema.clear();
    }

    pub fn has_partition_schema(&self) -> bool {
        self.partition_schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_schema(&mut self, v: super::common::PartitionSchemaPB) {
        self.partition_schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_schema(&mut self) -> &mut super::common::PartitionSchemaPB {
        if self.partition_schema.is_none() {
            self.partition_schema.set_default();
        };
        self.partition_schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_schema(&mut self) -> super::common::PartitionSchemaPB {
        self.partition_schema.take().unwrap_or_else(|| super::common::PartitionSchemaPB::new())
    }

    pub fn get_partition_schema(&self) -> &super::common::PartitionSchemaPB {
        self.partition_schema.as_ref().unwrap_or_else(|| super::common::PartitionSchemaPB::default_instance())
    }

    // optional int32 next_column_id = 8;

    pub fn clear_next_column_id(&mut self) {
        self.next_column_id = ::std::option::Option::None;
    }

    pub fn has_next_column_id(&self) -> bool {
        self.next_column_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_column_id(&mut self, v: i32) {
        self.next_column_id = ::std::option::Option::Some(v);
    }

    pub fn get_next_column_id(&self) -> i32 {
        self.next_column_id.unwrap_or(0)
    }

    // required int32 num_replicas = 5;

    pub fn clear_num_replicas(&mut self) {
        self.num_replicas = ::std::option::Option::None;
    }

    pub fn has_num_replicas(&self) -> bool {
        self.num_replicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_replicas(&mut self, v: i32) {
        self.num_replicas = ::std::option::Option::Some(v);
    }

    pub fn get_num_replicas(&self) -> i32 {
        self.num_replicas.unwrap_or(0)
    }

    // optional .kudu.master.SysTablesEntryPB.State state = 6;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: SysTablesEntryPB_State) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> SysTablesEntryPB_State {
        self.state.unwrap_or(SysTablesEntryPB_State::UNKNOWN)
    }

    // optional bytes state_msg = 7;

    pub fn clear_state_msg(&mut self) {
        self.state_msg.clear();
    }

    pub fn has_state_msg(&self) -> bool {
        self.state_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.state_msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.state_msg.is_none() {
            self.state_msg.set_default();
        };
        self.state_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_state_msg(&mut self) -> ::std::vec::Vec<u8> {
        self.state_msg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_state_msg(&self) -> &[u8] {
        match self.state_msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SysTablesEntryPB {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.version.is_none() {
            return false;
        };
        if self.schema.is_none() {
            return false;
        };
        if self.num_replicas.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fully_applied_schema));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.next_column_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.state_msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.fully_applied_schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.partition_schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.next_column_id.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.num_replicas.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in self.state_msg.iter() {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.name.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.version {
            try!(w.write_uint32(2, v));
        };
        if let Some(v) = self.schema.as_ref() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.fully_applied_schema.as_ref() {
            try!(w.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.partition_schema.as_ref() {
            try!(w.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.next_column_id {
            try!(w.write_int32(8, v));
        };
        if let Some(v) = self.num_replicas {
            try!(w.write_int32(5, v));
        };
        if let Some(v) = self.state {
            try!(w.write_enum(6, v.value()));
        };
        if let Some(v) = self.state_msg.as_ref() {
            try!(w.write_bytes(7, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SysTablesEntryPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SysTablesEntryPB {
    fn new() -> SysTablesEntryPB {
        SysTablesEntryPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<SysTablesEntryPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "name",
                    SysTablesEntryPB::has_name,
                    SysTablesEntryPB::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "version",
                    SysTablesEntryPB::has_version,
                    SysTablesEntryPB::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "schema",
                    SysTablesEntryPB::has_schema,
                    SysTablesEntryPB::get_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fully_applied_schema",
                    SysTablesEntryPB::has_fully_applied_schema,
                    SysTablesEntryPB::get_fully_applied_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition_schema",
                    SysTablesEntryPB::has_partition_schema,
                    SysTablesEntryPB::get_partition_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "next_column_id",
                    SysTablesEntryPB::has_next_column_id,
                    SysTablesEntryPB::get_next_column_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_replicas",
                    SysTablesEntryPB::has_num_replicas,
                    SysTablesEntryPB::get_num_replicas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    SysTablesEntryPB::has_state,
                    SysTablesEntryPB::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "state_msg",
                    SysTablesEntryPB::has_state_msg,
                    SysTablesEntryPB::get_state_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SysTablesEntryPB>(
                    "SysTablesEntryPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SysTablesEntryPB {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_version();
        self.clear_schema();
        self.clear_fully_applied_schema();
        self.clear_partition_schema();
        self.clear_next_column_id();
        self.clear_num_replicas();
        self.clear_state();
        self.clear_state_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SysTablesEntryPB {
    fn eq(&self, other: &SysTablesEntryPB) -> bool {
        self.name == other.name &&
        self.version == other.version &&
        self.schema == other.schema &&
        self.fully_applied_schema == other.fully_applied_schema &&
        self.partition_schema == other.partition_schema &&
        self.next_column_id == other.next_column_id &&
        self.num_replicas == other.num_replicas &&
        self.state == other.state &&
        self.state_msg == other.state_msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SysTablesEntryPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SysTablesEntryPB_State {
    UNKNOWN = 0,
    PREPARING = 1,
    RUNNING = 2,
    ALTERING = 3,
    REMOVED = 4,
}

impl ::protobuf::ProtobufEnum for SysTablesEntryPB_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SysTablesEntryPB_State> {
        match value {
            0 => ::std::option::Option::Some(SysTablesEntryPB_State::UNKNOWN),
            1 => ::std::option::Option::Some(SysTablesEntryPB_State::PREPARING),
            2 => ::std::option::Option::Some(SysTablesEntryPB_State::RUNNING),
            3 => ::std::option::Option::Some(SysTablesEntryPB_State::ALTERING),
            4 => ::std::option::Option::Some(SysTablesEntryPB_State::REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SysTablesEntryPB_State] = &[
            SysTablesEntryPB_State::UNKNOWN,
            SysTablesEntryPB_State::PREPARING,
            SysTablesEntryPB_State::RUNNING,
            SysTablesEntryPB_State::ALTERING,
            SysTablesEntryPB_State::REMOVED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SysTablesEntryPB_State>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SysTablesEntryPB_State", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SysTablesEntryPB_State {
}

#[derive(Clone,Default)]
pub struct PingRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingRequestPB {}

impl PingRequestPB {
    pub fn new() -> PingRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<PingRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingRequestPB,
        };
        unsafe {
            instance.get(|| {
                PingRequestPB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PingRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingRequestPB {
    fn new() -> PingRequestPB {
        PingRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PingRequestPB>(
                    "PingRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingRequestPB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingRequestPB {
    fn eq(&self, other: &PingRequestPB) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingResponsePB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResponsePB {}

impl PingResponsePB {
    pub fn new() -> PingResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<PingResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResponsePB,
        };
        unsafe {
            instance.get(|| {
                PingResponsePB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PingResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingResponsePB {
    fn new() -> PingResponsePB {
        PingResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PingResponsePB>(
                    "PingResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResponsePB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingResponsePB {
    fn eq(&self, other: &PingResponsePB) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSRegistrationPB {
    // message fields
    rpc_addresses: ::protobuf::RepeatedField<super::common::HostPortPB>,
    http_addresses: ::protobuf::RepeatedField<super::common::HostPortPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TSRegistrationPB {}

impl TSRegistrationPB {
    pub fn new() -> TSRegistrationPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSRegistrationPB {
        static mut instance: ::protobuf::lazy::Lazy<TSRegistrationPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSRegistrationPB,
        };
        unsafe {
            instance.get(|| {
                TSRegistrationPB {
                    rpc_addresses: ::protobuf::RepeatedField::new(),
                    http_addresses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kudu.HostPortPB rpc_addresses = 1;

    pub fn clear_rpc_addresses(&mut self) {
        self.rpc_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_rpc_addresses(&mut self, v: ::protobuf::RepeatedField<super::common::HostPortPB>) {
        self.rpc_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rpc_addresses(&mut self) -> &mut ::protobuf::RepeatedField<super::common::HostPortPB> {
        &mut self.rpc_addresses
    }

    // Take field
    pub fn take_rpc_addresses(&mut self) -> ::protobuf::RepeatedField<super::common::HostPortPB> {
        ::std::mem::replace(&mut self.rpc_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_rpc_addresses(&self) -> &[super::common::HostPortPB] {
        &self.rpc_addresses
    }

    // repeated .kudu.HostPortPB http_addresses = 2;

    pub fn clear_http_addresses(&mut self) {
        self.http_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_http_addresses(&mut self, v: ::protobuf::RepeatedField<super::common::HostPortPB>) {
        self.http_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_http_addresses(&mut self) -> &mut ::protobuf::RepeatedField<super::common::HostPortPB> {
        &mut self.http_addresses
    }

    // Take field
    pub fn take_http_addresses(&mut self) -> ::protobuf::RepeatedField<super::common::HostPortPB> {
        ::std::mem::replace(&mut self.http_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_http_addresses(&self) -> &[super::common::HostPortPB] {
        &self.http_addresses
    }
}

impl ::protobuf::Message for TSRegistrationPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rpc_addresses));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.http_addresses));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.rpc_addresses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.http_addresses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        for v in self.rpc_addresses.iter() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.http_addresses.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSRegistrationPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSRegistrationPB {
    fn new() -> TSRegistrationPB {
        TSRegistrationPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSRegistrationPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rpc_addresses",
                    TSRegistrationPB::get_rpc_addresses,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "http_addresses",
                    TSRegistrationPB::get_http_addresses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSRegistrationPB>(
                    "TSRegistrationPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSRegistrationPB {
    fn clear(&mut self) {
        self.clear_rpc_addresses();
        self.clear_http_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSRegistrationPB {
    fn eq(&self, other: &TSRegistrationPB) -> bool {
        self.rpc_addresses == other.rpc_addresses &&
        self.http_addresses == other.http_addresses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSRegistrationPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReportedTabletPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    state: ::std::option::Option<super::tablet_metadata::TabletStatePB>,
    tablet_data_state: ::std::option::Option<super::tablet_metadata::TabletDataState>,
    committed_consensus_state: ::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB>,
    error: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    schema_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportedTabletPB {}

impl ReportedTabletPB {
    pub fn new() -> ReportedTabletPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportedTabletPB {
        static mut instance: ::protobuf::lazy::Lazy<ReportedTabletPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportedTabletPB,
        };
        unsafe {
            instance.get(|| {
                ReportedTabletPB {
                    tablet_id: ::protobuf::SingularField::none(),
                    state: ::std::option::Option::None,
                    tablet_data_state: ::std::option::Option::None,
                    committed_consensus_state: ::protobuf::SingularPtrField::none(),
                    error: ::protobuf::SingularPtrField::none(),
                    schema_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes tablet_id = 1;

    pub fn clear_tablet_id(&mut self) {
        self.tablet_id.clear();
    }

    pub fn has_tablet_id(&self) -> bool {
        self.tablet_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tablet_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tablet_id.is_none() {
            self.tablet_id.set_default();
        };
        self.tablet_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_id(&mut self) -> ::std::vec::Vec<u8> {
        self.tablet_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tablet_id(&self) -> &[u8] {
        match self.tablet_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .kudu.tablet.TabletStatePB state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: super::tablet_metadata::TabletStatePB) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> super::tablet_metadata::TabletStatePB {
        self.state.unwrap_or(super::tablet_metadata::TabletStatePB::UNKNOWN)
    }

    // optional .kudu.tablet.TabletDataState tablet_data_state = 6;

    pub fn clear_tablet_data_state(&mut self) {
        self.tablet_data_state = ::std::option::Option::None;
    }

    pub fn has_tablet_data_state(&self) -> bool {
        self.tablet_data_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_data_state(&mut self, v: super::tablet_metadata::TabletDataState) {
        self.tablet_data_state = ::std::option::Option::Some(v);
    }

    pub fn get_tablet_data_state(&self) -> super::tablet_metadata::TabletDataState {
        self.tablet_data_state.unwrap_or(super::tablet_metadata::TabletDataState::TABLET_DATA_UNKNOWN)
    }

    // optional .kudu.consensus.ConsensusStatePB committed_consensus_state = 3;

    pub fn clear_committed_consensus_state(&mut self) {
        self.committed_consensus_state.clear();
    }

    pub fn has_committed_consensus_state(&self) -> bool {
        self.committed_consensus_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_committed_consensus_state(&mut self, v: super::consensus_metadata::ConsensusStatePB) {
        self.committed_consensus_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_committed_consensus_state(&mut self) -> &mut super::consensus_metadata::ConsensusStatePB {
        if self.committed_consensus_state.is_none() {
            self.committed_consensus_state.set_default();
        };
        self.committed_consensus_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_committed_consensus_state(&mut self) -> super::consensus_metadata::ConsensusStatePB {
        self.committed_consensus_state.take().unwrap_or_else(|| super::consensus_metadata::ConsensusStatePB::new())
    }

    pub fn get_committed_consensus_state(&self) -> &super::consensus_metadata::ConsensusStatePB {
        self.committed_consensus_state.as_ref().unwrap_or_else(|| super::consensus_metadata::ConsensusStatePB::default_instance())
    }

    // optional .kudu.AppStatusPB error = 4;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::wire_protocol::AppStatusPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut super::wire_protocol::AppStatusPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> super::wire_protocol::AppStatusPB {
        self.error.take().unwrap_or_else(|| super::wire_protocol::AppStatusPB::new())
    }

    pub fn get_error(&self) -> &super::wire_protocol::AppStatusPB {
        self.error.as_ref().unwrap_or_else(|| super::wire_protocol::AppStatusPB::default_instance())
    }

    // optional uint32 schema_version = 5;

    pub fn clear_schema_version(&mut self) {
        self.schema_version = ::std::option::Option::None;
    }

    pub fn has_schema_version(&self) -> bool {
        self.schema_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema_version(&mut self, v: u32) {
        self.schema_version = ::std::option::Option::Some(v);
    }

    pub fn get_schema_version(&self) -> u32 {
        self.schema_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for ReportedTabletPB {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.tablet_data_state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.committed_consensus_state));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablet_id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.tablet_data_state.iter() {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in self.committed_consensus_state.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.schema_version.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.tablet_id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.state {
            try!(w.write_enum(2, v.value()));
        };
        if let Some(v) = self.tablet_data_state {
            try!(w.write_enum(6, v.value()));
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.schema_version {
            try!(w.write_uint32(5, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReportedTabletPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReportedTabletPB {
    fn new() -> ReportedTabletPB {
        ReportedTabletPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportedTabletPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "tablet_id",
                    ReportedTabletPB::has_tablet_id,
                    ReportedTabletPB::get_tablet_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    ReportedTabletPB::has_state,
                    ReportedTabletPB::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "tablet_data_state",
                    ReportedTabletPB::has_tablet_data_state,
                    ReportedTabletPB::get_tablet_data_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "committed_consensus_state",
                    ReportedTabletPB::has_committed_consensus_state,
                    ReportedTabletPB::get_committed_consensus_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ReportedTabletPB::has_error,
                    ReportedTabletPB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "schema_version",
                    ReportedTabletPB::has_schema_version,
                    ReportedTabletPB::get_schema_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReportedTabletPB>(
                    "ReportedTabletPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportedTabletPB {
    fn clear(&mut self) {
        self.clear_tablet_id();
        self.clear_state();
        self.clear_tablet_data_state();
        self.clear_committed_consensus_state();
        self.clear_error();
        self.clear_schema_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReportedTabletPB {
    fn eq(&self, other: &ReportedTabletPB) -> bool {
        self.tablet_id == other.tablet_id &&
        self.state == other.state &&
        self.tablet_data_state == other.tablet_data_state &&
        self.committed_consensus_state == other.committed_consensus_state &&
        self.error == other.error &&
        self.schema_version == other.schema_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReportedTabletPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TabletReportPB {
    // message fields
    is_incremental: ::std::option::Option<bool>,
    updated_tablets: ::protobuf::RepeatedField<ReportedTabletPB>,
    removed_tablet_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    sequence_number: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TabletReportPB {}

impl TabletReportPB {
    pub fn new() -> TabletReportPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TabletReportPB {
        static mut instance: ::protobuf::lazy::Lazy<TabletReportPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TabletReportPB,
        };
        unsafe {
            instance.get(|| {
                TabletReportPB {
                    is_incremental: ::std::option::Option::None,
                    updated_tablets: ::protobuf::RepeatedField::new(),
                    removed_tablet_ids: ::protobuf::RepeatedField::new(),
                    sequence_number: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool is_incremental = 1;

    pub fn clear_is_incremental(&mut self) {
        self.is_incremental = ::std::option::Option::None;
    }

    pub fn has_is_incremental(&self) -> bool {
        self.is_incremental.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_incremental(&mut self, v: bool) {
        self.is_incremental = ::std::option::Option::Some(v);
    }

    pub fn get_is_incremental(&self) -> bool {
        self.is_incremental.unwrap_or(false)
    }

    // repeated .kudu.master.ReportedTabletPB updated_tablets = 2;

    pub fn clear_updated_tablets(&mut self) {
        self.updated_tablets.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated_tablets(&mut self, v: ::protobuf::RepeatedField<ReportedTabletPB>) {
        self.updated_tablets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updated_tablets(&mut self) -> &mut ::protobuf::RepeatedField<ReportedTabletPB> {
        &mut self.updated_tablets
    }

    // Take field
    pub fn take_updated_tablets(&mut self) -> ::protobuf::RepeatedField<ReportedTabletPB> {
        ::std::mem::replace(&mut self.updated_tablets, ::protobuf::RepeatedField::new())
    }

    pub fn get_updated_tablets(&self) -> &[ReportedTabletPB] {
        &self.updated_tablets
    }

    // repeated bytes removed_tablet_ids = 3;

    pub fn clear_removed_tablet_ids(&mut self) {
        self.removed_tablet_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_removed_tablet_ids(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.removed_tablet_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_removed_tablet_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.removed_tablet_ids
    }

    // Take field
    pub fn take_removed_tablet_ids(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.removed_tablet_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_removed_tablet_ids(&self) -> &[::std::vec::Vec<u8>] {
        &self.removed_tablet_ids
    }

    // required int32 sequence_number = 4;

    pub fn clear_sequence_number(&mut self) {
        self.sequence_number = ::std::option::Option::None;
    }

    pub fn has_sequence_number(&self) -> bool {
        self.sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_number(&mut self, v: i32) {
        self.sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_number(&self) -> i32 {
        self.sequence_number.unwrap_or(0)
    }
}

impl ::protobuf::Message for TabletReportPB {
    fn is_initialized(&self) -> bool {
        if self.is_incremental.is_none() {
            return false;
        };
        if self.sequence_number.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_incremental = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updated_tablets));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.removed_tablet_ids));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sequence_number = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.is_incremental.is_some() {
            my_size += 2;
        };
        for value in self.updated_tablets.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.removed_tablet_ids.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.sequence_number.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.is_incremental {
            try!(w.write_bool(1, v));
        };
        for v in self.updated_tablets.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.removed_tablet_ids.iter() {
            try!(w.write_bytes(3, &v));
        };
        if let Some(v) = self.sequence_number {
            try!(w.write_int32(4, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TabletReportPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TabletReportPB {
    fn new() -> TabletReportPB {
        TabletReportPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TabletReportPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_incremental",
                    TabletReportPB::has_is_incremental,
                    TabletReportPB::get_is_incremental,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "updated_tablets",
                    TabletReportPB::get_updated_tablets,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "removed_tablet_ids",
                    TabletReportPB::get_removed_tablet_ids,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sequence_number",
                    TabletReportPB::has_sequence_number,
                    TabletReportPB::get_sequence_number,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TabletReportPB>(
                    "TabletReportPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TabletReportPB {
    fn clear(&mut self) {
        self.clear_is_incremental();
        self.clear_updated_tablets();
        self.clear_removed_tablet_ids();
        self.clear_sequence_number();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TabletReportPB {
    fn eq(&self, other: &TabletReportPB) -> bool {
        self.is_incremental == other.is_incremental &&
        self.updated_tablets == other.updated_tablets &&
        self.removed_tablet_ids == other.removed_tablet_ids &&
        self.sequence_number == other.sequence_number &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TabletReportPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReportedTabletUpdatesPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    state_msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportedTabletUpdatesPB {}

impl ReportedTabletUpdatesPB {
    pub fn new() -> ReportedTabletUpdatesPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportedTabletUpdatesPB {
        static mut instance: ::protobuf::lazy::Lazy<ReportedTabletUpdatesPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportedTabletUpdatesPB,
        };
        unsafe {
            instance.get(|| {
                ReportedTabletUpdatesPB {
                    tablet_id: ::protobuf::SingularField::none(),
                    state_msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes tablet_id = 1;

    pub fn clear_tablet_id(&mut self) {
        self.tablet_id.clear();
    }

    pub fn has_tablet_id(&self) -> bool {
        self.tablet_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tablet_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tablet_id.is_none() {
            self.tablet_id.set_default();
        };
        self.tablet_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_id(&mut self) -> ::std::vec::Vec<u8> {
        self.tablet_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tablet_id(&self) -> &[u8] {
        match self.tablet_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string state_msg = 2;

    pub fn clear_state_msg(&mut self) {
        self.state_msg.clear();
    }

    pub fn has_state_msg(&self) -> bool {
        self.state_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_msg(&mut self, v: ::std::string::String) {
        self.state_msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state_msg(&mut self) -> &mut ::std::string::String {
        if self.state_msg.is_none() {
            self.state_msg.set_default();
        };
        self.state_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_state_msg(&mut self) -> ::std::string::String {
        self.state_msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_state_msg(&self) -> &str {
        match self.state_msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ReportedTabletUpdatesPB {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.state_msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablet_id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.state_msg.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.tablet_id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.state_msg.as_ref() {
            try!(w.write_string(2, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReportedTabletUpdatesPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReportedTabletUpdatesPB {
    fn new() -> ReportedTabletUpdatesPB {
        ReportedTabletUpdatesPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportedTabletUpdatesPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "tablet_id",
                    ReportedTabletUpdatesPB::has_tablet_id,
                    ReportedTabletUpdatesPB::get_tablet_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "state_msg",
                    ReportedTabletUpdatesPB::has_state_msg,
                    ReportedTabletUpdatesPB::get_state_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReportedTabletUpdatesPB>(
                    "ReportedTabletUpdatesPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportedTabletUpdatesPB {
    fn clear(&mut self) {
        self.clear_tablet_id();
        self.clear_state_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReportedTabletUpdatesPB {
    fn eq(&self, other: &ReportedTabletUpdatesPB) -> bool {
        self.tablet_id == other.tablet_id &&
        self.state_msg == other.state_msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReportedTabletUpdatesPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TabletReportUpdatesPB {
    // message fields
    tablets: ::protobuf::RepeatedField<ReportedTabletUpdatesPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TabletReportUpdatesPB {}

impl TabletReportUpdatesPB {
    pub fn new() -> TabletReportUpdatesPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TabletReportUpdatesPB {
        static mut instance: ::protobuf::lazy::Lazy<TabletReportUpdatesPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TabletReportUpdatesPB,
        };
        unsafe {
            instance.get(|| {
                TabletReportUpdatesPB {
                    tablets: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kudu.master.ReportedTabletUpdatesPB tablets = 1;

    pub fn clear_tablets(&mut self) {
        self.tablets.clear();
    }

    // Param is passed by value, moved
    pub fn set_tablets(&mut self, v: ::protobuf::RepeatedField<ReportedTabletUpdatesPB>) {
        self.tablets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tablets(&mut self) -> &mut ::protobuf::RepeatedField<ReportedTabletUpdatesPB> {
        &mut self.tablets
    }

    // Take field
    pub fn take_tablets(&mut self) -> ::protobuf::RepeatedField<ReportedTabletUpdatesPB> {
        ::std::mem::replace(&mut self.tablets, ::protobuf::RepeatedField::new())
    }

    pub fn get_tablets(&self) -> &[ReportedTabletUpdatesPB] {
        &self.tablets
    }
}

impl ::protobuf::Message for TabletReportUpdatesPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablets));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablets.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        for v in self.tablets.iter() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TabletReportUpdatesPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TabletReportUpdatesPB {
    fn new() -> TabletReportUpdatesPB {
        TabletReportUpdatesPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TabletReportUpdatesPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tablets",
                    TabletReportUpdatesPB::get_tablets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TabletReportUpdatesPB>(
                    "TabletReportUpdatesPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TabletReportUpdatesPB {
    fn clear(&mut self) {
        self.clear_tablets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TabletReportUpdatesPB {
    fn eq(&self, other: &TabletReportUpdatesPB) -> bool {
        self.tablets == other.tablets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TabletReportUpdatesPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSHeartbeatRequestPB {
    // message fields
    common: ::protobuf::SingularPtrField<TSToMasterCommonPB>,
    registration: ::protobuf::SingularPtrField<TSRegistrationPB>,
    tablet_report: ::protobuf::SingularPtrField<TabletReportPB>,
    num_live_tablets: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TSHeartbeatRequestPB {}

impl TSHeartbeatRequestPB {
    pub fn new() -> TSHeartbeatRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSHeartbeatRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<TSHeartbeatRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSHeartbeatRequestPB,
        };
        unsafe {
            instance.get(|| {
                TSHeartbeatRequestPB {
                    common: ::protobuf::SingularPtrField::none(),
                    registration: ::protobuf::SingularPtrField::none(),
                    tablet_report: ::protobuf::SingularPtrField::none(),
                    num_live_tablets: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TSToMasterCommonPB common = 1;

    pub fn clear_common(&mut self) {
        self.common.clear();
    }

    pub fn has_common(&self) -> bool {
        self.common.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common(&mut self, v: TSToMasterCommonPB) {
        self.common = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common(&mut self) -> &mut TSToMasterCommonPB {
        if self.common.is_none() {
            self.common.set_default();
        };
        self.common.as_mut().unwrap()
    }

    // Take field
    pub fn take_common(&mut self) -> TSToMasterCommonPB {
        self.common.take().unwrap_or_else(|| TSToMasterCommonPB::new())
    }

    pub fn get_common(&self) -> &TSToMasterCommonPB {
        self.common.as_ref().unwrap_or_else(|| TSToMasterCommonPB::default_instance())
    }

    // optional .kudu.master.TSRegistrationPB registration = 2;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: TSRegistrationPB) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut TSRegistrationPB {
        if self.registration.is_none() {
            self.registration.set_default();
        };
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> TSRegistrationPB {
        self.registration.take().unwrap_or_else(|| TSRegistrationPB::new())
    }

    pub fn get_registration(&self) -> &TSRegistrationPB {
        self.registration.as_ref().unwrap_or_else(|| TSRegistrationPB::default_instance())
    }

    // optional .kudu.master.TabletReportPB tablet_report = 3;

    pub fn clear_tablet_report(&mut self) {
        self.tablet_report.clear();
    }

    pub fn has_tablet_report(&self) -> bool {
        self.tablet_report.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_report(&mut self, v: TabletReportPB) {
        self.tablet_report = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_report(&mut self) -> &mut TabletReportPB {
        if self.tablet_report.is_none() {
            self.tablet_report.set_default();
        };
        self.tablet_report.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_report(&mut self) -> TabletReportPB {
        self.tablet_report.take().unwrap_or_else(|| TabletReportPB::new())
    }

    pub fn get_tablet_report(&self) -> &TabletReportPB {
        self.tablet_report.as_ref().unwrap_or_else(|| TabletReportPB::default_instance())
    }

    // optional int32 num_live_tablets = 4;

    pub fn clear_num_live_tablets(&mut self) {
        self.num_live_tablets = ::std::option::Option::None;
    }

    pub fn has_num_live_tablets(&self) -> bool {
        self.num_live_tablets.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_live_tablets(&mut self, v: i32) {
        self.num_live_tablets = ::std::option::Option::Some(v);
    }

    pub fn get_num_live_tablets(&self) -> i32 {
        self.num_live_tablets.unwrap_or(0)
    }
}

impl ::protobuf::Message for TSHeartbeatRequestPB {
    fn is_initialized(&self) -> bool {
        if self.common.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tablet_report));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_live_tablets = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.common.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.registration.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tablet_report.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.num_live_tablets.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.common.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.registration.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.tablet_report.as_ref() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.num_live_tablets {
            try!(w.write_int32(4, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSHeartbeatRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSHeartbeatRequestPB {
    fn new() -> TSHeartbeatRequestPB {
        TSHeartbeatRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSHeartbeatRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "common",
                    TSHeartbeatRequestPB::has_common,
                    TSHeartbeatRequestPB::get_common,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "registration",
                    TSHeartbeatRequestPB::has_registration,
                    TSHeartbeatRequestPB::get_registration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tablet_report",
                    TSHeartbeatRequestPB::has_tablet_report,
                    TSHeartbeatRequestPB::get_tablet_report,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_live_tablets",
                    TSHeartbeatRequestPB::has_num_live_tablets,
                    TSHeartbeatRequestPB::get_num_live_tablets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSHeartbeatRequestPB>(
                    "TSHeartbeatRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSHeartbeatRequestPB {
    fn clear(&mut self) {
        self.clear_common();
        self.clear_registration();
        self.clear_tablet_report();
        self.clear_num_live_tablets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSHeartbeatRequestPB {
    fn eq(&self, other: &TSHeartbeatRequestPB) -> bool {
        self.common == other.common &&
        self.registration == other.registration &&
        self.tablet_report == other.tablet_report &&
        self.num_live_tablets == other.num_live_tablets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSHeartbeatRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSHeartbeatResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    master_instance: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    needs_reregister: ::std::option::Option<bool>,
    needs_full_tablet_report: ::std::option::Option<bool>,
    tablet_report: ::protobuf::SingularPtrField<TabletReportUpdatesPB>,
    leader_master: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TSHeartbeatResponsePB {}

impl TSHeartbeatResponsePB {
    pub fn new() -> TSHeartbeatResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSHeartbeatResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<TSHeartbeatResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSHeartbeatResponsePB,
        };
        unsafe {
            instance.get(|| {
                TSHeartbeatResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    master_instance: ::protobuf::SingularPtrField::none(),
                    needs_reregister: ::std::option::Option::None,
                    needs_full_tablet_report: ::std::option::Option::None,
                    tablet_report: ::protobuf::SingularPtrField::none(),
                    leader_master: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional .kudu.NodeInstancePB master_instance = 2;

    pub fn clear_master_instance(&mut self) {
        self.master_instance.clear();
    }

    pub fn has_master_instance(&self) -> bool {
        self.master_instance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master_instance(&mut self, v: super::wire_protocol::NodeInstancePB) {
        self.master_instance = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_master_instance(&mut self) -> &mut super::wire_protocol::NodeInstancePB {
        if self.master_instance.is_none() {
            self.master_instance.set_default();
        };
        self.master_instance.as_mut().unwrap()
    }

    // Take field
    pub fn take_master_instance(&mut self) -> super::wire_protocol::NodeInstancePB {
        self.master_instance.take().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::new())
    }

    pub fn get_master_instance(&self) -> &super::wire_protocol::NodeInstancePB {
        self.master_instance.as_ref().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::default_instance())
    }

    // optional bool needs_reregister = 3;

    pub fn clear_needs_reregister(&mut self) {
        self.needs_reregister = ::std::option::Option::None;
    }

    pub fn has_needs_reregister(&self) -> bool {
        self.needs_reregister.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needs_reregister(&mut self, v: bool) {
        self.needs_reregister = ::std::option::Option::Some(v);
    }

    pub fn get_needs_reregister(&self) -> bool {
        self.needs_reregister.unwrap_or(false)
    }

    // optional bool needs_full_tablet_report = 4;

    pub fn clear_needs_full_tablet_report(&mut self) {
        self.needs_full_tablet_report = ::std::option::Option::None;
    }

    pub fn has_needs_full_tablet_report(&self) -> bool {
        self.needs_full_tablet_report.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needs_full_tablet_report(&mut self, v: bool) {
        self.needs_full_tablet_report = ::std::option::Option::Some(v);
    }

    pub fn get_needs_full_tablet_report(&self) -> bool {
        self.needs_full_tablet_report.unwrap_or(false)
    }

    // optional .kudu.master.TabletReportUpdatesPB tablet_report = 5;

    pub fn clear_tablet_report(&mut self) {
        self.tablet_report.clear();
    }

    pub fn has_tablet_report(&self) -> bool {
        self.tablet_report.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_report(&mut self, v: TabletReportUpdatesPB) {
        self.tablet_report = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_report(&mut self) -> &mut TabletReportUpdatesPB {
        if self.tablet_report.is_none() {
            self.tablet_report.set_default();
        };
        self.tablet_report.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_report(&mut self) -> TabletReportUpdatesPB {
        self.tablet_report.take().unwrap_or_else(|| TabletReportUpdatesPB::new())
    }

    pub fn get_tablet_report(&self) -> &TabletReportUpdatesPB {
        self.tablet_report.as_ref().unwrap_or_else(|| TabletReportUpdatesPB::default_instance())
    }

    // optional bool leader_master = 6;

    pub fn clear_leader_master(&mut self) {
        self.leader_master = ::std::option::Option::None;
    }

    pub fn has_leader_master(&self) -> bool {
        self.leader_master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_master(&mut self, v: bool) {
        self.leader_master = ::std::option::Option::Some(v);
    }

    pub fn get_leader_master(&self) -> bool {
        self.leader_master.unwrap_or(false)
    }
}

impl ::protobuf::Message for TSHeartbeatResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.master_instance));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.needs_reregister = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.needs_full_tablet_report = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tablet_report));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.leader_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.master_instance.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.needs_reregister.is_some() {
            my_size += 2;
        };
        if self.needs_full_tablet_report.is_some() {
            my_size += 2;
        };
        for value in self.tablet_report.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.leader_master.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.master_instance.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.needs_reregister {
            try!(w.write_bool(3, v));
        };
        if let Some(v) = self.needs_full_tablet_report {
            try!(w.write_bool(4, v));
        };
        if let Some(v) = self.tablet_report.as_ref() {
            try!(w.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.leader_master {
            try!(w.write_bool(6, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSHeartbeatResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSHeartbeatResponsePB {
    fn new() -> TSHeartbeatResponsePB {
        TSHeartbeatResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSHeartbeatResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    TSHeartbeatResponsePB::has_error,
                    TSHeartbeatResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "master_instance",
                    TSHeartbeatResponsePB::has_master_instance,
                    TSHeartbeatResponsePB::get_master_instance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "needs_reregister",
                    TSHeartbeatResponsePB::has_needs_reregister,
                    TSHeartbeatResponsePB::get_needs_reregister,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "needs_full_tablet_report",
                    TSHeartbeatResponsePB::has_needs_full_tablet_report,
                    TSHeartbeatResponsePB::get_needs_full_tablet_report,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tablet_report",
                    TSHeartbeatResponsePB::has_tablet_report,
                    TSHeartbeatResponsePB::get_tablet_report,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "leader_master",
                    TSHeartbeatResponsePB::has_leader_master,
                    TSHeartbeatResponsePB::get_leader_master,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSHeartbeatResponsePB>(
                    "TSHeartbeatResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSHeartbeatResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_master_instance();
        self.clear_needs_reregister();
        self.clear_needs_full_tablet_report();
        self.clear_tablet_report();
        self.clear_leader_master();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSHeartbeatResponsePB {
    fn eq(&self, other: &TSHeartbeatResponsePB) -> bool {
        self.error == other.error &&
        self.master_instance == other.master_instance &&
        self.needs_reregister == other.needs_reregister &&
        self.needs_full_tablet_report == other.needs_full_tablet_report &&
        self.tablet_report == other.tablet_report &&
        self.leader_master == other.leader_master &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSHeartbeatResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TabletLocationsPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition: ::protobuf::SingularPtrField<super::common::PartitionPB>,
    replicas: ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB>,
    stale: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TabletLocationsPB {}

impl TabletLocationsPB {
    pub fn new() -> TabletLocationsPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TabletLocationsPB {
        static mut instance: ::protobuf::lazy::Lazy<TabletLocationsPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TabletLocationsPB,
        };
        unsafe {
            instance.get(|| {
                TabletLocationsPB {
                    tablet_id: ::protobuf::SingularField::none(),
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    partition: ::protobuf::SingularPtrField::none(),
                    replicas: ::protobuf::RepeatedField::new(),
                    stale: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes tablet_id = 1;

    pub fn clear_tablet_id(&mut self) {
        self.tablet_id.clear();
    }

    pub fn has_tablet_id(&self) -> bool {
        self.tablet_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tablet_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tablet_id.is_none() {
            self.tablet_id.set_default();
        };
        self.tablet_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_id(&mut self) -> ::std::vec::Vec<u8> {
        self.tablet_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tablet_id(&self) -> &[u8] {
        match self.tablet_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes start_key = 2;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end_key = 3;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .kudu.PartitionPB partition = 6;

    pub fn clear_partition(&mut self) {
        self.partition.clear();
    }

    pub fn has_partition(&self) -> bool {
        self.partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition(&mut self, v: super::common::PartitionPB) {
        self.partition = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition(&mut self) -> &mut super::common::PartitionPB {
        if self.partition.is_none() {
            self.partition.set_default();
        };
        self.partition.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition(&mut self) -> super::common::PartitionPB {
        self.partition.take().unwrap_or_else(|| super::common::PartitionPB::new())
    }

    pub fn get_partition(&self) -> &super::common::PartitionPB {
        self.partition.as_ref().unwrap_or_else(|| super::common::PartitionPB::default_instance())
    }

    // repeated .kudu.master.TabletLocationsPB.ReplicaPB replicas = 4;

    pub fn clear_replicas(&mut self) {
        self.replicas.clear();
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB>) {
        self.replicas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replicas(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB> {
        &mut self.replicas
    }

    // Take field
    pub fn take_replicas(&mut self) -> ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB> {
        ::std::mem::replace(&mut self.replicas, ::protobuf::RepeatedField::new())
    }

    pub fn get_replicas(&self) -> &[TabletLocationsPB_ReplicaPB] {
        &self.replicas
    }

    // required bool stale = 5;

    pub fn clear_stale(&mut self) {
        self.stale = ::std::option::Option::None;
    }

    pub fn has_stale(&self) -> bool {
        self.stale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stale(&mut self, v: bool) {
        self.stale = ::std::option::Option::Some(v);
    }

    pub fn get_stale(&self) -> bool {
        self.stale.unwrap_or(false)
    }
}

impl ::protobuf::Message for TabletLocationsPB {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        if self.stale.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stale = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablet_id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.partition.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.replicas.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.stale.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.tablet_id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.start_key.as_ref() {
            try!(w.write_bytes(2, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(w.write_bytes(3, &v));
        };
        if let Some(v) = self.partition.as_ref() {
            try!(w.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.replicas.iter() {
            try!(w.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.stale {
            try!(w.write_bool(5, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TabletLocationsPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TabletLocationsPB {
    fn new() -> TabletLocationsPB {
        TabletLocationsPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TabletLocationsPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "tablet_id",
                    TabletLocationsPB::has_tablet_id,
                    TabletLocationsPB::get_tablet_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    TabletLocationsPB::has_start_key,
                    TabletLocationsPB::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    TabletLocationsPB::has_end_key,
                    TabletLocationsPB::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition",
                    TabletLocationsPB::has_partition,
                    TabletLocationsPB::get_partition,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replicas",
                    TabletLocationsPB::get_replicas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stale",
                    TabletLocationsPB::has_stale,
                    TabletLocationsPB::get_stale,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TabletLocationsPB>(
                    "TabletLocationsPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TabletLocationsPB {
    fn clear(&mut self) {
        self.clear_tablet_id();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_partition();
        self.clear_replicas();
        self.clear_stale();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TabletLocationsPB {
    fn eq(&self, other: &TabletLocationsPB) -> bool {
        self.tablet_id == other.tablet_id &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.partition == other.partition &&
        self.replicas == other.replicas &&
        self.stale == other.stale &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TabletLocationsPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TabletLocationsPB_ReplicaPB {
    // message fields
    ts_info: ::protobuf::SingularPtrField<TSInfoPB>,
    role: ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TabletLocationsPB_ReplicaPB {}

impl TabletLocationsPB_ReplicaPB {
    pub fn new() -> TabletLocationsPB_ReplicaPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TabletLocationsPB_ReplicaPB {
        static mut instance: ::protobuf::lazy::Lazy<TabletLocationsPB_ReplicaPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TabletLocationsPB_ReplicaPB,
        };
        unsafe {
            instance.get(|| {
                TabletLocationsPB_ReplicaPB {
                    ts_info: ::protobuf::SingularPtrField::none(),
                    role: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TSInfoPB ts_info = 1;

    pub fn clear_ts_info(&mut self) {
        self.ts_info.clear();
    }

    pub fn has_ts_info(&self) -> bool {
        self.ts_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts_info(&mut self, v: TSInfoPB) {
        self.ts_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ts_info(&mut self) -> &mut TSInfoPB {
        if self.ts_info.is_none() {
            self.ts_info.set_default();
        };
        self.ts_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_ts_info(&mut self) -> TSInfoPB {
        self.ts_info.take().unwrap_or_else(|| TSInfoPB::new())
    }

    pub fn get_ts_info(&self) -> &TSInfoPB {
        self.ts_info.as_ref().unwrap_or_else(|| TSInfoPB::default_instance())
    }

    // required .kudu.consensus.RaftPeerPB.Role role = 2;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: super::consensus_metadata::RaftPeerPB_Role) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role(&self) -> super::consensus_metadata::RaftPeerPB_Role {
        self.role.unwrap_or(super::consensus_metadata::RaftPeerPB_Role::UNKNOWN_ROLE)
    }
}

impl ::protobuf::Message for TabletLocationsPB_ReplicaPB {
    fn is_initialized(&self) -> bool {
        if self.ts_info.is_none() {
            return false;
        };
        if self.role.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ts_info));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.role = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ts_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.role.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.ts_info.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.role {
            try!(w.write_enum(2, v.value()));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TabletLocationsPB_ReplicaPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TabletLocationsPB_ReplicaPB {
    fn new() -> TabletLocationsPB_ReplicaPB {
        TabletLocationsPB_ReplicaPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TabletLocationsPB_ReplicaPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ts_info",
                    TabletLocationsPB_ReplicaPB::has_ts_info,
                    TabletLocationsPB_ReplicaPB::get_ts_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "role",
                    TabletLocationsPB_ReplicaPB::has_role,
                    TabletLocationsPB_ReplicaPB::get_role,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TabletLocationsPB_ReplicaPB>(
                    "TabletLocationsPB_ReplicaPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TabletLocationsPB_ReplicaPB {
    fn clear(&mut self) {
        self.clear_ts_info();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TabletLocationsPB_ReplicaPB {
    fn eq(&self, other: &TabletLocationsPB_ReplicaPB) -> bool {
        self.ts_info == other.ts_info &&
        self.role == other.role &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TabletLocationsPB_ReplicaPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TSInfoPB {
    // message fields
    permanent_uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rpc_addresses: ::protobuf::RepeatedField<super::common::HostPortPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TSInfoPB {}

impl TSInfoPB {
    pub fn new() -> TSInfoPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TSInfoPB {
        static mut instance: ::protobuf::lazy::Lazy<TSInfoPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TSInfoPB,
        };
        unsafe {
            instance.get(|| {
                TSInfoPB {
                    permanent_uuid: ::protobuf::SingularField::none(),
                    rpc_addresses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes permanent_uuid = 1;

    pub fn clear_permanent_uuid(&mut self) {
        self.permanent_uuid.clear();
    }

    pub fn has_permanent_uuid(&self) -> bool {
        self.permanent_uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permanent_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.permanent_uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permanent_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.permanent_uuid.is_none() {
            self.permanent_uuid.set_default();
        };
        self.permanent_uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_permanent_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.permanent_uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_permanent_uuid(&self) -> &[u8] {
        match self.permanent_uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .kudu.HostPortPB rpc_addresses = 2;

    pub fn clear_rpc_addresses(&mut self) {
        self.rpc_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_rpc_addresses(&mut self, v: ::protobuf::RepeatedField<super::common::HostPortPB>) {
        self.rpc_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rpc_addresses(&mut self) -> &mut ::protobuf::RepeatedField<super::common::HostPortPB> {
        &mut self.rpc_addresses
    }

    // Take field
    pub fn take_rpc_addresses(&mut self) -> ::protobuf::RepeatedField<super::common::HostPortPB> {
        ::std::mem::replace(&mut self.rpc_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_rpc_addresses(&self) -> &[super::common::HostPortPB] {
        &self.rpc_addresses
    }
}

impl ::protobuf::Message for TSInfoPB {
    fn is_initialized(&self) -> bool {
        if self.permanent_uuid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.permanent_uuid));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rpc_addresses));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.permanent_uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.rpc_addresses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.permanent_uuid.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        for v in self.rpc_addresses.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TSInfoPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TSInfoPB {
    fn new() -> TSInfoPB {
        TSInfoPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TSInfoPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "permanent_uuid",
                    TSInfoPB::has_permanent_uuid,
                    TSInfoPB::get_permanent_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rpc_addresses",
                    TSInfoPB::get_rpc_addresses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TSInfoPB>(
                    "TSInfoPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TSInfoPB {
    fn clear(&mut self) {
        self.clear_permanent_uuid();
        self.clear_rpc_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TSInfoPB {
    fn eq(&self, other: &TSInfoPB) -> bool {
        self.permanent_uuid == other.permanent_uuid &&
        self.rpc_addresses == other.rpc_addresses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TSInfoPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTabletLocationsRequestPB {
    // message fields
    tablet_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTabletLocationsRequestPB {}

impl GetTabletLocationsRequestPB {
    pub fn new() -> GetTabletLocationsRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTabletLocationsRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<GetTabletLocationsRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTabletLocationsRequestPB,
        };
        unsafe {
            instance.get(|| {
                GetTabletLocationsRequestPB {
                    tablet_ids: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes tablet_ids = 1;

    pub fn clear_tablet_ids(&mut self) {
        self.tablet_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_tablet_ids(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.tablet_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tablet_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.tablet_ids
    }

    // Take field
    pub fn take_tablet_ids(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.tablet_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_tablet_ids(&self) -> &[::std::vec::Vec<u8>] {
        &self.tablet_ids
    }
}

impl ::protobuf::Message for GetTabletLocationsRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.tablet_ids));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablet_ids.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        for v in self.tablet_ids.iter() {
            try!(w.write_bytes(1, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTabletLocationsRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTabletLocationsRequestPB {
    fn new() -> GetTabletLocationsRequestPB {
        GetTabletLocationsRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTabletLocationsRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "tablet_ids",
                    GetTabletLocationsRequestPB::get_tablet_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTabletLocationsRequestPB>(
                    "GetTabletLocationsRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTabletLocationsRequestPB {
    fn clear(&mut self) {
        self.clear_tablet_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTabletLocationsRequestPB {
    fn eq(&self, other: &GetTabletLocationsRequestPB) -> bool {
        self.tablet_ids == other.tablet_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTabletLocationsRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTabletLocationsResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tablet_locations: ::protobuf::RepeatedField<TabletLocationsPB>,
    errors: ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTabletLocationsResponsePB {}

impl GetTabletLocationsResponsePB {
    pub fn new() -> GetTabletLocationsResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTabletLocationsResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<GetTabletLocationsResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTabletLocationsResponsePB,
        };
        unsafe {
            instance.get(|| {
                GetTabletLocationsResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    tablet_locations: ::protobuf::RepeatedField::new(),
                    errors: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // repeated .kudu.master.TabletLocationsPB tablet_locations = 2;

    pub fn clear_tablet_locations(&mut self) {
        self.tablet_locations.clear();
    }

    // Param is passed by value, moved
    pub fn set_tablet_locations(&mut self, v: ::protobuf::RepeatedField<TabletLocationsPB>) {
        self.tablet_locations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tablet_locations(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB> {
        &mut self.tablet_locations
    }

    // Take field
    pub fn take_tablet_locations(&mut self) -> ::protobuf::RepeatedField<TabletLocationsPB> {
        ::std::mem::replace(&mut self.tablet_locations, ::protobuf::RepeatedField::new())
    }

    pub fn get_tablet_locations(&self) -> &[TabletLocationsPB] {
        &self.tablet_locations
    }

    // repeated .kudu.master.GetTabletLocationsResponsePB.Error errors = 3;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[GetTabletLocationsResponsePB_Error] {
        &self.errors
    }
}

impl ::protobuf::Message for GetTabletLocationsResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablet_locations));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tablet_locations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.errors.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.tablet_locations.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.errors.iter() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTabletLocationsResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTabletLocationsResponsePB {
    fn new() -> GetTabletLocationsResponsePB {
        GetTabletLocationsResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTabletLocationsResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    GetTabletLocationsResponsePB::has_error,
                    GetTabletLocationsResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tablet_locations",
                    GetTabletLocationsResponsePB::get_tablet_locations,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "errors",
                    GetTabletLocationsResponsePB::get_errors,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTabletLocationsResponsePB>(
                    "GetTabletLocationsResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTabletLocationsResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_tablet_locations();
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTabletLocationsResponsePB {
    fn eq(&self, other: &GetTabletLocationsResponsePB) -> bool {
        self.error == other.error &&
        self.tablet_locations == other.tablet_locations &&
        self.errors == other.errors &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTabletLocationsResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTabletLocationsResponsePB_Error {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    status: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTabletLocationsResponsePB_Error {}

impl GetTabletLocationsResponsePB_Error {
    pub fn new() -> GetTabletLocationsResponsePB_Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTabletLocationsResponsePB_Error {
        static mut instance: ::protobuf::lazy::Lazy<GetTabletLocationsResponsePB_Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTabletLocationsResponsePB_Error,
        };
        unsafe {
            instance.get(|| {
                GetTabletLocationsResponsePB_Error {
                    tablet_id: ::protobuf::SingularField::none(),
                    status: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes tablet_id = 1;

    pub fn clear_tablet_id(&mut self) {
        self.tablet_id.clear();
    }

    pub fn has_tablet_id(&self) -> bool {
        self.tablet_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tablet_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tablet_id.is_none() {
            self.tablet_id.set_default();
        };
        self.tablet_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_id(&mut self) -> ::std::vec::Vec<u8> {
        self.tablet_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tablet_id(&self) -> &[u8] {
        match self.tablet_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .kudu.AppStatusPB status = 2;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::wire_protocol::AppStatusPB) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::wire_protocol::AppStatusPB {
        if self.status.is_none() {
            self.status.set_default();
        };
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::wire_protocol::AppStatusPB {
        self.status.take().unwrap_or_else(|| super::wire_protocol::AppStatusPB::new())
    }

    pub fn get_status(&self) -> &super::wire_protocol::AppStatusPB {
        self.status.as_ref().unwrap_or_else(|| super::wire_protocol::AppStatusPB::default_instance())
    }
}

impl ::protobuf::Message for GetTabletLocationsResponsePB_Error {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        if self.status.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.tablet_id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.status.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.tablet_id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.status.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTabletLocationsResponsePB_Error>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTabletLocationsResponsePB_Error {
    fn new() -> GetTabletLocationsResponsePB_Error {
        GetTabletLocationsResponsePB_Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTabletLocationsResponsePB_Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "tablet_id",
                    GetTabletLocationsResponsePB_Error::has_tablet_id,
                    GetTabletLocationsResponsePB_Error::get_tablet_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "status",
                    GetTabletLocationsResponsePB_Error::has_status,
                    GetTabletLocationsResponsePB_Error::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTabletLocationsResponsePB_Error>(
                    "GetTabletLocationsResponsePB_Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTabletLocationsResponsePB_Error {
    fn clear(&mut self) {
        self.clear_tablet_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTabletLocationsResponsePB_Error {
    fn eq(&self, other: &GetTabletLocationsResponsePB_Error) -> bool {
        self.tablet_id == other.tablet_id &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTabletLocationsResponsePB_Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateTableRequestPB {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    split_rows: ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB>,
    partition_schema: ::protobuf::SingularPtrField<super::common::PartitionSchemaPB>,
    num_replicas: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateTableRequestPB {}

impl CreateTableRequestPB {
    pub fn new() -> CreateTableRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateTableRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<CreateTableRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateTableRequestPB,
        };
        unsafe {
            instance.get(|| {
                CreateTableRequestPB {
                    name: ::protobuf::SingularField::none(),
                    schema: ::protobuf::SingularPtrField::none(),
                    split_rows: ::protobuf::SingularPtrField::none(),
                    partition_schema: ::protobuf::SingularPtrField::none(),
                    num_replicas: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .kudu.SchemaPB schema = 2;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: super::common::SchemaPB) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut super::common::SchemaPB {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> super::common::SchemaPB {
        self.schema.take().unwrap_or_else(|| super::common::SchemaPB::new())
    }

    pub fn get_schema(&self) -> &super::common::SchemaPB {
        self.schema.as_ref().unwrap_or_else(|| super::common::SchemaPB::default_instance())
    }

    // optional .kudu.RowOperationsPB split_rows = 6;

    pub fn clear_split_rows(&mut self) {
        self.split_rows.clear();
    }

    pub fn has_split_rows(&self) -> bool {
        self.split_rows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split_rows(&mut self, v: super::wire_protocol::RowOperationsPB) {
        self.split_rows = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split_rows(&mut self) -> &mut super::wire_protocol::RowOperationsPB {
        if self.split_rows.is_none() {
            self.split_rows.set_default();
        };
        self.split_rows.as_mut().unwrap()
    }

    // Take field
    pub fn take_split_rows(&mut self) -> super::wire_protocol::RowOperationsPB {
        self.split_rows.take().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::new())
    }

    pub fn get_split_rows(&self) -> &super::wire_protocol::RowOperationsPB {
        self.split_rows.as_ref().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::default_instance())
    }

    // optional .kudu.PartitionSchemaPB partition_schema = 7;

    pub fn clear_partition_schema(&mut self) {
        self.partition_schema.clear();
    }

    pub fn has_partition_schema(&self) -> bool {
        self.partition_schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_schema(&mut self, v: super::common::PartitionSchemaPB) {
        self.partition_schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_schema(&mut self) -> &mut super::common::PartitionSchemaPB {
        if self.partition_schema.is_none() {
            self.partition_schema.set_default();
        };
        self.partition_schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_schema(&mut self) -> super::common::PartitionSchemaPB {
        self.partition_schema.take().unwrap_or_else(|| super::common::PartitionSchemaPB::new())
    }

    pub fn get_partition_schema(&self) -> &super::common::PartitionSchemaPB {
        self.partition_schema.as_ref().unwrap_or_else(|| super::common::PartitionSchemaPB::default_instance())
    }

    // optional int32 num_replicas = 4;

    pub fn clear_num_replicas(&mut self) {
        self.num_replicas = ::std::option::Option::None;
    }

    pub fn has_num_replicas(&self) -> bool {
        self.num_replicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_replicas(&mut self, v: i32) {
        self.num_replicas = ::std::option::Option::Some(v);
    }

    pub fn get_num_replicas(&self) -> i32 {
        self.num_replicas.unwrap_or(0)
    }
}

impl ::protobuf::Message for CreateTableRequestPB {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.schema.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.split_rows));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.split_rows.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.partition_schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.num_replicas.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.name.as_ref() {
            try!(w.write_string(1, &v));
        };
        if let Some(v) = self.schema.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.split_rows.as_ref() {
            try!(w.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.partition_schema.as_ref() {
            try!(w.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.num_replicas {
            try!(w.write_int32(4, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateTableRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateTableRequestPB {
    fn new() -> CreateTableRequestPB {
        CreateTableRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateTableRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CreateTableRequestPB::has_name,
                    CreateTableRequestPB::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "schema",
                    CreateTableRequestPB::has_schema,
                    CreateTableRequestPB::get_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "split_rows",
                    CreateTableRequestPB::has_split_rows,
                    CreateTableRequestPB::get_split_rows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition_schema",
                    CreateTableRequestPB::has_partition_schema,
                    CreateTableRequestPB::get_partition_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_replicas",
                    CreateTableRequestPB::has_num_replicas,
                    CreateTableRequestPB::get_num_replicas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateTableRequestPB>(
                    "CreateTableRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateTableRequestPB {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_schema();
        self.clear_split_rows();
        self.clear_partition_schema();
        self.clear_num_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateTableRequestPB {
    fn eq(&self, other: &CreateTableRequestPB) -> bool {
        self.name == other.name &&
        self.schema == other.schema &&
        self.split_rows == other.split_rows &&
        self.partition_schema == other.partition_schema &&
        self.num_replicas == other.num_replicas &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateTableResponsePB {}

impl CreateTableResponsePB {
    pub fn new() -> CreateTableResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateTableResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<CreateTableResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateTableResponsePB,
        };
        unsafe {
            instance.get(|| {
                CreateTableResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    table_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional bytes table_id = 2;

    pub fn clear_table_id(&mut self) {
        self.table_id.clear();
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.table_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table_id.is_none() {
            self.table_id.set_default();
        };
        self.table_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_id(&mut self) -> ::std::vec::Vec<u8> {
        self.table_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table_id(&self) -> &[u8] {
        match self.table_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CreateTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.table_id.as_ref() {
            try!(w.write_bytes(2, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateTableResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateTableResponsePB {
    fn new() -> CreateTableResponsePB {
        CreateTableResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateTableResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    CreateTableResponsePB::has_error,
                    CreateTableResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table_id",
                    CreateTableResponsePB::has_table_id,
                    CreateTableResponsePB::get_table_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateTableResponsePB>(
                    "CreateTableResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateTableResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_table_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateTableResponsePB {
    fn eq(&self, other: &CreateTableResponsePB) -> bool {
        self.error == other.error &&
        self.table_id == other.table_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsCreateTableDoneRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsCreateTableDoneRequestPB {}

impl IsCreateTableDoneRequestPB {
    pub fn new() -> IsCreateTableDoneRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsCreateTableDoneRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<IsCreateTableDoneRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsCreateTableDoneRequestPB,
        };
        unsafe {
            instance.get(|| {
                IsCreateTableDoneRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }
}

impl ::protobuf::Message for IsCreateTableDoneRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsCreateTableDoneRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsCreateTableDoneRequestPB {
    fn new() -> IsCreateTableDoneRequestPB {
        IsCreateTableDoneRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsCreateTableDoneRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    IsCreateTableDoneRequestPB::has_table,
                    IsCreateTableDoneRequestPB::get_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsCreateTableDoneRequestPB>(
                    "IsCreateTableDoneRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsCreateTableDoneRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsCreateTableDoneRequestPB {
    fn eq(&self, other: &IsCreateTableDoneRequestPB) -> bool {
        self.table == other.table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsCreateTableDoneRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsCreateTableDoneResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsCreateTableDoneResponsePB {}

impl IsCreateTableDoneResponsePB {
    pub fn new() -> IsCreateTableDoneResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsCreateTableDoneResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<IsCreateTableDoneResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsCreateTableDoneResponsePB,
        };
        unsafe {
            instance.get(|| {
                IsCreateTableDoneResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for IsCreateTableDoneResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.done {
            try!(w.write_bool(3, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsCreateTableDoneResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsCreateTableDoneResponsePB {
    fn new() -> IsCreateTableDoneResponsePB {
        IsCreateTableDoneResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsCreateTableDoneResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    IsCreateTableDoneResponsePB::has_error,
                    IsCreateTableDoneResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    IsCreateTableDoneResponsePB::has_done,
                    IsCreateTableDoneResponsePB::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsCreateTableDoneResponsePB>(
                    "IsCreateTableDoneResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsCreateTableDoneResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsCreateTableDoneResponsePB {
    fn eq(&self, other: &IsCreateTableDoneResponsePB) -> bool {
        self.error == other.error &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsCreateTableDoneResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteTableRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteTableRequestPB {}

impl DeleteTableRequestPB {
    pub fn new() -> DeleteTableRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteTableRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<DeleteTableRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteTableRequestPB,
        };
        unsafe {
            instance.get(|| {
                DeleteTableRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }
}

impl ::protobuf::Message for DeleteTableRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeleteTableRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteTableRequestPB {
    fn new() -> DeleteTableRequestPB {
        DeleteTableRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteTableRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    DeleteTableRequestPB::has_table,
                    DeleteTableRequestPB::get_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteTableRequestPB>(
                    "DeleteTableRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteTableRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteTableRequestPB {
    fn eq(&self, other: &DeleteTableRequestPB) -> bool {
        self.table == other.table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteTableResponsePB {}

impl DeleteTableResponsePB {
    pub fn new() -> DeleteTableResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteTableResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<DeleteTableResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteTableResponsePB,
        };
        unsafe {
            instance.get(|| {
                DeleteTableResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }
}

impl ::protobuf::Message for DeleteTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeleteTableResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteTableResponsePB {
    fn new() -> DeleteTableResponsePB {
        DeleteTableResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteTableResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    DeleteTableResponsePB::has_error,
                    DeleteTableResponsePB::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteTableResponsePB>(
                    "DeleteTableResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteTableResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteTableResponsePB {
    fn eq(&self, other: &DeleteTableResponsePB) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTablesRequestPB {
    // message fields
    name_filter: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTablesRequestPB {}

impl ListTablesRequestPB {
    pub fn new() -> ListTablesRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTablesRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<ListTablesRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTablesRequestPB,
        };
        unsafe {
            instance.get(|| {
                ListTablesRequestPB {
                    name_filter: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name_filter = 1;

    pub fn clear_name_filter(&mut self) {
        self.name_filter.clear();
    }

    pub fn has_name_filter(&self) -> bool {
        self.name_filter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_filter(&mut self, v: ::std::string::String) {
        self.name_filter = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name_filter(&mut self) -> &mut ::std::string::String {
        if self.name_filter.is_none() {
            self.name_filter.set_default();
        };
        self.name_filter.as_mut().unwrap()
    }

    // Take field
    pub fn take_name_filter(&mut self) -> ::std::string::String {
        self.name_filter.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name_filter(&self) -> &str {
        match self.name_filter.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ListTablesRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name_filter));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name_filter.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.name_filter.as_ref() {
            try!(w.write_string(1, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTablesRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTablesRequestPB {
    fn new() -> ListTablesRequestPB {
        ListTablesRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTablesRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name_filter",
                    ListTablesRequestPB::has_name_filter,
                    ListTablesRequestPB::get_name_filter,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListTablesRequestPB>(
                    "ListTablesRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTablesRequestPB {
    fn clear(&mut self) {
        self.clear_name_filter();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTablesRequestPB {
    fn eq(&self, other: &ListTablesRequestPB) -> bool {
        self.name_filter == other.name_filter &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTablesRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTablesResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tables: ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTablesResponsePB {}

impl ListTablesResponsePB {
    pub fn new() -> ListTablesResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTablesResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<ListTablesResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTablesResponsePB,
        };
        unsafe {
            instance.get(|| {
                ListTablesResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    tables: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // repeated .kudu.master.ListTablesResponsePB.TableInfo tables = 2;

    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo> {
        ::std::mem::replace(&mut self.tables, ::protobuf::RepeatedField::new())
    }

    pub fn get_tables(&self) -> &[ListTablesResponsePB_TableInfo] {
        &self.tables
    }
}

impl ::protobuf::Message for ListTablesResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tables.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.tables.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTablesResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTablesResponsePB {
    fn new() -> ListTablesResponsePB {
        ListTablesResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTablesResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ListTablesResponsePB::has_error,
                    ListTablesResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tables",
                    ListTablesResponsePB::get_tables,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListTablesResponsePB>(
                    "ListTablesResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTablesResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_tables();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTablesResponsePB {
    fn eq(&self, other: &ListTablesResponsePB) -> bool {
        self.error == other.error &&
        self.tables == other.tables &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTablesResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTablesResponsePB_TableInfo {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTablesResponsePB_TableInfo {}

impl ListTablesResponsePB_TableInfo {
    pub fn new() -> ListTablesResponsePB_TableInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTablesResponsePB_TableInfo {
        static mut instance: ::protobuf::lazy::Lazy<ListTablesResponsePB_TableInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTablesResponsePB_TableInfo,
        };
        unsafe {
            instance.get(|| {
                ListTablesResponsePB_TableInfo {
                    id: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ListTablesResponsePB_TableInfo {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.id.as_ref() {
            try!(w.write_bytes(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(w.write_string(2, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTablesResponsePB_TableInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTablesResponsePB_TableInfo {
    fn new() -> ListTablesResponsePB_TableInfo {
        ListTablesResponsePB_TableInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTablesResponsePB_TableInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "id",
                    ListTablesResponsePB_TableInfo::has_id,
                    ListTablesResponsePB_TableInfo::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    ListTablesResponsePB_TableInfo::has_name,
                    ListTablesResponsePB_TableInfo::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListTablesResponsePB_TableInfo>(
                    "ListTablesResponsePB_TableInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTablesResponsePB_TableInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTablesResponsePB_TableInfo {
    fn eq(&self, other: &ListTablesResponsePB_TableInfo) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTablesResponsePB_TableInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTableLocationsRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    partition_key_start: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition_key_end: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    max_returned_locations: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTableLocationsRequestPB {}

impl GetTableLocationsRequestPB {
    pub fn new() -> GetTableLocationsRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTableLocationsRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<GetTableLocationsRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTableLocationsRequestPB,
        };
        unsafe {
            instance.get(|| {
                GetTableLocationsRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    partition_key_start: ::protobuf::SingularField::none(),
                    partition_key_end: ::protobuf::SingularField::none(),
                    max_returned_locations: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }

    // optional bytes partition_key_start = 3;

    pub fn clear_partition_key_start(&mut self) {
        self.partition_key_start.clear();
    }

    pub fn has_partition_key_start(&self) -> bool {
        self.partition_key_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_key_start(&mut self, v: ::std::vec::Vec<u8>) {
        self.partition_key_start = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_key_start(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.partition_key_start.is_none() {
            self.partition_key_start.set_default();
        };
        self.partition_key_start.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_key_start(&mut self) -> ::std::vec::Vec<u8> {
        self.partition_key_start.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_partition_key_start(&self) -> &[u8] {
        match self.partition_key_start.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes partition_key_end = 4;

    pub fn clear_partition_key_end(&mut self) {
        self.partition_key_end.clear();
    }

    pub fn has_partition_key_end(&self) -> bool {
        self.partition_key_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_key_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.partition_key_end = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_key_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.partition_key_end.is_none() {
            self.partition_key_end.set_default();
        };
        self.partition_key_end.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_key_end(&mut self) -> ::std::vec::Vec<u8> {
        self.partition_key_end.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_partition_key_end(&self) -> &[u8] {
        match self.partition_key_end.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 max_returned_locations = 5;

    pub fn clear_max_returned_locations(&mut self) {
        self.max_returned_locations = ::std::option::Option::None;
    }

    pub fn has_max_returned_locations(&self) -> bool {
        self.max_returned_locations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_returned_locations(&mut self, v: u32) {
        self.max_returned_locations = ::std::option::Option::Some(v);
    }

    pub fn get_max_returned_locations(&self) -> u32 {
        self.max_returned_locations.unwrap_or(10u32)
    }
}

impl ::protobuf::Message for GetTableLocationsRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.partition_key_start));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.partition_key_end));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.max_returned_locations = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.partition_key_start.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.partition_key_end.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.max_returned_locations.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.partition_key_start.as_ref() {
            try!(w.write_bytes(3, &v));
        };
        if let Some(v) = self.partition_key_end.as_ref() {
            try!(w.write_bytes(4, &v));
        };
        if let Some(v) = self.max_returned_locations {
            try!(w.write_uint32(5, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTableLocationsRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTableLocationsRequestPB {
    fn new() -> GetTableLocationsRequestPB {
        GetTableLocationsRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTableLocationsRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    GetTableLocationsRequestPB::has_table,
                    GetTableLocationsRequestPB::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "partition_key_start",
                    GetTableLocationsRequestPB::has_partition_key_start,
                    GetTableLocationsRequestPB::get_partition_key_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "partition_key_end",
                    GetTableLocationsRequestPB::has_partition_key_end,
                    GetTableLocationsRequestPB::get_partition_key_end,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_returned_locations",
                    GetTableLocationsRequestPB::has_max_returned_locations,
                    GetTableLocationsRequestPB::get_max_returned_locations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTableLocationsRequestPB>(
                    "GetTableLocationsRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTableLocationsRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_partition_key_start();
        self.clear_partition_key_end();
        self.clear_max_returned_locations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTableLocationsRequestPB {
    fn eq(&self, other: &GetTableLocationsRequestPB) -> bool {
        self.table == other.table &&
        self.partition_key_start == other.partition_key_start &&
        self.partition_key_end == other.partition_key_end &&
        self.max_returned_locations == other.max_returned_locations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTableLocationsRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTableLocationsResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tablet_locations: ::protobuf::RepeatedField<TabletLocationsPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTableLocationsResponsePB {}

impl GetTableLocationsResponsePB {
    pub fn new() -> GetTableLocationsResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTableLocationsResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<GetTableLocationsResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTableLocationsResponsePB,
        };
        unsafe {
            instance.get(|| {
                GetTableLocationsResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    tablet_locations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // repeated .kudu.master.TabletLocationsPB tablet_locations = 2;

    pub fn clear_tablet_locations(&mut self) {
        self.tablet_locations.clear();
    }

    // Param is passed by value, moved
    pub fn set_tablet_locations(&mut self, v: ::protobuf::RepeatedField<TabletLocationsPB>) {
        self.tablet_locations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tablet_locations(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB> {
        &mut self.tablet_locations
    }

    // Take field
    pub fn take_tablet_locations(&mut self) -> ::protobuf::RepeatedField<TabletLocationsPB> {
        ::std::mem::replace(&mut self.tablet_locations, ::protobuf::RepeatedField::new())
    }

    pub fn get_tablet_locations(&self) -> &[TabletLocationsPB] {
        &self.tablet_locations
    }
}

impl ::protobuf::Message for GetTableLocationsResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablet_locations));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tablet_locations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.tablet_locations.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTableLocationsResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTableLocationsResponsePB {
    fn new() -> GetTableLocationsResponsePB {
        GetTableLocationsResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTableLocationsResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    GetTableLocationsResponsePB::has_error,
                    GetTableLocationsResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tablet_locations",
                    GetTableLocationsResponsePB::get_tablet_locations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTableLocationsResponsePB>(
                    "GetTableLocationsResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTableLocationsResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_tablet_locations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTableLocationsResponsePB {
    fn eq(&self, other: &GetTableLocationsResponsePB) -> bool {
        self.error == other.error &&
        self.tablet_locations == other.tablet_locations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTableLocationsResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlterTableRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    alter_schema_steps: ::protobuf::RepeatedField<AlterTableRequestPB_Step>,
    new_table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB {}

impl AlterTableRequestPB {
    pub fn new() -> AlterTableRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB,
        };
        unsafe {
            instance.get(|| {
                AlterTableRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    alter_schema_steps: ::protobuf::RepeatedField::new(),
                    new_table_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }

    // repeated .kudu.master.AlterTableRequestPB.Step alter_schema_steps = 2;

    pub fn clear_alter_schema_steps(&mut self) {
        self.alter_schema_steps.clear();
    }

    // Param is passed by value, moved
    pub fn set_alter_schema_steps(&mut self, v: ::protobuf::RepeatedField<AlterTableRequestPB_Step>) {
        self.alter_schema_steps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alter_schema_steps(&mut self) -> &mut ::protobuf::RepeatedField<AlterTableRequestPB_Step> {
        &mut self.alter_schema_steps
    }

    // Take field
    pub fn take_alter_schema_steps(&mut self) -> ::protobuf::RepeatedField<AlterTableRequestPB_Step> {
        ::std::mem::replace(&mut self.alter_schema_steps, ::protobuf::RepeatedField::new())
    }

    pub fn get_alter_schema_steps(&self) -> &[AlterTableRequestPB_Step] {
        &self.alter_schema_steps
    }

    // optional string new_table_name = 3;

    pub fn clear_new_table_name(&mut self) {
        self.new_table_name.clear();
    }

    pub fn has_new_table_name(&self) -> bool {
        self.new_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_table_name(&mut self, v: ::std::string::String) {
        self.new_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_table_name(&mut self) -> &mut ::std::string::String {
        if self.new_table_name.is_none() {
            self.new_table_name.set_default();
        };
        self.new_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_table_name(&mut self) -> ::std::string::String {
        self.new_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_new_table_name(&self) -> &str {
        match self.new_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AlterTableRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alter_schema_steps));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_table_name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.alter_schema_steps.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.new_table_name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.alter_schema_steps.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.new_table_name.as_ref() {
            try!(w.write_string(3, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB {
    fn new() -> AlterTableRequestPB {
        AlterTableRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    AlterTableRequestPB::has_table,
                    AlterTableRequestPB::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "alter_schema_steps",
                    AlterTableRequestPB::get_alter_schema_steps,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "new_table_name",
                    AlterTableRequestPB::has_new_table_name,
                    AlterTableRequestPB::get_new_table_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB>(
                    "AlterTableRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_alter_schema_steps();
        self.clear_new_table_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableRequestPB {
    fn eq(&self, other: &AlterTableRequestPB) -> bool {
        self.table == other.table &&
        self.alter_schema_steps == other.alter_schema_steps &&
        self.new_table_name == other.new_table_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlterTableRequestPB_AddColumn {
    // message fields
    schema: ::protobuf::SingularPtrField<super::common::ColumnSchemaPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_AddColumn {}

impl AlterTableRequestPB_AddColumn {
    pub fn new() -> AlterTableRequestPB_AddColumn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_AddColumn {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_AddColumn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_AddColumn,
        };
        unsafe {
            instance.get(|| {
                AlterTableRequestPB_AddColumn {
                    schema: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.ColumnSchemaPB schema = 1;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: super::common::ColumnSchemaPB) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut super::common::ColumnSchemaPB {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> super::common::ColumnSchemaPB {
        self.schema.take().unwrap_or_else(|| super::common::ColumnSchemaPB::new())
    }

    pub fn get_schema(&self) -> &super::common::ColumnSchemaPB {
        self.schema.as_ref().unwrap_or_else(|| super::common::ColumnSchemaPB::default_instance())
    }
}

impl ::protobuf::Message for AlterTableRequestPB_AddColumn {
    fn is_initialized(&self) -> bool {
        if self.schema.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.schema.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableRequestPB_AddColumn>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_AddColumn {
    fn new() -> AlterTableRequestPB_AddColumn {
        AlterTableRequestPB_AddColumn::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_AddColumn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "schema",
                    AlterTableRequestPB_AddColumn::has_schema,
                    AlterTableRequestPB_AddColumn::get_schema,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_AddColumn>(
                    "AlterTableRequestPB_AddColumn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_AddColumn {
    fn clear(&mut self) {
        self.clear_schema();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableRequestPB_AddColumn {
    fn eq(&self, other: &AlterTableRequestPB_AddColumn) -> bool {
        self.schema == other.schema &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_AddColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlterTableRequestPB_DropColumn {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_DropColumn {}

impl AlterTableRequestPB_DropColumn {
    pub fn new() -> AlterTableRequestPB_DropColumn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_DropColumn {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_DropColumn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_DropColumn,
        };
        unsafe {
            instance.get(|| {
                AlterTableRequestPB_DropColumn {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AlterTableRequestPB_DropColumn {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.name.as_ref() {
            try!(w.write_string(1, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableRequestPB_DropColumn>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_DropColumn {
    fn new() -> AlterTableRequestPB_DropColumn {
        AlterTableRequestPB_DropColumn::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_DropColumn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    AlterTableRequestPB_DropColumn::has_name,
                    AlterTableRequestPB_DropColumn::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_DropColumn>(
                    "AlterTableRequestPB_DropColumn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_DropColumn {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableRequestPB_DropColumn {
    fn eq(&self, other: &AlterTableRequestPB_DropColumn) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_DropColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlterTableRequestPB_RenameColumn {
    // message fields
    old_name: ::protobuf::SingularField<::std::string::String>,
    new_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_RenameColumn {}

impl AlterTableRequestPB_RenameColumn {
    pub fn new() -> AlterTableRequestPB_RenameColumn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_RenameColumn {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_RenameColumn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_RenameColumn,
        };
        unsafe {
            instance.get(|| {
                AlterTableRequestPB_RenameColumn {
                    old_name: ::protobuf::SingularField::none(),
                    new_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string old_name = 1;

    pub fn clear_old_name(&mut self) {
        self.old_name.clear();
    }

    pub fn has_old_name(&self) -> bool {
        self.old_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_name(&mut self, v: ::std::string::String) {
        self.old_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_name(&mut self) -> &mut ::std::string::String {
        if self.old_name.is_none() {
            self.old_name.set_default();
        };
        self.old_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_name(&mut self) -> ::std::string::String {
        self.old_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_old_name(&self) -> &str {
        match self.old_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string new_name = 2;

    pub fn clear_new_name(&mut self) {
        self.new_name.clear();
    }

    pub fn has_new_name(&self) -> bool {
        self.new_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_name(&mut self, v: ::std::string::String) {
        self.new_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_name(&mut self) -> &mut ::std::string::String {
        if self.new_name.is_none() {
            self.new_name.set_default();
        };
        self.new_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_name(&mut self) -> ::std::string::String {
        self.new_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_new_name(&self) -> &str {
        match self.new_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AlterTableRequestPB_RenameColumn {
    fn is_initialized(&self) -> bool {
        if self.old_name.is_none() {
            return false;
        };
        if self.new_name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.old_name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.old_name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.new_name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.old_name.as_ref() {
            try!(w.write_string(1, &v));
        };
        if let Some(v) = self.new_name.as_ref() {
            try!(w.write_string(2, &v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableRequestPB_RenameColumn>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_RenameColumn {
    fn new() -> AlterTableRequestPB_RenameColumn {
        AlterTableRequestPB_RenameColumn::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_RenameColumn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "old_name",
                    AlterTableRequestPB_RenameColumn::has_old_name,
                    AlterTableRequestPB_RenameColumn::get_old_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "new_name",
                    AlterTableRequestPB_RenameColumn::has_new_name,
                    AlterTableRequestPB_RenameColumn::get_new_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_RenameColumn>(
                    "AlterTableRequestPB_RenameColumn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_RenameColumn {
    fn clear(&mut self) {
        self.clear_old_name();
        self.clear_new_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableRequestPB_RenameColumn {
    fn eq(&self, other: &AlterTableRequestPB_RenameColumn) -> bool {
        self.old_name == other.old_name &&
        self.new_name == other.new_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_RenameColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlterTableRequestPB_Step {
    // message fields
    field_type: ::std::option::Option<AlterTableRequestPB_StepType>,
    add_column: ::protobuf::SingularPtrField<AlterTableRequestPB_AddColumn>,
    drop_column: ::protobuf::SingularPtrField<AlterTableRequestPB_DropColumn>,
    rename_column: ::protobuf::SingularPtrField<AlterTableRequestPB_RenameColumn>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_Step {}

impl AlterTableRequestPB_Step {
    pub fn new() -> AlterTableRequestPB_Step {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_Step {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_Step> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_Step,
        };
        unsafe {
            instance.get(|| {
                AlterTableRequestPB_Step {
                    field_type: ::std::option::Option::None,
                    add_column: ::protobuf::SingularPtrField::none(),
                    drop_column: ::protobuf::SingularPtrField::none(),
                    rename_column: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.AlterTableRequestPB.StepType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: AlterTableRequestPB_StepType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> AlterTableRequestPB_StepType {
        self.field_type.unwrap_or(AlterTableRequestPB_StepType::UNKNOWN)
    }

    // optional .kudu.master.AlterTableRequestPB.AddColumn add_column = 2;

    pub fn clear_add_column(&mut self) {
        self.add_column.clear();
    }

    pub fn has_add_column(&self) -> bool {
        self.add_column.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_column(&mut self, v: AlterTableRequestPB_AddColumn) {
        self.add_column = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_column(&mut self) -> &mut AlterTableRequestPB_AddColumn {
        if self.add_column.is_none() {
            self.add_column.set_default();
        };
        self.add_column.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_column(&mut self) -> AlterTableRequestPB_AddColumn {
        self.add_column.take().unwrap_or_else(|| AlterTableRequestPB_AddColumn::new())
    }

    pub fn get_add_column(&self) -> &AlterTableRequestPB_AddColumn {
        self.add_column.as_ref().unwrap_or_else(|| AlterTableRequestPB_AddColumn::default_instance())
    }

    // optional .kudu.master.AlterTableRequestPB.DropColumn drop_column = 3;

    pub fn clear_drop_column(&mut self) {
        self.drop_column.clear();
    }

    pub fn has_drop_column(&self) -> bool {
        self.drop_column.is_some()
    }

    // Param is passed by value, moved
    pub fn set_drop_column(&mut self, v: AlterTableRequestPB_DropColumn) {
        self.drop_column = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_drop_column(&mut self) -> &mut AlterTableRequestPB_DropColumn {
        if self.drop_column.is_none() {
            self.drop_column.set_default();
        };
        self.drop_column.as_mut().unwrap()
    }

    // Take field
    pub fn take_drop_column(&mut self) -> AlterTableRequestPB_DropColumn {
        self.drop_column.take().unwrap_or_else(|| AlterTableRequestPB_DropColumn::new())
    }

    pub fn get_drop_column(&self) -> &AlterTableRequestPB_DropColumn {
        self.drop_column.as_ref().unwrap_or_else(|| AlterTableRequestPB_DropColumn::default_instance())
    }

    // optional .kudu.master.AlterTableRequestPB.RenameColumn rename_column = 4;

    pub fn clear_rename_column(&mut self) {
        self.rename_column.clear();
    }

    pub fn has_rename_column(&self) -> bool {
        self.rename_column.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rename_column(&mut self, v: AlterTableRequestPB_RenameColumn) {
        self.rename_column = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rename_column(&mut self) -> &mut AlterTableRequestPB_RenameColumn {
        if self.rename_column.is_none() {
            self.rename_column.set_default();
        };
        self.rename_column.as_mut().unwrap()
    }

    // Take field
    pub fn take_rename_column(&mut self) -> AlterTableRequestPB_RenameColumn {
        self.rename_column.take().unwrap_or_else(|| AlterTableRequestPB_RenameColumn::new())
    }

    pub fn get_rename_column(&self) -> &AlterTableRequestPB_RenameColumn {
        self.rename_column.as_ref().unwrap_or_else(|| AlterTableRequestPB_RenameColumn::default_instance())
    }
}

impl ::protobuf::Message for AlterTableRequestPB_Step {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_column));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.drop_column));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rename_column));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.add_column.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.drop_column.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.rename_column.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.field_type {
            try!(w.write_enum(1, v.value()));
        };
        if let Some(v) = self.add_column.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.drop_column.as_ref() {
            try!(w.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.rename_column.as_ref() {
            try!(w.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableRequestPB_Step>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_Step {
    fn new() -> AlterTableRequestPB_Step {
        AlterTableRequestPB_Step::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_Step>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    AlterTableRequestPB_Step::has_field_type,
                    AlterTableRequestPB_Step::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "add_column",
                    AlterTableRequestPB_Step::has_add_column,
                    AlterTableRequestPB_Step::get_add_column,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "drop_column",
                    AlterTableRequestPB_Step::has_drop_column,
                    AlterTableRequestPB_Step::get_drop_column,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "rename_column",
                    AlterTableRequestPB_Step::has_rename_column,
                    AlterTableRequestPB_Step::get_rename_column,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_Step>(
                    "AlterTableRequestPB_Step",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_Step {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_add_column();
        self.clear_drop_column();
        self.clear_rename_column();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableRequestPB_Step {
    fn eq(&self, other: &AlterTableRequestPB_Step) -> bool {
        self.field_type == other.field_type &&
        self.add_column == other.add_column &&
        self.drop_column == other.drop_column &&
        self.rename_column == other.rename_column &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_Step {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AlterTableRequestPB_StepType {
    UNKNOWN = 0,
    ADD_COLUMN = 1,
    DROP_COLUMN = 2,
    RENAME_COLUMN = 3,
    ALTER_COLUMN = 4,
}

impl ::protobuf::ProtobufEnum for AlterTableRequestPB_StepType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AlterTableRequestPB_StepType> {
        match value {
            0 => ::std::option::Option::Some(AlterTableRequestPB_StepType::UNKNOWN),
            1 => ::std::option::Option::Some(AlterTableRequestPB_StepType::ADD_COLUMN),
            2 => ::std::option::Option::Some(AlterTableRequestPB_StepType::DROP_COLUMN),
            3 => ::std::option::Option::Some(AlterTableRequestPB_StepType::RENAME_COLUMN),
            4 => ::std::option::Option::Some(AlterTableRequestPB_StepType::ALTER_COLUMN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AlterTableRequestPB_StepType] = &[
            AlterTableRequestPB_StepType::UNKNOWN,
            AlterTableRequestPB_StepType::ADD_COLUMN,
            AlterTableRequestPB_StepType::DROP_COLUMN,
            AlterTableRequestPB_StepType::RENAME_COLUMN,
            AlterTableRequestPB_StepType::ALTER_COLUMN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AlterTableRequestPB_StepType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AlterTableRequestPB_StepType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AlterTableRequestPB_StepType {
}

#[derive(Clone,Default)]
pub struct AlterTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableResponsePB {}

impl AlterTableResponsePB {
    pub fn new() -> AlterTableResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableResponsePB,
        };
        unsafe {
            instance.get(|| {
                AlterTableResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    schema_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional uint32 schema_version = 2;

    pub fn clear_schema_version(&mut self) {
        self.schema_version = ::std::option::Option::None;
    }

    pub fn has_schema_version(&self) -> bool {
        self.schema_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema_version(&mut self, v: u32) {
        self.schema_version = ::std::option::Option::Some(v);
    }

    pub fn get_schema_version(&self) -> u32 {
        self.schema_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for AlterTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.schema_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.schema_version {
            try!(w.write_uint32(2, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlterTableResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableResponsePB {
    fn new() -> AlterTableResponsePB {
        AlterTableResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    AlterTableResponsePB::has_error,
                    AlterTableResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "schema_version",
                    AlterTableResponsePB::has_schema_version,
                    AlterTableResponsePB::get_schema_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableResponsePB>(
                    "AlterTableResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_schema_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlterTableResponsePB {
    fn eq(&self, other: &AlterTableResponsePB) -> bool {
        self.error == other.error &&
        self.schema_version == other.schema_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlterTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsAlterTableDoneRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsAlterTableDoneRequestPB {}

impl IsAlterTableDoneRequestPB {
    pub fn new() -> IsAlterTableDoneRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsAlterTableDoneRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<IsAlterTableDoneRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsAlterTableDoneRequestPB,
        };
        unsafe {
            instance.get(|| {
                IsAlterTableDoneRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }
}

impl ::protobuf::Message for IsAlterTableDoneRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsAlterTableDoneRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsAlterTableDoneRequestPB {
    fn new() -> IsAlterTableDoneRequestPB {
        IsAlterTableDoneRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsAlterTableDoneRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    IsAlterTableDoneRequestPB::has_table,
                    IsAlterTableDoneRequestPB::get_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsAlterTableDoneRequestPB>(
                    "IsAlterTableDoneRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsAlterTableDoneRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsAlterTableDoneRequestPB {
    fn eq(&self, other: &IsAlterTableDoneRequestPB) -> bool {
        self.table == other.table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsAlterTableDoneRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsAlterTableDoneResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema_version: ::std::option::Option<u32>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsAlterTableDoneResponsePB {}

impl IsAlterTableDoneResponsePB {
    pub fn new() -> IsAlterTableDoneResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsAlterTableDoneResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<IsAlterTableDoneResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsAlterTableDoneResponsePB,
        };
        unsafe {
            instance.get(|| {
                IsAlterTableDoneResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    schema_version: ::std::option::Option::None,
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional uint32 schema_version = 2;

    pub fn clear_schema_version(&mut self) {
        self.schema_version = ::std::option::Option::None;
    }

    pub fn has_schema_version(&self) -> bool {
        self.schema_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema_version(&mut self, v: u32) {
        self.schema_version = ::std::option::Option::Some(v);
    }

    pub fn get_schema_version(&self) -> u32 {
        self.schema_version.unwrap_or(0)
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for IsAlterTableDoneResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.schema_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.schema_version {
            try!(w.write_uint32(2, v));
        };
        if let Some(v) = self.done {
            try!(w.write_bool(3, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsAlterTableDoneResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsAlterTableDoneResponsePB {
    fn new() -> IsAlterTableDoneResponsePB {
        IsAlterTableDoneResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsAlterTableDoneResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    IsAlterTableDoneResponsePB::has_error,
                    IsAlterTableDoneResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "schema_version",
                    IsAlterTableDoneResponsePB::has_schema_version,
                    IsAlterTableDoneResponsePB::get_schema_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    IsAlterTableDoneResponsePB::has_done,
                    IsAlterTableDoneResponsePB::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsAlterTableDoneResponsePB>(
                    "IsAlterTableDoneResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsAlterTableDoneResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_schema_version();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsAlterTableDoneResponsePB {
    fn eq(&self, other: &IsAlterTableDoneResponsePB) -> bool {
        self.error == other.error &&
        self.schema_version == other.schema_version &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsAlterTableDoneResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTableSchemaRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTableSchemaRequestPB {}

impl GetTableSchemaRequestPB {
    pub fn new() -> GetTableSchemaRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTableSchemaRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<GetTableSchemaRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTableSchemaRequestPB,
        };
        unsafe {
            instance.get(|| {
                GetTableSchemaRequestPB {
                    table: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.master.TableIdentifierPB table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableIdentifierPB) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableIdentifierPB {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableIdentifierPB {
        self.table.take().unwrap_or_else(|| TableIdentifierPB::new())
    }

    pub fn get_table(&self) -> &TableIdentifierPB {
        self.table.as_ref().unwrap_or_else(|| TableIdentifierPB::default_instance())
    }
}

impl ::protobuf::Message for GetTableSchemaRequestPB {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.table.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTableSchemaRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTableSchemaRequestPB {
    fn new() -> GetTableSchemaRequestPB {
        GetTableSchemaRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTableSchemaRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    GetTableSchemaRequestPB::has_table,
                    GetTableSchemaRequestPB::get_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTableSchemaRequestPB>(
                    "GetTableSchemaRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTableSchemaRequestPB {
    fn clear(&mut self) {
        self.clear_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTableSchemaRequestPB {
    fn eq(&self, other: &GetTableSchemaRequestPB) -> bool {
        self.table == other.table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTableSchemaRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetTableSchemaResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    partition_schema: ::protobuf::SingularPtrField<super::common::PartitionSchemaPB>,
    num_replicas: ::std::option::Option<i32>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    create_table_done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTableSchemaResponsePB {}

impl GetTableSchemaResponsePB {
    pub fn new() -> GetTableSchemaResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTableSchemaResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<GetTableSchemaResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTableSchemaResponsePB,
        };
        unsafe {
            instance.get(|| {
                GetTableSchemaResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    schema: ::protobuf::SingularPtrField::none(),
                    partition_schema: ::protobuf::SingularPtrField::none(),
                    num_replicas: ::std::option::Option::None,
                    table_id: ::protobuf::SingularField::none(),
                    create_table_done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // optional .kudu.SchemaPB schema = 2;

    pub fn clear_schema(&mut self) {
        self.schema.clear();
    }

    pub fn has_schema(&self) -> bool {
        self.schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema(&mut self, v: super::common::SchemaPB) {
        self.schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema(&mut self) -> &mut super::common::SchemaPB {
        if self.schema.is_none() {
            self.schema.set_default();
        };
        self.schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema(&mut self) -> super::common::SchemaPB {
        self.schema.take().unwrap_or_else(|| super::common::SchemaPB::new())
    }

    pub fn get_schema(&self) -> &super::common::SchemaPB {
        self.schema.as_ref().unwrap_or_else(|| super::common::SchemaPB::default_instance())
    }

    // optional .kudu.PartitionSchemaPB partition_schema = 5;

    pub fn clear_partition_schema(&mut self) {
        self.partition_schema.clear();
    }

    pub fn has_partition_schema(&self) -> bool {
        self.partition_schema.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_schema(&mut self, v: super::common::PartitionSchemaPB) {
        self.partition_schema = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_schema(&mut self) -> &mut super::common::PartitionSchemaPB {
        if self.partition_schema.is_none() {
            self.partition_schema.set_default();
        };
        self.partition_schema.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_schema(&mut self) -> super::common::PartitionSchemaPB {
        self.partition_schema.take().unwrap_or_else(|| super::common::PartitionSchemaPB::new())
    }

    pub fn get_partition_schema(&self) -> &super::common::PartitionSchemaPB {
        self.partition_schema.as_ref().unwrap_or_else(|| super::common::PartitionSchemaPB::default_instance())
    }

    // optional int32 num_replicas = 3;

    pub fn clear_num_replicas(&mut self) {
        self.num_replicas = ::std::option::Option::None;
    }

    pub fn has_num_replicas(&self) -> bool {
        self.num_replicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_replicas(&mut self, v: i32) {
        self.num_replicas = ::std::option::Option::Some(v);
    }

    pub fn get_num_replicas(&self) -> i32 {
        self.num_replicas.unwrap_or(0)
    }

    // optional bytes table_id = 4;

    pub fn clear_table_id(&mut self) {
        self.table_id.clear();
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.table_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table_id.is_none() {
            self.table_id.set_default();
        };
        self.table_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_id(&mut self) -> ::std::vec::Vec<u8> {
        self.table_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table_id(&self) -> &[u8] {
        match self.table_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool create_table_done = 6;

    pub fn clear_create_table_done(&mut self) {
        self.create_table_done = ::std::option::Option::None;
    }

    pub fn has_create_table_done(&self) -> bool {
        self.create_table_done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_table_done(&mut self, v: bool) {
        self.create_table_done = ::std::option::Option::Some(v);
    }

    pub fn get_create_table_done(&self) -> bool {
        self.create_table_done.unwrap_or(false)
    }
}

impl ::protobuf::Message for GetTableSchemaResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.create_table_done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.partition_schema.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.num_replicas.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.table_id.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        if self.create_table_done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.schema.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.partition_schema.as_ref() {
            try!(w.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.num_replicas {
            try!(w.write_int32(3, v));
        };
        if let Some(v) = self.table_id.as_ref() {
            try!(w.write_bytes(4, &v));
        };
        if let Some(v) = self.create_table_done {
            try!(w.write_bool(6, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetTableSchemaResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTableSchemaResponsePB {
    fn new() -> GetTableSchemaResponsePB {
        GetTableSchemaResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTableSchemaResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    GetTableSchemaResponsePB::has_error,
                    GetTableSchemaResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "schema",
                    GetTableSchemaResponsePB::has_schema,
                    GetTableSchemaResponsePB::get_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition_schema",
                    GetTableSchemaResponsePB::has_partition_schema,
                    GetTableSchemaResponsePB::get_partition_schema,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_replicas",
                    GetTableSchemaResponsePB::has_num_replicas,
                    GetTableSchemaResponsePB::get_num_replicas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table_id",
                    GetTableSchemaResponsePB::has_table_id,
                    GetTableSchemaResponsePB::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "create_table_done",
                    GetTableSchemaResponsePB::has_create_table_done,
                    GetTableSchemaResponsePB::get_create_table_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTableSchemaResponsePB>(
                    "GetTableSchemaResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTableSchemaResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_schema();
        self.clear_partition_schema();
        self.clear_num_replicas();
        self.clear_table_id();
        self.clear_create_table_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetTableSchemaResponsePB {
    fn eq(&self, other: &GetTableSchemaResponsePB) -> bool {
        self.error == other.error &&
        self.schema == other.schema &&
        self.partition_schema == other.partition_schema &&
        self.num_replicas == other.num_replicas &&
        self.table_id == other.table_id &&
        self.create_table_done == other.create_table_done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetTableSchemaResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTabletServersRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTabletServersRequestPB {}

impl ListTabletServersRequestPB {
    pub fn new() -> ListTabletServersRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTabletServersRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<ListTabletServersRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTabletServersRequestPB,
        };
        unsafe {
            instance.get(|| {
                ListTabletServersRequestPB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ListTabletServersRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTabletServersRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTabletServersRequestPB {
    fn new() -> ListTabletServersRequestPB {
        ListTabletServersRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTabletServersRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListTabletServersRequestPB>(
                    "ListTabletServersRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTabletServersRequestPB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTabletServersRequestPB {
    fn eq(&self, other: &ListTabletServersRequestPB) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTabletServersRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTabletServersResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    servers: ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTabletServersResponsePB {}

impl ListTabletServersResponsePB {
    pub fn new() -> ListTabletServersResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTabletServersResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<ListTabletServersResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTabletServersResponsePB,
        };
        unsafe {
            instance.get(|| {
                ListTabletServersResponsePB {
                    error: ::protobuf::SingularPtrField::none(),
                    servers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.master.MasterErrorPB error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }

    // repeated .kudu.master.ListTabletServersResponsePB.Entry servers = 2;

    pub fn clear_servers(&mut self) {
        self.servers.clear();
    }

    // Param is passed by value, moved
    pub fn set_servers(&mut self, v: ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry>) {
        self.servers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_servers(&mut self) -> &mut ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry> {
        &mut self.servers
    }

    // Take field
    pub fn take_servers(&mut self) -> ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry> {
        ::std::mem::replace(&mut self.servers, ::protobuf::RepeatedField::new())
    }

    pub fn get_servers(&self) -> &[ListTabletServersResponsePB_Entry] {
        &self.servers
    }
}

impl ::protobuf::Message for ListTabletServersResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.servers));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.servers.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        for v in self.servers.iter() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTabletServersResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTabletServersResponsePB {
    fn new() -> ListTabletServersResponsePB {
        ListTabletServersResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTabletServersResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ListTabletServersResponsePB::has_error,
                    ListTabletServersResponsePB::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "servers",
                    ListTabletServersResponsePB::get_servers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListTabletServersResponsePB>(
                    "ListTabletServersResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTabletServersResponsePB {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_servers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTabletServersResponsePB {
    fn eq(&self, other: &ListTabletServersResponsePB) -> bool {
        self.error == other.error &&
        self.servers == other.servers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTabletServersResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListTabletServersResponsePB_Entry {
    // message fields
    instance_id: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    registration: ::protobuf::SingularPtrField<TSRegistrationPB>,
    millis_since_heartbeat: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListTabletServersResponsePB_Entry {}

impl ListTabletServersResponsePB_Entry {
    pub fn new() -> ListTabletServersResponsePB_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListTabletServersResponsePB_Entry {
        static mut instance: ::protobuf::lazy::Lazy<ListTabletServersResponsePB_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListTabletServersResponsePB_Entry,
        };
        unsafe {
            instance.get(|| {
                ListTabletServersResponsePB_Entry {
                    instance_id: ::protobuf::SingularPtrField::none(),
                    registration: ::protobuf::SingularPtrField::none(),
                    millis_since_heartbeat: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.NodeInstancePB instance_id = 1;

    pub fn clear_instance_id(&mut self) {
        self.instance_id.clear();
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: super::wire_protocol::NodeInstancePB) {
        self.instance_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_id(&mut self) -> &mut super::wire_protocol::NodeInstancePB {
        if self.instance_id.is_none() {
            self.instance_id.set_default();
        };
        self.instance_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_instance_id(&mut self) -> super::wire_protocol::NodeInstancePB {
        self.instance_id.take().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::new())
    }

    pub fn get_instance_id(&self) -> &super::wire_protocol::NodeInstancePB {
        self.instance_id.as_ref().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::default_instance())
    }

    // optional .kudu.master.TSRegistrationPB registration = 2;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: TSRegistrationPB) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut TSRegistrationPB {
        if self.registration.is_none() {
            self.registration.set_default();
        };
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> TSRegistrationPB {
        self.registration.take().unwrap_or_else(|| TSRegistrationPB::new())
    }

    pub fn get_registration(&self) -> &TSRegistrationPB {
        self.registration.as_ref().unwrap_or_else(|| TSRegistrationPB::default_instance())
    }

    // optional int32 millis_since_heartbeat = 3;

    pub fn clear_millis_since_heartbeat(&mut self) {
        self.millis_since_heartbeat = ::std::option::Option::None;
    }

    pub fn has_millis_since_heartbeat(&self) -> bool {
        self.millis_since_heartbeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_millis_since_heartbeat(&mut self, v: i32) {
        self.millis_since_heartbeat = ::std::option::Option::Some(v);
    }

    pub fn get_millis_since_heartbeat(&self) -> i32 {
        self.millis_since_heartbeat.unwrap_or(0)
    }
}

impl ::protobuf::Message for ListTabletServersResponsePB_Entry {
    fn is_initialized(&self) -> bool {
        if self.instance_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instance_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.millis_since_heartbeat = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.instance_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.registration.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.millis_since_heartbeat.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.instance_id.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.registration.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.millis_since_heartbeat {
            try!(w.write_int32(3, v));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListTabletServersResponsePB_Entry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListTabletServersResponsePB_Entry {
    fn new() -> ListTabletServersResponsePB_Entry {
        ListTabletServersResponsePB_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListTabletServersResponsePB_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "instance_id",
                    ListTabletServersResponsePB_Entry::has_instance_id,
                    ListTabletServersResponsePB_Entry::get_instance_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "registration",
                    ListTabletServersResponsePB_Entry::has_registration,
                    ListTabletServersResponsePB_Entry::get_registration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "millis_since_heartbeat",
                    ListTabletServersResponsePB_Entry::has_millis_since_heartbeat,
                    ListTabletServersResponsePB_Entry::get_millis_since_heartbeat,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListTabletServersResponsePB_Entry>(
                    "ListTabletServersResponsePB_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListTabletServersResponsePB_Entry {
    fn clear(&mut self) {
        self.clear_instance_id();
        self.clear_registration();
        self.clear_millis_since_heartbeat();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListTabletServersResponsePB_Entry {
    fn eq(&self, other: &ListTabletServersResponsePB_Entry) -> bool {
        self.instance_id == other.instance_id &&
        self.registration == other.registration &&
        self.millis_since_heartbeat == other.millis_since_heartbeat &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListTabletServersResponsePB_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetMasterRegistrationRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMasterRegistrationRequestPB {}

impl GetMasterRegistrationRequestPB {
    pub fn new() -> GetMasterRegistrationRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMasterRegistrationRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<GetMasterRegistrationRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMasterRegistrationRequestPB,
        };
        unsafe {
            instance.get(|| {
                GetMasterRegistrationRequestPB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetMasterRegistrationRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetMasterRegistrationRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMasterRegistrationRequestPB {
    fn new() -> GetMasterRegistrationRequestPB {
        GetMasterRegistrationRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMasterRegistrationRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetMasterRegistrationRequestPB>(
                    "GetMasterRegistrationRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMasterRegistrationRequestPB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetMasterRegistrationRequestPB {
    fn eq(&self, other: &GetMasterRegistrationRequestPB) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetMasterRegistrationRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetMasterRegistrationResponsePB {
    // message fields
    instance_id: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    registration: ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB>,
    role: ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role>,
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMasterRegistrationResponsePB {}

impl GetMasterRegistrationResponsePB {
    pub fn new() -> GetMasterRegistrationResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMasterRegistrationResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<GetMasterRegistrationResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMasterRegistrationResponsePB,
        };
        unsafe {
            instance.get(|| {
                GetMasterRegistrationResponsePB {
                    instance_id: ::protobuf::SingularPtrField::none(),
                    registration: ::protobuf::SingularPtrField::none(),
                    role: ::std::option::Option::None,
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .kudu.NodeInstancePB instance_id = 1;

    pub fn clear_instance_id(&mut self) {
        self.instance_id.clear();
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: super::wire_protocol::NodeInstancePB) {
        self.instance_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_id(&mut self) -> &mut super::wire_protocol::NodeInstancePB {
        if self.instance_id.is_none() {
            self.instance_id.set_default();
        };
        self.instance_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_instance_id(&mut self) -> super::wire_protocol::NodeInstancePB {
        self.instance_id.take().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::new())
    }

    pub fn get_instance_id(&self) -> &super::wire_protocol::NodeInstancePB {
        self.instance_id.as_ref().unwrap_or_else(|| super::wire_protocol::NodeInstancePB::default_instance())
    }

    // optional .kudu.ServerRegistrationPB registration = 2;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::wire_protocol::ServerRegistrationPB) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::wire_protocol::ServerRegistrationPB {
        if self.registration.is_none() {
            self.registration.set_default();
        };
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::wire_protocol::ServerRegistrationPB {
        self.registration.take().unwrap_or_else(|| super::wire_protocol::ServerRegistrationPB::new())
    }

    pub fn get_registration(&self) -> &super::wire_protocol::ServerRegistrationPB {
        self.registration.as_ref().unwrap_or_else(|| super::wire_protocol::ServerRegistrationPB::default_instance())
    }

    // optional .kudu.consensus.RaftPeerPB.Role role = 3;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: super::consensus_metadata::RaftPeerPB_Role) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role(&self) -> super::consensus_metadata::RaftPeerPB_Role {
        self.role.unwrap_or(super::consensus_metadata::RaftPeerPB_Role::UNKNOWN_ROLE)
    }

    // optional .kudu.master.MasterErrorPB error = 4;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: MasterErrorPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut MasterErrorPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> MasterErrorPB {
        self.error.take().unwrap_or_else(|| MasterErrorPB::new())
    }

    pub fn get_error(&self) -> &MasterErrorPB {
        self.error.as_ref().unwrap_or_else(|| MasterErrorPB::default_instance())
    }
}

impl ::protobuf::Message for GetMasterRegistrationResponsePB {
    fn is_initialized(&self) -> bool {
        if self.instance_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instance_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.role = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.instance_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.registration.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.role.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        if let Some(v) = self.instance_id.as_ref() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.registration.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.role {
            try!(w.write_enum(3, v.value()));
        };
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetMasterRegistrationResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMasterRegistrationResponsePB {
    fn new() -> GetMasterRegistrationResponsePB {
        GetMasterRegistrationResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMasterRegistrationResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "instance_id",
                    GetMasterRegistrationResponsePB::has_instance_id,
                    GetMasterRegistrationResponsePB::get_instance_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "registration",
                    GetMasterRegistrationResponsePB::has_registration,
                    GetMasterRegistrationResponsePB::get_registration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "role",
                    GetMasterRegistrationResponsePB::has_role,
                    GetMasterRegistrationResponsePB::get_role,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    GetMasterRegistrationResponsePB::has_error,
                    GetMasterRegistrationResponsePB::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMasterRegistrationResponsePB>(
                    "GetMasterRegistrationResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMasterRegistrationResponsePB {
    fn clear(&mut self) {
        self.clear_instance_id();
        self.clear_registration();
        self.clear_role();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetMasterRegistrationResponsePB {
    fn eq(&self, other: &GetMasterRegistrationResponsePB) -> bool {
        self.instance_id == other.instance_id &&
        self.registration == other.registration &&
        self.role == other.role &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetMasterRegistrationResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListMastersRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListMastersRequestPB {}

impl ListMastersRequestPB {
    pub fn new() -> ListMastersRequestPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListMastersRequestPB {
        static mut instance: ::protobuf::lazy::Lazy<ListMastersRequestPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListMastersRequestPB,
        };
        unsafe {
            instance.get(|| {
                ListMastersRequestPB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ListMastersRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListMastersRequestPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListMastersRequestPB {
    fn new() -> ListMastersRequestPB {
        ListMastersRequestPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListMastersRequestPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListMastersRequestPB>(
                    "ListMastersRequestPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListMastersRequestPB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListMastersRequestPB {
    fn eq(&self, other: &ListMastersRequestPB) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListMastersRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListMastersResponsePB {
    // message fields
    masters: ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB>,
    error: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListMastersResponsePB {}

impl ListMastersResponsePB {
    pub fn new() -> ListMastersResponsePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListMastersResponsePB {
        static mut instance: ::protobuf::lazy::Lazy<ListMastersResponsePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListMastersResponsePB,
        };
        unsafe {
            instance.get(|| {
                ListMastersResponsePB {
                    masters: ::protobuf::RepeatedField::new(),
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kudu.ServerEntryPB masters = 1;

    pub fn clear_masters(&mut self) {
        self.masters.clear();
    }

    // Param is passed by value, moved
    pub fn set_masters(&mut self, v: ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB>) {
        self.masters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_masters(&mut self) -> &mut ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB> {
        &mut self.masters
    }

    // Take field
    pub fn take_masters(&mut self) -> ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB> {
        ::std::mem::replace(&mut self.masters, ::protobuf::RepeatedField::new())
    }

    pub fn get_masters(&self) -> &[super::wire_protocol::ServerEntryPB] {
        &self.masters
    }

    // optional .kudu.AppStatusPB error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::wire_protocol::AppStatusPB) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut super::wire_protocol::AppStatusPB {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> super::wire_protocol::AppStatusPB {
        self.error.take().unwrap_or_else(|| super::wire_protocol::AppStatusPB::new())
    }

    pub fn get_error(&self) -> &super::wire_protocol::AppStatusPB {
        self.error.as_ref().unwrap_or_else(|| super::wire_protocol::AppStatusPB::default_instance())
    }
}

impl ::protobuf::Message for ListMastersResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.masters));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.masters.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ::protobuf::ProtobufResult<()> where W: ::std::io::Write {
        for v in self.masters.iter() {
            try!(w.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        if let Some(v) = self.error.as_ref() {
            try!(w.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(w.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(w));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ListMastersResponsePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListMastersResponsePB {
    fn new() -> ListMastersResponsePB {
        ListMastersResponsePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListMastersResponsePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "masters",
                    ListMastersResponsePB::get_masters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ListMastersResponsePB::has_error,
                    ListMastersResponsePB::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListMastersResponsePB>(
                    "ListMastersResponsePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListMastersResponsePB {
    fn clear(&mut self) {
        self.clear_masters();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListMastersResponsePB {
    fn eq(&self, other: &ListMastersResponsePB) -> bool {
        self.masters == other.masters &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListMastersResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x1a, 0x18, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1f, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x77,
    0x69, 0x72, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1d, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73,
    0x75, 0x73, 0x2f, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1a, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2f, 0x6d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb1, 0x02,
    0x0a, 0x0d, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12,
    0x2d, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1f, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x21,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50,
    0x42, 0x22, 0xcd, 0x01, 0x0a, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x01, 0x12, 0x12, 0x0a,
    0x0e, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x10,
    0x02, 0x12, 0x13, 0x0a, 0x0f, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46,
    0x4f, 0x55, 0x4e, 0x44, 0x10, 0x03, 0x12, 0x19, 0x0a, 0x15, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x5f,
    0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x50, 0x52, 0x45, 0x53, 0x45, 0x4e, 0x54, 0x10,
    0x04, 0x12, 0x14, 0x0a, 0x10, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x54, 0x41,
    0x42, 0x4c, 0x45, 0x54, 0x53, 0x10, 0x05, 0x12, 0x23, 0x0a, 0x1f, 0x43, 0x41, 0x54, 0x41, 0x4c,
    0x4f, 0x47, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49,
    0x4e, 0x49, 0x54, 0x49, 0x41, 0x4c, 0x49, 0x5a, 0x45, 0x44, 0x10, 0x06, 0x12, 0x12, 0x0a, 0x0e,
    0x4e, 0x4f, 0x54, 0x5f, 0x54, 0x48, 0x45, 0x5f, 0x4c, 0x45, 0x41, 0x44, 0x45, 0x52, 0x10, 0x07,
    0x12, 0x1f, 0x0a, 0x1b, 0x52, 0x45, 0x50, 0x4c, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
    0x46, 0x41, 0x43, 0x54, 0x4f, 0x52, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x48, 0x49, 0x47, 0x48, 0x10,
    0x08, 0x22, 0x3f, 0x0a, 0x12, 0x54, 0x53, 0x54, 0x6f, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x43,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x0b, 0x74, 0x73, 0x5f, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x50, 0x42, 0x22, 0x39, 0x0a, 0x11, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0xf7, 0x02,
    0x0a, 0x11, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x50, 0x42, 0x12, 0x1c, 0x0a, 0x14, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45,
    0x44, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x1a, 0x0a, 0x12, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x5f,
    0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x24, 0x0a,
    0x09, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x50, 0x42, 0x12, 0x43, 0x0a, 0x19, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64,
    0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75,
    0x73, 0x53, 0x74, 0x61, 0x74, 0x65, 0x50, 0x42, 0x12, 0x3c, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x24, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x07, 0x55,
    0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5f,
    0x6d, 0x73, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x5a, 0x0a, 0x05, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x12, 0x0c, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10,
    0xe7, 0x07, 0x12, 0x0d, 0x0a, 0x09, 0x50, 0x52, 0x45, 0x50, 0x41, 0x52, 0x49, 0x4e, 0x47, 0x10,
    0x00, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x52, 0x45, 0x41, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12,
    0x0b, 0x0a, 0x07, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08,
    0x52, 0x45, 0x50, 0x4c, 0x41, 0x43, 0x45, 0x44, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45,
    0x4c, 0x45, 0x54, 0x45, 0x44, 0x10, 0x04, 0x22, 0xfd, 0x02, 0x0a, 0x10, 0x53, 0x79, 0x73, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x12, 0x0c, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x1e, 0x0a, 0x06, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x2c, 0x0a, 0x14, 0x66,
    0x75, 0x6c, 0x6c, 0x79, 0x5f, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x31, 0x0a, 0x10, 0x70, 0x61, 0x72,
    0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e,
    0x6e, 0x65, 0x78, 0x74, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x05, 0x12, 0x3b, 0x0a, 0x05, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x07,
    0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x5f, 0x6d, 0x73, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4b, 0x0a, 0x05, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00,
    0x12, 0x0d, 0x0a, 0x09, 0x50, 0x52, 0x45, 0x50, 0x41, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12,
    0x0b, 0x0a, 0x07, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08,
    0x41, 0x4c, 0x54, 0x45, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45,
    0x4d, 0x4f, 0x56, 0x45, 0x44, 0x10, 0x04, 0x22, 0x0f, 0x0a, 0x0d, 0x50, 0x69, 0x6e, 0x67, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0x10, 0x0a, 0x0e, 0x50, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x22, 0x65, 0x0a, 0x10, 0x54, 0x53,
    0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x27,
    0x0a, 0x0d, 0x72, 0x70, 0x63, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x48, 0x6f, 0x73,
    0x74, 0x50, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x12, 0x28, 0x0a, 0x0e, 0x68, 0x74, 0x74, 0x70, 0x5f,
    0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x6f, 0x72, 0x74, 0x50,
    0x42, 0x22, 0xa6, 0x02, 0x0a, 0x10, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x50, 0x42, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x32, 0x0a, 0x05, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x50, 0x42, 0x3a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x4c, 0x0a,
    0x11, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x44, 0x61, 0x74,
    0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x13, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x54, 0x5f, 0x44,
    0x41, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x43, 0x0a, 0x19, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73,
    0x75, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e,
    0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x53, 0x74, 0x61, 0x74, 0x65, 0x50, 0x42,
    0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x95, 0x01, 0x0a, 0x0e, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x12, 0x16, 0x0a,
    0x0e, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x36, 0x0a, 0x0f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x50, 0x42, 0x12, 0x1a, 0x0a,
    0x12, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f,
    0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x65, 0x71,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x04, 0x20, 0x02,
    0x28, 0x05, 0x22, 0x3f, 0x0a, 0x17, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x50, 0x42, 0x12, 0x11, 0x0a,
    0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x22, 0x4e, 0x0a, 0x15, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x50, 0x42, 0x12, 0x35, 0x0a, 0x07,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x50, 0x42, 0x22, 0xca, 0x01, 0x0a, 0x14, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2f, 0x0a, 0x06,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x54, 0x6f, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x33, 0x0a,
    0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x54, 0x53, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x50, 0x42, 0x12, 0x32, 0x0a, 0x0d, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x12, 0x18, 0x0a, 0x10, 0x6e, 0x75, 0x6d, 0x5f, 0x6c, 0x69,
    0x76, 0x65, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05,
    0x22, 0x8d, 0x02, 0x0a, 0x15, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x0f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x5f,
    0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e,
    0x63, 0x65, 0x50, 0x42, 0x12, 0x1f, 0x0a, 0x10, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x72, 0x65,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05,
    0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x18, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x66,
    0x75, 0x6c, 0x6c, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x39,
    0x0a, 0x0d, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x50, 0x42, 0x12, 0x15, 0x0a, 0x0d, 0x6c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x5f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08,
    0x22, 0x9f, 0x02, 0x0a, 0x11, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07,
    0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x24, 0x0a,
    0x09, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x50, 0x42, 0x12, 0x3a, 0x0a, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x50, 0x42, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x50, 0x42, 0x12,
    0x0d, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x1a, 0x62,
    0x0a, 0x09, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x50, 0x42, 0x12, 0x26, 0x0a, 0x07, 0x74,
    0x73, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x49, 0x6e, 0x66,
    0x6f, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x1f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73,
    0x75, 0x73, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x50, 0x65, 0x65, 0x72, 0x50, 0x42, 0x2e, 0x52, 0x6f,
    0x6c, 0x65, 0x22, 0x4b, 0x0a, 0x08, 0x54, 0x53, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x42, 0x12, 0x16,
    0x0a, 0x0e, 0x70, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x5f, 0x75, 0x75, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x27, 0x0a, 0x0d, 0x72, 0x70, 0x63, 0x5f, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x22,
    0x31, 0x0a, 0x1b, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x12,
    0x0a, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0c, 0x22, 0x83, 0x02, 0x0a, 0x1c, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x38,
    0x0a, 0x10, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x3f, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x1a, 0x3d, 0x0a, 0x05, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x21, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x22, 0xb8, 0x01, 0x0a, 0x14, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x42, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x1e, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12,
    0x29, 0x0a, 0x0a, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x52, 0x6f, 0x77, 0x4f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x31, 0x0a, 0x10, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x14, 0x0a,
    0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x05, 0x22, 0x54, 0x0a, 0x15, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4b, 0x0a, 0x1a, 0x49, 0x73, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x22, 0x56, 0x0a, 0x1b, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42,
    0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x45,
    0x0a, 0x14, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x50, 0x42, 0x22, 0x42, 0x0a, 0x15, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x22, 0x2a, 0x0a, 0x13, 0x4c, 0x69, 0x73,
    0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0xa5, 0x01, 0x0a, 0x14, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x3b, 0x0a, 0x06, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x2e, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x1a, 0x25, 0x0a, 0x09, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0xa7, 0x01,
    0x0a, 0x1a, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x12, 0x1b, 0x0a, 0x13, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0c, 0x12, 0x22, 0x0a, 0x16, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0d, 0x3a, 0x02, 0x31, 0x30, 0x22, 0x82, 0x01, 0x0a, 0x1b, 0x47, 0x65, 0x74, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x50, 0x42, 0x12, 0x38, 0x0a, 0x10, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x22, 0x94, 0x05, 0x0a,
    0x13, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x50, 0x42, 0x12, 0x41, 0x0a, 0x12, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x25, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c,
    0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x42, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x12, 0x16, 0x0a, 0x0e, 0x6e, 0x65, 0x77, 0x5f, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x1a, 0x31,
    0x0a, 0x09, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x24, 0x0a, 0x06, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50,
    0x42, 0x1a, 0x1a, 0x0a, 0x0a, 0x44, 0x72, 0x6f, 0x70, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x1a, 0x32, 0x0a,
    0x0c, 0x52, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x10, 0x0a,
    0x08, 0x6f, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x10, 0x0a, 0x08, 0x6e, 0x65, 0x77, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x09, 0x1a, 0x90, 0x02, 0x0a, 0x04, 0x53, 0x74, 0x65, 0x70, 0x12, 0x40, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x29, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x54,
    0x79, 0x70, 0x65, 0x3a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x3e, 0x0a, 0x0a,
    0x61, 0x64, 0x64, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41,
    0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x42, 0x2e, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x40, 0x0a, 0x0b,
    0x64, 0x72, 0x6f, 0x70, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x2b, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x2e, 0x44, 0x72, 0x6f, 0x70, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x44,
    0x0a, 0x0d, 0x72, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x52, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x43, 0x6f,
    0x6c, 0x75, 0x6d, 0x6e, 0x22, 0x5d, 0x0a, 0x08, 0x53, 0x74, 0x65, 0x70, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0e, 0x0a,
    0x0a, 0x41, 0x44, 0x44, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10, 0x01, 0x12, 0x0f, 0x0a,
    0x0b, 0x44, 0x52, 0x4f, 0x50, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10, 0x02, 0x12, 0x11,
    0x0a, 0x0d, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10,
    0x03, 0x12, 0x10, 0x0a, 0x0c, 0x41, 0x4c, 0x54, 0x45, 0x52, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d,
    0x4e, 0x10, 0x04, 0x22, 0x59, 0x0a, 0x14, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x4a,
    0x0a, 0x19, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f,
    0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x22, 0x6d, 0x0a, 0x1a, 0x49, 0x73,
    0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x64,
    0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x48, 0x0a, 0x17, 0x47, 0x65, 0x74,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x50, 0x42, 0x22, 0xdb, 0x01, 0x0a, 0x18, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42,
    0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x1e, 0x0a, 0x06, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x31, 0x0a, 0x10, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72,
    0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x14,
    0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x22, 0x1c, 0x0a, 0x1a, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22,
    0x93, 0x02, 0x0a, 0x1b, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12,
    0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x3f, 0x0a, 0x07, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x2e, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x1a, 0x87, 0x01, 0x0a, 0x05,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x29, 0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x42,
    0x12, 0x33, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x1e, 0x0a, 0x16, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x5f,
    0x73, 0x69, 0x6e, 0x63, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x20, 0x0a, 0x1e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0xd8, 0x01, 0x0a, 0x1f, 0x47, 0x65, 0x74, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x0b, 0x69,
    0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x73, 0x74,
    0x61, 0x6e, 0x63, 0x65, 0x50, 0x42, 0x12, 0x30, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x50, 0x65, 0x65, 0x72,
    0x50, 0x42, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x50, 0x42, 0x22, 0x16, 0x0a, 0x14, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0x5f, 0x0a, 0x15, 0x4c, 0x69,
    0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x12, 0x24, 0x0a, 0x07, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x41, 0x70, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x32, 0xa9, 0x0a, 0x0a, 0x0d,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x3f, 0x0a,
    0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x42, 0x1a, 0x1b, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54,
    0x0a, 0x0b, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x21, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x48, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54,
    0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x50, 0x42, 0x12, 0x69, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x28, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x50, 0x42, 0x1a, 0x29, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12,
    0x54, 0x0a, 0x0b, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x21,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x42, 0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66, 0x0a, 0x11, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x12, 0x27, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44,
    0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a,
    0x0b, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x21, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a,
    0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x12, 0x51, 0x0a, 0x0a, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x12, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x1a, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x63, 0x0a, 0x10, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65,
    0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x12, 0x26, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x42, 0x1a, 0x27, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x51, 0x0a, 0x0a, 0x4c,
    0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x21, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66,
    0x0a, 0x11, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x12, 0x27, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x5d, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x12, 0x24, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x25,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66, 0x0a, 0x11, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x12, 0x27, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a,
    0x0b, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x12, 0x21, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a,
    0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69,
    0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x12, 0x72, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x2c, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x42, 0x13, 0x0a, 0x11, 0x6f, 0x72, 0x67, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x64, 0x62, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x4a, 0xa0, 0xa2, 0x01,
    0x0a, 0x07, 0x12, 0x05, 0x10, 0x00, 0xc0, 0x04, 0x01, 0x0a, 0x8c, 0x06, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x10, 0x08, 0x13, 0x1a, 0x81, 0x06, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x53,
    0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x28, 0x41, 0x53, 0x46, 0x29, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x6f,
    0x6e, 0x65, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20,
    0x61, 0x67, 0x72, 0x65, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x20, 0x53, 0x65, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x4e, 0x4f, 0x54, 0x49, 0x43, 0x45, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x72, 0x65, 0x67, 0x61, 0x72, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x41, 0x53, 0x46,
    0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32, 0x2e,
    0x30, 0x20, 0x28, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x22, 0x29, 0x3b, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x78,
    0x63, 0x65, 0x70, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61, 0x6e,
    0x63, 0x65, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f,
    0x62, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a,
    0x20, 0x20, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x61, 0x70,
    0x61, 0x63, 0x68, 0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x73, 0x2f, 0x4c, 0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x20,
    0x55, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x61,
    0x77, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x0a, 0x20, 0x73, 0x6f, 0x66, 0x74,
    0x77, 0x61, 0x72, 0x65, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64,
    0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e,
    0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53, 0x22,
    0x20, 0x42, 0x41, 0x53, 0x49, 0x53, 0x2c, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f, 0x55, 0x54, 0x20,
    0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f,
    0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41, 0x4e, 0x59, 0x0a,
    0x20, 0x4b, 0x49, 0x4e, 0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x64,
    0x2e, 0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e,
    0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x67, 0x6f,
    0x76, 0x65, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x12,
    0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x12, 0x00, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x12, 0x16, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x14, 0x07, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x07, 0x28,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x16, 0x07, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x17, 0x07, 0x23, 0x0a, 0x37, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1e, 0x00,
    0x47, 0x01, 0x1a, 0x2b, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x63, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x02, 0x3e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x07, 0x0b, 0x0a, 0xdb, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x25, 0x04, 0x16, 0x1a, 0xcb, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x68, 0x61, 0x73, 0x20, 0x6e, 0x6f,
    0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x64, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x27, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x27, 0x20, 0x6d, 0x61, 0x79,
    0x20, 0x72, 0x65, 0x76, 0x65, 0x61, 0x6c, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x52, 0x50, 0x43, 0x73, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x2c, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20,
    0x63, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x62, 0x65, 0x0a, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x61, 0x73,
    0x69, 0x6c, 0x79, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x25, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x25, 0x14, 0x15, 0x0a, 0x47, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x28, 0x04, 0x17, 0x1a, 0x38, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2d, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x04, 0x12, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x28, 0x15, 0x16, 0x0a, 0x33,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x04, 0x18, 0x1a, 0x24, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78, 0x69,
    0x73, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2b, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x2b, 0x16, 0x17, 0x0a, 0x43, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2e,
    0x04, 0x1e, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79,
    0x20, 0x69, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x5a, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x31, 0x04, 0x19, 0x1a, 0x4b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20,
    0x6e, 0x65, 0x77, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x76, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x20, 0x54, 0x53, 0x20, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x31, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x31, 0x17, 0x18, 0x0a, 0x38, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x34, 0x04, 0x28, 0x1a, 0x29, 0x20, 0x43, 0x61, 0x74, 0x61, 0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x61,
    0x6e, 0x61, 0x67, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79, 0x65, 0x74,
    0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x34, 0x04, 0x23, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x34, 0x26, 0x27, 0x0a, 0x94,
    0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x39, 0x04, 0x17, 0x1a, 0x84,
    0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x6f, 0x6e,
    0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x64, 0x20, 0x61, 0x67,
    0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x73, 0x69,
    0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x20, 0x69, 0x73,
    0x6e, 0x27, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x39, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x02,
    0x12, 0x03, 0x39, 0x15, 0x16, 0x0a, 0x6d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x3d, 0x04, 0x24, 0x1a, 0x5e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x69, 0x73, 0x20, 0x67, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x69, 0x76, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x73, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x3d, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x3d, 0x22, 0x23, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x41, 0x02,
    0x19, 0x1a, 0x11, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f,
    0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x10, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x17, 0x18, 0x0a, 0xb7, 0x01, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x46, 0x02, 0x22, 0x1a, 0xa9, 0x01, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65,
    0x20, 0x61, 0x20, 0x74, 0x65, 0x78, 0x74, 0x75, 0x61, 0x6c, 0x0a, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20,
    0x6d, 0x6f, 0x72, 0x65, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2c, 0x0a, 0x20, 0x74, 0x68,
    0x6f, 0x75, 0x67, 0x68, 0x20, 0x69, 0x74, 0x73, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63,
    0x6f, 0x64, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x63, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x46,
    0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x17, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x20, 0x21, 0x0a, 0x5f,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x4b, 0x00, 0x4e, 0x01, 0x1a, 0x53, 0x20, 0x43, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x73, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x1a, 0x0a, 0x47, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x02, 0x2a, 0x1a, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69,
    0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65,
    0x61, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4d, 0x0b, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x1a, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x50, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x50, 0x08, 0x19, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x52, 0x02,
    0x1e, 0x1a, 0x1d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x49, 0x44,
    0x20, 0x74, 0x6f, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x1c, 0x1d, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x55, 0x02, 0x21, 0x1a, 0x1f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20,
    0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x55,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x55, 0x1f, 0x20, 0x0a, 0x62,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x5e, 0x00, 0x78, 0x01, 0x1a, 0x56, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6f, 0x6e, 0x2d, 0x64, 0x69, 0x73, 0x6b, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x2e, 0x63, 0x61, 0x74, 0x61, 0x6c, 0x6f,
    0x67, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x28, 0x22, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x22, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x29, 0x20, 0x66, 0x6f, 0x72, 0x0a,
    0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x5f, 0x02, 0x66, 0x03, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x07, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x60, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x60, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x61, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x61, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x62, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x62, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x63, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x63, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x63, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x64, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x64, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x65, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x65, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x65, 0x0e, 0x0f, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x69, 0x02,
    0x2a, 0x1a, 0x26, 0x20, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x20,
    0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x27, 0x70, 0x61, 0x72,
    0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x27, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x69, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x69, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x28,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x02, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x6a, 0x28, 0x29, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x6d, 0x02, 0x25, 0x1a, 0x13, 0x20, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x70, 0x61, 0x72,
    0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x6d, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6d,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6d, 0x23, 0x24,
    0x0a, 0x6b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x70, 0x02, 0x44, 0x1a, 0x5e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x70, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x70, 0x26, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x70, 0x42, 0x43, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x73,
    0x02, 0x31, 0x1a, 0x1d, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x73, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x73, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x73, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x08, 0x12, 0x03, 0x73, 0x1b, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x07,
    0x12, 0x03, 0x73, 0x27, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x74,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x74, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x74, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x74, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x74, 0x1d, 0x1e, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x06, 0x12, 0x03, 0x77, 0x02, 0x1e, 0x1a, 0x1e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x69, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x77, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x77, 0x11,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x77, 0x1c, 0x1d, 0x0a,
    0x62, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x05, 0x7c, 0x00, 0xa2, 0x01, 0x01, 0x1a, 0x55, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6f, 0x6e, 0x2d, 0x64, 0x69, 0x73, 0x6b, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x2e, 0x63, 0x61, 0x74, 0x61,
    0x6c, 0x6f, 0x67, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x28, 0x22, 0x6d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x22, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x29, 0x20, 0x66, 0x6f,
    0x72, 0x0a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x18, 0x0a,
    0x0d, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x05, 0x7d, 0x02, 0x83, 0x01, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x07, 0x0c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7e, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x7e, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7f, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x7f, 0x10, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x80, 0x01, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x80, 0x01, 0x0e, 0x0f, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x04, 0x11, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x81, 0x01, 0x0f, 0x10, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x10, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x82, 0x01, 0x0e, 0x0f,
    0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x02, 0x1a, 0x1a, 0x0c,
    0x20, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x18, 0x19, 0x0a, 0x78, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01,
    0x12, 0x04, 0x8a, 0x01, 0x02, 0x1e, 0x1a, 0x6a, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x2d, 0x69, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x20, 0x55, 0x73,
    0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2d, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x73, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x22, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x2d, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x22, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x12, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x1c, 0x1d, 0x0a, 0x47,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x02, 0x1f, 0x1a, 0x39, 0x20, 0x4e,
    0x65, 0x77, 0x65, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x20, 0x28, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x54, 0x53, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x68, 0x61, 0x76,
    0x65, 0x20, 0x69, 0x74, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x8d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12,
    0x04, 0x8d, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x8d, 0x01, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8d,
    0x01, 0x1d, 0x1e, 0x0a, 0xc5, 0x01, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x04, 0x93, 0x01,
    0x02, 0x2d, 0x1a, 0xb6, 0x01, 0x20, 0x4c, 0x61, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61,
    0x76, 0x65, 0x20, 0x72, 0x65, 0x61, 0x63, 0x68, 0x65, 0x64, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79,
    0x20, 0x54, 0x53, 0x2c, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68, 0x0a, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x6e, 0x65, 0x63, 0x65, 0x73, 0x73, 0x61, 0x72, 0x69, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x65, 0x77, 0x65, 0x73, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a,
    0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63,
    0x68, 0x65, 0x6d, 0x61, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x2d, 0x3e, 0x47, 0x65, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x28, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x04, 0x93, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x93, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x93, 0x01, 0x14, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x93, 0x01, 0x2b, 0x2c, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12,
    0x04, 0x96, 0x01, 0x02, 0x32, 0x1a, 0x22, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x27, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x96, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x96, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x96, 0x01, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x96, 0x01, 0x30, 0x31, 0x0a, 0x72, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x04, 0x9a,
    0x01, 0x02, 0x24, 0x1a, 0x64, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x63,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x73, 0x73, 0x69,
    0x67, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x65, 0x77, 0x6c, 0x79, 0x20, 0x61, 0x64, 0x64, 0x65,
    0x64, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70,
    0x72, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x49,
    0x44, 0x20, 0x72, 0x65, 0x75, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05,
    0x05, 0x12, 0x04, 0x9a, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x9a, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12,
    0x04, 0x9a, 0x01, 0x22, 0x23, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x04, 0x9d,
    0x01, 0x02, 0x22, 0x1a, 0x17, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x54, 0x53, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x06, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x06, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x20, 0x21, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07,
    0x12, 0x04, 0xa0, 0x01, 0x02, 0x31, 0x1a, 0x1c, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x04, 0xa0,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x06, 0x12, 0x04, 0xa0, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x19, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x08, 0x12, 0x04, 0xa0, 0x01, 0x1b, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x07, 0x12, 0x04, 0xa0, 0x01, 0x27, 0x2e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x08, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06,
    0xa8, 0x01, 0x00, 0xa9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0xa8,
    0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x06, 0xab, 0x01, 0x00, 0xac, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x16, 0x0a, 0xa6,
    0x01, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0xb1, 0x01, 0x00, 0xb6, 0x01, 0x01, 0x1a, 0x97, 0x01,
    0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x53, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x68, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e,
    0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x63, 0x65, 0x73, 0x73,
    0x61, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x54, 0x53, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04,
    0xb1, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x01,
    0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x0b, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x16, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06,
    0xb8, 0x01, 0x00, 0xc4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0xb8,
    0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x01, 0x02,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb9, 0x01, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x11, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x02, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0xba, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xba, 0x01, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xba, 0x01, 0x20, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xba, 0x01, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x08,
    0x12, 0x04, 0xba, 0x01, 0x2a, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x07, 0x12,
    0x04, 0xba, 0x01, 0x36, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0xbb,
    0x01, 0x02, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x0b,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x22, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x36, 0x37, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x08, 0x12, 0x04, 0xbb, 0x01, 0x38, 0x59, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x07, 0x12, 0x04, 0xbb, 0x01, 0x44, 0x57, 0x0a, 0x96, 0x01,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x44, 0x1a, 0x87, 0x01, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x5f, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x74, 0x65, 0x64, 0x5f, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x0a, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x69, 0x66, 0x20,
    0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x42, 0x4f, 0x4f, 0x54, 0x53, 0x54, 0x52, 0x41, 0x50, 0x50,
    0x49, 0x4e, 0x47, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xc0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x04,
    0xc0, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc0,
    0x01, 0x26, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc0, 0x01,
    0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x04, 0xc2, 0x01, 0x0b, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xc3, 0x01, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xc3, 0x01, 0x23, 0x24, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0xc7, 0x01,
    0x00, 0xdc, 0x01, 0x01, 0x1a, 0x4b, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x68, 0x6f,
    0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x54, 0x53, 0x2e,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x16, 0x0a, 0x87,
    0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0xca, 0x01, 0x02, 0x23, 0x1a, 0x79, 0x20,
    0x49, 0x66, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x20, 0x72, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x70, 0x72,
    0x69, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x62, 0x6f, 0x75, 0x74, 0x0a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x68,
    0x6f, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64,
    0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xca, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xca, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xca, 0x01, 0x10, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xca, 0x01, 0x21, 0x22, 0x0a, 0x94, 0x02, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0xd0,
    0x01, 0x02, 0x30, 0x1a, 0x85, 0x02, 0x20, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x27, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x6c, 0x27, 0x20, 0x69, 0x73, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x75, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x73, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x77, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x62, 0x75, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x62, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75,
    0x6d, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x62, 0x65, 0x65, 0x6e,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x1c, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xd0, 0x01, 0x2e, 0x2f, 0x0a, 0xa8, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02,
    0x12, 0x04, 0xd5, 0x01, 0x02, 0x28, 0x1a, 0x99, 0x01, 0x20, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x20, 0x49, 0x44, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x65, 0x72, 0x20, 0x62, 0x65,
    0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d,
    0x70, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x69, 0x6e, 0x63,
    0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x0a, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x11, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x26, 0x27, 0x0a, 0x8c,
    0x02, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x02, 0x25, 0x1a, 0xfd, 0x01,
    0x20, 0x45, 0x76, 0x65, 0x72, 0x79, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x54, 0x53, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65,
    0x6e, 0x63, 0x65, 0x0a, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20,
    0x69, 0x6e, 0x20, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x69,
    0x6e, 0x67, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x73, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79, 0x65, 0x74, 0x20, 0x62,
    0x65, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x20, 0x28, 0x6e, 0x6f, 0x6e, 0x2d, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x61, 0x6c, 0x29, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x30, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x11, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12,
    0x06, 0xde, 0x01, 0x00, 0xe1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04,
    0xde, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x01,
    0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdf, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdf, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x11, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x1d, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x1e, 0x1f, 0x0a, 0x5f, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06,
    0xe4, 0x01, 0x00, 0xe6, 0x01, 0x01, 0x1a, 0x51, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54,
    0x53, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20,
    0x28, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61,
    0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x73, 0x29, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01,
    0x12, 0x04, 0xe4, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04,
    0xe5, 0x01, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe5,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe5, 0x01,
    0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x23,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x2d, 0x2e,
    0x0a, 0x7e, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xea, 0x01, 0x00, 0xfd, 0x01, 0x01, 0x1a, 0x70,
    0x20, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2d,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69,
    0x73, 0x68, 0x20, 0x6c, 0x69, 0x76, 0x65, 0x6e, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x79, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xea, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xeb, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x1e, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xeb, 0x01, 0x27, 0x28, 0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12,
    0x04, 0xef, 0x01, 0x02, 0x2d, 0x1a, 0x5e, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x75, 0x70, 0x6f,
    0x6e, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x2d, 0x75, 0x70, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x54, 0x53, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x27, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x72,
    0x65, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x27, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20,
    0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xef, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0xef,
    0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xef, 0x01,
    0x1c, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xef, 0x01, 0x2b,
    0x2c, 0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0xf3, 0x01, 0x02, 0x2c, 0x1a,
    0x5e, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x2c, 0x20,
    0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74,
    0x6f, 0x0a, 0x20, 0x27, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x66, 0x75, 0x6c, 0x6c, 0x5f, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x27, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x0b, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x1a, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x2a, 0x2b, 0x0a, 0x92, 0x01, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x26, 0x1a, 0x83, 0x01, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x42, 0x4f, 0x4f,
    0x54, 0x53, 0x54, 0x52, 0x41, 0x50, 0x50, 0x49, 0x4e, 0x47, 0x20, 0x6f, 0x72, 0x20, 0x52, 0x55,
    0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x2e, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65,
    0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x11, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x06, 0xff, 0x01, 0x00, 0x94, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0xff, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x04, 0x80, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x80, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x80,
    0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x02,
    0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x02, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x86, 0x02, 0x02, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x86, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0x86, 0x02, 0x0b, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x86, 0x02, 0x1a, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x86, 0x02, 0x2c, 0x2d, 0x0a, 0x9e, 0x01, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x39, 0x1a, 0x8f, 0x01, 0x20, 0x49, 0x6e,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x68, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x2d, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x0a, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20,
    0x2d, 0x2d, 0x20, 0x69, 0x2e, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x68, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x27, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x27, 0x0a,
    0x20, 0x66, 0x69, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x10, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x08, 0x12, 0x04, 0x8b, 0x02, 0x25, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x07,
    0x12, 0x04, 0x8b, 0x02, 0x31, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04,
    0x8d, 0x02, 0x02, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8d,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8d, 0x02,
    0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x10,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x2b, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x08, 0x12, 0x04, 0x8d, 0x02, 0x2d, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x07, 0x12, 0x04, 0x8d, 0x02, 0x39, 0x3e, 0x0a, 0x3c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x04, 0x90, 0x02, 0x02, 0x33, 0x1a, 0x2e, 0x20, 0x53,
    0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x73, 0x20, 0x61, 0x20, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x04, 0x90, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x04, 0x06, 0x12, 0x04, 0x90, 0x02, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x90, 0x02, 0x21, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x90, 0x02, 0x31, 0x32, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05,
    0x12, 0x04, 0x93, 0x02, 0x02, 0x22, 0x1a, 0x37, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79,
    0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x04, 0x93, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x04, 0x93, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0x93, 0x02, 0x10, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0x93, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x06, 0x9a, 0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01,
    0x12, 0x04, 0x9a, 0x02, 0x08, 0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0e, 0x03, 0x00, 0x12, 0x06,
    0x9b, 0x02, 0x02, 0x9e, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x03, 0x00, 0x01, 0x12,
    0x04, 0x9b, 0x02, 0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0x9c, 0x02, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x9c, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x03, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x9c, 0x02, 0x0d, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x16, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e,
    0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9d, 0x02, 0x0d, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0e, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x27, 0x2b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0e, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa0, 0x02, 0x1d, 0x1e, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12,
    0x04, 0xa3, 0x02, 0x02, 0x1f, 0x1a, 0x0d, 0x20, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54,
    0x45, 0x44, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x02,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x11,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x02, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x02, 0x02, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xa6, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xa6, 0x02, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xa6, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x02,
    0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x06, 0x12, 0x04, 0xa8, 0x02, 0x0b, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x15, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x20, 0x21, 0x0a, 0xb5,
    0x02, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x04, 0xae, 0x02, 0x02, 0x1a, 0x1a, 0xa6, 0x02,
    0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x20,
    0x62, 0x75, 0x74, 0x20, 0x6e, 0x6f, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x20, 0x69, 0x74, 0x20, 0x79, 0x65, 0x74, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e,
    0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x68, 0x61,
    0x70, 0x70, 0x65, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x72,
    0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x69, 0x73, 0x73, 0x75, 0x65,
    0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x53,
    0x20, 0x68, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xae, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xae, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x04, 0xae,
    0x02, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x04, 0xae, 0x02,
    0x18, 0x19, 0x0a, 0xb1, 0x02, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xb6, 0x02, 0x00, 0xba, 0x02,
    0x01, 0x1a, 0xa2, 0x02, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20,
    0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x61,
    0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47,
    0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20,
    0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x65,
    0x61, 0x63, 0x68, 0x20, 0x54, 0x53, 0x20, 0x55, 0x55, 0x49, 0x44, 0x20, 0x69, 0x73, 0x20, 0x6c,
    0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x6e, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x77, 0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x61, 0x6c,
    0x73, 0x6f, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x6d, 0x69, 0x74, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2c, 0x0a, 0x20, 0x6c, 0x6f, 0x61, 0x64,
    0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2c, 0x20, 0x74, 0x6f, 0x70, 0x6f, 0x6c, 0x6f, 0x67, 0x79, 0x2c,
    0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xb6,
    0x02, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x02, 0x02,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7, 0x02, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x11, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xb9, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xb9, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xbc,
    0x02, 0x00, 0xbf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xbc, 0x02,
    0x08, 0x23, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xbe, 0x02, 0x02, 0x20,
    0x1a, 0x2b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x49, 0x44,
    0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x6f,
    0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12,
    0x06, 0xc1, 0x02, 0x00, 0xcb, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04,
    0xc1, 0x02, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x02,
    0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc2, 0x02, 0x0b, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x19, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xc4, 0x02, 0x02, 0x32, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc4, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x02, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc4, 0x02, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x03, 0x00,
    0x12, 0x06, 0xc6, 0x02, 0x02, 0xc9, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x03, 0x00,
    0x01, 0x12, 0x04, 0xc6, 0x02, 0x0a, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xc7, 0x02, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x13, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc8, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc8, 0x02, 0x0d, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xca, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xca, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x02, 0x06, 0x12, 0x04, 0xca, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xca, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xca, 0x02, 0x1a, 0x1b, 0x0a, 0xb5, 0x01, 0x0a, 0x02, 0x04, 0x12,
    0x12, 0x06, 0xd0, 0x02, 0x00, 0xd8, 0x02, 0x01, 0x1a, 0xa6, 0x01, 0x20, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x0a, 0x20, 0x20, 0x43, 0x61, 0x74, 0x61, 0x6c,
    0x6f, 0x67, 0x0a, 0x20, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xd0, 0x02, 0x08, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xd1, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd1, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xd1, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01,
    0x12, 0x04, 0xd2, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xd2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xd2, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2,
    0x02, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x02,
    0x1d, 0x1e, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xd5, 0x02, 0x02, 0x2a,
    0x1a, 0x4b, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x20, 0x70, 0x72, 0x65, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x73,
    0x20, 0x3d, 0x20, 0x33, 0x3b, 0x0a, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x52, 0x6f, 0x77, 0x50, 0x42, 0x20, 0x73, 0x70, 0x6c,
    0x69, 0x74, 0x5f, 0x72, 0x6f, 0x77, 0x73, 0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd5, 0x02, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd5, 0x02, 0x1b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02,
    0x03, 0x12, 0x04, 0xd6, 0x02, 0x02, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x04,
    0x12, 0x04, 0xd6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x06, 0x12,
    0x04, 0xd6, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xd6, 0x02, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd6,
    0x02, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x02,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd7, 0x02, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x11, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd7, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x13, 0x12, 0x06, 0xda, 0x02, 0x00, 0xdf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x13, 0x01, 0x12, 0x04, 0xda, 0x02, 0x08, 0x1d, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00,
    0x12, 0x04, 0xdc, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdc, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xdc, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01,
    0x12, 0x04, 0xde, 0x02, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xde, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xde, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xde,
    0x02, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xde, 0x02,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xe1, 0x02, 0x00, 0xe3, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xe2, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe2, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xe5,
    0x02, 0x00, 0xeb, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xe5, 0x02,
    0x08, 0x23, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xe7, 0x02, 0x02, 0x23,
    0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66,
    0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xe7, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe7,
    0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe7, 0x02,
    0x21, 0x22, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xea, 0x02, 0x02, 0x19,
    0x1a, 0x3c, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x61,
    0x6c, 0x73, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xea, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xea, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xea, 0x02, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xea, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16,
    0x12, 0x06, 0xed, 0x02, 0x00, 0xef, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12,
    0x04, 0xed, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xee,
    0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xee, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xee, 0x02, 0x0b,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xee, 0x02, 0x1d, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xee, 0x02, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xf1, 0x02, 0x00, 0xf4, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x08, 0x1d, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x00, 0x12, 0x04, 0xf3, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf3, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf3, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18,
    0x12, 0x06, 0xf6, 0x02, 0x00, 0xf9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12,
    0x04, 0xf6, 0x02, 0x08, 0x1b, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xf8,
    0x02, 0x02, 0x22, 0x1a, 0x4f, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2c,
    0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x73, 0x61, 0x74, 0x69, 0x73, 0x66,
    0x79, 0x20, 0x61, 0x20, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x20, 0x6f, 0x6e, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf8,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf8, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf8, 0x02, 0x12,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf8, 0x02, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xfb, 0x02, 0x00, 0x85, 0x03, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x1c, 0x0a, 0x42, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x19, 0x03, 0x00, 0x12, 0x06, 0xff, 0x02, 0x02, 0x82, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x03, 0x00, 0x01, 0x12, 0x04, 0xff, 0x02, 0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x80, 0x03, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x19, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x80, 0x03, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x03, 0x13, 0x15, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x03, 0x18, 0x19, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x81, 0x03, 0x04, 0x1d, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x81, 0x03, 0x04, 0x0c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x81, 0x03, 0x0d, 0x13,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x03, 0x14,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x81, 0x03,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0x84, 0x03, 0x02, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0x84, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x06, 0x12, 0x04, 0x84, 0x03, 0x0b, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x03, 0x15, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0x84, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1a, 0x12, 0x06, 0x87, 0x03, 0x00, 0x8f, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a,
    0x01, 0x12, 0x04, 0x87, 0x03, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12,
    0x04, 0x88, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x88, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x88,
    0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x03,
    0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x03, 0x25,
    0x26, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x03, 0x02, 0x29, 0x1a,
    0x16, 0x20, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2d, 0x6b, 0x65, 0x79, 0x20,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8b, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x8b, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8b, 0x03, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b,
    0x03, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x03, 0x02,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8c, 0x03, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x11, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x03, 0x02, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8e, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x03, 0x05, 0x12, 0x04, 0x8e, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x12, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x8e, 0x03, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x08,
    0x12, 0x04, 0x8e, 0x03, 0x2d, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x07, 0x12,
    0x04, 0x8e, 0x03, 0x39, 0x3b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x91, 0x03, 0x00,
    0x96, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x91, 0x03, 0x08, 0x23,
    0x0a, 0x42, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x93, 0x03, 0x02, 0x23, 0x1a, 0x34,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61,
    0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x93, 0x03,
    0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x03, 0x19,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x03, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0x95, 0x03, 0x02, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x01, 0x06, 0x12, 0x04, 0x95, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x03, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x03, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c,
    0x12, 0x06, 0x98, 0x03, 0x00, 0xbf, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12,
    0x04, 0x98, 0x03, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1c, 0x04, 0x00, 0x12, 0x06, 0x99,
    0x03, 0x02, 0xa2, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x04, 0x00, 0x01, 0x12, 0x04,
    0x99, 0x03, 0x07, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x9a, 0x03, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9a, 0x03, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x9a, 0x03, 0x0e, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x9b, 0x03, 0x04, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x9b, 0x03, 0x04, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x04, 0x9b, 0x03, 0x11, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x04, 0x9c, 0x03, 0x04, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x9c, 0x03, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x9d, 0x03, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x04, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x9d, 0x03, 0x14, 0x15, 0x0a, 0x6f, 0x0a, 0x06,
    0x04, 0x1c, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xa1, 0x03, 0x04, 0x15, 0x1a, 0x5f, 0x20, 0x54,
    0x4f, 0x44, 0x4f, 0x28, 0x4b, 0x55, 0x44, 0x55, 0x2d, 0x38, 0x36, 0x31, 0x29, 0x3a, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x65,
    0x20, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x2c, 0x20,
    0x62, 0x75, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79, 0x65, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x69, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa1, 0x03, 0x04, 0x10, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xa1, 0x03, 0x13, 0x14, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x1c, 0x03, 0x00, 0x12, 0x06, 0xa3, 0x03, 0x02, 0xa8, 0x03, 0x03, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x03, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x03, 0x0a, 0x13, 0x0a, 0x86,
    0x01, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x03, 0x04, 0x27, 0x1a,
    0x76, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x64, 0x64, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x27, 0x69, 0x64, 0x27, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x2d, 0x2d, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x20,
    0x61, 0x6e, 0x20, 0x49, 0x44, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa7, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xa7, 0x03, 0x0d, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x03, 0x1c, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x03, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x1c, 0x03, 0x01, 0x12, 0x06, 0xa9, 0x03, 0x02, 0xac, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x03, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x03, 0x0a, 0x14, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x1c,
    0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xab, 0x03, 0x04, 0x1d, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20,
    0x74, 0x6f, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x03, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x03, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1c, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x03, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x1c, 0x03, 0x02, 0x12, 0x06, 0xad, 0x03, 0x02, 0xb1, 0x03, 0x03, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x03, 0x02, 0x01, 0x12, 0x04, 0xad, 0x03, 0x0a, 0x16, 0x0a, 0x2f, 0x0a, 0x06,
    0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x12, 0x04, 0xaf, 0x03, 0x04, 0x21, 0x1a, 0x1f, 0x20, 0x4e,
    0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x3b, 0x0a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaf, 0x03, 0x04, 0x0c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaf, 0x03, 0x0d, 0x13, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaf, 0x03, 0x14, 0x1c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaf, 0x03, 0x1f,
    0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x03, 0x04,
    0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb0, 0x03,
    0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb0,
    0x03, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xb0, 0x03, 0x14, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb0, 0x03, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1c, 0x03, 0x03, 0x12, 0x06, 0xb3,
    0x03, 0x02, 0xba, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x03, 0x03, 0x01, 0x12, 0x04,
    0xb3, 0x03, 0x0a, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04,
    0xb4, 0x03, 0x04, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xb4, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xb4, 0x03, 0x0d, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb4, 0x03, 0x16, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb4, 0x03, 0x1d, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03,
    0x02, 0x00, 0x08, 0x12, 0x04, 0xb4, 0x03, 0x1f, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x03, 0x02, 0x00, 0x07, 0x12, 0x04, 0xb4, 0x03, 0x2b, 0x32, 0x0a, 0x4b, 0x0a, 0x06, 0x04, 0x1c,
    0x03, 0x03, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x03, 0x04, 0x26, 0x1a, 0x3b, 0x20, 0x45, 0x78, 0x61,
    0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20,
    0x27, 0x74, 0x79, 0x70, 0x65, 0x27, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb7, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xb7, 0x03, 0x0d, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb7, 0x03, 0x17, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x03, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb7, 0x03, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x1c, 0x03, 0x03, 0x02, 0x02, 0x12, 0x04, 0xb8, 0x03, 0x04, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb8, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1c, 0x03, 0x03, 0x02, 0x02, 0x06, 0x12, 0x04, 0xb8, 0x03, 0x0d, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x18, 0x23, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb8, 0x03, 0x26, 0x27, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x03, 0x04, 0x2c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb9, 0x03, 0x04, 0x0c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x03, 0x06, 0x12, 0x04, 0xb9, 0x03, 0x0d,
    0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb9, 0x03,
    0x1a, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb9,
    0x03, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x03, 0x02,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbc, 0x03, 0x0b, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x03, 0x1d, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbd, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xbd, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xbd, 0x03, 0x10, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xbd, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12,
    0x04, 0xbe, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xbe, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbe,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x03,
    0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbe, 0x03, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xc1, 0x03, 0x00, 0xc6, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xc1, 0x03, 0x08, 0x1c, 0x0a, 0x42, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xc3, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc3, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc3, 0x03, 0x0b, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc3, 0x03, 0x19, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc3, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xc5, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xc5, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xc5, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xc5, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xc5, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xc8, 0x03,
    0x00, 0xca, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xc8, 0x03, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xc9, 0x03, 0x02, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc9, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc9, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc9, 0x03, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc9, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x1f, 0x12, 0x06, 0xcc, 0x03, 0x00, 0xd5, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01,
    0x12, 0x04, 0xcc, 0x03, 0x08, 0x22, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04,
    0xce, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xce, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xce, 0x03, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xce, 0x03, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xce, 0x03, 0x21, 0x22, 0x0a, 0x62, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01, 0x12, 0x04,
    0xd1, 0x03, 0x02, 0x25, 0x1a, 0x54, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x6e, 0x65, 0x77, 0x27, 0x20,
    0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69,
    0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e,
    0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xd1, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xd1, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xd1, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xd1, 0x03, 0x23, 0x24, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04,
    0xd4, 0x03, 0x02, 0x19, 0x1a, 0x3b, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c,
    0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd4, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd4, 0x03, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x03, 0x10, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd4, 0x03, 0x17, 0x18, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x20, 0x12, 0x06, 0xd7, 0x03, 0x00, 0xd9, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x20, 0x01, 0x12, 0x04, 0xd7, 0x03, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00,
    0x12, 0x04, 0xd8, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xd8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xd8, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8,
    0x03, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x03,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xdb, 0x03, 0x00, 0xef, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xdb, 0x03, 0x08, 0x20, 0x0a, 0x42, 0x0a,
    0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xdd, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdd, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdd, 0x03, 0x0b, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x03, 0x19, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdd, 0x03, 0x21, 0x22, 0x0a, 0xe8, 0x01,
    0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xe3, 0x03, 0x02, 0x1f, 0x1a, 0xd9, 0x01, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x54, 0x53,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x69,
    0x66, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x6b, 0x65, 0x65, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x20,
    0x49, 0x6e, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6c,
    0x74, 0x65, 0x72, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x70, 0x72, 0x6f,
    0x67, 0x72, 0x65, 0x73, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x3b, 0x0a, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x20, 0x69,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20,
    0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xe3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x06,
    0x12, 0x04, 0xe3, 0x03, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xe3, 0x03, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xe3, 0x03, 0x1d, 0x1e, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x02, 0x12, 0x04, 0xe6, 0x03,
    0x02, 0x32, 0x1a, 0x1f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x27, 0x73,
    0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe6, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x06, 0x12, 0x04, 0xe6, 0x03, 0x0b,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe6, 0x03, 0x1d, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe6, 0x03, 0x30, 0x31, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x03, 0x12, 0x04, 0xe8, 0x03, 0x02, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe8, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe8, 0x03, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xe8, 0x03, 0x20, 0x21, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x21, 0x02,
    0x04, 0x12, 0x04, 0xeb, 0x03, 0x02, 0x1e, 0x1a, 0x16, 0x20, 0x54, 0x68, 0x65, 0x20, 0x49, 0x44,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x04, 0x04, 0x12, 0x04, 0xeb, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x04, 0x05, 0x12, 0x04, 0xeb, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x04, 0x01, 0x12, 0x04, 0xeb, 0x03, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x04, 0x03, 0x12, 0x04, 0xeb, 0x03, 0x1c, 0x1d, 0x0a, 0x4b, 0x0a, 0x04, 0x04,
    0x21, 0x02, 0x05, 0x12, 0x04, 0xee, 0x03, 0x02, 0x26, 0x1a, 0x3d, 0x20, 0x54, 0x72, 0x75, 0x65,
    0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70,
    0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x6f, 0x74, 0x68,
    0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xee, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xee, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xee, 0x03, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xee, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xf5, 0x03, 0x00, 0xf6,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xf5, 0x03, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xf8, 0x03, 0x00, 0x81, 0x04, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xf8, 0x03, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23,
    0x02, 0x00, 0x12, 0x04, 0xf9, 0x03, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xf9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xf9, 0x03, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xf9, 0x03, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xf9, 0x03, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x23, 0x03, 0x00, 0x12, 0x06, 0xfb, 0x03,
    0x02, 0xff, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x03, 0x00, 0x01, 0x12, 0x04, 0xfb,
    0x03, 0x0a, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfc,
    0x03, 0x04, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xfc, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xfc, 0x03, 0x0d, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xfc, 0x03, 0x1c, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xfc, 0x03, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xfd, 0x03, 0x04, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xfd, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xfd, 0x03, 0x0d, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfd, 0x03, 0x1e, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfd, 0x03, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x23, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xfe, 0x03, 0x04, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x23, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfe, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfe, 0x03, 0x0d, 0x12, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x03, 0x13, 0x29, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfe, 0x03, 0x2c, 0x2d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0x80, 0x04, 0x02, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x01, 0x06, 0x12, 0x04, 0x80, 0x04, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x04, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x04, 0x1b, 0x1c, 0x0a, 0x7a, 0x0a, 0x02, 0x04, 0x24, 0x12,
    0x06, 0x85, 0x04, 0x00, 0x86, 0x04, 0x01, 0x1a, 0x6c, 0x20, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a,
    0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63,
    0x65, 0x20, 0x69, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x52,
    0x50, 0x43, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0x85, 0x04,
    0x08, 0x26, 0x0a, 0x37, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0x89, 0x04, 0x00, 0x96, 0x04, 0x01,
    0x1a, 0x29, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x4a, 0x75, 0x73, 0x74, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x25, 0x01, 0x12, 0x04, 0x89, 0x04, 0x08, 0x27, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00,
    0x12, 0x04, 0x8b, 0x04, 0x02, 0x2a, 0x1a, 0x2a, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x73, 0x65, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8b, 0x04, 0x0b, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x04, 0x1a, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x04, 0x28, 0x29, 0x0a, 0x7a,
    0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0x8f, 0x04, 0x02, 0x31, 0x1a, 0x6c, 0x20, 0x54,
    0x68, 0x65, 0x73, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x79, 0x20, 0x77, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x27, 0x73, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x2f, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x8f, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x8f, 0x04, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x8f, 0x04, 0x20, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x8f, 0x04, 0x2f, 0x30, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04,
    0x92, 0x04, 0x02, 0x2e, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x27, 0x73, 0x20, 0x72, 0x6f, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x92, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x92, 0x04, 0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x92, 0x04, 0x25, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x92, 0x04, 0x2c, 0x2d, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x03, 0x12, 0x04,
    0x95, 0x04, 0x02, 0x23, 0x1a, 0x40, 0x20, 0x53, 0x65, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x72, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x74,
    0x72, 0x69, 0x65, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x95, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x06, 0x12, 0x04,
    0x95, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x01, 0x12, 0x04, 0x95,
    0x04, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x03, 0x12, 0x04, 0x95, 0x04,
    0x21, 0x22, 0x0a, 0x79, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0x9a, 0x04, 0x00, 0x9b, 0x04, 0x01,
    0x1a, 0x6b, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20,
    0x67, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x0a, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0x9a, 0x04, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27,
    0x12, 0x06, 0x9d, 0x04, 0x00, 0xa4, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0x9d, 0x04, 0x08, 0x1d, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0x9f,
    0x04, 0x02, 0x25, 0x1a, 0x2d, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75,
    0x61, 0x6c, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9f, 0x04, 0x0b, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x04, 0x19, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x04, 0x23, 0x24, 0x0a, 0x90,
    0x01, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x04, 0x02, 0x21, 0x1a, 0x81, 0x01,
    0x20, 0x53, 0x65, 0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x72, 0x65, 0x27, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e,
    0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x20,
    0x6f, 0x72, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x67, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x27, 0x73, 0x20, 0x6f, 0x77, 0x6e,
    0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3, 0x04, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa3, 0x04, 0x0b, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x04, 0x17, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x04, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x02, 0x06, 0x00, 0x12, 0x06, 0xa6, 0x04, 0x00, 0xc0, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x06,
    0x00, 0x01, 0x12, 0x04, 0xa6, 0x04, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xa7, 0x04, 0x02, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa7, 0x04, 0x06, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xa7, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7,
    0x04, 0x23, 0x31, 0x0a, 0x1f, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x04, 0x02,
    0x48, 0x1a, 0x11, 0x20, 0x54, 0x53, 0x2d, 0x3e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x52,
    0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa,
    0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xaa, 0x04,
    0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x04, 0x31,
    0x46, 0x0a, 0x23, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x04, 0xad, 0x04, 0x02, 0x5d, 0x1a,
    0x15, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x3e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x20, 0x52, 0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xad, 0x04, 0x06, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04,
    0xad, 0x04, 0x19, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xad,
    0x04, 0x3f, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04, 0xaf, 0x04, 0x02,
    0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xaf, 0x04, 0x06, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xaf, 0x04, 0x12, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xaf, 0x04, 0x31, 0x46, 0x0a, 0x0c,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x04, 0x02, 0x5a, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb0, 0x04, 0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb0, 0x04, 0x18, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xb0, 0x04, 0x3d, 0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x05, 0x12, 0x04, 0xb2, 0x04, 0x02, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xb2, 0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x02, 0x12,
    0x04, 0xb2, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xb2, 0x04, 0x31, 0x46, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x06, 0x12, 0x04, 0xb4, 0x04,
    0x02, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb4, 0x04, 0x06,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xb4, 0x04, 0x11, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xb4, 0x04, 0x2f, 0x43, 0x0a,
    0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x07, 0x12, 0x04, 0xb5, 0x04, 0x02, 0x57, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xb5, 0x04, 0x06, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0xb5, 0x04, 0x17, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0xb5, 0x04, 0x3b, 0x55, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x08, 0x12, 0x04, 0xb7, 0x04, 0x02, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xb7, 0x04, 0x06, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x08, 0x02,
    0x12, 0x04, 0xb7, 0x04, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x08, 0x03, 0x12,
    0x04, 0xb7, 0x04, 0x2f, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x09, 0x12, 0x04, 0xb8,
    0x04, 0x02, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xb8, 0x04,
    0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xb8, 0x04, 0x18,
    0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0xb8, 0x04, 0x3d, 0x58,
    0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xb9, 0x04, 0x02, 0x51, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xb9, 0x04, 0x06, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xb9, 0x04, 0x15, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xb9, 0x04, 0x37, 0x4f, 0x0a, 0x2e, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x0b, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x5a, 0x1a, 0x20, 0x20, 0x41, 0x64, 0x6d, 0x69,
    0x6e, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x76, 0x65, 0x2f, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
    0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xbc, 0x04, 0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x0b, 0x02, 0x12, 0x04, 0xbc, 0x04, 0x18, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0xbc, 0x04, 0x3d, 0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0c,
    0x12, 0x04, 0xbd, 0x04, 0x02, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0xbd, 0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x04,
    0xbd, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xbd,
    0x04, 0x31, 0x46, 0x0a, 0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0d, 0x12, 0x06, 0xbe, 0x04, 0x02,
    0xbf, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xbe, 0x04,
    0x06, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x04, 0xbe, 0x04, 0x1c,
    0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xbf, 0x04, 0x05, 0x24,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
