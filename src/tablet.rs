use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use krpc;

use Partition;
use RaftRole;
use TabletId;
use TabletServerId;

/// Metadata pertaining to a Kudu tablet.
pub(crate) struct Tablet {
    /// The tablet ID.
    id: TabletId,

    /// The tablet partition.
    partition: Partition,

    /// The tablet replicas.
    replicas: Vec<TabletReplica>,

    /// The deadline when the tablet metadata expires.
    ttl: Instant,
    is_stale: AtomicBool,
}

impl Tablet {

    /// Returns the unique tablet ID.
    pub fn id(&self) -> TabletId {
        self.id
    }

    /// Returns the tablet partition.
    pub fn partition(&self) -> &Partition {
        &self.partition
    }

    /// Returns the tablet replicas.
    pub fn replicas(&self) -> &[TabletReplica] {
        &self.replicas
    }

    /// Returns true if the tablet metadata is known to be stale.
    ///
    /// The metadata may be stale because the leader changed, the set of replicas changed, or it
    /// has expired.
    pub fn is_stale(&self) -> bool {
        self.is_stale.load(Ordering::Relaxed) || Instant::now() > self.ttl
    }

    /// Marks the tablet metadata as stale.
    pub fn set_stale(&self) {
        self.is_stale.store(true, Ordering::Relaxed)
    }

    /*
    /// Creates a new `Tablet` from a tablet locations protobuf message.
    pub(crate) fn from_pb(primary_key_schema: &Schema,
                          partition_schema: PartitionSchema,
                          pb: TabletLocationsPb,
                          deadline: Instant)
                          -> Result<Tablet> {
        /*


        let id = TabletId::parse_bytes(&pb.tablet_id)?;
        let partition = Partition::from_pb(primary_key_schema,
                                           partition_schema,
                                           pb.partition.expect_field("TabletLocationsPB", "partition")?)?;

        let replicas = pb.replicas.into_iter().map(TabletReplica::from_pb).collect::<Result<Vec<_>>>()?;

        Ok(Tablet(Arc::new((id, partition, replicas, deadline))))
        */
        Ok(unimplemented!())
    }
        */
}

/// Tablet replica belonging to a tablet server.
pub(crate) struct TabletReplica {
    id: TabletServerId,
    proxy: krpc::Proxy,
    role: RaftRole,
    is_stale: AtomicBool,
}

impl TabletReplica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> TabletServerId {
        self.id
    }

    /// Returns a KRPC proxy to the tablet server hosting the replica.
    pub fn proxy(&self) -> krpc::Proxy {
        self.proxy.clone()
    }

    /// Returns the Raft role of this replica.
    pub fn role(&self) -> RaftRole {
        self.role
    }
}
