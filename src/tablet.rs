use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::fmt;
use std::sync::Arc;

use krpc;

use HostPort;
use Partition;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TabletId;
use TabletServerId;
use partition::PartitionKey;
use pb::ExpectField;
use pb::master::{TabletLocationsPb, tablet_locations_pb::ReplicaPb as ReplicaPb};
use replica::{Replica, ReplicaSet};

/// Metadata pertaining to a Kudu tablet.
// TODO: when replacing a tablet in the table locations map, carry through the leader flag.
#[derive(Debug)]
pub(crate) struct Tablet {

    /// The tablet ID.
    pub id: TabletId,

    /// The tablet lower-bound partition key.
    pub lower_bound: PartitionKey,

    /// The tablet upper-bound partition key.
    pub upper_bound: PartitionKey,

    /// The tablet replicas.
    pub replicas: Vec<TabletReplica>,
}

impl Tablet {

    /// Returns the unique tablet ID.
    pub fn id(&self) -> TabletId {
        self.id
    }

    pub fn lower_bound(&self) -> &PartitionKey {
        &self.lower_bound
    }

    pub fn upper_bound(&self) -> &PartitionKey {
        &self.upper_bound
    }

    pub(crate) fn info(&self,
                       primary_key_schema: &Schema,
                       partition_schema: PartitionSchema) -> Result<TabletInfo> {
        let id = self.id;
        let partition = Partition::from_bounds(primary_key_schema,
                                               partition_schema,
                                               self.lower_bound.clone(),
                                               self.upper_bound.clone())?;

        let replicas = self.replicas
                           .iter()
                           .map(TabletReplica::info)
                           .collect::<Vec<_>>()
                           .into_boxed_slice();
        Ok(TabletInfo { id, partition, replicas })
    }
}

impl ReplicaSet for Arc<Tablet> {
    type Replica = TabletReplica;

    fn replicas(&self) -> &[TabletReplica] {
        &self.replicas
    }
}

/// Tablet replica belonging to a tablet server.
pub(crate) struct TabletReplica {
    pub id: TabletServerId,
    pub rpc_addrs: Box<[HostPort]>,
    pub proxy: krpc::Proxy,
    pub is_leader: AtomicBool,
    pub is_stale: AtomicBool,
}

impl TabletReplica {
    fn info(&self) -> ReplicaInfo {
        let role = if self.is_leader() { RaftRole::Leader } else { RaftRole::Follower };
        let id = self.id;
        let rpc_addrs = self.rpc_addrs.clone();
        ReplicaInfo { id, rpc_addrs, role }
    }
}

impl Replica for TabletReplica {

    fn proxy(&self) -> krpc::Proxy {
        self.proxy.clone()
    }

    fn is_leader(&self) -> bool {
        self.is_leader.load(Relaxed)
    }

    fn mark_leader(&self) {
        self.is_leader.store(true, Relaxed)
    }

    fn mark_follower(&self) {
        self.is_leader.store(false, Relaxed)
    }

    fn is_stale(&self) -> bool {
        self.is_stale.load(Relaxed)
    }

    fn mark_stale(&self) {
        self.is_stale.store(true, Relaxed)
    }
}

impl fmt::Debug for TabletReplica {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TabletReplica")
            .field("id", &self.id)
            .field("is_leader", &self.is_leader)
            .field("is_stale", &self.is_stale)
            .finish()
    }
}

impl TabletReplica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> TabletServerId {
        self.id
    }
}


/// Information about a Kudu tablet.
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

/// Information about a tablet replica belonging to a tablet server.
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
