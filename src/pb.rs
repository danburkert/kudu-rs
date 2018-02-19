#![allow(unknown_lints, doc_markdown)]

use Error;
use HostPort;
use TableId;

mod kudu {
    include!(concat!(env!("OUT_DIR"), "/kudu.rs"));

    pub mod client {
        include!(concat!(env!("OUT_DIR"), "/kudu.client.rs"));
    }
    pub mod consensus {
        include!(concat!(env!("OUT_DIR"), "/kudu.consensus.rs"));
    }
    pub mod master {
        include!(concat!(env!("OUT_DIR"), "/kudu.master.rs"));

    }
    pub mod tablet {
        include!(concat!(env!("OUT_DIR"), "/kudu.tablet.rs"));
    }
    pub mod security {
        include!(concat!(env!("OUT_DIR"), "/kudu.security.rs"));
    }
    #[cfg(test)]
    pub mod tools {
        include!(concat!(env!("OUT_DIR"), "/kudu.tools.rs"));
    }
    pub mod tserver {
        include!(concat!(env!("OUT_DIR"), "/kudu.tserver.rs"));
    }
}

pub use pb::kudu::*;

// Some convenient conversions.

impl From<TableId> for master::TableIdentifierPb {
    fn from(table_id: TableId) -> master::TableIdentifierPb {
        master::TableIdentifierPb {
            table_id: Some(table_id.to_string().into_bytes()),
            ..Default::default()
        }
    }
}

impl From<String> for master::TableIdentifierPb {
    fn from(table_name: String) -> master::TableIdentifierPb {
        master::TableIdentifierPb {
            table_name: Some(table_name),
            ..Default::default()
        }
    }
}

impl From<String> for partition_schema_pb::ColumnIdentifierPb {
    fn from(column_name: String) -> partition_schema_pb::ColumnIdentifierPb {
        partition_schema_pb::ColumnIdentifierPb {
            identifier: Some(partition_schema_pb::column_identifier_pb::Identifier::Name(column_name)),
        }
    }
}

impl From<HostPortPb> for HostPort {
    fn from(hostport: HostPortPb) -> HostPort {
        HostPort::new(hostport.host, hostport.port as u16)
    }
}

pub trait ExpectField {
    type Value;
    fn expect_field(self, message: &'static str, field: &'static str) -> Result<Self::Value, Error>;
}

impl <T> ExpectField for Option<T> {
    type Value = T;
    fn expect_field(self, message: &'static str, field: &'static str) -> Result<T, Error> {
        match self {
            Some(value) => Ok(value),
            None => Err(Error::Serialization(format!("required field absent: {}::{}",
                                                     message, field))),
        }
    }
}
