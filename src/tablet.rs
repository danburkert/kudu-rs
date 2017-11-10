use pb::master::TabletLocationsPb;
use pb::master::tablet_locations_pb::ReplicaPb;
use pb::ExpectField;

use HostPort;
use Partition;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TabletId;
use TabletServerId;

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
                          pb: TabletLocationsPb)
                          -> Result<Tablet> {
        let id = TabletId::parse_bytes(&pb.tablet_id)?;
        let partition = Partition::from_pb(primary_key_schema,
                                           partition_schema,
                                           pb.partition.expect_field("TabletLocationsPB", "partition")?)?;

        let mut replicas = Vec::with_capacity(pb.replicas.len());
        for replica in pb.replicas {
            replicas.push(Replica::from_pb(replica)?);
        }

        Ok(Tablet { id, partition, replicas })
    }
}

/// Tablet replica belonging to a tablet server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replica {
    id: TabletServerId,
    rpc_addrs: Box<[HostPort]>,
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

    /// Creates a new `Replica` from a replica protobuf message.
    pub(crate) fn from_pb(pb: ReplicaPb) -> Result<Replica> {
        let role = pb.role();
        let id = TabletServerId::parse_bytes(&pb.ts_info.permanent_uuid)?;
        let rpc_addrs = pb.ts_info.rpc_addresses.into_iter().map(HostPort::from).collect::<Vec<_>>().into_boxed_slice();
        Ok(Replica { id, rpc_addrs, role })
    }
}
