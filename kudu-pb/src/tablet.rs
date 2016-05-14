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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct MemStoreTargetPB {
    // message fields
    mrs_id: ::std::option::Option<i64>,
    rs_id: ::std::option::Option<i64>,
    dms_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemStoreTargetPB {}

impl MemStoreTargetPB {
    pub fn new() -> MemStoreTargetPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemStoreTargetPB {
        static mut instance: ::protobuf::lazy::Lazy<MemStoreTargetPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemStoreTargetPB,
        };
        unsafe {
            instance.get(|| {
                MemStoreTargetPB {
                    mrs_id: ::std::option::Option::None,
                    rs_id: ::std::option::Option::None,
                    dms_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 mrs_id = 1;

    pub fn clear_mrs_id(&mut self) {
        self.mrs_id = ::std::option::Option::None;
    }

    pub fn has_mrs_id(&self) -> bool {
        self.mrs_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mrs_id(&mut self, v: i64) {
        self.mrs_id = ::std::option::Option::Some(v);
    }

    pub fn get_mrs_id(&self) -> i64 {
        self.mrs_id.unwrap_or(-1i64)
    }

    // optional int64 rs_id = 2;

    pub fn clear_rs_id(&mut self) {
        self.rs_id = ::std::option::Option::None;
    }

    pub fn has_rs_id(&self) -> bool {
        self.rs_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rs_id(&mut self, v: i64) {
        self.rs_id = ::std::option::Option::Some(v);
    }

    pub fn get_rs_id(&self) -> i64 {
        self.rs_id.unwrap_or(-1i64)
    }

    // optional int64 dms_id = 3;

    pub fn clear_dms_id(&mut self) {
        self.dms_id = ::std::option::Option::None;
    }

    pub fn has_dms_id(&self) -> bool {
        self.dms_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dms_id(&mut self, v: i64) {
        self.dms_id = ::std::option::Option::Some(v);
    }

    pub fn get_dms_id(&self) -> i64 {
        self.dms_id.unwrap_or(-1i64)
    }
}

impl ::protobuf::Message for MemStoreTargetPB {
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
                    let tmp = try!(is.read_int64());
                    self.mrs_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.rs_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.dms_id = ::std::option::Option::Some(tmp);
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
        for value in self.mrs_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.rs_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dms_id.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mrs_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.rs_id {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.dms_id {
            try!(os.write_int64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<MemStoreTargetPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MemStoreTargetPB {
    fn new() -> MemStoreTargetPB {
        MemStoreTargetPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemStoreTargetPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "mrs_id",
                    MemStoreTargetPB::has_mrs_id,
                    MemStoreTargetPB::get_mrs_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "rs_id",
                    MemStoreTargetPB::has_rs_id,
                    MemStoreTargetPB::get_rs_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "dms_id",
                    MemStoreTargetPB::has_dms_id,
                    MemStoreTargetPB::get_dms_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemStoreTargetPB>(
                    "MemStoreTargetPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemStoreTargetPB {
    fn clear(&mut self) {
        self.clear_mrs_id();
        self.clear_rs_id();
        self.clear_dms_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MemStoreTargetPB {
    fn eq(&self, other: &MemStoreTargetPB) -> bool {
        self.mrs_id == other.mrs_id &&
        self.rs_id == other.rs_id &&
        self.dms_id == other.dms_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MemStoreTargetPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OperationResultPB {
    // message fields
    flushed: ::std::option::Option<bool>,
    failed_status: ::protobuf::SingularPtrField<super::wire_protocol::AppStatusPB>,
    mutated_stores: ::protobuf::RepeatedField<MemStoreTargetPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OperationResultPB {}

impl OperationResultPB {
    pub fn new() -> OperationResultPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OperationResultPB {
        static mut instance: ::protobuf::lazy::Lazy<OperationResultPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OperationResultPB,
        };
        unsafe {
            instance.get(|| {
                OperationResultPB {
                    flushed: ::std::option::Option::None,
                    failed_status: ::protobuf::SingularPtrField::none(),
                    mutated_stores: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool flushed = 1;

    pub fn clear_flushed(&mut self) {
        self.flushed = ::std::option::Option::None;
    }

    pub fn has_flushed(&self) -> bool {
        self.flushed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flushed(&mut self, v: bool) {
        self.flushed = ::std::option::Option::Some(v);
    }

    pub fn get_flushed(&self) -> bool {
        self.flushed.unwrap_or(false)
    }

    // optional .kudu.AppStatusPB failed_status = 2;

    pub fn clear_failed_status(&mut self) {
        self.failed_status.clear();
    }

    pub fn has_failed_status(&self) -> bool {
        self.failed_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failed_status(&mut self, v: super::wire_protocol::AppStatusPB) {
        self.failed_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_failed_status(&mut self) -> &mut super::wire_protocol::AppStatusPB {
        if self.failed_status.is_none() {
            self.failed_status.set_default();
        };
        self.failed_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_failed_status(&mut self) -> super::wire_protocol::AppStatusPB {
        self.failed_status.take().unwrap_or_else(|| super::wire_protocol::AppStatusPB::new())
    }

    pub fn get_failed_status(&self) -> &super::wire_protocol::AppStatusPB {
        self.failed_status.as_ref().unwrap_or_else(|| super::wire_protocol::AppStatusPB::default_instance())
    }

    // repeated .kudu.tablet.MemStoreTargetPB mutated_stores = 3;

    pub fn clear_mutated_stores(&mut self) {
        self.mutated_stores.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutated_stores(&mut self, v: ::protobuf::RepeatedField<MemStoreTargetPB>) {
        self.mutated_stores = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutated_stores(&mut self) -> &mut ::protobuf::RepeatedField<MemStoreTargetPB> {
        &mut self.mutated_stores
    }

    // Take field
    pub fn take_mutated_stores(&mut self) -> ::protobuf::RepeatedField<MemStoreTargetPB> {
        ::std::mem::replace(&mut self.mutated_stores, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutated_stores(&self) -> &[MemStoreTargetPB] {
        &self.mutated_stores
    }
}

impl ::protobuf::Message for OperationResultPB {
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
                    let tmp = try!(is.read_bool());
                    self.flushed = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.failed_status));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutated_stores));
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
        if self.flushed.is_some() {
            my_size += 2;
        };
        for value in self.failed_status.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.mutated_stores.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flushed {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.failed_status.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.mutated_stores.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<OperationResultPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OperationResultPB {
    fn new() -> OperationResultPB {
        OperationResultPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<OperationResultPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "flushed",
                    OperationResultPB::has_flushed,
                    OperationResultPB::get_flushed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "failed_status",
                    OperationResultPB::has_failed_status,
                    OperationResultPB::get_failed_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "mutated_stores",
                    OperationResultPB::get_mutated_stores,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OperationResultPB>(
                    "OperationResultPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OperationResultPB {
    fn clear(&mut self) {
        self.clear_flushed();
        self.clear_failed_status();
        self.clear_mutated_stores();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OperationResultPB {
    fn eq(&self, other: &OperationResultPB) -> bool {
        self.flushed == other.flushed &&
        self.failed_status == other.failed_status &&
        self.mutated_stores == other.mutated_stores &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OperationResultPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TxResultPB {
    // message fields
    ops: ::protobuf::RepeatedField<OperationResultPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxResultPB {}

impl TxResultPB {
    pub fn new() -> TxResultPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxResultPB {
        static mut instance: ::protobuf::lazy::Lazy<TxResultPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxResultPB,
        };
        unsafe {
            instance.get(|| {
                TxResultPB {
                    ops: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kudu.tablet.OperationResultPB ops = 1;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<OperationResultPB>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<OperationResultPB> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<OperationResultPB> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[OperationResultPB] {
        &self.ops
    }
}

impl ::protobuf::Message for TxResultPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops));
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
        for value in self.ops.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.ops.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<TxResultPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TxResultPB {
    fn new() -> TxResultPB {
        TxResultPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxResultPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ops",
                    TxResultPB::get_ops,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxResultPB>(
                    "TxResultPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxResultPB {
    fn clear(&mut self) {
        self.clear_ops();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TxResultPB {
    fn eq(&self, other: &TxResultPB) -> bool {
        self.ops == other.ops &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TxResultPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeltaStatsPB {
    // message fields
    delete_count: ::std::option::Option<i64>,
    min_timestamp: ::std::option::Option<u64>,
    max_timestamp: ::std::option::Option<u64>,
    column_stats: ::protobuf::RepeatedField<DeltaStatsPB_ColumnStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeltaStatsPB {}

impl DeltaStatsPB {
    pub fn new() -> DeltaStatsPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeltaStatsPB {
        static mut instance: ::protobuf::lazy::Lazy<DeltaStatsPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeltaStatsPB,
        };
        unsafe {
            instance.get(|| {
                DeltaStatsPB {
                    delete_count: ::std::option::Option::None,
                    min_timestamp: ::std::option::Option::None,
                    max_timestamp: ::std::option::Option::None,
                    column_stats: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 delete_count = 1;

    pub fn clear_delete_count(&mut self) {
        self.delete_count = ::std::option::Option::None;
    }

    pub fn has_delete_count(&self) -> bool {
        self.delete_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_count(&mut self, v: i64) {
        self.delete_count = ::std::option::Option::Some(v);
    }

    pub fn get_delete_count(&self) -> i64 {
        self.delete_count.unwrap_or(0)
    }

    // required fixed64 min_timestamp = 3;

    pub fn clear_min_timestamp(&mut self) {
        self.min_timestamp = ::std::option::Option::None;
    }

    pub fn has_min_timestamp(&self) -> bool {
        self.min_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_timestamp(&mut self, v: u64) {
        self.min_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_min_timestamp(&self) -> u64 {
        self.min_timestamp.unwrap_or(0)
    }

    // required fixed64 max_timestamp = 4;

    pub fn clear_max_timestamp(&mut self) {
        self.max_timestamp = ::std::option::Option::None;
    }

    pub fn has_max_timestamp(&self) -> bool {
        self.max_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_timestamp(&mut self, v: u64) {
        self.max_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_max_timestamp(&self) -> u64 {
        self.max_timestamp.unwrap_or(0)
    }

    // repeated .kudu.tablet.DeltaStatsPB.ColumnStats column_stats = 5;

    pub fn clear_column_stats(&mut self) {
        self.column_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_column_stats(&mut self, v: ::protobuf::RepeatedField<DeltaStatsPB_ColumnStats>) {
        self.column_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_column_stats(&mut self) -> &mut ::protobuf::RepeatedField<DeltaStatsPB_ColumnStats> {
        &mut self.column_stats
    }

    // Take field
    pub fn take_column_stats(&mut self) -> ::protobuf::RepeatedField<DeltaStatsPB_ColumnStats> {
        ::std::mem::replace(&mut self.column_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_column_stats(&self) -> &[DeltaStatsPB_ColumnStats] {
        &self.column_stats
    }
}

impl ::protobuf::Message for DeltaStatsPB {
    fn is_initialized(&self) -> bool {
        if self.delete_count.is_none() {
            return false;
        };
        if self.min_timestamp.is_none() {
            return false;
        };
        if self.max_timestamp.is_none() {
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
                    let tmp = try!(is.read_int64());
                    self.delete_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.min_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.max_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.column_stats));
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
        for value in self.delete_count.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.min_timestamp.is_some() {
            my_size += 9;
        };
        if self.max_timestamp.is_some() {
            my_size += 9;
        };
        for value in self.column_stats.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.delete_count {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.min_timestamp {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.max_timestamp {
            try!(os.write_fixed64(4, v));
        };
        for v in self.column_stats.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<DeltaStatsPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeltaStatsPB {
    fn new() -> DeltaStatsPB {
        DeltaStatsPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeltaStatsPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "delete_count",
                    DeltaStatsPB::has_delete_count,
                    DeltaStatsPB::get_delete_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "min_timestamp",
                    DeltaStatsPB::has_min_timestamp,
                    DeltaStatsPB::get_min_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "max_timestamp",
                    DeltaStatsPB::has_max_timestamp,
                    DeltaStatsPB::get_max_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "column_stats",
                    DeltaStatsPB::get_column_stats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeltaStatsPB>(
                    "DeltaStatsPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeltaStatsPB {
    fn clear(&mut self) {
        self.clear_delete_count();
        self.clear_min_timestamp();
        self.clear_max_timestamp();
        self.clear_column_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeltaStatsPB {
    fn eq(&self, other: &DeltaStatsPB) -> bool {
        self.delete_count == other.delete_count &&
        self.min_timestamp == other.min_timestamp &&
        self.max_timestamp == other.max_timestamp &&
        self.column_stats == other.column_stats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeltaStatsPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeltaStatsPB_ColumnStats {
    // message fields
    col_id: ::std::option::Option<i32>,
    update_count: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeltaStatsPB_ColumnStats {}

impl DeltaStatsPB_ColumnStats {
    pub fn new() -> DeltaStatsPB_ColumnStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeltaStatsPB_ColumnStats {
        static mut instance: ::protobuf::lazy::Lazy<DeltaStatsPB_ColumnStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeltaStatsPB_ColumnStats,
        };
        unsafe {
            instance.get(|| {
                DeltaStatsPB_ColumnStats {
                    col_id: ::std::option::Option::None,
                    update_count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 col_id = 1;

    pub fn clear_col_id(&mut self) {
        self.col_id = ::std::option::Option::None;
    }

    pub fn has_col_id(&self) -> bool {
        self.col_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_col_id(&mut self, v: i32) {
        self.col_id = ::std::option::Option::Some(v);
    }

    pub fn get_col_id(&self) -> i32 {
        self.col_id.unwrap_or(0)
    }

    // optional int64 update_count = 2;

    pub fn clear_update_count(&mut self) {
        self.update_count = ::std::option::Option::None;
    }

    pub fn has_update_count(&self) -> bool {
        self.update_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_count(&mut self, v: i64) {
        self.update_count = ::std::option::Option::Some(v);
    }

    pub fn get_update_count(&self) -> i64 {
        self.update_count.unwrap_or(0i64)
    }
}

impl ::protobuf::Message for DeltaStatsPB_ColumnStats {
    fn is_initialized(&self) -> bool {
        if self.col_id.is_none() {
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
                    let tmp = try!(is.read_int32());
                    self.col_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.update_count = ::std::option::Option::Some(tmp);
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
        for value in self.col_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.update_count.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.col_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.update_count {
            try!(os.write_int64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<DeltaStatsPB_ColumnStats>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeltaStatsPB_ColumnStats {
    fn new() -> DeltaStatsPB_ColumnStats {
        DeltaStatsPB_ColumnStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeltaStatsPB_ColumnStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "col_id",
                    DeltaStatsPB_ColumnStats::has_col_id,
                    DeltaStatsPB_ColumnStats::get_col_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "update_count",
                    DeltaStatsPB_ColumnStats::has_update_count,
                    DeltaStatsPB_ColumnStats::get_update_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeltaStatsPB_ColumnStats>(
                    "DeltaStatsPB_ColumnStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeltaStatsPB_ColumnStats {
    fn clear(&mut self) {
        self.clear_col_id();
        self.clear_update_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeltaStatsPB_ColumnStats {
    fn eq(&self, other: &DeltaStatsPB_ColumnStats) -> bool {
        self.col_id == other.col_id &&
        self.update_count == other.update_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeltaStatsPB_ColumnStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TabletStatusPB {
    // message fields
    tablet_id: ::protobuf::SingularField<::std::string::String>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<super::tablet_metadata::TabletStatePB>,
    tablet_data_state: ::std::option::Option<super::tablet_metadata::TabletDataState>,
    last_status: ::protobuf::SingularField<::std::string::String>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    partition: ::protobuf::SingularPtrField<super::common::PartitionPB>,
    estimated_on_disk_size: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TabletStatusPB {}

impl TabletStatusPB {
    pub fn new() -> TabletStatusPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TabletStatusPB {
        static mut instance: ::protobuf::lazy::Lazy<TabletStatusPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TabletStatusPB,
        };
        unsafe {
            instance.get(|| {
                TabletStatusPB {
                    tablet_id: ::protobuf::SingularField::none(),
                    table_name: ::protobuf::SingularField::none(),
                    state: ::std::option::Option::None,
                    tablet_data_state: ::std::option::Option::None,
                    last_status: ::protobuf::SingularField::none(),
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    partition: ::protobuf::SingularPtrField::none(),
                    estimated_on_disk_size: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string tablet_id = 1;

    pub fn clear_tablet_id(&mut self) {
        self.tablet_id.clear();
    }

    pub fn has_tablet_id(&self) -> bool {
        self.tablet_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tablet_id(&mut self, v: ::std::string::String) {
        self.tablet_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tablet_id(&mut self) -> &mut ::std::string::String {
        if self.tablet_id.is_none() {
            self.tablet_id.set_default();
        };
        self.tablet_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_tablet_id(&mut self) -> ::std::string::String {
        self.tablet_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tablet_id(&self) -> &str {
        match self.tablet_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string table_name = 2;

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

    // optional .kudu.tablet.TabletStatePB state = 3;

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

    // optional .kudu.tablet.TabletDataState tablet_data_state = 8;

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

    // required string last_status = 4;

    pub fn clear_last_status(&mut self) {
        self.last_status.clear();
    }

    pub fn has_last_status(&self) -> bool {
        self.last_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_status(&mut self, v: ::std::string::String) {
        self.last_status = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_status(&mut self) -> &mut ::std::string::String {
        if self.last_status.is_none() {
            self.last_status.set_default();
        };
        self.last_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_status(&mut self) -> ::std::string::String {
        self.last_status.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_last_status(&self) -> &str {
        match self.last_status.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes start_key = 5;

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

    // optional bytes end_key = 6;

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

    // optional .kudu.PartitionPB partition = 9;

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

    // optional int64 estimated_on_disk_size = 7;

    pub fn clear_estimated_on_disk_size(&mut self) {
        self.estimated_on_disk_size = ::std::option::Option::None;
    }

    pub fn has_estimated_on_disk_size(&self) -> bool {
        self.estimated_on_disk_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_estimated_on_disk_size(&mut self, v: i64) {
        self.estimated_on_disk_size = ::std::option::Option::Some(v);
    }

    pub fn get_estimated_on_disk_size(&self) -> i64 {
        self.estimated_on_disk_size.unwrap_or(0)
    }
}

impl ::protobuf::Message for TabletStatusPB {
    fn is_initialized(&self) -> bool {
        if self.tablet_id.is_none() {
            return false;
        };
        if self.table_name.is_none() {
            return false;
        };
        if self.last_status.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tablet_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.tablet_data_state = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.last_status));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.estimated_on_disk_size = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.table_name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.tablet_data_state.iter() {
            my_size += ::protobuf::rt::enum_size(8, *value);
        };
        for value in self.last_status.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        for value in self.end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in self.partition.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.estimated_on_disk_size.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tablet_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.table_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.state {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.tablet_data_state {
            try!(os.write_enum(8, v.value()));
        };
        if let Some(v) = self.last_status.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.partition.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.estimated_on_disk_size {
            try!(os.write_int64(7, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<TabletStatusPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TabletStatusPB {
    fn new() -> TabletStatusPB {
        TabletStatusPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<TabletStatusPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "tablet_id",
                    TabletStatusPB::has_tablet_id,
                    TabletStatusPB::get_tablet_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "table_name",
                    TabletStatusPB::has_table_name,
                    TabletStatusPB::get_table_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    TabletStatusPB::has_state,
                    TabletStatusPB::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "tablet_data_state",
                    TabletStatusPB::has_tablet_data_state,
                    TabletStatusPB::get_tablet_data_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "last_status",
                    TabletStatusPB::has_last_status,
                    TabletStatusPB::get_last_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    TabletStatusPB::has_start_key,
                    TabletStatusPB::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    TabletStatusPB::has_end_key,
                    TabletStatusPB::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition",
                    TabletStatusPB::has_partition,
                    TabletStatusPB::get_partition,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "estimated_on_disk_size",
                    TabletStatusPB::has_estimated_on_disk_size,
                    TabletStatusPB::get_estimated_on_disk_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TabletStatusPB>(
                    "TabletStatusPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TabletStatusPB {
    fn clear(&mut self) {
        self.clear_tablet_id();
        self.clear_table_name();
        self.clear_state();
        self.clear_tablet_data_state();
        self.clear_last_status();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_partition();
        self.clear_estimated_on_disk_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TabletStatusPB {
    fn eq(&self, other: &TabletStatusPB) -> bool {
        self.tablet_id == other.tablet_id &&
        self.table_name == other.table_name &&
        self.state == other.state &&
        self.tablet_data_state == other.tablet_data_state &&
        self.last_status == other.last_status &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.partition == other.partition &&
        self.estimated_on_disk_size == other.estimated_on_disk_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TabletStatusPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MaintenanceManagerStatusPB {
    // message fields
    best_op: ::protobuf::SingularPtrField<MaintenanceManagerStatusPB_MaintenanceOpPB>,
    registered_operations: ::protobuf::RepeatedField<MaintenanceManagerStatusPB_MaintenanceOpPB>,
    completed_operations: ::protobuf::RepeatedField<MaintenanceManagerStatusPB_CompletedOpPB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MaintenanceManagerStatusPB {}

impl MaintenanceManagerStatusPB {
    pub fn new() -> MaintenanceManagerStatusPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MaintenanceManagerStatusPB {
        static mut instance: ::protobuf::lazy::Lazy<MaintenanceManagerStatusPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MaintenanceManagerStatusPB,
        };
        unsafe {
            instance.get(|| {
                MaintenanceManagerStatusPB {
                    best_op: ::protobuf::SingularPtrField::none(),
                    registered_operations: ::protobuf::RepeatedField::new(),
                    completed_operations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.tablet.MaintenanceManagerStatusPB.MaintenanceOpPB best_op = 1;

    pub fn clear_best_op(&mut self) {
        self.best_op.clear();
    }

    pub fn has_best_op(&self) -> bool {
        self.best_op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_op(&mut self, v: MaintenanceManagerStatusPB_MaintenanceOpPB) {
        self.best_op = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_best_op(&mut self) -> &mut MaintenanceManagerStatusPB_MaintenanceOpPB {
        if self.best_op.is_none() {
            self.best_op.set_default();
        };
        self.best_op.as_mut().unwrap()
    }

    // Take field
    pub fn take_best_op(&mut self) -> MaintenanceManagerStatusPB_MaintenanceOpPB {
        self.best_op.take().unwrap_or_else(|| MaintenanceManagerStatusPB_MaintenanceOpPB::new())
    }

    pub fn get_best_op(&self) -> &MaintenanceManagerStatusPB_MaintenanceOpPB {
        self.best_op.as_ref().unwrap_or_else(|| MaintenanceManagerStatusPB_MaintenanceOpPB::default_instance())
    }

    // repeated .kudu.tablet.MaintenanceManagerStatusPB.MaintenanceOpPB registered_operations = 2;

    pub fn clear_registered_operations(&mut self) {
        self.registered_operations.clear();
    }

    // Param is passed by value, moved
    pub fn set_registered_operations(&mut self, v: ::protobuf::RepeatedField<MaintenanceManagerStatusPB_MaintenanceOpPB>) {
        self.registered_operations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_registered_operations(&mut self) -> &mut ::protobuf::RepeatedField<MaintenanceManagerStatusPB_MaintenanceOpPB> {
        &mut self.registered_operations
    }

    // Take field
    pub fn take_registered_operations(&mut self) -> ::protobuf::RepeatedField<MaintenanceManagerStatusPB_MaintenanceOpPB> {
        ::std::mem::replace(&mut self.registered_operations, ::protobuf::RepeatedField::new())
    }

    pub fn get_registered_operations(&self) -> &[MaintenanceManagerStatusPB_MaintenanceOpPB] {
        &self.registered_operations
    }

    // repeated .kudu.tablet.MaintenanceManagerStatusPB.CompletedOpPB completed_operations = 3;

    pub fn clear_completed_operations(&mut self) {
        self.completed_operations.clear();
    }

    // Param is passed by value, moved
    pub fn set_completed_operations(&mut self, v: ::protobuf::RepeatedField<MaintenanceManagerStatusPB_CompletedOpPB>) {
        self.completed_operations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_completed_operations(&mut self) -> &mut ::protobuf::RepeatedField<MaintenanceManagerStatusPB_CompletedOpPB> {
        &mut self.completed_operations
    }

    // Take field
    pub fn take_completed_operations(&mut self) -> ::protobuf::RepeatedField<MaintenanceManagerStatusPB_CompletedOpPB> {
        ::std::mem::replace(&mut self.completed_operations, ::protobuf::RepeatedField::new())
    }

    pub fn get_completed_operations(&self) -> &[MaintenanceManagerStatusPB_CompletedOpPB] {
        &self.completed_operations
    }
}

impl ::protobuf::Message for MaintenanceManagerStatusPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.best_op));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.registered_operations));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.completed_operations));
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
        for value in self.best_op.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.registered_operations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.completed_operations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.best_op.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.registered_operations.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.completed_operations.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<MaintenanceManagerStatusPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MaintenanceManagerStatusPB {
    fn new() -> MaintenanceManagerStatusPB {
        MaintenanceManagerStatusPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<MaintenanceManagerStatusPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "best_op",
                    MaintenanceManagerStatusPB::has_best_op,
                    MaintenanceManagerStatusPB::get_best_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "registered_operations",
                    MaintenanceManagerStatusPB::get_registered_operations,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "completed_operations",
                    MaintenanceManagerStatusPB::get_completed_operations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MaintenanceManagerStatusPB>(
                    "MaintenanceManagerStatusPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MaintenanceManagerStatusPB {
    fn clear(&mut self) {
        self.clear_best_op();
        self.clear_registered_operations();
        self.clear_completed_operations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MaintenanceManagerStatusPB {
    fn eq(&self, other: &MaintenanceManagerStatusPB) -> bool {
        self.best_op == other.best_op &&
        self.registered_operations == other.registered_operations &&
        self.completed_operations == other.completed_operations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MaintenanceManagerStatusPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MaintenanceManagerStatusPB_MaintenanceOpPB {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    running: ::std::option::Option<u32>,
    runnable: ::std::option::Option<bool>,
    ram_anchored_bytes: ::std::option::Option<u64>,
    logs_retained_bytes: ::std::option::Option<i64>,
    perf_improvement: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MaintenanceManagerStatusPB_MaintenanceOpPB {}

impl MaintenanceManagerStatusPB_MaintenanceOpPB {
    pub fn new() -> MaintenanceManagerStatusPB_MaintenanceOpPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MaintenanceManagerStatusPB_MaintenanceOpPB {
        static mut instance: ::protobuf::lazy::Lazy<MaintenanceManagerStatusPB_MaintenanceOpPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MaintenanceManagerStatusPB_MaintenanceOpPB,
        };
        unsafe {
            instance.get(|| {
                MaintenanceManagerStatusPB_MaintenanceOpPB {
                    name: ::protobuf::SingularField::none(),
                    running: ::std::option::Option::None,
                    runnable: ::std::option::Option::None,
                    ram_anchored_bytes: ::std::option::Option::None,
                    logs_retained_bytes: ::std::option::Option::None,
                    perf_improvement: ::std::option::Option::None,
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

    // required uint32 running = 2;

    pub fn clear_running(&mut self) {
        self.running = ::std::option::Option::None;
    }

    pub fn has_running(&self) -> bool {
        self.running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_running(&mut self, v: u32) {
        self.running = ::std::option::Option::Some(v);
    }

    pub fn get_running(&self) -> u32 {
        self.running.unwrap_or(0)
    }

    // required bool runnable = 3;

    pub fn clear_runnable(&mut self) {
        self.runnable = ::std::option::Option::None;
    }

    pub fn has_runnable(&self) -> bool {
        self.runnable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_runnable(&mut self, v: bool) {
        self.runnable = ::std::option::Option::Some(v);
    }

    pub fn get_runnable(&self) -> bool {
        self.runnable.unwrap_or(false)
    }

    // required uint64 ram_anchored_bytes = 4;

    pub fn clear_ram_anchored_bytes(&mut self) {
        self.ram_anchored_bytes = ::std::option::Option::None;
    }

    pub fn has_ram_anchored_bytes(&self) -> bool {
        self.ram_anchored_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ram_anchored_bytes(&mut self, v: u64) {
        self.ram_anchored_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_ram_anchored_bytes(&self) -> u64 {
        self.ram_anchored_bytes.unwrap_or(0)
    }

    // required int64 logs_retained_bytes = 5;

    pub fn clear_logs_retained_bytes(&mut self) {
        self.logs_retained_bytes = ::std::option::Option::None;
    }

    pub fn has_logs_retained_bytes(&self) -> bool {
        self.logs_retained_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logs_retained_bytes(&mut self, v: i64) {
        self.logs_retained_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_logs_retained_bytes(&self) -> i64 {
        self.logs_retained_bytes.unwrap_or(0)
    }

    // required double perf_improvement = 6;

    pub fn clear_perf_improvement(&mut self) {
        self.perf_improvement = ::std::option::Option::None;
    }

    pub fn has_perf_improvement(&self) -> bool {
        self.perf_improvement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perf_improvement(&mut self, v: f64) {
        self.perf_improvement = ::std::option::Option::Some(v);
    }

    pub fn get_perf_improvement(&self) -> f64 {
        self.perf_improvement.unwrap_or(0.)
    }
}

impl ::protobuf::Message for MaintenanceManagerStatusPB_MaintenanceOpPB {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.running.is_none() {
            return false;
        };
        if self.runnable.is_none() {
            return false;
        };
        if self.ram_anchored_bytes.is_none() {
            return false;
        };
        if self.logs_retained_bytes.is_none() {
            return false;
        };
        if self.perf_improvement.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.running = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.runnable = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ram_anchored_bytes = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.logs_retained_bytes = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.perf_improvement = ::std::option::Option::Some(tmp);
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
        for value in self.running.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.runnable.is_some() {
            my_size += 2;
        };
        for value in self.ram_anchored_bytes.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.logs_retained_bytes.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.perf_improvement.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.running {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.runnable {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.ram_anchored_bytes {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.logs_retained_bytes {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.perf_improvement {
            try!(os.write_double(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<MaintenanceManagerStatusPB_MaintenanceOpPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MaintenanceManagerStatusPB_MaintenanceOpPB {
    fn new() -> MaintenanceManagerStatusPB_MaintenanceOpPB {
        MaintenanceManagerStatusPB_MaintenanceOpPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<MaintenanceManagerStatusPB_MaintenanceOpPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_name,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "running",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_running,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_running,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "runnable",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_runnable,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_runnable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ram_anchored_bytes",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_ram_anchored_bytes,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_ram_anchored_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "logs_retained_bytes",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_logs_retained_bytes,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_logs_retained_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "perf_improvement",
                    MaintenanceManagerStatusPB_MaintenanceOpPB::has_perf_improvement,
                    MaintenanceManagerStatusPB_MaintenanceOpPB::get_perf_improvement,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MaintenanceManagerStatusPB_MaintenanceOpPB>(
                    "MaintenanceManagerStatusPB_MaintenanceOpPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MaintenanceManagerStatusPB_MaintenanceOpPB {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_running();
        self.clear_runnable();
        self.clear_ram_anchored_bytes();
        self.clear_logs_retained_bytes();
        self.clear_perf_improvement();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MaintenanceManagerStatusPB_MaintenanceOpPB {
    fn eq(&self, other: &MaintenanceManagerStatusPB_MaintenanceOpPB) -> bool {
        self.name == other.name &&
        self.running == other.running &&
        self.runnable == other.runnable &&
        self.ram_anchored_bytes == other.ram_anchored_bytes &&
        self.logs_retained_bytes == other.logs_retained_bytes &&
        self.perf_improvement == other.perf_improvement &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MaintenanceManagerStatusPB_MaintenanceOpPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MaintenanceManagerStatusPB_CompletedOpPB {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    duration_millis: ::std::option::Option<i32>,
    secs_since_start: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MaintenanceManagerStatusPB_CompletedOpPB {}

impl MaintenanceManagerStatusPB_CompletedOpPB {
    pub fn new() -> MaintenanceManagerStatusPB_CompletedOpPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MaintenanceManagerStatusPB_CompletedOpPB {
        static mut instance: ::protobuf::lazy::Lazy<MaintenanceManagerStatusPB_CompletedOpPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MaintenanceManagerStatusPB_CompletedOpPB,
        };
        unsafe {
            instance.get(|| {
                MaintenanceManagerStatusPB_CompletedOpPB {
                    name: ::protobuf::SingularField::none(),
                    duration_millis: ::std::option::Option::None,
                    secs_since_start: ::std::option::Option::None,
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

    // required int32 duration_millis = 2;

    pub fn clear_duration_millis(&mut self) {
        self.duration_millis = ::std::option::Option::None;
    }

    pub fn has_duration_millis(&self) -> bool {
        self.duration_millis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_millis(&mut self, v: i32) {
        self.duration_millis = ::std::option::Option::Some(v);
    }

    pub fn get_duration_millis(&self) -> i32 {
        self.duration_millis.unwrap_or(0)
    }

    // required int32 secs_since_start = 3;

    pub fn clear_secs_since_start(&mut self) {
        self.secs_since_start = ::std::option::Option::None;
    }

    pub fn has_secs_since_start(&self) -> bool {
        self.secs_since_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secs_since_start(&mut self, v: i32) {
        self.secs_since_start = ::std::option::Option::Some(v);
    }

    pub fn get_secs_since_start(&self) -> i32 {
        self.secs_since_start.unwrap_or(0)
    }
}

impl ::protobuf::Message for MaintenanceManagerStatusPB_CompletedOpPB {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.duration_millis.is_none() {
            return false;
        };
        if self.secs_since_start.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.duration_millis = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.secs_since_start = ::std::option::Option::Some(tmp);
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
        for value in self.duration_millis.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.secs_since_start.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.duration_millis {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.secs_since_start {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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
        ::std::any::TypeId::of::<MaintenanceManagerStatusPB_CompletedOpPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MaintenanceManagerStatusPB_CompletedOpPB {
    fn new() -> MaintenanceManagerStatusPB_CompletedOpPB {
        MaintenanceManagerStatusPB_CompletedOpPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<MaintenanceManagerStatusPB_CompletedOpPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    MaintenanceManagerStatusPB_CompletedOpPB::has_name,
                    MaintenanceManagerStatusPB_CompletedOpPB::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "duration_millis",
                    MaintenanceManagerStatusPB_CompletedOpPB::has_duration_millis,
                    MaintenanceManagerStatusPB_CompletedOpPB::get_duration_millis,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "secs_since_start",
                    MaintenanceManagerStatusPB_CompletedOpPB::has_secs_since_start,
                    MaintenanceManagerStatusPB_CompletedOpPB::get_secs_since_start,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MaintenanceManagerStatusPB_CompletedOpPB>(
                    "MaintenanceManagerStatusPB_CompletedOpPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MaintenanceManagerStatusPB_CompletedOpPB {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_duration_millis();
        self.clear_secs_since_start();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MaintenanceManagerStatusPB_CompletedOpPB {
    fn eq(&self, other: &MaintenanceManagerStatusPB_CompletedOpPB) -> bool {
        self.name == other.name &&
        self.duration_millis == other.duration_millis &&
        self.secs_since_start == other.secs_since_start &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MaintenanceManagerStatusPB_CompletedOpPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2f, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x1a, 0x18, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1f, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x77,
    0x69, 0x72, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1a, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2f,
    0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4d,
    0x0a, 0x10, 0x4d, 0x65, 0x6d, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x50, 0x42, 0x12, 0x12, 0x0a, 0x06, 0x6d, 0x72, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x3a, 0x02, 0x2d, 0x31, 0x12, 0x11, 0x0a, 0x05, 0x72, 0x73, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x03, 0x3a, 0x02, 0x2d, 0x31, 0x12, 0x12, 0x0a, 0x06, 0x64, 0x6d, 0x73,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x3a, 0x02, 0x2d, 0x31, 0x22, 0x8c, 0x01,
    0x0a, 0x11, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x07, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x65, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x28, 0x0a, 0x0d, 0x66,
    0x61, 0x69, 0x6c, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x41, 0x70, 0x70, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x50, 0x42, 0x12, 0x35, 0x0a, 0x0e, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x4d, 0x65, 0x6d, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x50, 0x42, 0x22, 0x39, 0x0a, 0x0a,
    0x54, 0x78, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x50, 0x42, 0x12, 0x2b, 0x0a, 0x03, 0x6f, 0x70,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x50, 0x42, 0x22, 0xc7, 0x01, 0x0a, 0x0c, 0x44, 0x65, 0x6c, 0x74,
    0x61, 0x53, 0x74, 0x61, 0x74, 0x73, 0x50, 0x42, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x15,
    0x0a, 0x0d, 0x6d, 0x69, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x06, 0x12, 0x15, 0x0a, 0x0d, 0x6d, 0x61, 0x78, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x02, 0x28, 0x06, 0x12, 0x3b, 0x0a, 0x0c,
    0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74,
    0x2e, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x53, 0x74, 0x61, 0x74, 0x73, 0x50, 0x42, 0x2e, 0x43, 0x6f,
    0x6c, 0x75, 0x6d, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x1a, 0x36, 0x0a, 0x0b, 0x43, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6f, 0x6c, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0c, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x3a, 0x01,
    0x30, 0x22, 0xb8, 0x02, 0x0a, 0x0e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x50, 0x42, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x32, 0x0a, 0x05, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x50, 0x42, 0x3a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12,
    0x4c, 0x0a, 0x11, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x6b, 0x75, 0x64,
    0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x44,
    0x61, 0x74, 0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x13, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x54,
    0x5f, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x12, 0x13, 0x0a,
    0x0b, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x24, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x75, 0x64, 0x75,
    0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x12, 0x1e, 0x0a, 0x16,
    0x65, 0x73, 0x74, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x6f, 0x6e, 0x5f, 0x64, 0x69, 0x73,
    0x6b, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x03, 0x22, 0xfd, 0x03, 0x0a,
    0x1a, 0x4d, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x61,
    0x67, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x12, 0x48, 0x0a, 0x07, 0x62,
    0x65, 0x73, 0x74, 0x5f, 0x6f, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x6b,
    0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x4d, 0x61, 0x69, 0x6e, 0x74,
    0x65, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x50, 0x42, 0x2e, 0x4d, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x63,
    0x65, 0x4f, 0x70, 0x50, 0x42, 0x12, 0x56, 0x0a, 0x15, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65,
    0x72, 0x65, 0x64, 0x5f, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x74, 0x2e, 0x4d, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x61,
    0x6e, 0x61, 0x67, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x42, 0x2e, 0x4d, 0x61,
    0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x4f, 0x70, 0x50, 0x42, 0x12, 0x53, 0x0a,
    0x14, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x6f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x6b, 0x75,
    0x64, 0x75, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x74, 0x2e, 0x4d, 0x61, 0x69, 0x6e, 0x74, 0x65,
    0x6e, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x50, 0x42, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x70,
    0x50, 0x42, 0x1a, 0x95, 0x01, 0x0a, 0x0f, 0x4d, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e,
    0x63, 0x65, 0x4f, 0x70, 0x50, 0x42, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x75, 0x6e, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x1a, 0x0a, 0x12, 0x72, 0x61, 0x6d, 0x5f, 0x61,
    0x6e, 0x63, 0x68, 0x6f, 0x72, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x04, 0x12, 0x1b, 0x0a, 0x13, 0x6c, 0x6f, 0x67, 0x73, 0x5f, 0x72, 0x65, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x03,
    0x12, 0x18, 0x0a, 0x10, 0x70, 0x65, 0x72, 0x66, 0x5f, 0x69, 0x6d, 0x70, 0x72, 0x6f, 0x76, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x02, 0x28, 0x01, 0x1a, 0x50, 0x0a, 0x0d, 0x43, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x70, 0x50, 0x42, 0x12, 0x0c, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a, 0x0f, 0x64, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x05, 0x12, 0x18, 0x0a, 0x10, 0x73, 0x65, 0x63, 0x73, 0x5f, 0x73, 0x69, 0x6e, 0x63, 0x65,
    0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x42, 0x13, 0x0a, 0x11,
    0x6f, 0x72, 0x67, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x64, 0x62, 0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x74, 0x4a, 0xa7, 0x29, 0x0a, 0x06, 0x12, 0x04, 0x10, 0x00, 0x7e, 0x01, 0x0a, 0x8c, 0x06, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x10, 0x08, 0x13, 0x1a, 0x81, 0x06, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68,
    0x65, 0x20, 0x53, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x41, 0x53, 0x46, 0x29, 0x20, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e,
    0x73, 0x65, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x20,
    0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4e, 0x4f, 0x54, 0x49, 0x43, 0x45, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x69,
    0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x72, 0x65, 0x67, 0x61,
    0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20,
    0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x41, 0x53, 0x46, 0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x75,
    0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20,
    0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x32, 0x2e, 0x30, 0x20, 0x28, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x22, 0x29, 0x3b, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c,
    0x69, 0x61, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61,
    0x79, 0x20, 0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61,
    0x74, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77,
    0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c, 0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30,
    0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x0a, 0x20, 0x73,
    0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20,
    0x49, 0x53, 0x22, 0x20, 0x42, 0x41, 0x53, 0x49, 0x53, 0x2c, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f,
    0x55, 0x54, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52,
    0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41,
    0x4e, 0x59, 0x0a, 0x20, 0x4b, 0x49, 0x4e, 0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x2e, 0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x12, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x12,
    0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x12, 0x16, 0x29, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x14, 0x07, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x15, 0x07, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x16, 0x07, 0x23, 0x0a, 0xa0,
    0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1a, 0x00, 0x24, 0x01, 0x1a, 0x8f, 0x01, 0x20, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4d, 0x65, 0x6d, 0x52, 0x6f, 0x77, 0x53, 0x65, 0x74, 0x20, 0x28, 0x66,
    0x6f, 0x72, 0x20, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x75,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x73, 0x74, 0x20,
    0x4d, 0x52, 0x53, 0x29, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x28, 0x72, 0x6f, 0x77, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x20,
    0x49, 0x44, 0x29, 0x20, 0x70, 0x61, 0x69, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x75, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x61,
    0x20, 0x44, 0x69, 0x73, 0x6b, 0x52, 0x6f, 0x77, 0x53, 0x65, 0x74, 0x2e, 0x0a, 0x22, 0x81, 0x01,
    0x20, 0x2d, 0x31, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x2c, 0x20, 0x69,
    0x66, 0x20, 0x61, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x67, 0x65,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x68, 0x61, 0x73, 0x5f,
    0x6d, 0x72, 0x73, 0x5f, 0x69, 0x64, 0x28, 0x29, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20,
    0x77, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x61, 0x63, 0x63, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x61, 0x6c,
    0x6c, 0x79, 0x20, 0x73, 0x65, 0x65, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x2d, 0x6c, 0x6f, 0x6f, 0x6b,
    0x69, 0x6e, 0x67, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x20, 0x30, 0x29, 0x20, 0x49, 0x44, 0x73, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x18, 0x0a, 0x23, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x2c, 0x1a, 0x16, 0x20, 0x45, 0x69, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2e, 0x2e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x1f, 0x1c, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x07, 0x12, 0x03, 0x1f, 0x28, 0x2a, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x22, 0x02, 0x2c, 0x1a, 0x2e, 0x20, 0x2e, 0x2e, 0x2e, 0x20, 0x6f, 0x72, 0x20, 0x62, 0x6f, 0x74,
    0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69,
    0x6e, 0x67, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x65,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x22, 0x1b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x07, 0x12, 0x03, 0x22, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x23, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x23, 0x1c, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x07, 0x12, 0x03, 0x23, 0x28, 0x2a, 0x0a, 0x37, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x27, 0x00, 0x34, 0x01, 0x1a, 0x2b, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x49,
    0x6e, 0x73, 0x65, 0x72, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x65, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x27, 0x08, 0x19, 0x0a, 0x43, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x02, 0x30, 0x1a, 0x36, 0x20, 0x73, 0x65, 0x74,
    0x20, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x61, 0x73,
    0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x2a, 0x1c, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x07, 0x12, 0x03, 0x2a, 0x28, 0x2d, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x2d, 0x02, 0x2e, 0x1a, 0x29, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x6f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x1c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2d, 0x2c, 0x2d, 0x0a, 0xbc, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x33, 0x02, 0x2f, 0x1a, 0xae, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x66, 0x66, 0x65, 0x63, 0x74, 0x65, 0x64, 0x2e, 0x0a,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x49, 0x4e, 0x53, 0x45, 0x52, 0x54, 0x73, 0x2c, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x62,
    0x65, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x2e, 0x0a, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x4d, 0x55, 0x54, 0x41, 0x54, 0x45, 0x2c, 0x20, 0x69,
    0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68,
    0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x75,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x72, 0x72, 0x69, 0x76, 0x65, 0x64, 0x20, 0x64,
    0x75, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x33, 0x0b,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x1c, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x2d, 0x2e, 0x0a, 0x64, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x38, 0x00, 0x3b, 0x01, 0x1a, 0x58, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x69,
    0x6e, 0x63, 0x6c, 0x75, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x64, 0x69,
    0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x0a, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x38, 0x08, 0x12, 0x0a,
    0x35, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x02, 0x25, 0x1a, 0x28, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3a,
    0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x1d, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x23, 0x24, 0x0a, 0x37,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3e, 0x00, 0x53, 0x01, 0x1a, 0x2b, 0x20, 0x44, 0x65, 0x6c,
    0x74, 0x61, 0x20, 0x73, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x20, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x65, 0x64, 0x20, 0x64, 0x65, 0x6c, 0x74,
    0x61, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x3e, 0x08, 0x14, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x40, 0x02, 0x22,
    0x1a, 0x41, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x73, 0x20, 0x28, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x20, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x72, 0x65, 0x20, 0x72, 0x6f,
    0x77, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x11, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x20, 0x21, 0x0a, 0x3f, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x47, 0x02, 0x25, 0x1a, 0x32, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d,
    0x69, 0x6e, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x47, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x47, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x47, 0x23, 0x24, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x49, 0x02,
    0x25, 0x1a, 0x32, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65,
    0x6c, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x49, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x49, 0x13, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49, 0x23, 0x24, 0x0a, 0x3c, 0x0a,
    0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x4c, 0x02, 0x51, 0x03, 0x1a, 0x2e, 0x20, 0x50, 0x65,
    0x72, 0x2d, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74,
    0x69, 0x63, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64,
    0x65, 0x6c, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x0a, 0x15, 0x0a, 0x1f, 0x0a, 0x06, 0x04, 0x03, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x04, 0x1e, 0x1a, 0x10, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63,
    0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x49, 0x44, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4e, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x13, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x1c, 0x1d, 0x0a, 0x45, 0x0a, 0x06, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x50, 0x04, 0x34, 0x1a, 0x36, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x20, 0x49, 0x44, 0x2e,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x50, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x0d,
    0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x50, 0x13,
    0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50, 0x22,
    0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x50, 0x24,
    0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x50, 0x30,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x52, 0x02, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x52, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x52, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x52, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x52, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x55, 0x00,
    0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x55, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x56, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x56, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x56, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x56, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x57, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x57, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02,
    0x12, 0x03, 0x58, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x58, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x58, 0x0b,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x58, 0x19, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x58, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x58, 0x23, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x58, 0x2f, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x59, 0x02, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x59,
    0x0b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x59, 0x22, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x59, 0x36, 0x37, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x08, 0x12, 0x03, 0x59, 0x38, 0x59, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x07, 0x12, 0x03, 0x59, 0x44, 0x57, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x5a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5a, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5a, 0x20, 0x21, 0x0a,
    0x1a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x5c, 0x02, 0x1f, 0x1a, 0x0d, 0x20, 0x44,
    0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x5c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x5c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x5c, 0x1d, 0x1e, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x5e, 0x02, 0x1d,
    0x1a, 0x0d, 0x20, 0x44, 0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x5e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x5e, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x5e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12,
    0x03, 0x5f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03, 0x5f,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x06, 0x12, 0x03, 0x5f, 0x0b, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x5f, 0x17, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x5f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x60, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x60, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05,
    0x12, 0x03, 0x60, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x60, 0x11, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x60, 0x2a,
    0x2b, 0x0a, 0x47, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x64, 0x00, 0x7e, 0x01, 0x1a, 0x3b, 0x20,
    0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x20,
    0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x27, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x64, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12, 0x04,
    0x65, 0x02, 0x6d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x00, 0x01, 0x12, 0x03, 0x65,
    0x0a, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04,
    0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x04,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x0d,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x14,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x1b,
    0x1c, 0x0a, 0x45, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x68, 0x04, 0x20,
    0x1a, 0x36, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x69, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x72,
    0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x68, 0x14, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x68, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x69, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x69, 0x12, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x69, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x6a, 0x04, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x6a, 0x14, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x6a, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x6b, 0x04, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x6b, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x6b, 0x13, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x6b, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x6c, 0x04, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x6c, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x6c, 0x14, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x6c, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x01, 0x12, 0x04, 0x6f, 0x02,
    0x74, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x01, 0x01, 0x12, 0x03, 0x6f, 0x0a, 0x17,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x1d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x14, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x1b, 0x1c, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x71, 0x04, 0x27, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x04, 0x0c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x0d, 0x12, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x13, 0x22, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x25, 0x26, 0x0a, 0x40,
    0x0a, 0x06, 0x04, 0x05, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x73, 0x04, 0x28, 0x1a, 0x31, 0x20,
    0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x04, 0x0c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x73, 0x0d, 0x12,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x73, 0x13, 0x23,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x73, 0x26, 0x27,
    0x0a, 0x31, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x77, 0x02, 0x27, 0x1a, 0x24, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x72, 0x75,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x77, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x77, 0x0b, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x1b, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x25, 0x26, 0x0a, 0x2a, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x02, 0x35, 0x1a, 0x1d, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x7a, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7a, 0x1b,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7a, 0x33, 0x34, 0x0a,
    0x62, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x7d, 0x02, 0x32, 0x1a, 0x55, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x69,
    0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x2e, 0x20, 0x43, 0x61, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x75, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7d, 0x0b, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x19, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7d, 0x30, 0x31,
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
