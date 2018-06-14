use HostPort;
use Partition;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TabletId;
use TabletServerId;
use pb::ExpectField;
use pb::master::{TabletLocationsPb, tablet_locations_pb::ReplicaPb as ReplicaPb};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabletInfo {
    id: TabletId,
    partition: Partition,
    replicas: Box<[ReplicaInfo]>,
}

impl TabletInfo {

    pub fn id(&self) -> TabletId {
        self.id
    }

    pub fn partition(&self) -> &Partition {
        &self.partition
    }

    pub fn replicas(&self) -> &[ReplicaInfo] {
        &self.replicas
    }

    /// Creates a new `TabletInfo` from a tablet locations protobuf message.
    pub(crate) fn from_pb(primary_key_schema: &Schema,
                          partition_schema: PartitionSchema,
                          pb: TabletLocationsPb)
                          -> Result<TabletInfo> {
        let id = TabletId::parse_bytes(&pb.tablet_id)?;
        let partition = Partition::from_pb(primary_key_schema,
                                           partition_schema,
                                           pb.partition.expect_field("TabletLocationsPb",
                                                                     "partition")?)?;
        let replicas = pb.replicas
                         .into_iter()
                         .map(ReplicaInfo::from_pb)
                         .collect::<Result<Vec<_>>>()?
                         .into_boxed_slice();
        Ok(TabletInfo { id, partition, replicas })
    }
}

/// Tablet replica belonging to a tablet server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplicaInfo {
    id: TabletServerId,
    rpc_addrs: Box<[HostPort]>,
    role: RaftRole,
}

impl ReplicaInfo {

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

    /// Creates a new `ReplicaInfo` from a replica protobuf message.
    pub(crate) fn from_pb(pb: ReplicaPb) -> Result<ReplicaInfo> {
        let role = pb.role();
        let id = TabletServerId::parse_bytes(&pb.ts_info.permanent_uuid)?;
        let rpc_addrs = pb.ts_info
                          .rpc_addresses
                          .into_iter()
                          .map(HostPort::from)
                          .collect::<Vec<_>>()
                          .into_boxed_slice();
        Ok(ReplicaInfo { id, rpc_addrs, role })
    }
}
