// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use std::io::Write;

use protobuf::CodedOutputStream;
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct MasterErrorPB {
    // message fields
    code: ::std::option::Option<MasterErrorPB_Code>,
    status: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(MasterErrorPB::new)
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

    fn get_code_for_reflect(&self) -> &::std::option::Option<MasterErrorPB_Code> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::std::option::Option<MasterErrorPB_Code> {
        &mut self.code
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

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &mut self.status
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.code {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.status.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MasterErrorPB_Code>>(
                    "code",
                    MasterErrorPB::get_code_for_reflect,
                    MasterErrorPB::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::AppStatusPB>>(
                    "status",
                    MasterErrorPB::get_status_for_reflect,
                    MasterErrorPB::mut_status_for_reflect,
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

impl ::std::fmt::Debug for MasterErrorPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MasterErrorPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
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
    TABLET_NOT_RUNNING = 9,
    EVEN_REPLICATION_FACTOR = 10,
    ILLEGAL_REPLICATION_FACTOR = 11,
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
            9 => ::std::option::Option::Some(MasterErrorPB_Code::TABLET_NOT_RUNNING),
            10 => ::std::option::Option::Some(MasterErrorPB_Code::EVEN_REPLICATION_FACTOR),
            11 => ::std::option::Option::Some(MasterErrorPB_Code::ILLEGAL_REPLICATION_FACTOR),
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
            MasterErrorPB_Code::TABLET_NOT_RUNNING,
            MasterErrorPB_Code::EVEN_REPLICATION_FACTOR,
            MasterErrorPB_Code::ILLEGAL_REPLICATION_FACTOR,
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

impl ::protobuf::reflect::ProtobufValue for MasterErrorPB_Code {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TSToMasterCommonPB {
    // message fields
    ts_instance: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TSToMasterCommonPB::new)
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

    fn get_ts_instance_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &self.ts_instance
    }

    fn mut_ts_instance_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &mut self.ts_instance
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ts_instance)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ts_instance.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ts_instance.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::NodeInstancePB>>(
                    "ts_instance",
                    TSToMasterCommonPB::get_ts_instance_for_reflect,
                    TSToMasterCommonPB::mut_ts_instance_for_reflect,
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

impl ::std::fmt::Debug for TSToMasterCommonPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TSToMasterCommonPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TableIdentifierPB {
    // message fields
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TableIdentifierPB::new)
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

    fn get_table_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.table_id
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

    fn get_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.table_name
    }

    fn mut_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.table_name
    }
}

impl ::protobuf::Message for TableIdentifierPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.table_name.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "table_id",
                    TableIdentifierPB::get_table_id_for_reflect,
                    TableIdentifierPB::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "table_name",
                    TableIdentifierPB::get_table_name_for_reflect,
                    TableIdentifierPB::mut_table_name_for_reflect,
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

impl ::std::fmt::Debug for TableIdentifierPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TableIdentifierPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SysTabletsEntryPB::new)
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

    fn get_DEPRECATED_start_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.DEPRECATED_start_key
    }

    fn mut_DEPRECATED_start_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.DEPRECATED_start_key
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

    fn get_DEPRECATED_end_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.DEPRECATED_end_key
    }

    fn mut_DEPRECATED_end_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.DEPRECATED_end_key
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

    fn get_partition_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PartitionPB> {
        &self.partition
    }

    fn mut_partition_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PartitionPB> {
        &mut self.partition
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

    fn get_committed_consensus_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB> {
        &self.committed_consensus_state
    }

    fn mut_committed_consensus_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB> {
        &mut self.committed_consensus_state
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

    fn get_state_for_reflect(&self) -> &::std::option::Option<SysTabletsEntryPB_State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<SysTabletsEntryPB_State> {
        &mut self.state
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

    fn get_state_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.state_msg
    }

    fn mut_state_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.state_msg
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

    fn get_table_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.table_id
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.DEPRECATED_start_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.DEPRECATED_end_key)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.committed_consensus_state)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.state_msg)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.DEPRECATED_start_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.DEPRECATED_end_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.partition.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        if let Some(v) = self.state_msg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.table_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.DEPRECATED_start_key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.DEPRECATED_end_key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.partition.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.state {
            os.write_enum(4, v.value())?;
        };
        if let Some(v) = self.state_msg.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.table_id.as_ref() {
            os.write_bytes(6, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "DEPRECATED_start_key",
                    SysTabletsEntryPB::get_DEPRECATED_start_key_for_reflect,
                    SysTabletsEntryPB::mut_DEPRECATED_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "DEPRECATED_end_key",
                    SysTabletsEntryPB::get_DEPRECATED_end_key_for_reflect,
                    SysTabletsEntryPB::mut_DEPRECATED_end_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PartitionPB>>(
                    "partition",
                    SysTabletsEntryPB::get_partition_for_reflect,
                    SysTabletsEntryPB::mut_partition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::consensus_metadata::ConsensusStatePB>>(
                    "committed_consensus_state",
                    SysTabletsEntryPB::get_committed_consensus_state_for_reflect,
                    SysTabletsEntryPB::mut_committed_consensus_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SysTabletsEntryPB_State>>(
                    "state",
                    SysTabletsEntryPB::get_state_for_reflect,
                    SysTabletsEntryPB::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_msg",
                    SysTabletsEntryPB::get_state_msg_for_reflect,
                    SysTabletsEntryPB::mut_state_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "table_id",
                    SysTabletsEntryPB::get_table_id_for_reflect,
                    SysTabletsEntryPB::mut_table_id_for_reflect,
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

impl ::std::fmt::Debug for SysTabletsEntryPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SysTabletsEntryPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
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

impl ::protobuf::reflect::ProtobufValue for SysTabletsEntryPB_State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(SysTablesEntryPB::new)
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

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
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

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
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

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::SchemaPB> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::SchemaPB> {
        &mut self.schema
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

    fn get_fully_applied_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::SchemaPB> {
        &self.fully_applied_schema
    }

    fn mut_fully_applied_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::SchemaPB> {
        &mut self.fully_applied_schema
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

    fn get_partition_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &self.partition_schema
    }

    fn mut_partition_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &mut self.partition_schema
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

    fn get_next_column_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.next_column_id
    }

    fn mut_next_column_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.next_column_id
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

    fn get_num_replicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_replicas
    }

    fn mut_num_replicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_replicas
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

    fn get_state_for_reflect(&self) -> &::std::option::Option<SysTablesEntryPB_State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<SysTablesEntryPB_State> {
        &mut self.state
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

    fn get_state_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.state_msg
    }

    fn mut_state_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.state_msg
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fully_applied_schema)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.next_column_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.state_msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.fully_applied_schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.next_column_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_replicas {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.state_msg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.schema.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.fully_applied_schema.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.next_column_id {
            os.write_int32(8, v)?;
        };
        if let Some(v) = self.num_replicas {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.state {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.state_msg.as_ref() {
            os.write_bytes(7, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    SysTablesEntryPB::get_name_for_reflect,
                    SysTablesEntryPB::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    SysTablesEntryPB::get_version_for_reflect,
                    SysTablesEntryPB::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SchemaPB>>(
                    "schema",
                    SysTablesEntryPB::get_schema_for_reflect,
                    SysTablesEntryPB::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SchemaPB>>(
                    "fully_applied_schema",
                    SysTablesEntryPB::get_fully_applied_schema_for_reflect,
                    SysTablesEntryPB::mut_fully_applied_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PartitionSchemaPB>>(
                    "partition_schema",
                    SysTablesEntryPB::get_partition_schema_for_reflect,
                    SysTablesEntryPB::mut_partition_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "next_column_id",
                    SysTablesEntryPB::get_next_column_id_for_reflect,
                    SysTablesEntryPB::mut_next_column_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_replicas",
                    SysTablesEntryPB::get_num_replicas_for_reflect,
                    SysTablesEntryPB::mut_num_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SysTablesEntryPB_State>>(
                    "state",
                    SysTablesEntryPB::get_state_for_reflect,
                    SysTablesEntryPB::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "state_msg",
                    SysTablesEntryPB::get_state_msg_for_reflect,
                    SysTablesEntryPB::mut_state_msg_for_reflect,
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

impl ::std::fmt::Debug for SysTablesEntryPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SysTablesEntryPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
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

impl ::protobuf::reflect::ProtobufValue for SysTablesEntryPB_State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(PingRequestPB::new)
        }
    }
}

impl ::protobuf::Message for PingRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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

impl ::std::fmt::Debug for PingRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingResponsePB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(PingResponsePB::new)
        }
    }
}

impl ::protobuf::Message for PingResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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

impl ::std::fmt::Debug for PingResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ReportedTabletPB::new)
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

    fn get_tablet_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.tablet_id
    }

    fn mut_tablet_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.tablet_id
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

    fn get_state_for_reflect(&self) -> &::std::option::Option<super::tablet_metadata::TabletStatePB> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::tablet_metadata::TabletStatePB> {
        &mut self.state
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

    fn get_tablet_data_state_for_reflect(&self) -> &::std::option::Option<super::tablet_metadata::TabletDataState> {
        &self.tablet_data_state
    }

    fn mut_tablet_data_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::tablet_metadata::TabletDataState> {
        &mut self.tablet_data_state
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

    fn get_committed_consensus_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB> {
        &self.committed_consensus_state
    }

    fn mut_committed_consensus_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::consensus_metadata::ConsensusStatePB> {
        &mut self.committed_consensus_state
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &mut self.error
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

    fn get_schema_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schema_version
    }

    fn mut_schema_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schema_version
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.tablet_data_state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.committed_consensus_state)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tablet_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.tablet_data_state {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.schema_version {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tablet_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.tablet_data_state {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.committed_consensus_state.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.error.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.schema_version {
            os.write_uint32(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tablet_id",
                    ReportedTabletPB::get_tablet_id_for_reflect,
                    ReportedTabletPB::mut_tablet_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::tablet_metadata::TabletStatePB>>(
                    "state",
                    ReportedTabletPB::get_state_for_reflect,
                    ReportedTabletPB::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::tablet_metadata::TabletDataState>>(
                    "tablet_data_state",
                    ReportedTabletPB::get_tablet_data_state_for_reflect,
                    ReportedTabletPB::mut_tablet_data_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::consensus_metadata::ConsensusStatePB>>(
                    "committed_consensus_state",
                    ReportedTabletPB::get_committed_consensus_state_for_reflect,
                    ReportedTabletPB::mut_committed_consensus_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::AppStatusPB>>(
                    "error",
                    ReportedTabletPB::get_error_for_reflect,
                    ReportedTabletPB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schema_version",
                    ReportedTabletPB::get_schema_version_for_reflect,
                    ReportedTabletPB::mut_schema_version_for_reflect,
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

impl ::std::fmt::Debug for ReportedTabletPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReportedTabletPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TabletReportPB {
    // message fields
    is_incremental: ::std::option::Option<bool>,
    updated_tablets: ::protobuf::RepeatedField<ReportedTabletPB>,
    removed_tablet_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    sequence_number: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TabletReportPB::new)
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

    fn get_is_incremental_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_incremental
    }

    fn mut_is_incremental_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_incremental
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

    fn get_updated_tablets_for_reflect(&self) -> &::protobuf::RepeatedField<ReportedTabletPB> {
        &self.updated_tablets
    }

    fn mut_updated_tablets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReportedTabletPB> {
        &mut self.updated_tablets
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

    fn get_removed_tablet_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.removed_tablet_ids
    }

    fn mut_removed_tablet_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.removed_tablet_ids
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

    fn get_sequence_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_number
    }

    fn mut_sequence_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_number
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_incremental = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updated_tablets)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.removed_tablet_ids)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sequence_number = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.is_incremental {
            my_size += 2;
        };
        for value in &self.updated_tablets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.removed_tablet_ids {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if let Some(v) = self.sequence_number {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_incremental {
            os.write_bool(1, v)?;
        };
        for v in &self.updated_tablets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.removed_tablet_ids {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.sequence_number {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_incremental",
                    TabletReportPB::get_is_incremental_for_reflect,
                    TabletReportPB::mut_is_incremental_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReportedTabletPB>>(
                    "updated_tablets",
                    TabletReportPB::get_updated_tablets_for_reflect,
                    TabletReportPB::mut_updated_tablets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "removed_tablet_ids",
                    TabletReportPB::get_removed_tablet_ids_for_reflect,
                    TabletReportPB::mut_removed_tablet_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_number",
                    TabletReportPB::get_sequence_number_for_reflect,
                    TabletReportPB::mut_sequence_number_for_reflect,
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

impl ::std::fmt::Debug for TabletReportPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TabletReportPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReportedTabletUpdatesPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    state_msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ReportedTabletUpdatesPB::new)
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

    fn get_tablet_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.tablet_id
    }

    fn mut_tablet_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.tablet_id
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

    fn get_state_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.state_msg
    }

    fn mut_state_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.state_msg
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.state_msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tablet_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.state_msg.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tablet_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.state_msg.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tablet_id",
                    ReportedTabletUpdatesPB::get_tablet_id_for_reflect,
                    ReportedTabletUpdatesPB::mut_tablet_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "state_msg",
                    ReportedTabletUpdatesPB::get_state_msg_for_reflect,
                    ReportedTabletUpdatesPB::mut_state_msg_for_reflect,
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

impl ::std::fmt::Debug for ReportedTabletUpdatesPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReportedTabletUpdatesPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TabletReportUpdatesPB {
    // message fields
    tablets: ::protobuf::RepeatedField<ReportedTabletUpdatesPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TabletReportUpdatesPB::new)
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

    fn get_tablets_for_reflect(&self) -> &::protobuf::RepeatedField<ReportedTabletUpdatesPB> {
        &self.tablets
    }

    fn mut_tablets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReportedTabletUpdatesPB> {
        &mut self.tablets
    }
}

impl ::protobuf::Message for TabletReportUpdatesPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablets)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tablets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tablets {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReportedTabletUpdatesPB>>(
                    "tablets",
                    TabletReportUpdatesPB::get_tablets_for_reflect,
                    TabletReportUpdatesPB::mut_tablets_for_reflect,
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

impl ::std::fmt::Debug for TabletReportUpdatesPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TabletReportUpdatesPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TSHeartbeatRequestPB {
    // message fields
    common: ::protobuf::SingularPtrField<TSToMasterCommonPB>,
    registration: ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB>,
    tablet_report: ::protobuf::SingularPtrField<TabletReportPB>,
    num_live_tablets: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TSHeartbeatRequestPB::new)
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

    fn get_common_for_reflect(&self) -> &::protobuf::SingularPtrField<TSToMasterCommonPB> {
        &self.common
    }

    fn mut_common_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TSToMasterCommonPB> {
        &mut self.common
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

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &mut self.registration
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

    fn get_tablet_report_for_reflect(&self) -> &::protobuf::SingularPtrField<TabletReportPB> {
        &self.tablet_report
    }

    fn mut_tablet_report_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TabletReportPB> {
        &mut self.tablet_report
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

    fn get_num_live_tablets_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_live_tablets
    }

    fn mut_num_live_tablets_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_live_tablets
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tablet_report)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_live_tablets = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.common.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.tablet_report.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.num_live_tablets {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.common.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.registration.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.tablet_report.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.num_live_tablets {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TSToMasterCommonPB>>(
                    "common",
                    TSHeartbeatRequestPB::get_common_for_reflect,
                    TSHeartbeatRequestPB::mut_common_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::ServerRegistrationPB>>(
                    "registration",
                    TSHeartbeatRequestPB::get_registration_for_reflect,
                    TSHeartbeatRequestPB::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TabletReportPB>>(
                    "tablet_report",
                    TSHeartbeatRequestPB::get_tablet_report_for_reflect,
                    TSHeartbeatRequestPB::mut_tablet_report_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_live_tablets",
                    TSHeartbeatRequestPB::get_num_live_tablets_for_reflect,
                    TSHeartbeatRequestPB::mut_num_live_tablets_for_reflect,
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

impl ::std::fmt::Debug for TSHeartbeatRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TSHeartbeatRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TSHeartbeatResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_master_instance_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &self.master_instance
    }

    fn mut_master_instance_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &mut self.master_instance
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

    fn get_needs_reregister_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needs_reregister
    }

    fn mut_needs_reregister_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needs_reregister
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

    fn get_needs_full_tablet_report_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needs_full_tablet_report
    }

    fn mut_needs_full_tablet_report_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needs_full_tablet_report
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

    fn get_tablet_report_for_reflect(&self) -> &::protobuf::SingularPtrField<TabletReportUpdatesPB> {
        &self.tablet_report
    }

    fn mut_tablet_report_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TabletReportUpdatesPB> {
        &mut self.tablet_report
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

    fn get_leader_master_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.leader_master
    }

    fn mut_leader_master_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.leader_master
    }
}

impl ::protobuf::Message for TSHeartbeatResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.master_instance)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.needs_reregister = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.needs_full_tablet_report = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tablet_report)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.leader_master = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.master_instance.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.needs_reregister {
            my_size += 2;
        };
        if let Some(v) = self.needs_full_tablet_report {
            my_size += 2;
        };
        if let Some(v) = self.tablet_report.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.leader_master {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.master_instance.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.needs_reregister {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.needs_full_tablet_report {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.tablet_report.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.leader_master {
            os.write_bool(6, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    TSHeartbeatResponsePB::get_error_for_reflect,
                    TSHeartbeatResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::NodeInstancePB>>(
                    "master_instance",
                    TSHeartbeatResponsePB::get_master_instance_for_reflect,
                    TSHeartbeatResponsePB::mut_master_instance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needs_reregister",
                    TSHeartbeatResponsePB::get_needs_reregister_for_reflect,
                    TSHeartbeatResponsePB::mut_needs_reregister_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needs_full_tablet_report",
                    TSHeartbeatResponsePB::get_needs_full_tablet_report_for_reflect,
                    TSHeartbeatResponsePB::mut_needs_full_tablet_report_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TabletReportUpdatesPB>>(
                    "tablet_report",
                    TSHeartbeatResponsePB::get_tablet_report_for_reflect,
                    TSHeartbeatResponsePB::mut_tablet_report_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "leader_master",
                    TSHeartbeatResponsePB::get_leader_master_for_reflect,
                    TSHeartbeatResponsePB::mut_leader_master_for_reflect,
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

impl ::std::fmt::Debug for TSHeartbeatResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TSHeartbeatResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TabletLocationsPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition: ::protobuf::SingularPtrField<super::common::PartitionPB>,
    replicas: ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB>,
    DEPRECATED_stale: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TabletLocationsPB::new)
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

    fn get_tablet_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.tablet_id
    }

    fn mut_tablet_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.tablet_id
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

    fn get_start_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.start_key
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

    fn get_end_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.end_key
    }

    fn mut_end_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.end_key
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

    fn get_partition_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PartitionPB> {
        &self.partition
    }

    fn mut_partition_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PartitionPB> {
        &mut self.partition
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

    fn get_replicas_for_reflect(&self) -> &::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB> {
        &self.replicas
    }

    fn mut_replicas_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB_ReplicaPB> {
        &mut self.replicas
    }

    // optional bool DEPRECATED_stale = 5;

    pub fn clear_DEPRECATED_stale(&mut self) {
        self.DEPRECATED_stale = ::std::option::Option::None;
    }

    pub fn has_DEPRECATED_stale(&self) -> bool {
        self.DEPRECATED_stale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_DEPRECATED_stale(&mut self, v: bool) {
        self.DEPRECATED_stale = ::std::option::Option::Some(v);
    }

    pub fn get_DEPRECATED_stale(&self) -> bool {
        self.DEPRECATED_stale.unwrap_or(false)
    }

    fn get_DEPRECATED_stale_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.DEPRECATED_stale
    }

    fn mut_DEPRECATED_stale_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.DEPRECATED_stale
    }
}

impl ::protobuf::Message for TabletLocationsPB {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.DEPRECATED_stale = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tablet_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.start_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.end_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.partition.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.replicas {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.DEPRECATED_stale {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tablet_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.start_key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.end_key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.partition.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.replicas {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.DEPRECATED_stale {
            os.write_bool(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tablet_id",
                    TabletLocationsPB::get_tablet_id_for_reflect,
                    TabletLocationsPB::mut_tablet_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "start_key",
                    TabletLocationsPB::get_start_key_for_reflect,
                    TabletLocationsPB::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "end_key",
                    TabletLocationsPB::get_end_key_for_reflect,
                    TabletLocationsPB::mut_end_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PartitionPB>>(
                    "partition",
                    TabletLocationsPB::get_partition_for_reflect,
                    TabletLocationsPB::mut_partition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TabletLocationsPB_ReplicaPB>>(
                    "replicas",
                    TabletLocationsPB::get_replicas_for_reflect,
                    TabletLocationsPB::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "DEPRECATED_stale",
                    TabletLocationsPB::get_DEPRECATED_stale_for_reflect,
                    TabletLocationsPB::mut_DEPRECATED_stale_for_reflect,
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
        self.clear_DEPRECATED_stale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TabletLocationsPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TabletLocationsPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TabletLocationsPB_ReplicaPB {
    // message fields
    ts_info: ::protobuf::SingularPtrField<TSInfoPB>,
    role: ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TabletLocationsPB_ReplicaPB::new)
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

    fn get_ts_info_for_reflect(&self) -> &::protobuf::SingularPtrField<TSInfoPB> {
        &self.ts_info
    }

    fn mut_ts_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TSInfoPB> {
        &mut self.ts_info
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

    fn get_role_for_reflect(&self) -> &::std::option::Option<super::consensus_metadata::RaftPeerPB_Role> {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role> {
        &mut self.role
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ts_info)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.role = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ts_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.role {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ts_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.role {
            os.write_enum(2, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TSInfoPB>>(
                    "ts_info",
                    TabletLocationsPB_ReplicaPB::get_ts_info_for_reflect,
                    TabletLocationsPB_ReplicaPB::mut_ts_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::consensus_metadata::RaftPeerPB_Role>>(
                    "role",
                    TabletLocationsPB_ReplicaPB::get_role_for_reflect,
                    TabletLocationsPB_ReplicaPB::mut_role_for_reflect,
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

impl ::std::fmt::Debug for TabletLocationsPB_ReplicaPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TabletLocationsPB_ReplicaPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TSInfoPB {
    // message fields
    permanent_uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rpc_addresses: ::protobuf::RepeatedField<super::common::HostPortPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TSInfoPB::new)
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

    fn get_permanent_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.permanent_uuid
    }

    fn mut_permanent_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.permanent_uuid
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

    fn get_rpc_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<super::common::HostPortPB> {
        &self.rpc_addresses
    }

    fn mut_rpc_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::common::HostPortPB> {
        &mut self.rpc_addresses
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.permanent_uuid)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rpc_addresses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.permanent_uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        for value in &self.rpc_addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.permanent_uuid.as_ref() {
            os.write_bytes(1, &v)?;
        };
        for v in &self.rpc_addresses {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "permanent_uuid",
                    TSInfoPB::get_permanent_uuid_for_reflect,
                    TSInfoPB::mut_permanent_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::HostPortPB>>(
                    "rpc_addresses",
                    TSInfoPB::get_rpc_addresses_for_reflect,
                    TSInfoPB::mut_rpc_addresses_for_reflect,
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

impl ::std::fmt::Debug for TSInfoPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TSInfoPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTabletLocationsRequestPB {
    // message fields
    tablet_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTabletLocationsRequestPB::new)
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

    fn get_tablet_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.tablet_ids
    }

    fn mut_tablet_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.tablet_ids
    }
}

impl ::protobuf::Message for GetTabletLocationsRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.tablet_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tablet_ids {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tablet_ids {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tablet_ids",
                    GetTabletLocationsRequestPB::get_tablet_ids_for_reflect,
                    GetTabletLocationsRequestPB::mut_tablet_ids_for_reflect,
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

impl ::std::fmt::Debug for GetTabletLocationsRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTabletLocationsRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTabletLocationsResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tablet_locations: ::protobuf::RepeatedField<TabletLocationsPB>,
    errors: ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTabletLocationsResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_tablet_locations_for_reflect(&self) -> &::protobuf::RepeatedField<TabletLocationsPB> {
        &self.tablet_locations
    }

    fn mut_tablet_locations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB> {
        &mut self.tablet_locations
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

    fn get_errors_for_reflect(&self) -> &::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error> {
        &self.errors
    }

    fn mut_errors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GetTabletLocationsResponsePB_Error> {
        &mut self.errors
    }
}

impl ::protobuf::Message for GetTabletLocationsResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablet_locations)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tablet_locations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tablet_locations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.errors {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    GetTabletLocationsResponsePB::get_error_for_reflect,
                    GetTabletLocationsResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TabletLocationsPB>>(
                    "tablet_locations",
                    GetTabletLocationsResponsePB::get_tablet_locations_for_reflect,
                    GetTabletLocationsResponsePB::mut_tablet_locations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetTabletLocationsResponsePB_Error>>(
                    "errors",
                    GetTabletLocationsResponsePB::get_errors_for_reflect,
                    GetTabletLocationsResponsePB::mut_errors_for_reflect,
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

impl ::std::fmt::Debug for GetTabletLocationsResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTabletLocationsResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTabletLocationsResponsePB_Error {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    status: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTabletLocationsResponsePB_Error::new)
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

    fn get_tablet_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.tablet_id
    }

    fn mut_tablet_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.tablet_id
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

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &mut self.status
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tablet_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tablet_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tablet_id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.status.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tablet_id",
                    GetTabletLocationsResponsePB_Error::get_tablet_id_for_reflect,
                    GetTabletLocationsResponsePB_Error::mut_tablet_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::AppStatusPB>>(
                    "status",
                    GetTabletLocationsResponsePB_Error::get_status_for_reflect,
                    GetTabletLocationsResponsePB_Error::mut_status_for_reflect,
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

impl ::std::fmt::Debug for GetTabletLocationsResponsePB_Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTabletLocationsResponsePB_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateTableRequestPB {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    split_rows_range_bounds: ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB>,
    partition_schema: ::protobuf::SingularPtrField<super::common::PartitionSchemaPB>,
    num_replicas: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(CreateTableRequestPB::new)
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

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
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

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::SchemaPB> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::SchemaPB> {
        &mut self.schema
    }

    // optional .kudu.RowOperationsPB split_rows_range_bounds = 6;

    pub fn clear_split_rows_range_bounds(&mut self) {
        self.split_rows_range_bounds.clear();
    }

    pub fn has_split_rows_range_bounds(&self) -> bool {
        self.split_rows_range_bounds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split_rows_range_bounds(&mut self, v: super::wire_protocol::RowOperationsPB) {
        self.split_rows_range_bounds = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split_rows_range_bounds(&mut self) -> &mut super::wire_protocol::RowOperationsPB {
        if self.split_rows_range_bounds.is_none() {
            self.split_rows_range_bounds.set_default();
        };
        self.split_rows_range_bounds.as_mut().unwrap()
    }

    // Take field
    pub fn take_split_rows_range_bounds(&mut self) -> super::wire_protocol::RowOperationsPB {
        self.split_rows_range_bounds.take().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::new())
    }

    pub fn get_split_rows_range_bounds(&self) -> &super::wire_protocol::RowOperationsPB {
        self.split_rows_range_bounds.as_ref().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::default_instance())
    }

    fn get_split_rows_range_bounds_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &self.split_rows_range_bounds
    }

    fn mut_split_rows_range_bounds_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &mut self.split_rows_range_bounds
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

    fn get_partition_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &self.partition_schema
    }

    fn mut_partition_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &mut self.partition_schema
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

    fn get_num_replicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_replicas
    }

    fn mut_num_replicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_replicas
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.split_rows_range_bounds)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.split_rows_range_bounds.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.num_replicas {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.schema.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.split_rows_range_bounds.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.num_replicas {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CreateTableRequestPB::get_name_for_reflect,
                    CreateTableRequestPB::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SchemaPB>>(
                    "schema",
                    CreateTableRequestPB::get_schema_for_reflect,
                    CreateTableRequestPB::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::RowOperationsPB>>(
                    "split_rows_range_bounds",
                    CreateTableRequestPB::get_split_rows_range_bounds_for_reflect,
                    CreateTableRequestPB::mut_split_rows_range_bounds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PartitionSchemaPB>>(
                    "partition_schema",
                    CreateTableRequestPB::get_partition_schema_for_reflect,
                    CreateTableRequestPB::mut_partition_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_replicas",
                    CreateTableRequestPB::get_num_replicas_for_reflect,
                    CreateTableRequestPB::mut_num_replicas_for_reflect,
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
        self.clear_split_rows_range_bounds();
        self.clear_partition_schema();
        self.clear_num_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateTableRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(CreateTableResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_table_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.table_id
    }
}

impl ::protobuf::Message for CreateTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.table_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.table_id.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    CreateTableResponsePB::get_error_for_reflect,
                    CreateTableResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "table_id",
                    CreateTableResponsePB::get_table_id_for_reflect,
                    CreateTableResponsePB::mut_table_id_for_reflect,
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

impl ::std::fmt::Debug for CreateTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateTableResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsCreateTableDoneRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(IsCreateTableDoneRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    IsCreateTableDoneRequestPB::get_table_for_reflect,
                    IsCreateTableDoneRequestPB::mut_table_for_reflect,
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

impl ::std::fmt::Debug for IsCreateTableDoneRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsCreateTableDoneRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsCreateTableDoneResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(IsCreateTableDoneResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for IsCreateTableDoneResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.done {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    IsCreateTableDoneResponsePB::get_error_for_reflect,
                    IsCreateTableDoneResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    IsCreateTableDoneResponsePB::get_done_for_reflect,
                    IsCreateTableDoneResponsePB::mut_done_for_reflect,
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

impl ::std::fmt::Debug for IsCreateTableDoneResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsCreateTableDoneResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteTableRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(DeleteTableRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    DeleteTableRequestPB::get_table_for_reflect,
                    DeleteTableRequestPB::mut_table_for_reflect,
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

impl ::std::fmt::Debug for DeleteTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteTableRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(DeleteTableResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
    }
}

impl ::protobuf::Message for DeleteTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    DeleteTableResponsePB::get_error_for_reflect,
                    DeleteTableResponsePB::mut_error_for_reflect,
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

impl ::std::fmt::Debug for DeleteTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteTableResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTablesRequestPB {
    // message fields
    name_filter: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTablesRequestPB::new)
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

    fn get_name_filter_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name_filter
    }

    fn mut_name_filter_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name_filter
    }
}

impl ::protobuf::Message for ListTablesRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name_filter)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name_filter.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name_filter.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name_filter",
                    ListTablesRequestPB::get_name_filter_for_reflect,
                    ListTablesRequestPB::mut_name_filter_for_reflect,
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

impl ::std::fmt::Debug for ListTablesRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTablesRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTablesResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tables: ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTablesResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_tables_for_reflect(&self) -> &::protobuf::RepeatedField<ListTablesResponsePB_TableInfo> {
        &self.tables
    }

    fn mut_tables_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListTablesResponsePB_TableInfo> {
        &mut self.tables
    }
}

impl ::protobuf::Message for ListTablesResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tables {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    ListTablesResponsePB::get_error_for_reflect,
                    ListTablesResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListTablesResponsePB_TableInfo>>(
                    "tables",
                    ListTablesResponsePB::get_tables_for_reflect,
                    ListTablesResponsePB::mut_tables_for_reflect,
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

impl ::std::fmt::Debug for ListTablesResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTablesResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTablesResponsePB_TableInfo {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTablesResponsePB_TableInfo::new)
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

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
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

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    ListTablesResponsePB_TableInfo::get_id_for_reflect,
                    ListTablesResponsePB_TableInfo::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ListTablesResponsePB_TableInfo::get_name_for_reflect,
                    ListTablesResponsePB_TableInfo::mut_name_for_reflect,
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

impl ::std::fmt::Debug for ListTablesResponsePB_TableInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTablesResponsePB_TableInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTableLocationsRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    partition_key_start: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition_key_end: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    max_returned_locations: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTableLocationsRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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

    fn get_partition_key_start_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.partition_key_start
    }

    fn mut_partition_key_start_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.partition_key_start
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

    fn get_partition_key_end_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.partition_key_end
    }

    fn mut_partition_key_end_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.partition_key_end
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

    fn get_max_returned_locations_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_returned_locations
    }

    fn mut_max_returned_locations_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_returned_locations
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.partition_key_start)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.partition_key_end)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_returned_locations = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.partition_key_start.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.partition_key_end.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.max_returned_locations {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.partition_key_start.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.partition_key_end.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.max_returned_locations {
            os.write_uint32(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    GetTableLocationsRequestPB::get_table_for_reflect,
                    GetTableLocationsRequestPB::mut_table_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "partition_key_start",
                    GetTableLocationsRequestPB::get_partition_key_start_for_reflect,
                    GetTableLocationsRequestPB::mut_partition_key_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "partition_key_end",
                    GetTableLocationsRequestPB::get_partition_key_end_for_reflect,
                    GetTableLocationsRequestPB::mut_partition_key_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_returned_locations",
                    GetTableLocationsRequestPB::get_max_returned_locations_for_reflect,
                    GetTableLocationsRequestPB::mut_max_returned_locations_for_reflect,
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

impl ::std::fmt::Debug for GetTableLocationsRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTableLocationsRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTableLocationsResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    tablet_locations: ::protobuf::RepeatedField<TabletLocationsPB>,
    ttl_millis: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTableLocationsResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_tablet_locations_for_reflect(&self) -> &::protobuf::RepeatedField<TabletLocationsPB> {
        &self.tablet_locations
    }

    fn mut_tablet_locations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TabletLocationsPB> {
        &mut self.tablet_locations
    }

    // optional uint32 ttl_millis = 3;

    pub fn clear_ttl_millis(&mut self) {
        self.ttl_millis = ::std::option::Option::None;
    }

    pub fn has_ttl_millis(&self) -> bool {
        self.ttl_millis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl_millis(&mut self, v: u32) {
        self.ttl_millis = ::std::option::Option::Some(v);
    }

    pub fn get_ttl_millis(&self) -> u32 {
        self.ttl_millis.unwrap_or(36000000u32)
    }

    fn get_ttl_millis_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ttl_millis
    }

    fn mut_ttl_millis_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ttl_millis
    }
}

impl ::protobuf::Message for GetTableLocationsResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tablet_locations)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ttl_millis = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tablet_locations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.ttl_millis {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tablet_locations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.ttl_millis {
            os.write_uint32(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    GetTableLocationsResponsePB::get_error_for_reflect,
                    GetTableLocationsResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TabletLocationsPB>>(
                    "tablet_locations",
                    GetTableLocationsResponsePB::get_tablet_locations_for_reflect,
                    GetTableLocationsResponsePB::mut_tablet_locations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ttl_millis",
                    GetTableLocationsResponsePB::get_ttl_millis_for_reflect,
                    GetTableLocationsResponsePB::mut_ttl_millis_for_reflect,
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
        self.clear_ttl_millis();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTableLocationsResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTableLocationsResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    alter_schema_steps: ::protobuf::RepeatedField<AlterTableRequestPB_Step>,
    new_table_name: ::protobuf::SingularField<::std::string::String>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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

    fn get_alter_schema_steps_for_reflect(&self) -> &::protobuf::RepeatedField<AlterTableRequestPB_Step> {
        &self.alter_schema_steps
    }

    fn mut_alter_schema_steps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlterTableRequestPB_Step> {
        &mut self.alter_schema_steps
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

    fn get_new_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.new_table_name
    }

    fn mut_new_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.new_table_name
    }

    // optional .kudu.SchemaPB schema = 4;

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

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::SchemaPB> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::SchemaPB> {
        &mut self.schema
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alter_schema_steps)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_table_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alter_schema_steps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.new_table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.alter_schema_steps {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.new_table_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.schema.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    AlterTableRequestPB::get_table_for_reflect,
                    AlterTableRequestPB::mut_table_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_Step>>(
                    "alter_schema_steps",
                    AlterTableRequestPB::get_alter_schema_steps_for_reflect,
                    AlterTableRequestPB::mut_alter_schema_steps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "new_table_name",
                    AlterTableRequestPB::get_new_table_name_for_reflect,
                    AlterTableRequestPB::mut_new_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SchemaPB>>(
                    "schema",
                    AlterTableRequestPB::get_schema_for_reflect,
                    AlterTableRequestPB::mut_schema_for_reflect,
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
        self.clear_schema();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_AddColumn {
    // message fields
    schema: ::protobuf::SingularPtrField<super::common::ColumnSchemaPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableRequestPB_AddColumn::new)
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

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ColumnSchemaPB> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ColumnSchemaPB> {
        &mut self.schema
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.schema.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ColumnSchemaPB>>(
                    "schema",
                    AlterTableRequestPB_AddColumn::get_schema_for_reflect,
                    AlterTableRequestPB_AddColumn::mut_schema_for_reflect,
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

impl ::std::fmt::Debug for AlterTableRequestPB_AddColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_AddColumn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_DropColumn {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableRequestPB_DropColumn::new)
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

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    AlterTableRequestPB_DropColumn::get_name_for_reflect,
                    AlterTableRequestPB_DropColumn::mut_name_for_reflect,
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

impl ::std::fmt::Debug for AlterTableRequestPB_DropColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_DropColumn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_RenameColumn {
    // message fields
    old_name: ::protobuf::SingularField<::std::string::String>,
    new_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableRequestPB_RenameColumn::new)
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

    fn get_old_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.old_name
    }

    fn mut_old_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.old_name
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

    fn get_new_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.new_name
    }

    fn mut_new_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.new_name
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.old_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.old_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.new_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.old_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.new_name.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "old_name",
                    AlterTableRequestPB_RenameColumn::get_old_name_for_reflect,
                    AlterTableRequestPB_RenameColumn::mut_old_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "new_name",
                    AlterTableRequestPB_RenameColumn::get_new_name_for_reflect,
                    AlterTableRequestPB_RenameColumn::mut_new_name_for_reflect,
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

impl ::std::fmt::Debug for AlterTableRequestPB_RenameColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_RenameColumn {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_AddRangePartition {
    // message fields
    range_bounds: ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_AddRangePartition {}

impl AlterTableRequestPB_AddRangePartition {
    pub fn new() -> AlterTableRequestPB_AddRangePartition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_AddRangePartition {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_AddRangePartition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_AddRangePartition,
        };
        unsafe {
            instance.get(AlterTableRequestPB_AddRangePartition::new)
        }
    }

    // optional .kudu.RowOperationsPB range_bounds = 1;

    pub fn clear_range_bounds(&mut self) {
        self.range_bounds.clear();
    }

    pub fn has_range_bounds(&self) -> bool {
        self.range_bounds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_bounds(&mut self, v: super::wire_protocol::RowOperationsPB) {
        self.range_bounds = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_bounds(&mut self) -> &mut super::wire_protocol::RowOperationsPB {
        if self.range_bounds.is_none() {
            self.range_bounds.set_default();
        };
        self.range_bounds.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_bounds(&mut self) -> super::wire_protocol::RowOperationsPB {
        self.range_bounds.take().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::new())
    }

    pub fn get_range_bounds(&self) -> &super::wire_protocol::RowOperationsPB {
        self.range_bounds.as_ref().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::default_instance())
    }

    fn get_range_bounds_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &self.range_bounds
    }

    fn mut_range_bounds_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &mut self.range_bounds
    }
}

impl ::protobuf::Message for AlterTableRequestPB_AddRangePartition {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.range_bounds)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.range_bounds.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.range_bounds.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_AddRangePartition {
    fn new() -> AlterTableRequestPB_AddRangePartition {
        AlterTableRequestPB_AddRangePartition::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_AddRangePartition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::RowOperationsPB>>(
                    "range_bounds",
                    AlterTableRequestPB_AddRangePartition::get_range_bounds_for_reflect,
                    AlterTableRequestPB_AddRangePartition::mut_range_bounds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_AddRangePartition>(
                    "AlterTableRequestPB_AddRangePartition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_AddRangePartition {
    fn clear(&mut self) {
        self.clear_range_bounds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_AddRangePartition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_AddRangePartition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_DropRangePartition {
    // message fields
    range_bounds: ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlterTableRequestPB_DropRangePartition {}

impl AlterTableRequestPB_DropRangePartition {
    pub fn new() -> AlterTableRequestPB_DropRangePartition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlterTableRequestPB_DropRangePartition {
        static mut instance: ::protobuf::lazy::Lazy<AlterTableRequestPB_DropRangePartition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlterTableRequestPB_DropRangePartition,
        };
        unsafe {
            instance.get(AlterTableRequestPB_DropRangePartition::new)
        }
    }

    // optional .kudu.RowOperationsPB range_bounds = 1;

    pub fn clear_range_bounds(&mut self) {
        self.range_bounds.clear();
    }

    pub fn has_range_bounds(&self) -> bool {
        self.range_bounds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_bounds(&mut self, v: super::wire_protocol::RowOperationsPB) {
        self.range_bounds = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_bounds(&mut self) -> &mut super::wire_protocol::RowOperationsPB {
        if self.range_bounds.is_none() {
            self.range_bounds.set_default();
        };
        self.range_bounds.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_bounds(&mut self) -> super::wire_protocol::RowOperationsPB {
        self.range_bounds.take().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::new())
    }

    pub fn get_range_bounds(&self) -> &super::wire_protocol::RowOperationsPB {
        self.range_bounds.as_ref().unwrap_or_else(|| super::wire_protocol::RowOperationsPB::default_instance())
    }

    fn get_range_bounds_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &self.range_bounds
    }

    fn mut_range_bounds_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::RowOperationsPB> {
        &mut self.range_bounds
    }
}

impl ::protobuf::Message for AlterTableRequestPB_DropRangePartition {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.range_bounds)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.range_bounds.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.range_bounds.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlterTableRequestPB_DropRangePartition {
    fn new() -> AlterTableRequestPB_DropRangePartition {
        AlterTableRequestPB_DropRangePartition::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlterTableRequestPB_DropRangePartition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::RowOperationsPB>>(
                    "range_bounds",
                    AlterTableRequestPB_DropRangePartition::get_range_bounds_for_reflect,
                    AlterTableRequestPB_DropRangePartition::mut_range_bounds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlterTableRequestPB_DropRangePartition>(
                    "AlterTableRequestPB_DropRangePartition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlterTableRequestPB_DropRangePartition {
    fn clear(&mut self) {
        self.clear_range_bounds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_DropRangePartition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_DropRangePartition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableRequestPB_Step {
    // message fields
    field_type: ::std::option::Option<AlterTableRequestPB_StepType>,
    add_column: ::protobuf::SingularPtrField<AlterTableRequestPB_AddColumn>,
    drop_column: ::protobuf::SingularPtrField<AlterTableRequestPB_DropColumn>,
    rename_column: ::protobuf::SingularPtrField<AlterTableRequestPB_RenameColumn>,
    add_range_partition: ::protobuf::SingularPtrField<AlterTableRequestPB_AddRangePartition>,
    drop_range_partition: ::protobuf::SingularPtrField<AlterTableRequestPB_DropRangePartition>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableRequestPB_Step::new)
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

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<AlterTableRequestPB_StepType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<AlterTableRequestPB_StepType> {
        &mut self.field_type
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

    fn get_add_column_for_reflect(&self) -> &::protobuf::SingularPtrField<AlterTableRequestPB_AddColumn> {
        &self.add_column
    }

    fn mut_add_column_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AlterTableRequestPB_AddColumn> {
        &mut self.add_column
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

    fn get_drop_column_for_reflect(&self) -> &::protobuf::SingularPtrField<AlterTableRequestPB_DropColumn> {
        &self.drop_column
    }

    fn mut_drop_column_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AlterTableRequestPB_DropColumn> {
        &mut self.drop_column
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

    fn get_rename_column_for_reflect(&self) -> &::protobuf::SingularPtrField<AlterTableRequestPB_RenameColumn> {
        &self.rename_column
    }

    fn mut_rename_column_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AlterTableRequestPB_RenameColumn> {
        &mut self.rename_column
    }

    // optional .kudu.master.AlterTableRequestPB.AddRangePartition add_range_partition = 5;

    pub fn clear_add_range_partition(&mut self) {
        self.add_range_partition.clear();
    }

    pub fn has_add_range_partition(&self) -> bool {
        self.add_range_partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_range_partition(&mut self, v: AlterTableRequestPB_AddRangePartition) {
        self.add_range_partition = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_range_partition(&mut self) -> &mut AlterTableRequestPB_AddRangePartition {
        if self.add_range_partition.is_none() {
            self.add_range_partition.set_default();
        };
        self.add_range_partition.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_range_partition(&mut self) -> AlterTableRequestPB_AddRangePartition {
        self.add_range_partition.take().unwrap_or_else(|| AlterTableRequestPB_AddRangePartition::new())
    }

    pub fn get_add_range_partition(&self) -> &AlterTableRequestPB_AddRangePartition {
        self.add_range_partition.as_ref().unwrap_or_else(|| AlterTableRequestPB_AddRangePartition::default_instance())
    }

    fn get_add_range_partition_for_reflect(&self) -> &::protobuf::SingularPtrField<AlterTableRequestPB_AddRangePartition> {
        &self.add_range_partition
    }

    fn mut_add_range_partition_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AlterTableRequestPB_AddRangePartition> {
        &mut self.add_range_partition
    }

    // optional .kudu.master.AlterTableRequestPB.DropRangePartition drop_range_partition = 6;

    pub fn clear_drop_range_partition(&mut self) {
        self.drop_range_partition.clear();
    }

    pub fn has_drop_range_partition(&self) -> bool {
        self.drop_range_partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_drop_range_partition(&mut self, v: AlterTableRequestPB_DropRangePartition) {
        self.drop_range_partition = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_drop_range_partition(&mut self) -> &mut AlterTableRequestPB_DropRangePartition {
        if self.drop_range_partition.is_none() {
            self.drop_range_partition.set_default();
        };
        self.drop_range_partition.as_mut().unwrap()
    }

    // Take field
    pub fn take_drop_range_partition(&mut self) -> AlterTableRequestPB_DropRangePartition {
        self.drop_range_partition.take().unwrap_or_else(|| AlterTableRequestPB_DropRangePartition::new())
    }

    pub fn get_drop_range_partition(&self) -> &AlterTableRequestPB_DropRangePartition {
        self.drop_range_partition.as_ref().unwrap_or_else(|| AlterTableRequestPB_DropRangePartition::default_instance())
    }

    fn get_drop_range_partition_for_reflect(&self) -> &::protobuf::SingularPtrField<AlterTableRequestPB_DropRangePartition> {
        &self.drop_range_partition
    }

    fn mut_drop_range_partition_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AlterTableRequestPB_DropRangePartition> {
        &mut self.drop_range_partition
    }
}

impl ::protobuf::Message for AlterTableRequestPB_Step {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_column)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.drop_column)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rename_column)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_range_partition)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.drop_range_partition)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.add_column.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.drop_column.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rename_column.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.add_range_partition.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.drop_range_partition.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.add_column.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.drop_column.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rename_column.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.add_range_partition.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.drop_range_partition.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AlterTableRequestPB_StepType>>(
                    "type",
                    AlterTableRequestPB_Step::get_field_type_for_reflect,
                    AlterTableRequestPB_Step::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_AddColumn>>(
                    "add_column",
                    AlterTableRequestPB_Step::get_add_column_for_reflect,
                    AlterTableRequestPB_Step::mut_add_column_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_DropColumn>>(
                    "drop_column",
                    AlterTableRequestPB_Step::get_drop_column_for_reflect,
                    AlterTableRequestPB_Step::mut_drop_column_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_RenameColumn>>(
                    "rename_column",
                    AlterTableRequestPB_Step::get_rename_column_for_reflect,
                    AlterTableRequestPB_Step::mut_rename_column_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_AddRangePartition>>(
                    "add_range_partition",
                    AlterTableRequestPB_Step::get_add_range_partition_for_reflect,
                    AlterTableRequestPB_Step::mut_add_range_partition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlterTableRequestPB_DropRangePartition>>(
                    "drop_range_partition",
                    AlterTableRequestPB_Step::get_drop_range_partition_for_reflect,
                    AlterTableRequestPB_Step::mut_drop_range_partition_for_reflect,
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
        self.clear_add_range_partition();
        self.clear_drop_range_partition();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlterTableRequestPB_Step {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_Step {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AlterTableRequestPB_StepType {
    UNKNOWN = 0,
    ADD_COLUMN = 1,
    DROP_COLUMN = 2,
    RENAME_COLUMN = 3,
    ALTER_COLUMN = 4,
    ADD_RANGE_PARTITION = 5,
    DROP_RANGE_PARTITION = 6,
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
            5 => ::std::option::Option::Some(AlterTableRequestPB_StepType::ADD_RANGE_PARTITION),
            6 => ::std::option::Option::Some(AlterTableRequestPB_StepType::DROP_RANGE_PARTITION),
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
            AlterTableRequestPB_StepType::ADD_RANGE_PARTITION,
            AlterTableRequestPB_StepType::DROP_RANGE_PARTITION,
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

impl ::protobuf::reflect::ProtobufValue for AlterTableRequestPB_StepType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlterTableResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema_version: ::std::option::Option<u32>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(AlterTableResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_schema_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schema_version
    }

    fn mut_schema_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schema_version
    }

    // optional bytes table_id = 3;

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

    fn get_table_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.table_id
    }
}

impl ::protobuf::Message for AlterTableResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.schema_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.table_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.schema_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.table_id.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    AlterTableResponsePB::get_error_for_reflect,
                    AlterTableResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schema_version",
                    AlterTableResponsePB::get_schema_version_for_reflect,
                    AlterTableResponsePB::mut_schema_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "table_id",
                    AlterTableResponsePB::get_table_id_for_reflect,
                    AlterTableResponsePB::mut_table_id_for_reflect,
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
        self.clear_table_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlterTableResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlterTableResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsAlterTableDoneRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(IsAlterTableDoneRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    IsAlterTableDoneRequestPB::get_table_for_reflect,
                    IsAlterTableDoneRequestPB::mut_table_for_reflect,
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

impl ::std::fmt::Debug for IsAlterTableDoneRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsAlterTableDoneRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsAlterTableDoneResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema_version: ::std::option::Option<u32>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(IsAlterTableDoneResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_schema_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schema_version
    }

    fn mut_schema_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schema_version
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

    fn get_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.done
    }

    fn mut_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.done
    }
}

impl ::protobuf::Message for IsAlterTableDoneResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.schema_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.schema_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.done {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.schema_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.done {
            os.write_bool(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    IsAlterTableDoneResponsePB::get_error_for_reflect,
                    IsAlterTableDoneResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schema_version",
                    IsAlterTableDoneResponsePB::get_schema_version_for_reflect,
                    IsAlterTableDoneResponsePB::mut_schema_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "done",
                    IsAlterTableDoneResponsePB::get_done_for_reflect,
                    IsAlterTableDoneResponsePB::mut_done_for_reflect,
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

impl ::std::fmt::Debug for IsAlterTableDoneResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsAlterTableDoneResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTableSchemaRequestPB {
    // message fields
    table: ::protobuf::SingularPtrField<TableIdentifierPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTableSchemaRequestPB::new)
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

    fn get_table_for_reflect(&self) -> &::protobuf::SingularPtrField<TableIdentifierPB> {
        &self.table
    }

    fn mut_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TableIdentifierPB> {
        &mut self.table
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TableIdentifierPB>>(
                    "table",
                    GetTableSchemaRequestPB::get_table_for_reflect,
                    GetTableSchemaRequestPB::mut_table_for_reflect,
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

impl ::std::fmt::Debug for GetTableSchemaRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTableSchemaRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTableSchemaResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    schema: ::protobuf::SingularPtrField<super::common::SchemaPB>,
    partition_schema: ::protobuf::SingularPtrField<super::common::PartitionSchemaPB>,
    num_replicas: ::std::option::Option<i32>,
    table_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    create_table_done: ::std::option::Option<bool>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetTableSchemaResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::SchemaPB> {
        &self.schema
    }

    fn mut_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::SchemaPB> {
        &mut self.schema
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

    fn get_partition_schema_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &self.partition_schema
    }

    fn mut_partition_schema_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PartitionSchemaPB> {
        &mut self.partition_schema
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

    fn get_num_replicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_replicas
    }

    fn mut_num_replicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_replicas
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

    fn get_table_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.table_id
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

    fn get_create_table_done_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.create_table_done
    }

    fn mut_create_table_done_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.create_table_done
    }

    // optional string table_name = 7;

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

    fn get_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.table_name
    }

    fn mut_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.table_name
    }
}

impl ::protobuf::Message for GetTableSchemaResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.schema)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_schema)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_replicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table_id)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.create_table_done = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.num_replicas {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.table_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.create_table_done {
            my_size += 2;
        };
        if let Some(v) = self.table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.schema.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.partition_schema.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.num_replicas {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.table_id.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.create_table_done {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.table_name.as_ref() {
            os.write_string(7, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    GetTableSchemaResponsePB::get_error_for_reflect,
                    GetTableSchemaResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SchemaPB>>(
                    "schema",
                    GetTableSchemaResponsePB::get_schema_for_reflect,
                    GetTableSchemaResponsePB::mut_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PartitionSchemaPB>>(
                    "partition_schema",
                    GetTableSchemaResponsePB::get_partition_schema_for_reflect,
                    GetTableSchemaResponsePB::mut_partition_schema_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_replicas",
                    GetTableSchemaResponsePB::get_num_replicas_for_reflect,
                    GetTableSchemaResponsePB::mut_num_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "table_id",
                    GetTableSchemaResponsePB::get_table_id_for_reflect,
                    GetTableSchemaResponsePB::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "create_table_done",
                    GetTableSchemaResponsePB::get_create_table_done_for_reflect,
                    GetTableSchemaResponsePB::mut_create_table_done_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "table_name",
                    GetTableSchemaResponsePB::get_table_name_for_reflect,
                    GetTableSchemaResponsePB::mut_table_name_for_reflect,
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
        self.clear_table_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTableSchemaResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTableSchemaResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTabletServersRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTabletServersRequestPB::new)
        }
    }
}

impl ::protobuf::Message for ListTabletServersRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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

impl ::std::fmt::Debug for ListTabletServersRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTabletServersRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTabletServersResponsePB {
    // message fields
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    servers: ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTabletServersResponsePB::new)
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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

    fn get_servers_for_reflect(&self) -> &::protobuf::RepeatedField<ListTabletServersResponsePB_Entry> {
        &self.servers
    }

    fn mut_servers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ListTabletServersResponsePB_Entry> {
        &mut self.servers
    }
}

impl ::protobuf::Message for ListTabletServersResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.servers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.servers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.servers {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    ListTabletServersResponsePB::get_error_for_reflect,
                    ListTabletServersResponsePB::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListTabletServersResponsePB_Entry>>(
                    "servers",
                    ListTabletServersResponsePB::get_servers_for_reflect,
                    ListTabletServersResponsePB::mut_servers_for_reflect,
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

impl ::std::fmt::Debug for ListTabletServersResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTabletServersResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListTabletServersResponsePB_Entry {
    // message fields
    instance_id: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    registration: ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB>,
    millis_since_heartbeat: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListTabletServersResponsePB_Entry::new)
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

    fn get_instance_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &self.instance_id
    }

    fn mut_instance_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &mut self.instance_id
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

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &mut self.registration
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

    fn get_millis_since_heartbeat_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.millis_since_heartbeat
    }

    fn mut_millis_since_heartbeat_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.millis_since_heartbeat
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instance_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.millis_since_heartbeat = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.instance_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.millis_since_heartbeat {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.instance_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.registration.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.millis_since_heartbeat {
            os.write_int32(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::NodeInstancePB>>(
                    "instance_id",
                    ListTabletServersResponsePB_Entry::get_instance_id_for_reflect,
                    ListTabletServersResponsePB_Entry::mut_instance_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::ServerRegistrationPB>>(
                    "registration",
                    ListTabletServersResponsePB_Entry::get_registration_for_reflect,
                    ListTabletServersResponsePB_Entry::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "millis_since_heartbeat",
                    ListTabletServersResponsePB_Entry::get_millis_since_heartbeat_for_reflect,
                    ListTabletServersResponsePB_Entry::mut_millis_since_heartbeat_for_reflect,
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

impl ::std::fmt::Debug for ListTabletServersResponsePB_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListTabletServersResponsePB_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMasterRegistrationRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetMasterRegistrationRequestPB::new)
        }
    }
}

impl ::protobuf::Message for GetMasterRegistrationRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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

impl ::std::fmt::Debug for GetMasterRegistrationRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMasterRegistrationRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMasterRegistrationResponsePB {
    // message fields
    instance_id: ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB>,
    registration: ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB>,
    role: ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role>,
    error: ::protobuf::SingularPtrField<MasterErrorPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(GetMasterRegistrationResponsePB::new)
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

    fn get_instance_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &self.instance_id
    }

    fn mut_instance_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::NodeInstancePB> {
        &mut self.instance_id
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

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::ServerRegistrationPB> {
        &mut self.registration
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

    fn get_role_for_reflect(&self) -> &::std::option::Option<super::consensus_metadata::RaftPeerPB_Role> {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::option::Option<super::consensus_metadata::RaftPeerPB_Role> {
        &mut self.role
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<MasterErrorPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MasterErrorPB> {
        &mut self.error
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
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instance_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.role = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.instance_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.role {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.instance_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.registration.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.role {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.error.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::NodeInstancePB>>(
                    "instance_id",
                    GetMasterRegistrationResponsePB::get_instance_id_for_reflect,
                    GetMasterRegistrationResponsePB::mut_instance_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::ServerRegistrationPB>>(
                    "registration",
                    GetMasterRegistrationResponsePB::get_registration_for_reflect,
                    GetMasterRegistrationResponsePB::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::consensus_metadata::RaftPeerPB_Role>>(
                    "role",
                    GetMasterRegistrationResponsePB::get_role_for_reflect,
                    GetMasterRegistrationResponsePB::mut_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MasterErrorPB>>(
                    "error",
                    GetMasterRegistrationResponsePB::get_error_for_reflect,
                    GetMasterRegistrationResponsePB::mut_error_for_reflect,
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

impl ::std::fmt::Debug for GetMasterRegistrationResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMasterRegistrationResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListMastersRequestPB {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListMastersRequestPB::new)
        }
    }
}

impl ::protobuf::Message for ListMastersRequestPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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

impl ::std::fmt::Debug for ListMastersRequestPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListMastersRequestPB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListMastersResponsePB {
    // message fields
    masters: ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB>,
    error: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(ListMastersResponsePB::new)
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

    fn get_masters_for_reflect(&self) -> &::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB> {
        &self.masters
    }

    fn mut_masters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::wire_protocol::ServerEntryPB> {
        &mut self.masters
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

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB> {
        &mut self.error
    }
}

impl ::protobuf::Message for ListMastersResponsePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.masters)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.masters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut os: &mut Write) -> ::protobuf::ProtobufResult<()> {
        for v in &self.masters {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
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
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::ServerEntryPB>>(
                    "masters",
                    ListMastersResponsePB::get_masters_for_reflect,
                    ListMastersResponsePB::mut_masters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wire_protocol::AppStatusPB>>(
                    "error",
                    ListMastersResponsePB::get_error_for_reflect,
                    ListMastersResponsePB::mut_error_for_reflect,
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

impl ::std::fmt::Debug for ListMastersResponsePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListMastersResponsePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MasterFeatures {
    UNKNOWN_FEATURE = 0,
    RANGE_PARTITION_BOUNDS = 1,
    ADD_DROP_RANGE_PARTITIONS = 2,
}

impl ::protobuf::ProtobufEnum for MasterFeatures {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MasterFeatures> {
        match value {
            0 => ::std::option::Option::Some(MasterFeatures::UNKNOWN_FEATURE),
            1 => ::std::option::Option::Some(MasterFeatures::RANGE_PARTITION_BOUNDS),
            2 => ::std::option::Option::Some(MasterFeatures::ADD_DROP_RANGE_PARTITIONS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MasterFeatures] = &[
            MasterFeatures::UNKNOWN_FEATURE,
            MasterFeatures::RANGE_PARTITION_BOUNDS,
            MasterFeatures::ADD_DROP_RANGE_PARTITIONS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MasterFeatures>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MasterFeatures", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MasterFeatures {
}

impl ::protobuf::reflect::ProtobufValue for MasterFeatures {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
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
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x6b,
    0x75, 0x64, 0x75, 0x2f, 0x75, 0x74, 0x69, 0x6c, 0x2f, 0x70, 0x62, 0x5f, 0x75, 0x74, 0x69, 0x6c,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x86, 0x03, 0x0a, 0x0d, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x50, 0x42, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41,
    0x70, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x22, 0xa2, 0x02, 0x0a, 0x04, 0x43,
    0x6f, 0x64, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x54, 0x41,
    0x42, 0x4c, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x03, 0x12,
    0x19, 0x0a, 0x15, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59,
    0x5f, 0x50, 0x52, 0x45, 0x53, 0x45, 0x4e, 0x54, 0x10, 0x04, 0x12, 0x14, 0x0a, 0x10, 0x54, 0x4f,
    0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x54, 0x53, 0x10, 0x05,
    0x12, 0x23, 0x0a, 0x1f, 0x43, 0x41, 0x54, 0x41, 0x4c, 0x4f, 0x47, 0x5f, 0x4d, 0x41, 0x4e, 0x41,
    0x47, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x49, 0x54, 0x49, 0x41, 0x4c, 0x49,
    0x5a, 0x45, 0x44, 0x10, 0x06, 0x12, 0x12, 0x0a, 0x0e, 0x4e, 0x4f, 0x54, 0x5f, 0x54, 0x48, 0x45,
    0x5f, 0x4c, 0x45, 0x41, 0x44, 0x45, 0x52, 0x10, 0x07, 0x12, 0x1f, 0x0a, 0x1b, 0x52, 0x45, 0x50,
    0x4c, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x41, 0x43, 0x54, 0x4f, 0x52, 0x5f,
    0x54, 0x4f, 0x4f, 0x5f, 0x48, 0x49, 0x47, 0x48, 0x10, 0x08, 0x12, 0x16, 0x0a, 0x12, 0x54, 0x41,
    0x42, 0x4c, 0x45, 0x54, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47,
    0x10, 0x09, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x56, 0x45, 0x4e, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x49,
    0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x41, 0x43, 0x54, 0x4f, 0x52, 0x10, 0x0a, 0x12,
    0x1e, 0x0a, 0x1a, 0x49, 0x4c, 0x4c, 0x45, 0x47, 0x41, 0x4c, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x49,
    0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x41, 0x43, 0x54, 0x4f, 0x52, 0x10, 0x0b, 0x22,
    0x3f, 0x0a, 0x12, 0x54, 0x53, 0x54, 0x6f, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x0b, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x74,
    0x61, 0x6e, 0x63, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x42,
    0x22, 0x39, 0x0a, 0x11, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x50, 0x42, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0xf7, 0x02, 0x0a, 0x11,
    0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50,
    0x42, 0x12, 0x1c, 0x0a, 0x14, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x5f,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x1a, 0x0a, 0x12, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x5f, 0x65, 0x6e,
    0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x24, 0x0a, 0x09, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x50,
    0x42, 0x12, 0x43, 0x0a, 0x19, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x5f, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73,
    0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x50, 0x42, 0x12, 0x3c, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x24, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x07, 0x55, 0x4e, 0x4b,
    0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x6d, 0x73,
    0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x5a, 0x0a, 0x05, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x12, 0x0c, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0xe7, 0x07,
    0x12, 0x0d, 0x0a, 0x09, 0x50, 0x52, 0x45, 0x50, 0x41, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x00, 0x12,
    0x0c, 0x0a, 0x08, 0x43, 0x52, 0x45, 0x41, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x0b, 0x0a,
    0x07, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x45,
    0x50, 0x4c, 0x41, 0x43, 0x45, 0x44, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45, 0x4c, 0x45,
    0x54, 0x45, 0x44, 0x10, 0x04, 0x22, 0xfd, 0x02, 0x0a, 0x10, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x1e, 0x0a, 0x06, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x2c, 0x0a, 0x14, 0x66, 0x75, 0x6c,
    0x6c, 0x79, 0x5f, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x31, 0x0a, 0x10, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x6e, 0x65,
    0x78, 0x74, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x05, 0x12, 0x3b, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x53, 0x79, 0x73, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x07, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x6d,
    0x73, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4b, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0d,
    0x0a, 0x09, 0x50, 0x52, 0x45, 0x50, 0x41, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x0b, 0x0a,
    0x07, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x41, 0x4c,
    0x54, 0x45, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45, 0x4d, 0x4f,
    0x56, 0x45, 0x44, 0x10, 0x04, 0x22, 0x0f, 0x0a, 0x0d, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0x10, 0x0a, 0x0e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x22, 0xa6, 0x02, 0x0a, 0x10, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x50, 0x42, 0x12, 0x11, 0x0a,
    0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x32, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x50, 0x42, 0x3a, 0x07, 0x55, 0x4e, 0x4b,
    0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x4c, 0x0a, 0x11, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x64,
    0x61, 0x74, 0x61, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1c, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x13, 0x54,
    0x41, 0x42, 0x4c, 0x45, 0x54, 0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f,
    0x57, 0x4e, 0x12, 0x43, 0x0a, 0x19, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x5f,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x50, 0x42, 0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70,
    0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0d, 0x22, 0x95, 0x01, 0x0a, 0x0e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x63, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x36, 0x0a, 0x0f,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x50, 0x42, 0x12, 0x1a, 0x0a, 0x12, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x5f,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c,
    0x12, 0x17, 0x0a, 0x0f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x22, 0x3f, 0x0a, 0x17, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x73, 0x50, 0x42, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x5f, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x4e, 0x0a, 0x15, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x50, 0x42, 0x12, 0x35, 0x0a, 0x07, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x50, 0x42, 0x22, 0xc7, 0x01, 0x0a, 0x14, 0x54,
    0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x12, 0x2f, 0x0a, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x54, 0x53, 0x54, 0x6f, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x50, 0x42, 0x12, 0x30, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x32, 0x0a, 0x0d, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x12, 0x18, 0x0a, 0x10, 0x6e, 0x75,
    0x6d, 0x5f, 0x6c, 0x69, 0x76, 0x65, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x22, 0x8d, 0x02, 0x0a, 0x15, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74,
    0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x0f, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49, 0x6e,
    0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x42, 0x12, 0x1f, 0x0a, 0x10, 0x6e, 0x65, 0x65, 0x64,
    0x73, 0x5f, 0x72, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x18, 0x6e, 0x65, 0x65,
    0x64, 0x73, 0x5f, 0x66, 0x75, 0x6c, 0x6c, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c,
    0x73, 0x65, 0x12, 0x39, 0x0a, 0x0d, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x50, 0x42, 0x12, 0x15, 0x0a,
    0x0d, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x08, 0x22, 0xaa, 0x02, 0x0a, 0x11, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a,
    0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x24, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x3a, 0x0a, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x50, 0x42, 0x12, 0x18, 0x0a, 0x10, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45,
    0x44, 0x5f, 0x73, 0x74, 0x61, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x1a, 0x62, 0x0a,
    0x09, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x50, 0x42, 0x12, 0x26, 0x0a, 0x07, 0x74, 0x73,
    0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x49, 0x6e, 0x66, 0x6f,
    0x50, 0x42, 0x12, 0x2d, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x1f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75,
    0x73, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x50, 0x65, 0x65, 0x72, 0x50, 0x42, 0x2e, 0x52, 0x6f, 0x6c,
    0x65, 0x22, 0x4b, 0x0a, 0x08, 0x54, 0x53, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x42, 0x12, 0x16, 0x0a,
    0x0e, 0x70, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x27, 0x0a, 0x0d, 0x72, 0x70, 0x63, 0x5f, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x6f, 0x72, 0x74, 0x50, 0x42, 0x22, 0x31,
    0x0a, 0x1b, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x12, 0x0a,
    0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0c, 0x22, 0x83, 0x02, 0x0a, 0x1c, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x38, 0x0a,
    0x10, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x3f, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x42, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x1a, 0x3d, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x21, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x22, 0xc5, 0x01, 0x0a, 0x14, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1e,
    0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x36,
    0x0a, 0x17, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x72, 0x6f, 0x77, 0x73, 0x5f, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x15, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x52, 0x6f, 0x77, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x12, 0x31, 0x0a, 0x10, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x75, 0x6d,
    0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x54, 0x0a, 0x15, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x50, 0x42, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4b, 0x0a, 0x1a, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x50, 0x42, 0x22, 0x56, 0x0a, 0x1b, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50,
    0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x0c, 0x0a, 0x04,
    0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x45, 0x0a, 0x14, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50,
    0x42, 0x22, 0x42, 0x0a, 0x15, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x50, 0x42, 0x22, 0x2a, 0x0a, 0x13, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x13, 0x0a, 0x0b,
    0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x22, 0xa5, 0x01, 0x0a, 0x14, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x3b, 0x0a, 0x06, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x1a, 0x25, 0x0a, 0x09, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0xb3, 0x01, 0x0a, 0x1a, 0x47, 0x65,
    0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x12, 0x21, 0x0a, 0x13, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x42, 0x04, 0x88, 0xb5, 0x18, 0x01, 0x12, 0x1f, 0x0a, 0x11, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x65, 0x6e, 0x64, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x04, 0x88, 0xb5, 0x18, 0x01, 0x12, 0x22, 0x0a, 0x16, 0x6d,
    0x61, 0x78, 0x5f, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x02, 0x31, 0x30, 0x22,
    0xa0, 0x01, 0x0a, 0x1b, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12,
    0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x38, 0x0a, 0x10, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x50, 0x42, 0x12, 0x1c, 0x0a, 0x0a, 0x74, 0x74, 0x6c, 0x5f, 0x6d, 0x69, 0x6c, 0x6c,
    0x69, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x08, 0x33, 0x36, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x22, 0x91, 0x08, 0x0a, 0x13, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x12, 0x41, 0x0a, 0x12, 0x61, 0x6c, 0x74,
    0x65, 0x72, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x5f, 0x73, 0x74, 0x65, 0x70, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x12, 0x16, 0x0a, 0x0e,
    0x6e, 0x65, 0x77, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x1e, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x50, 0x42, 0x1a, 0x31, 0x0a, 0x09, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x12, 0x24, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x53,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42, 0x1a, 0x1a, 0x0a, 0x0a, 0x44, 0x72, 0x6f, 0x70, 0x43,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x09, 0x1a, 0x32, 0x0a, 0x0c, 0x52, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x43, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6e, 0x65, 0x77, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x1a, 0x40, 0x0a, 0x11, 0x41, 0x64, 0x64, 0x52, 0x61,
    0x6e, 0x67, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a, 0x0c,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x52, 0x6f, 0x77, 0x4f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x1a, 0x41, 0x0a, 0x12, 0x44, 0x72, 0x6f,
    0x70, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x2b, 0x0a, 0x0c, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x52, 0x6f, 0x77,
    0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x42, 0x1a, 0xb4, 0x03, 0x0a,
    0x04, 0x53, 0x74, 0x65, 0x70, 0x12, 0x40, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x29, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x07,
    0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x3e, 0x0a, 0x0a, 0x61, 0x64, 0x64, 0x5f, 0x63,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x41, 0x64,
    0x64, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x40, 0x0a, 0x0b, 0x64, 0x72, 0x6f, 0x70, 0x5f,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x44,
    0x72, 0x6f, 0x70, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12, 0x44, 0x0a, 0x0d, 0x72, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x5f, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2d, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41,
    0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x42, 0x2e, 0x52, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x12,
    0x4f, 0x0a, 0x13, 0x61, 0x64, 0x64, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70, 0x61, 0x72,
    0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x2e, 0x41,
    0x64, 0x64, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x51, 0x0a, 0x14, 0x64, 0x72, 0x6f, 0x70, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74,
    0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x2e, 0x44, 0x72, 0x6f, 0x70, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0x90, 0x01, 0x0a, 0x08, 0x53, 0x74, 0x65, 0x70, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0e, 0x0a,
    0x0a, 0x41, 0x44, 0x44, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10, 0x01, 0x12, 0x0f, 0x0a,
    0x0b, 0x44, 0x52, 0x4f, 0x50, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10, 0x02, 0x12, 0x11,
    0x0a, 0x0d, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x10,
    0x03, 0x12, 0x10, 0x0a, 0x0c, 0x41, 0x4c, 0x54, 0x45, 0x52, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d,
    0x4e, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x41, 0x44, 0x44, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45,
    0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x05, 0x12, 0x18, 0x0a, 0x14,
    0x44, 0x52, 0x4f, 0x50, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49,
    0x54, 0x49, 0x4f, 0x4e, 0x10, 0x06, 0x22, 0x6b, 0x0a, 0x14, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x4a, 0x0a, 0x19, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x22,
    0x6d, 0x0a, 0x1a, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44,
    0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x48,
    0x0a, 0x17, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x12, 0x2d, 0x0a, 0x05, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x50, 0x42, 0x22, 0xef, 0x01, 0x0a, 0x18, 0x47, 0x65, 0x74,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42,
    0x12, 0x1e, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x50, 0x42,
    0x12, 0x31, 0x0a, 0x10, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x63,
    0x68, 0x65, 0x6d, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x50, 0x42, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x63,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x6f, 0x6e, 0x65,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x22, 0x1c, 0x0a, 0x1a, 0x4c, 0x69,
    0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0x90, 0x02, 0x0a, 0x1b, 0x4c, 0x69, 0x73,
    0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x29, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x50, 0x42, 0x12, 0x3f, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x2e, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x1a, 0x84, 0x01, 0x0a, 0x05, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x29,
    0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x49,
    0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x42, 0x12, 0x30, 0x0a, 0x0c, 0x72, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x1e, 0x0a, 0x16, 0x6d,
    0x69, 0x6c, 0x6c, 0x69, 0x73, 0x5f, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x72,
    0x74, 0x62, 0x65, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x20, 0x0a, 0x1e, 0x47,
    0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x22, 0xd8, 0x01,
    0x0a, 0x1f, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50,
    0x42, 0x12, 0x29, 0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x4e, 0x6f,
    0x64, 0x65, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x50, 0x42, 0x12, 0x30, 0x0a, 0x0c,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x2d,
    0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x52, 0x61,
    0x66, 0x74, 0x50, 0x65, 0x65, 0x72, 0x50, 0x42, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x12, 0x29, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x22, 0x16, 0x0a, 0x14, 0x4c, 0x69, 0x73, 0x74,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42,
    0x22, 0x5f, 0x0a, 0x15, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x24, 0x0a, 0x07, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x42, 0x12,
    0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50,
    0x42, 0x2a, 0x60, 0x0a, 0x0e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x46, 0x65, 0x61, 0x74, 0x75,
    0x72, 0x65, 0x73, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x46,
    0x45, 0x41, 0x54, 0x55, 0x52, 0x45, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x16, 0x52, 0x41, 0x4e, 0x47,
    0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x42, 0x4f, 0x55, 0x4e,
    0x44, 0x53, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x41, 0x44, 0x44, 0x5f, 0x44, 0x52, 0x4f, 0x50,
    0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x54, 0x49, 0x4f, 0x4e,
    0x53, 0x10, 0x02, 0x32, 0xa9, 0x0a, 0x0a, 0x0d, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x3f, 0x0a, 0x04, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x1a, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x1b, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a, 0x0b, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72,
    0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x54, 0x53, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65,
    0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x69, 0x0a, 0x12,
    0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x12, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x29, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a, 0x0b, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66, 0x0a,
    0x11, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f,
    0x6e, 0x65, 0x12, 0x27, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f,
    0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x73, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a, 0x0b, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x12, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x51, 0x0a, 0x0a, 0x41,
    0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x21, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x63,
    0x0a, 0x10, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f,
    0x6e, 0x65, 0x12, 0x26, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x27, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x73, 0x41, 0x6c, 0x74, 0x65, 0x72,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x42, 0x12, 0x51, 0x0a, 0x0a, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x73, 0x12, 0x20, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x42, 0x1a, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x27, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x5d,
    0x0a, 0x0e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x12, 0x24, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47,
    0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x25, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x53, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x66, 0x0a,
    0x11, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x73, 0x12, 0x27, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x28, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x54, 0x0a, 0x0b, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x73, 0x12, 0x21, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x42, 0x1a, 0x22, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x12, 0x72, 0x0a, 0x15, 0x47,
    0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x42, 0x1a, 0x2c, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x47, 0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x42, 0x42,
    0x18, 0x0a, 0x16, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x4a, 0xe4, 0xb5, 0x01, 0x0a, 0x07, 0x12,
    0x05, 0x10, 0x00, 0xef, 0x04, 0x01, 0x0a, 0x8c, 0x06, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x10, 0x08,
    0x13, 0x1a, 0x81, 0x06, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x53, 0x6f, 0x66, 0x74,
    0x77, 0x61, 0x72, 0x65, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x28, 0x41, 0x53, 0x46, 0x29, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x65, 0x0a,
    0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x6f, 0x72, 0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x67, 0x72,
    0x65, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x4e, 0x4f, 0x54, 0x49, 0x43, 0x45, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x20, 0x64,
    0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x64,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x72, 0x65, 0x67, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x41, 0x53, 0x46, 0x20, 0x6c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x0a, 0x20, 0x74, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32, 0x2e, 0x30, 0x20, 0x28,
    0x74, 0x68, 0x65, 0x0a, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x22, 0x29, 0x3b,
    0x20, 0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70,
    0x74, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61, 0x6e, 0x63, 0x65, 0x0a,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x2e, 0x20, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f, 0x62, 0x74, 0x61,
    0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a, 0x20, 0x20, 0x20,
    0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68,
    0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c,
    0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c,
    0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f,
    0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77,
    0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x0a, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72,
    0x65, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x75, 0x6e,
    0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f,
    0x6e, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53, 0x22, 0x20, 0x42, 0x41,
    0x53, 0x49, 0x53, 0x2c, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f, 0x55, 0x54, 0x20, 0x57, 0x41, 0x52,
    0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49,
    0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41, 0x4e, 0x59, 0x0a, 0x20, 0x4b, 0x49,
    0x4e, 0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x20, 0x20,
    0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72,
    0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x0a, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x12, 0x00, 0x2f, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x12, 0x00, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x12, 0x16, 0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x14,
    0x07, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x07, 0x28, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x16, 0x07, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x17, 0x07, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x18, 0x07, 0x20, 0x0a, 0x37,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x00, 0x51, 0x01, 0x1a, 0x2b, 0x20, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x73, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x1f, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x20, 0x02, 0x48,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x20, 0x07, 0x0b, 0x0a,
    0xdb, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x26, 0x04, 0x16, 0x1a,
    0xcb, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x68, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64,
    0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x27, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x27, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x72, 0x65, 0x76, 0x65, 0x61, 0x6c, 0x20,
    0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x20,
    0x52, 0x50, 0x43, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x76, 0x6f, 0x69,
    0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x2c, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x73, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x61, 0x73, 0x69, 0x6c, 0x79, 0x20, 0x70, 0x61, 0x72, 0x73,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x04, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x26, 0x14, 0x15, 0x0a, 0x47, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x17, 0x1a, 0x38, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2d, 0x66, 0x6f,
    0x72, 0x6d, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x29, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x29, 0x15, 0x16, 0x0a, 0x33, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x2c, 0x04, 0x18, 0x1a, 0x24, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x64, 0x6f, 0x65, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x16, 0x17, 0x0a, 0x43, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x04, 0x1e, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x04, 0x19,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x2f, 0x1c, 0x1d,
    0x0a, 0x5a, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x32, 0x04, 0x19, 0x1a,
    0x4b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65,
    0x72, 0x20, 0x54, 0x53, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x32, 0x17, 0x18, 0x0a, 0x38, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x35, 0x04, 0x28, 0x1a, 0x29, 0x20, 0x43, 0x61,
    0x74, 0x61, 0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x20, 0x69, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x35, 0x04, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05,
    0x02, 0x12, 0x03, 0x35, 0x26, 0x27, 0x0a, 0x94, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x3a, 0x04, 0x17, 0x1a, 0x84, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x65,
    0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e,
    0x76, 0x6f, 0x6b, 0x65, 0x64, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x65, 0x69,
    0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6e, 0x6f, 0x6e,
    0x2d, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x20, 0x69, 0x73, 0x6e, 0x27, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x12, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x3a, 0x15, 0x16, 0x0a, 0x87, 0x01,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x3e, 0x04, 0x24, 0x1a, 0x78, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64,
    0x20, 0x69, 0x73, 0x20, 0x67, 0x72, 0x65, 0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x6c,
    0x69, 0x76, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x0a, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x61,
    0x78, 0x69, 0x6d, 0x75, 0x6d, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x07, 0x02, 0x12, 0x03, 0x3e, 0x22, 0x23, 0x0a, 0x54, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x08, 0x12, 0x03, 0x41, 0x04, 0x1b, 0x1a, 0x45, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x20, 0x61, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x79, 0x65, 0x74, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x41, 0x04, 0x16, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x41, 0x19, 0x1a, 0x0a, 0x3a, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x44, 0x04, 0x21, 0x1a, 0x2b, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70,
    0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20,
    0x69, 0x73, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x44, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x44, 0x1e, 0x20, 0x0a, 0x4f, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x0a, 0x12, 0x03, 0x47, 0x04, 0x24, 0x1a, 0x40, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e,
    0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x69, 0x73, 0x20, 0x69,
    0x6c, 0x6c, 0x65, 0x67, 0x61, 0x6c, 0x20, 0x28, 0x65, 0x67, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x70,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x76, 0x65, 0x29, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x47, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x47, 0x21, 0x23, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x19, 0x1a, 0x11, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x4b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4b, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b,
    0x17, 0x18, 0x0a, 0xb7, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x50, 0x02, 0x22,
    0x1a, 0xa9, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x69,
    0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x61, 0x20, 0x74, 0x65, 0x78, 0x74, 0x75, 0x61, 0x6c,
    0x0a, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75,
    0x6c, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20,
    0x6c, 0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2c, 0x20, 0x65, 0x74,
    0x63, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x69, 0x74, 0x73, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x73,
    0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x50, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x50, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x50, 0x20, 0x21, 0x0a, 0x5f, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x55, 0x00, 0x58, 0x01,
    0x1a, 0x53, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x55, 0x08,
    0x1a, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x57, 0x02, 0x2a, 0x1a, 0x3a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68,
    0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x57, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x57, 0x1a, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57,
    0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x5a, 0x00, 0x60, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x19, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x1e, 0x1a, 0x1d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20,
    0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5c,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x11, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x1c, 0x1d, 0x0a, 0x2c,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x02, 0x21, 0x1a, 0x1f, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x66, 0x65, 0x74, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x5f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x5f, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x5f, 0x1f, 0x20, 0x0a, 0x63, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x05, 0x68, 0x00, 0x82, 0x01,
    0x01, 0x1a, 0x56, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x6e, 0x2d, 0x64, 0x69, 0x73, 0x6b, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73,
    0x2e, 0x63, 0x61, 0x74, 0x61, 0x6c, 0x6f, 0x67, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x28,
    0x22, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x29, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x68, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x69,
    0x02, 0x70, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x69, 0x07,
    0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x04, 0x12,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x04, 0x0b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x6a, 0x0e, 0x11,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x04, 0x12, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x04, 0x0d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x6b, 0x10, 0x11, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x04, 0x11, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x0f, 0x10, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x6d, 0x04, 0x10, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6d, 0x04, 0x0b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x6d, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x6e, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x6f, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x6f, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x6f, 0x0e, 0x0f, 0x0a, 0x33, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x73, 0x02, 0x2a, 0x1a, 0x26, 0x20, 0x44, 0x45, 0x50, 0x52, 0x45,
    0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x27, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x27, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x73, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x74, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x74, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x74, 0x11, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x74, 0x28, 0x29, 0x0a, 0x20, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x77, 0x02, 0x25, 0x1a, 0x13, 0x20, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x77, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x77, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x77, 0x23, 0x24, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12,
    0x03, 0x7a, 0x02, 0x44, 0x1a, 0x46, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73,
    0x74, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x7a, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x7a, 0x26, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x7a, 0x42, 0x43, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x7d, 0x02,
    0x31, 0x1a, 0x1d, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x7d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x7d, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x08, 0x12, 0x03, 0x7d, 0x1b, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x07, 0x12,
    0x03, 0x7d, 0x27, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x7e, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7e, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7e, 0x1d, 0x1e, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x06, 0x12, 0x04, 0x81, 0x01, 0x02, 0x1e, 0x1a, 0x1e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x69, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12,
    0x04, 0x81, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x81, 0x01, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x04, 0x81,
    0x01, 0x1c, 0x1d, 0x0a, 0x63, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x06, 0x86, 0x01, 0x00, 0xac, 0x01,
    0x01, 0x1a, 0x55, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x6e, 0x2d, 0x64, 0x69, 0x73, 0x6b, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73,
    0x2e, 0x63, 0x61, 0x74, 0x61, 0x6c, 0x6f, 0x67, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x28,
    0x22, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x29, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x04, 0x86, 0x01, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x06, 0x87,
    0x01, 0x02, 0x8d, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x04,
    0x87, 0x01, 0x07, 0x0c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x88, 0x01, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x88, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x88, 0x01, 0x0e, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x89, 0x01, 0x04, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x89, 0x01, 0x04, 0x0d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x04, 0x89, 0x01, 0x10, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x0e, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x0f, 0x10, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x0e, 0x0f, 0x0a, 0x1a,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x02, 0x1a, 0x1a, 0x0c, 0x20, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x90, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x90, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x90, 0x01, 0x18, 0x19, 0x0a, 0x78, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x04,
    0x94, 0x01, 0x02, 0x1e, 0x1a, 0x6a, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x2d,
    0x69, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x64,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2d, 0x72, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x22, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x2d, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x22,
    0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1c, 0x1d, 0x0a, 0x47, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x02, 0x12, 0x04, 0x97, 0x01, 0x02, 0x1f, 0x1a, 0x39, 0x20, 0x4e, 0x65, 0x77,
    0x65, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x20, 0x28, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x54, 0x53, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
    0x69, 0x74, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x97, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x04, 0x97,
    0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97, 0x01,
    0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x04, 0x97, 0x01, 0x1d,
    0x1e, 0x0a, 0xc5, 0x01, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x2d,
    0x1a, 0xb6, 0x01, 0x20, 0x4c, 0x61, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x67, 0x75,
    0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x76, 0x65,
    0x20, 0x72, 0x65, 0x61, 0x63, 0x68, 0x65, 0x64, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x54,
    0x53, 0x2c, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68, 0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e,
    0x65, 0x63, 0x65, 0x73, 0x73, 0x61, 0x72, 0x69, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e,
    0x65, 0x77, 0x65, 0x73, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a, 0x0a, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x2d, 0x3e, 0x47, 0x65, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x28, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x06, 0x12, 0x04, 0x9d, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x9d, 0x01, 0x14, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x9d, 0x01, 0x2b, 0x2c, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x04, 0xa0,
    0x01, 0x02, 0x32, 0x1a, 0x22, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x27,
    0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xa0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06, 0x12,
    0x04, 0xa0, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xa0, 0x01, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa0,
    0x01, 0x30, 0x31, 0x0a, 0x72, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x02,
    0x24, 0x1a, 0x64, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x63, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x6e, 0x65, 0x77, 0x6c, 0x79, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x49, 0x44, 0x20,
    0x72, 0x65, 0x75, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xa4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xa4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xa4, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa4,
    0x01, 0x22, 0x23, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x04, 0xa7, 0x01, 0x02,
    0x22, 0x1a, 0x17, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x54, 0x53,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x06, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x06, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xa7, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03,
    0x12, 0x04, 0xa7, 0x01, 0x20, 0x21, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x04,
    0xaa, 0x01, 0x02, 0x31, 0x1a, 0x1c, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x19, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x08, 0x12, 0x04, 0xaa, 0x01, 0x1b, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x07, 0x12, 0x04, 0xaa, 0x01, 0x27, 0x2e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x08, 0x12, 0x04, 0xab, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x08, 0x04, 0x12, 0x04, 0xab, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x05, 0x12, 0x04, 0xab, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xab, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xab, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06, 0xb2, 0x01,
    0x00, 0xb3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x08,
    0x15, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x06, 0xb5, 0x01, 0x00, 0xb6, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x06, 0xb8, 0x01, 0x00, 0xc4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07,
    0x01, 0x12, 0x04, 0xb8, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x04, 0xb9, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xb9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb9,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x01,
    0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x1d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x02, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0xba, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x04, 0xba, 0x01, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba, 0x01, 0x20, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x01, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x08, 0x12, 0x04, 0xba, 0x01, 0x2a, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x07, 0x12, 0x04, 0xba, 0x01, 0x36, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x02, 0x12, 0x04, 0xbb, 0x01, 0x02, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xbb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12,
    0x04, 0xbb, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xbb, 0x01, 0x22, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbb,
    0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x08, 0x12, 0x04, 0xbb, 0x01,
    0x38, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x07, 0x12, 0x04, 0xbb, 0x01, 0x44,
    0x57, 0x0a, 0x96, 0x01, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x44,
    0x1a, 0x87, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x5f,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x5f, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x73, 0x75, 0x73, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6e,
    0x67, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x52, 0x55, 0x4e, 0x4e,
    0x49, 0x4e, 0x47, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x0a, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e,
    0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x42, 0x4f, 0x4f, 0x54, 0x53, 0x54,
    0x52, 0x41, 0x50, 0x50, 0x49, 0x4e, 0x47, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x06, 0x12, 0x04, 0xc0, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xc0, 0x01, 0x26, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xc0, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x04,
    0xc2, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0xc2,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x04, 0xc2, 0x01,
    0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x17,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x23, 0x24, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x06, 0xc7, 0x01, 0x00, 0xdc, 0x01, 0x01, 0x1a, 0x4b, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x73, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x54, 0x53, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0xc7, 0x01,
    0x08, 0x16, 0x0a, 0x87, 0x01, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0xca, 0x01, 0x02,
    0x23, 0x1a, 0x79, 0x20, 0x49, 0x66, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x66, 0x75, 0x6c,
    0x6c, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e,
    0x79, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x0a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x73, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xca, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0xca, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x01, 0x10, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xca, 0x01, 0x21, 0x22, 0x0a, 0x94, 0x02, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x30, 0x1a, 0x85, 0x02, 0x20, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x27, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x63, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x27, 0x20, 0x69, 0x73, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65,
    0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x77, 0x61, 0x72,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x75, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x62, 0x65, 0x20,
    0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
    0x62, 0x65, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x1c, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x2e, 0x2f, 0x0a, 0xa8, 0x01, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0xd5, 0x01, 0x02, 0x28, 0x1a, 0x99, 0x01, 0x20, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x20, 0x49, 0x44, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x68, 0x61, 0x73, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x65,
    0x72, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x65, 0x64,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x20, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x62,
    0x65, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x6e,
    0x2d, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x0a, 0x20, 0x72, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xd5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xd5, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd5,
    0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x01,
    0x26, 0x27, 0x0a, 0x8c, 0x02, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x02,
    0x25, 0x1a, 0xfd, 0x01, 0x20, 0x45, 0x76, 0x65, 0x72, 0x79, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x54, 0x53, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x73,
    0x20, 0x61, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x2c, 0x20, 0x69, 0x74, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x66, 0x75, 0x6c, 0x20, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67, 0x69, 0x6e, 0x67,
    0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79,
    0x65, 0x74, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x28, 0x6e, 0x6f, 0x6e, 0x2d, 0x69, 0x6e,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x29, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x30, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x11, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x09, 0x12, 0x06, 0xde, 0x01, 0x00, 0xe1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x04, 0xde, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x04, 0xdf, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xdf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xdf, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf,
    0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdf, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x1e, 0x1f, 0x0a, 0x5f, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x06, 0xe4, 0x01, 0x00, 0xe6, 0x01, 0x01, 0x1a, 0x51, 0x20, 0x53, 0x65, 0x6e,
    0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20,
    0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x54, 0x53, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x28, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x73, 0x29, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x04, 0xe5, 0x01, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xe5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xe5, 0x01, 0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xe5, 0x01, 0x23, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xe5, 0x01, 0x2d, 0x2e, 0x0a, 0x7e, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xea, 0x01, 0x00, 0xfd,
    0x01, 0x01, 0x1a, 0x70, 0x20, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x2d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x73, 0x74,
    0x61, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x20, 0x6c, 0x69, 0x76, 0x65, 0x6e, 0x65, 0x73, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20,
    0x61, 0x6e, 0x79, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xea, 0x01, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xeb, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x1e, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x27, 0x28, 0x0a, 0x6c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x01, 0x12, 0x04, 0xef, 0x01, 0x02, 0x31, 0x1a, 0x5e, 0x20, 0x53, 0x65, 0x6e, 0x74,
    0x20, 0x75, 0x70, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x2d, 0x75, 0x70, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x53, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x27, 0x6e, 0x65, 0x65,
    0x64, 0x73, 0x5f, 0x72, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x27, 0x20, 0x6f,
    0x6e, 0x20, 0x61, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x0a, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xef, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xef, 0x01, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xef, 0x01, 0x20, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xef, 0x01, 0x2f, 0x30, 0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xf3,
    0x01, 0x02, 0x2c, 0x1a, 0x5e, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x64, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x27, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x66, 0x75,
    0x6c, 0x6c, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x27, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x0b,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x1a, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x2a, 0x2b, 0x0a,
    0x92, 0x01, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x26, 0x1a, 0x83,
    0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x42, 0x4f, 0x4f, 0x54, 0x53, 0x54, 0x52, 0x41, 0x50, 0x50, 0x49, 0x4e, 0x47, 0x20, 0x6f,
    0x72, 0x20, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x2e, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74,
    0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x6c, 0x6f, 0x61, 0x64,
    0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xfc,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xfc, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x11,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xff, 0x01, 0x00, 0x94, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xff, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x00, 0x12, 0x04, 0x80, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x80, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x80, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x80, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x80, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x86,
    0x02, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x86, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0x86, 0x02, 0x0b,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x86, 0x02, 0x1a, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x86, 0x02, 0x2c, 0x2d, 0x0a,
    0x9e, 0x01, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x39, 0x1a, 0x8f,
    0x01, 0x20, 0x49, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6e, 0x65,
    0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x2d, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x65, 0x72, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x20, 0x2d, 0x2d, 0x20, 0x69, 0x2e, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20,
    0x61, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x27, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x0b, 0x0f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x10, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x08, 0x12, 0x04, 0x8b, 0x02, 0x25, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x07, 0x12, 0x04, 0x8b, 0x02, 0x31, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x8d, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x8d, 0x02, 0x10, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x8d, 0x02, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x08, 0x12, 0x04, 0x8d,
    0x02, 0x2d, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x07, 0x12, 0x04, 0x8d, 0x02,
    0x39, 0x3e, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0x90, 0x02, 0x02, 0x33,
    0x1a, 0x2e, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x73,
    0x20, 0x61, 0x20, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x04, 0x90, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x06, 0x12, 0x04, 0x90, 0x02, 0x0b, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x90, 0x02, 0x21, 0x2e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0x90, 0x02, 0x31, 0x32, 0x0a, 0x45, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x05, 0x12, 0x04, 0x93, 0x02, 0x02, 0x22, 0x1a, 0x37, 0x20, 0x53, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x79, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x04, 0x93, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05, 0x12, 0x04, 0x93, 0x02, 0x0b,
    0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x04, 0x93, 0x02, 0x10, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x04, 0x93, 0x02, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x9a, 0x02, 0x00, 0xac, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0d,
    0x03, 0x00, 0x12, 0x06, 0x9b, 0x02, 0x02, 0x9e, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x03, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x9c, 0x02, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9c, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9c, 0x02, 0x0d, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0d, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x16, 0x1d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x20, 0x21, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x30, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x0c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9d, 0x02, 0x0d, 0x26, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x27, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x2e,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x1d, 0x1e, 0x0a, 0x1b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x02, 0x1f, 0x1a, 0x0d, 0x20, 0x44, 0x45, 0x50, 0x52,
    0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa3, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa3, 0x02, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa3, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x02,
    0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x1b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa6, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04,
    0x12, 0x04, 0xa8, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x04, 0x12,
    0x04, 0xa8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06, 0x12, 0x04,
    0xa8, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa8,
    0x02, 0x15, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa8, 0x02,
    0x20, 0x21, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04, 0xab, 0x02, 0x02, 0x25,
    0x1a, 0x45, 0x20, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x20, 0x53,
    0x74, 0x69, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x73, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xab, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xab, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xab, 0x02, 0x10, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xab,
    0x02, 0x23, 0x24, 0x0a, 0xb1, 0x02, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xb3, 0x02, 0x00, 0xb7,
    0x02, 0x01, 0x1a, 0xa2, 0x02, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74,
    0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x65, 0x61, 0x63, 0x68, 0x20, 0x54, 0x53, 0x20, 0x55, 0x55, 0x49, 0x44, 0x20, 0x69, 0x73, 0x20,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x6e, 0x0a, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x77, 0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x61,
    0x6c, 0x73, 0x6f, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x6d, 0x69, 0x74, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2c, 0x0a, 0x20, 0x6c, 0x6f, 0x61,
    0x64, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2c, 0x20, 0x74, 0x6f, 0x70, 0x6f, 0x6c, 0x6f, 0x67, 0x79,
    0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04,
    0xb3, 0x02, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x02,
    0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb4, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb4, 0x02, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x11, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb4, 0x02, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb6, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb6, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06,
    0xb9, 0x02, 0x00, 0xbc, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xb9,
    0x02, 0x08, 0x23, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x02, 0x02,
    0x20, 0x1a, 0x2b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x49,
    0x44, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74,
    0x6f, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbb, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10,
    0x12, 0x06, 0xbe, 0x02, 0x00, 0xc8, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12,
    0x04, 0xbe, 0x02, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xbf,
    0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbf, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbf, 0x02, 0x0b,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x19, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x02, 0x32, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc1, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03,
    0x00, 0x12, 0x06, 0xc3, 0x02, 0x02, 0xc6, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03,
    0x00, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x0a, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc4, 0x02, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc4, 0x02, 0x13, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x10, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc4, 0x02, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc5, 0x02, 0x0d, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xc7, 0x02, 0x02, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x02, 0x06, 0x12, 0x04, 0xc7, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x1a, 0x1b, 0x0a, 0xb5, 0x01, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x06, 0xcd, 0x02, 0x00, 0xd6, 0x02, 0x01, 0x1a, 0xa6, 0x01, 0x20, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x0a, 0x20, 0x20, 0x43, 0x61, 0x74, 0x61,
    0x6c, 0x6f, 0x67, 0x0a, 0x20, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d, 0x3d,
    0x3d, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x1c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xce, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xce, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xce, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xce, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xcf, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xcf, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xcf, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xcf, 0x02, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcf,
    0x02, 0x1d, 0x1e, 0x0a, 0xa3, 0x01, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x02,
    0x02, 0x37, 0x1a, 0x94, 0x01, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x20, 0x70, 0x72, 0x65, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x73, 0x20, 0x3d, 0x20, 0x33, 0x3b, 0x0a, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x52, 0x6f, 0x77, 0x50, 0x42, 0x20,
    0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x72, 0x6f, 0x77, 0x73, 0x20, 0x3d, 0x20, 0x35, 0x3b, 0x0a,
    0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x72, 0x6f, 0x77, 0x73, 0x20, 0x6f, 0x72, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x73,
    0x20, 0x28, 0x6f, 0x72, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xd3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xd3, 0x02, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xd3, 0x02, 0x1b, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xd3, 0x02, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0xd4,
    0x02, 0x02, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd4, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x06, 0x12, 0x04, 0xd4, 0x02, 0x0b,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x1d, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4, 0x02, 0x30, 0x31, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd5, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x02, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12,
    0x06, 0xd8, 0x02, 0x00, 0xdd, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04,
    0xd8, 0x02, 0x08, 0x1d, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xda, 0x02,
    0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xda, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xda, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xda, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xda, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xdc, 0x02,
    0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xdc, 0x02, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x11, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdc, 0x02, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xdf, 0x02, 0x00, 0xe1, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x13, 0x01, 0x12, 0x04, 0xdf, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x00, 0x12, 0x04, 0xe0, 0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xe0, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe0, 0x02, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe0,
    0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xe3, 0x02, 0x00, 0xe9, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xe3, 0x02, 0x08, 0x23, 0x0a, 0x42,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xe5, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe5, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe5, 0x02, 0x0b, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x19, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe5, 0x02, 0x21, 0x22, 0x0a, 0x4a,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x19, 0x1a, 0x3c, 0x20, 0x74,
    0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x63,
    0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20,
    0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xe8, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xe8, 0x02, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xe8, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xeb, 0x02,
    0x00, 0xed, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xeb, 0x02, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xec, 0x02, 0x02, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xec, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x02, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x16, 0x12, 0x06, 0xef, 0x02, 0x00, 0xf2, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01,
    0x12, 0x04, 0xef, 0x02, 0x08, 0x1d, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04,
    0xf1, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xf1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xf1, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xf1, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xf1, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xf4, 0x02,
    0x00, 0xf7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xf4, 0x02, 0x08,
    0x1b, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xf6, 0x02, 0x02, 0x22, 0x1a,
    0x4f, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2c, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x73, 0x61, 0x74, 0x69, 0x73, 0x66, 0x79, 0x20, 0x61, 0x20,
    0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20,
    0x6f, 0x6e, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf6, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf6, 0x02, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf6, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x18, 0x12, 0x06, 0xf9, 0x02, 0x00, 0x83, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18,
    0x01, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x1c, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12,
    0x04, 0xfb, 0x02, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f,
    0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xfb, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xfb, 0x02, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x18, 0x03, 0x00, 0x12,
    0x06, 0xfd, 0x02, 0x02, 0x80, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x03, 0x00, 0x01,
    0x12, 0x04, 0xfd, 0x02, 0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xfe, 0x02, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xfe, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xfe, 0x02, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x02, 0x13, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfe, 0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xff, 0x02, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xff, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x18, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xff, 0x02, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xff, 0x02, 0x14, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xff, 0x02, 0x1b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0x82, 0x03, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0x82, 0x03, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x03, 0x15, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x82, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06,
    0x85, 0x03, 0x00, 0x8d, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0x85,
    0x03, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0x86, 0x03, 0x02,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0x86, 0x03, 0x0b, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x03, 0x1d, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x03, 0x25, 0x26, 0x0a, 0x24, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0x89, 0x03, 0x02, 0x40, 0x1a, 0x16, 0x20, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2d, 0x6b, 0x65, 0x79, 0x20, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0x89, 0x03, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x03, 0x11, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x03, 0x27, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x08, 0x12, 0x04, 0x89, 0x03, 0x29, 0x3f, 0x0a, 0x10,
    0x0a, 0x08, 0x04, 0x19, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x89, 0x03, 0x2a, 0x3e,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x19, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x89,
    0x03, 0x2a, 0x37, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x19, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x89, 0x03, 0x2a, 0x37, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x19, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x89, 0x03, 0x2b, 0x36, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x19, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x89, 0x03, 0x3a, 0x3e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x03, 0x02, 0x3e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8a, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x03, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x03, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x02, 0x08, 0x12, 0x04, 0x8a, 0x03, 0x27, 0x3d, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x19, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x8a, 0x03, 0x28, 0x3c, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x19,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x8a, 0x03, 0x28, 0x35, 0x0a, 0x12, 0x0a,
    0x0a, 0x04, 0x19, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x03, 0x28,
    0x35, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x19, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8a, 0x03, 0x29, 0x34, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x19, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x03, 0x38, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02,
    0x03, 0x12, 0x04, 0x8c, 0x03, 0x02, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x8c, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x05, 0x12,
    0x04, 0x8c, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x8c, 0x03, 0x12, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8c,
    0x03, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x08, 0x12, 0x04, 0x8c, 0x03,
    0x2d, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x07, 0x12, 0x04, 0x8c, 0x03, 0x39,
    0x3b, 0x0a, 0xfb, 0x05, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x9c, 0x03, 0x00, 0xa5, 0x03, 0x01,
    0x1a, 0xec, 0x05, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x47, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x52, 0x50, 0x43, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x27,
    0x73, 0x20, 0x27, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x5f,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x27, 0x2e, 0x0a, 0x20, 0x2a, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x73, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72,
    0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x20, 0x2a, 0x20,
    0x49, 0x66, 0x20, 0x2a, 0x61, 0x6e, 0x79, 0x2a, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x72, 0x65, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x66, 0x61, 0x69, 0x6c, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x50, 0x42, 0x3a, 0x3a, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x54,
    0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x2c, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x20, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x0a, 0x20,
    0x2a, 0x20, 0x41, 0x20, 0x67, 0x61, 0x70, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6b, 0x65,
    0x79, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x73, 0x20,
    0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x20, 0x6e,
    0x6f, 0x6e, 0x2d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x20, 0x2a, 0x20, 0x49,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x27, 0x73, 0x20,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6b, 0x65, 0x79, 0x20, 0x66, 0x61, 0x6c, 0x6c, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6e,
    0x6f, 0x6e, 0x2d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x74, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20,
    0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x6e, 0x6f,
    0x6e, 0x2d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2c,
    0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x08, 0x23, 0x0a, 0x42, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9e, 0x03, 0x0b, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x19, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xa0, 0x03, 0x02, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xa0, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xa0, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xa0, 0x03, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xa0, 0x03, 0x30, 0x31, 0x0a, 0x82, 0x01, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12,
    0x04, 0xa4, 0x03, 0x02, 0x36, 0x1a, 0x74, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x73, 0x20, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6c, 0x69, 0x76, 0x65, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x65, 0x72,
    0x0a, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x6f, 0x75, 0x74, 0x2e, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x6f, 0x6e, 0x65, 0x20, 0x68, 0x6f, 0x75, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xa4, 0x03, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x08,
    0x12, 0x04, 0xa4, 0x03, 0x21, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x07, 0x12,
    0x04, 0xa4, 0x03, 0x2c, 0x34, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xa7, 0x03, 0x00,
    0xe0, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xa7, 0x03, 0x08, 0x1b,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b, 0x04, 0x00, 0x12, 0x06, 0xa8, 0x03, 0x02, 0xb3, 0x03, 0x03,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x04, 0x00, 0x01, 0x12, 0x04, 0xa8, 0x03, 0x07, 0x0f, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x03, 0x04, 0x10, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x03, 0x04, 0x0b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xa9, 0x03, 0x0e,
    0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x04,
    0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x03,
    0x04, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xaa,
    0x03, 0x11, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xab,
    0x03, 0x04, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xab, 0x03, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xab, 0x03, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x04, 0xac, 0x03, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xac, 0x03, 0x04, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x04, 0xac, 0x03, 0x14, 0x15, 0x0a, 0x6f, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x04, 0xb0, 0x03, 0x04, 0x15, 0x1a, 0x5f, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x4b,
    0x55, 0x44, 0x55, 0x2d, 0x38, 0x36, 0x31, 0x29, 0x3a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x52, 0x45, 0x4e, 0x41,
    0x4d, 0x45, 0x5f, 0x43, 0x4f, 0x4c, 0x55, 0x4d, 0x4e, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x79, 0x65, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x65, 0x64, 0x0a, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x73, 0x69, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xb0, 0x03, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb0, 0x03, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xb1, 0x03, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb1, 0x03, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xb1, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1b, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0xb2, 0x03, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb2, 0x03, 0x04, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xb2, 0x03, 0x1b, 0x1c, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x1b, 0x03, 0x00, 0x12, 0x06, 0xb4, 0x03, 0x02, 0xb9, 0x03, 0x03, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x03, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x0a, 0x13, 0x0a, 0x86, 0x01,
    0x0a, 0x06, 0x04, 0x1b, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb8, 0x03, 0x04, 0x27, 0x1a, 0x76,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x64, 0x64, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27,
    0x69, 0x64, 0x27, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x68,
    0x65, 0x72, 0x65, 0x20, 0x2d, 0x2d, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x20, 0x61,
    0x6e, 0x20, 0x49, 0x44, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb8, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xb8, 0x03, 0x0d, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x1c, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb8, 0x03, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b,
    0x03, 0x01, 0x12, 0x06, 0xba, 0x03, 0x02, 0xbd, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x03, 0x01, 0x01, 0x12, 0x04, 0xba, 0x03, 0x0a, 0x14, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x1b, 0x03,
    0x01, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x03, 0x04, 0x1d, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x74,
    0x6f, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x03, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x03, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x03, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x1b, 0x03, 0x02, 0x12, 0x06, 0xbe, 0x03, 0x02, 0xc2, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x03, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x0a, 0x16, 0x0a, 0x2f, 0x0a, 0x06, 0x04,
    0x1b, 0x03, 0x02, 0x02, 0x00, 0x12, 0x04, 0xc0, 0x03, 0x04, 0x21, 0x1a, 0x1f, 0x20, 0x4e, 0x61,
    0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x3b, 0x0a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc0, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc0, 0x03, 0x0d, 0x13, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc0, 0x03, 0x14, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc0, 0x03, 0x1f, 0x20,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x03, 0x04, 0x21,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc1, 0x03, 0x04,
    0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc1, 0x03,
    0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc1,
    0x03, 0x14, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xc1, 0x03, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b, 0x03, 0x03, 0x12, 0x06, 0xc3, 0x03,
    0x02, 0xc7, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x03, 0x03, 0x01, 0x12, 0x04, 0xc3,
    0x03, 0x0a, 0x1b, 0x0a, 0x7d, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04, 0xc6,
    0x03, 0x04, 0x2e, 0x1a, 0x6d, 0x20, 0x41, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72,
    0x6f, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x77,
    0x65, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x70, 0x70, 0x65, 0x72, 0x20, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x72, 0x6f, 0x70,
    0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6,
    0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x03, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xc6, 0x03, 0x0d, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc6, 0x03, 0x1d, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc6, 0x03, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b, 0x03, 0x04, 0x12, 0x06,
    0xc8, 0x03, 0x02, 0xcc, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x03, 0x04, 0x01, 0x12,
    0x04, 0xc8, 0x03, 0x0a, 0x1c, 0x0a, 0x7d, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x04, 0x02, 0x00, 0x12,
    0x04, 0xcb, 0x03, 0x04, 0x2e, 0x1a, 0x6d, 0x20, 0x41, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x72, 0x6f, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c,
    0x6f, 0x77, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x70, 0x70, 0x65, 0x72, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x20, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x72,
    0x6f, 0x70, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xcb, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xcb, 0x03, 0x0d, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xcb, 0x03, 0x1d, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xcb, 0x03, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b, 0x03, 0x05,
    0x12, 0x06, 0xce, 0x03, 0x02, 0xd7, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x03, 0x05,
    0x01, 0x12, 0x04, 0xce, 0x03, 0x0a, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x05, 0x02,
    0x00, 0x12, 0x04, 0xcf, 0x03, 0x04, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xcf, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xcf, 0x03, 0x0d, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcf, 0x03, 0x16, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x03, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcf, 0x03, 0x1d, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x03, 0x05, 0x02, 0x00, 0x08, 0x12, 0x04, 0xcf, 0x03, 0x1f, 0x34, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x03, 0x05, 0x02, 0x00, 0x07, 0x12, 0x04, 0xcf, 0x03, 0x2b, 0x32, 0x0a, 0x4b, 0x0a,
    0x06, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x03, 0x04, 0x26, 0x1a, 0x3b, 0x20,
    0x45, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x75, 0x73,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20,
    0x6f, 0x6e, 0x20, 0x27, 0x74, 0x79, 0x70, 0x65, 0x27, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x03, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x03, 0x05, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd2, 0x03, 0x0d, 0x16, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x03, 0x05, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x03, 0x17, 0x21, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x03, 0x24, 0x25, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x03, 0x04, 0x28, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x03, 0x04, 0x0c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd3, 0x03, 0x0d, 0x17,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x03, 0x18,
    0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd3, 0x03,
    0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x03, 0x12, 0x04, 0xd4, 0x03,
    0x04, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd4,
    0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x03, 0x06, 0x12, 0x04,
    0xd4, 0x03, 0x0d, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xd4, 0x03, 0x1a, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xd4, 0x03, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x04,
    0x12, 0x04, 0xd5, 0x03, 0x04, 0x37, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xd5, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05, 0x02,
    0x04, 0x06, 0x12, 0x04, 0xd5, 0x03, 0x0d, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03, 0x05,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x03, 0x1f, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x03,
    0x05, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x03, 0x35, 0x36, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b,
    0x03, 0x05, 0x02, 0x05, 0x12, 0x04, 0xd6, 0x03, 0x04, 0x39, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x03, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04, 0xd6, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x03, 0x05, 0x02, 0x05, 0x06, 0x12, 0x04, 0xd6, 0x03, 0x0d, 0x1f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x03, 0x05, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd6, 0x03, 0x20, 0x34, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x03, 0x05, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd6, 0x03, 0x37, 0x38, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xd9, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd9, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd9, 0x03, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xd9, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01,
    0x12, 0x04, 0xda, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xda, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xda, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xda,
    0x03, 0x10, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xda, 0x03,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0xdb, 0x03, 0x02, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xdb, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xdb, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xdb, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x03, 0x23, 0x24, 0x0a, 0x92, 0x01, 0x0a,
    0x04, 0x04, 0x1b, 0x02, 0x03, 0x12, 0x04, 0xdf, 0x03, 0x02, 0x1f, 0x1a, 0x83, 0x01, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20,
    0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x63, 0x6f,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x62,
    0x6f, 0x75, 0x6e, 0x64, 0x20, 0x72, 0x6f, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x0a, 0x20, 0x6e, 0x65, 0x63, 0x65, 0x73,
    0x73, 0x61, 0x72, 0x79, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x61,
    0x64, 0x64, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xdf, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x06, 0x12, 0x04, 0xdf, 0x03, 0x0b, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdf, 0x03, 0x14, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xdf, 0x03, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x1c, 0x12, 0x06, 0xe2, 0x03, 0x00, 0xeb, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1c, 0x01, 0x12, 0x04, 0xe2, 0x03, 0x08, 0x1c, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00,
    0x12, 0x04, 0xe4, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe4, 0x03, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xe4, 0x03, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xe4, 0x03, 0x21, 0x22, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01,
    0x12, 0x04, 0xe7, 0x03, 0x02, 0x25, 0x1a, 0x2a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68,
    0x65, 0x6d, 0x61, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe7, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x03, 0x12, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe7, 0x03, 0x23, 0x24, 0x0a, 0x32,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0xea, 0x03, 0x02, 0x1e, 0x1a, 0x24, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xea, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xea, 0x03, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xea, 0x03, 0x11, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xea, 0x03, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xed, 0x03, 0x00, 0xef, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x1d, 0x01, 0x12, 0x04, 0xed, 0x03, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02,
    0x00, 0x12, 0x04, 0xee, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xee, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xee, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xee, 0x03, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xee,
    0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xf1, 0x03, 0x00, 0xfa, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xf1, 0x03, 0x08, 0x22, 0x0a, 0x42,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xf3, 0x03, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf3, 0x03, 0x0b, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x19, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf3, 0x03, 0x21, 0x22, 0x0a, 0x62,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x03, 0x02, 0x25, 0x1a, 0x54, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x27, 0x6e, 0x65, 0x77, 0x27, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73,
    0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf6, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf6, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x03, 0x12, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf6, 0x03, 0x23, 0x24, 0x0a, 0x49,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xf9, 0x03, 0x02, 0x19, 0x1a, 0x3b, 0x20, 0x74,
    0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x6f,
    0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xf9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xf9, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xf9, 0x03, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xf9, 0x03, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xfc, 0x03, 0x00,
    0xfe, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xfc, 0x03, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x03, 0x02, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x03, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x03, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20,
    0x12, 0x06, 0x80, 0x04, 0x00, 0x97, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12,
    0x04, 0x80, 0x04, 0x08, 0x20, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0x82,
    0x04, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2c,
    0x20, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x82, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x82, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x82, 0x04, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x82, 0x04, 0x21, 0x22, 0x0a, 0xe8, 0x01, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04,
    0x88, 0x04, 0x02, 0x1f, 0x1a, 0xd9, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x54, 0x53, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72,
    0x73, 0x74, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x69, 0x66, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x61,
    0x6c, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x6b, 0x65, 0x65, 0x70, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61,
    0x74, 0x69, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x20, 0x49, 0x6e, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x69, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x2c, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x76, 0x69,
    0x6f, 0x75, 0x73, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x3b, 0x0a, 0x20, 0x6f, 0x74, 0x68,
    0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0x88, 0x04, 0x0b, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x04, 0x14, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x04, 0x1d, 0x1e, 0x0a, 0x2d, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0x8b, 0x04, 0x02, 0x32, 0x1a, 0x1f, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x27, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x02, 0x06, 0x12, 0x04, 0x8b, 0x04, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x8b, 0x04, 0x1d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x8b, 0x04, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x03, 0x12,
    0x04, 0x8d, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x8d, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8d,
    0x04, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x04,
    0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8d, 0x04, 0x20,
    0x21, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x04, 0x12, 0x04, 0x90, 0x04, 0x02, 0x1e, 0x1a,
    0x16, 0x20, 0x54, 0x68, 0x65, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x90, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x90, 0x04, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x90, 0x04, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x03, 0x12, 0x04, 0x90,
    0x04, 0x1c, 0x1d, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x05, 0x12, 0x04, 0x93, 0x04, 0x02,
    0x26, 0x1a, 0x3d, 0x20, 0x54, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x66,
    0x61, 0x6c, 0x73, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x04, 0x12, 0x04, 0x93, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x05, 0x12, 0x04, 0x93, 0x04, 0x0b, 0x0f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x01, 0x12, 0x04, 0x93, 0x04, 0x10, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x05, 0x03, 0x12, 0x04, 0x93, 0x04, 0x24, 0x25, 0x0a, 0x1f, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x06, 0x12, 0x04, 0x96, 0x04, 0x02, 0x21, 0x1a, 0x11, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x06, 0x04, 0x12, 0x04, 0x96, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x06, 0x05, 0x12, 0x04, 0x96, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x06, 0x01, 0x12, 0x04, 0x96, 0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x06, 0x03, 0x12, 0x04, 0x96, 0x04, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12,
    0x06, 0x9d, 0x04, 0x00, 0x9e, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04,
    0x9d, 0x04, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xa0, 0x04, 0x00, 0xa9,
    0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xa0, 0x04, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0xa1, 0x04, 0x02, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa1, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa1, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa1, 0x04, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa1, 0x04, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x22, 0x03,
    0x00, 0x12, 0x06, 0xa3, 0x04, 0x02, 0xa7, 0x04, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x03,
    0x00, 0x01, 0x12, 0x04, 0xa3, 0x04, 0x0a, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x22, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xa4, 0x04, 0x04, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x04, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa4, 0x04, 0x0d, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x04, 0x1c, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x22, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x04, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x22, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x04, 0x04, 0x33, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x22, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x04, 0x04, 0x0c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa5, 0x04, 0x0d, 0x21, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x04, 0x22, 0x2e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x04, 0x31, 0x32,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x22, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x04, 0x04, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa6, 0x04, 0x04,
    0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa6, 0x04,
    0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa6,
    0x04, 0x13, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x22, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xa6, 0x04, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x04,
    0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa8, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa8, 0x04, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa8, 0x04, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa8, 0x04, 0x1b, 0x1c, 0x0a, 0x7a,
    0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xad, 0x04, 0x00, 0xae, 0x04, 0x01, 0x1a, 0x6c, 0x20, 0x47,
    0x65, 0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x48,
    0x54, 0x54, 0x50, 0x2f, 0x52, 0x50, 0x43, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23,
    0x01, 0x12, 0x04, 0xad, 0x04, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xb0,
    0x04, 0x00, 0xbd, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xb0, 0x04,
    0x08, 0x27, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x04, 0x02, 0x2a,
    0x1a, 0x2a, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb2, 0x04, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x04, 0x1a, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb2, 0x04, 0x28, 0x29, 0x0a, 0x7a, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x01,
    0x12, 0x04, 0xb6, 0x04, 0x02, 0x31, 0x1a, 0x6c, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20, 0x77, 0x6f, 0x6e, 0x27,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72,
    0x65, 0x27, 0x73, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65,
    0x74, 0x72, 0x69, 0x65, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x2f, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb6, 0x04,
    0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x04, 0x20,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x04, 0x2f, 0x30,
    0x0a, 0x42, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x04, 0x02, 0x2e, 0x1a, 0x34,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x27, 0x73, 0x20, 0x72,
    0x6f, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x73, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb9,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x06, 0x12, 0x04, 0xb9, 0x04,
    0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x04, 0x25,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x04, 0x2c, 0x2d,
    0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x23, 0x1a, 0x40,
    0x20, 0x53, 0x65, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbc, 0x04, 0x0b, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbc, 0x04, 0x19, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbc, 0x04, 0x21, 0x22, 0x0a, 0x79, 0x0a, 0x02,
    0x04, 0x25, 0x12, 0x06, 0xc1, 0x04, 0x00, 0xc2, 0x04, 0x01, 0x1a, 0x6b, 0x20, 0x4c, 0x69, 0x73,
    0x74, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x3a, 0x20, 0x67, 0x65, 0x74, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20,
    0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x0a, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73,
    0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04,
    0xc1, 0x04, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xc4, 0x04, 0x00, 0xcb,
    0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xc4, 0x04, 0x08, 0x1d, 0x0a,
    0x3b, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x04, 0x02, 0x25, 0x1a, 0x2d, 0x20,
    0x41, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63,
    0x68, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x20, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc6, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x04, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xc6, 0x04, 0x23, 0x24, 0x0a, 0x90, 0x01, 0x0a, 0x04, 0x04, 0x26, 0x02,
    0x01, 0x12, 0x04, 0xca, 0x04, 0x02, 0x21, 0x1a, 0x81, 0x01, 0x20, 0x53, 0x65, 0x74, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x27, 0x73, 0x20, 0x61,
    0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69,
    0x65, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x69, 0x6e,
    0x20, 0x67, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x27, 0x73, 0x20, 0x6f, 0x77, 0x6e, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0xca, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xca, 0x04, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xca, 0x04, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xca, 0x04, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x06, 0xcd,
    0x04, 0x00, 0xd3, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x04,
    0x05, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x04, 0xce, 0x04, 0x02, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x04, 0x02, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xce, 0x04, 0x14, 0x15, 0x0a, 0x57,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x04, 0x02, 0x1d, 0x1a, 0x49, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x73, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x63, 0x6f, 0x76, 0x65,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xd0, 0x04, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x04, 0xd0, 0x04, 0x1b, 0x1c, 0x0a, 0x49, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd2,
    0x04, 0x02, 0x20, 0x1a, 0x3b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x04, 0x02, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xd2, 0x04, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x06, 0xd5, 0x04, 0x00, 0xef, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x04, 0xd5, 0x04, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xd6, 0x04, 0x02, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xd6, 0x04, 0x06, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x04, 0xd6, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd6, 0x04, 0x23, 0x31, 0x0a, 0x1f, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x04,
    0x02, 0x48, 0x1a, 0x11, 0x20, 0x54, 0x53, 0x2d, 0x3e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20,
    0x52, 0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xd9, 0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xd9,
    0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd9, 0x04,
    0x31, 0x46, 0x0a, 0x23, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x04, 0xdc, 0x04, 0x02, 0x5d,
    0x1a, 0x15, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x3e, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x52, 0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xdc, 0x04, 0x06, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xdc, 0x04, 0x19, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xdc, 0x04, 0x3f, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04, 0xde, 0x04,
    0x02, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xde, 0x04, 0x06,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xde, 0x04, 0x12, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xde, 0x04, 0x31, 0x46, 0x0a,
    0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x04, 0xdf, 0x04, 0x02, 0x5a, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xdf, 0x04, 0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xdf, 0x04, 0x18, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x04, 0xdf, 0x04, 0x3d, 0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x05, 0x12, 0x04, 0xe1, 0x04, 0x02, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xe1, 0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x04, 0xe1, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xe1, 0x04, 0x31, 0x46, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x06, 0x12, 0x04, 0xe3,
    0x04, 0x02, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe3, 0x04,
    0x06, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xe3, 0x04, 0x11,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xe3, 0x04, 0x2f, 0x43,
    0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x07, 0x12, 0x04, 0xe4, 0x04, 0x02, 0x57, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xe4, 0x04, 0x06, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0xe4, 0x04, 0x17, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0xe4, 0x04, 0x3b, 0x55, 0x0a, 0x0c, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x08, 0x12, 0x04, 0xe6, 0x04, 0x02, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x04, 0xe6, 0x04, 0x06, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x08,
    0x02, 0x12, 0x04, 0xe6, 0x04, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xe6, 0x04, 0x2f, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x09, 0x12, 0x04,
    0xe7, 0x04, 0x02, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xe7,
    0x04, 0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xe7, 0x04,
    0x18, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0xe7, 0x04, 0x3d,
    0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xe8, 0x04, 0x02, 0x51, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xe8, 0x04, 0x06, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xe8, 0x04, 0x15, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xe8, 0x04, 0x37, 0x4f, 0x0a, 0x2e, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x0b, 0x12, 0x04, 0xeb, 0x04, 0x02, 0x5a, 0x1a, 0x20, 0x20, 0x41, 0x64, 0x6d,
    0x69, 0x6e, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x76, 0x65, 0x2f, 0x6d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x50, 0x43, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xeb, 0x04, 0x06, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xeb, 0x04, 0x18, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x0b, 0x03, 0x12, 0x04, 0xeb, 0x04, 0x3d, 0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x0c, 0x12, 0x04, 0xec, 0x04, 0x02, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x01,
    0x12, 0x04, 0xec, 0x04, 0x06, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x02, 0x12,
    0x04, 0xec, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0xec, 0x04, 0x31, 0x46, 0x0a, 0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x0d, 0x12, 0x06, 0xed, 0x04,
    0x02, 0xee, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xed,
    0x04, 0x06, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x04, 0xed, 0x04,
    0x1c, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xee, 0x04, 0x05,
    0x24,
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
