use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

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
use tserver::TabletServer;

#[derive(Clone)]
pub struct Tablet(Arc<(
        // The tablet ID.
        TabletId,

        // The tablet partition.
        Partition,

        // The tablet replicas.
        Vec<TabletReplica>,

        // The deadline when the replica metadata expires.
        Instant,
)>);

impl Tablet {

    /// Returns the unique tablet ID.
    pub fn id(&self) -> TabletId {
        (self.0).0
    }

    /// Returns the tablet partition.
    pub fn partition(&self) -> &Partition {
        &(self.0).1
    }

    /// Returns the tablet replicas.
    pub fn replicas(&self) -> &[TabletReplica] {
        &(self.0).2
    }

    pub(crate) fn is_stale(&self) -> bool {
        // TODO
        false
    }

    /// Creates a new `Tablet` from a tablet locations protobuf message.
    pub(crate) fn from_pb(primary_key_schema: &Schema,
                          partition_schema: PartitionSchema,
                          pb: TabletLocationsPb,
                          deadline: Instant)
                          -> Result<Tablet> {


        let id = TabletId::parse_bytes(&pb.tablet_id)?;
        let partition = Partition::from_pb(primary_key_schema,
                                           partition_schema,
                                           pb.partition.expect_field("TabletLocationsPB", "partition")?)?;

        let replicas = pb.replicas.into_iter().map(TabletReplica::from_pb).collect::<Result<Vec<_>>>()?;

        Ok(Tablet(Arc::new((id, partition, replicas, deadline))))
    }
}

/// Tablet replica belonging to a tablet server.
pub struct TabletReplica {
    tserver: TabletServer,
    role: RaftRole,
    is_stale: AtomicBool,
}

impl TabletReplica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> TabletServerId {
        self.tserver.id()
    }

    /// Returns the RPC addresses of the tablet server which owns this replica.
    pub fn rpc_addrs(&self) -> Box<[HostPort]> {
        self.tserver.rpc_addrs()
    }

    /// Returns the Raft role of this replica.
    pub fn role(&self) -> RaftRole {
        self.role
    }

    /// Creates a new `Replica` from a replica protobuf message.
    pub(crate) fn from_pb(pb: ReplicaPb) -> Result<TabletReplica> {
        let role = pb.role();
        let id = TabletServerId::parse_bytes(&pb.ts_info.permanent_uuid)?;
        let rpc_addrs = pb.ts_info.rpc_addresses.into_iter().map(HostPort::from).collect::<Vec<_>>().into_boxed_slice();
        let tserver: TabletServer = unimplemented!();
        Ok(TabletReplica { tserver, role, is_stale: AtomicBool::new(false) })
    }
}
