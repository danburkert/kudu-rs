use std::net::SocketAddr;

use pb::master::TabletLocationsPb;
use pb::master::tablet_locations_pb::ReplicaPb;

use Error;
use HostPort;
use Partition;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TabletId;
use TabletServerId;
use util;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tablet {
    id: TabletId,
    partition: Partition,
    replicas: Vec<Replica>,
}

impl Tablet {

    pub fn id(&self) -> TabletId {
        self.id
    }

    pub fn partition(&self) -> &Partition {
        &self.partition
    }

    pub fn replicas(&self) -> &[Replica] {
        &self.replicas
    }

    /// Creates a new `Tablet` from a tablet locations protobuf message.
    pub(crate) fn from_pb(primary_key_schema: &Schema,
                          partition_schema: PartitionSchema,
                          mut pb: TabletLocationsPb)
                          -> Result<Tablet> {
        let id = TabletId::parse_bytes(&pb.tablet_id)?;
        let partition = match pb.partition {
            Some(partition) => Partition::from_pb(primary_key_schema, partition_schema, partition),
            None => Err(Error::Serialization(
                    "required field absent: TabletLocationsPb::partition".to_string())),
        }?;

        let mut replicas = Vec::with_capacity(pb.replicas.len());
        for replica in pb.replicas.into_iter() {
            replicas.push(Replica::from_pb(replica)?);
        }

        Ok(Tablet { id, partition, replicas })
    }
}

/// Tablet replica belonging to a tablet server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replica {
    id: TabletServerId,
    rpc_addrs: Vec<HostPort>,
    role: RaftRole,
}

impl Replica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> &TabletServerId {
        &self.id
    }

    /// Returns the RPC addresses of the tablet server which owns this replica.
    pub fn rpc_addrs(&self) -> &[HostPort] {
        &self.rpc_addrs
    }

    /// The Raft role of this replica.
    pub fn role(&self) -> RaftRole {
        self.role
    }

    /*
    /// The resolved RPC addresses.
    pub fn resolved_rpc_addrs(&self) -> &[SocketAddr] {
        &self.resolved_rpc_addrs
    }
    */

    /// Creates a new `Replica` from a replica protobuf message.
    pub(crate) fn from_pb(mut pb: ReplicaPb) -> Result<Replica> {
        let role = pb.role();
        let id = TabletServerId::parse_bytes(&pb.ts_info.permanent_uuid)?;
        let rpc_addrs = pb.ts_info.rpc_addresses.into_iter().map(HostPort::from).collect();
        Ok(Replica { id, rpc_addrs, role })
    }
}
