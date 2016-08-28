use std::net::SocketAddr;

use kudu_pb::master::{TabletLocationsPB, TabletLocationsPB_ReplicaPB as ReplicaPB};

use Partition;
use PartitionSchema;
use RaftRole;
use Result;
use Schema;
use TabletId;
use TabletServerId;
use dns;

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
    pub fn from_pb(primary_key_schema: &Schema,
                   partition_schema: &PartitionSchema,
                   mut pb: TabletLocationsPB)
                   -> Result<Tablet> {
        let id = try!(TabletId::parse_bytes(pb.get_tablet_id()));
        let partition = try!(Partition::from_pb(primary_key_schema,
                                                partition_schema,
                                                pb.take_partition()));
        let mut replicas = Vec::with_capacity(pb.get_replicas().len());
        for replica in pb.take_replicas().into_iter() {
            replicas.push(try!(Replica::from_pb(replica)));
        }
        Ok(Tablet {
            id: id,
            partition: partition,
            replicas: replicas,
        })
    }
}

/// Tablet replica belonging to a tablet server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replica {
    id: TabletServerId,
    rpc_addrs: Vec<(String, u16)>,
    resolved_rpc_addrs: Vec<SocketAddr>,
    role: RaftRole,
    is_local: bool,
}

impl Replica {

    /// Returns the ID of the tablet server which owns this replica.
    pub fn id(&self) -> &TabletServerId {
        &self.id
    }

    /// Returns the RPC addresses of the tablet server which owns this replica.
    pub fn rpc_addrs(&self) -> &[(String, u16)] {
        &self.rpc_addrs
    }

    /// The Raft role of this replica.
    pub fn role(&self) -> RaftRole {
        self.role
    }

    /// The resolved RPC addresses.
    pub fn resolved_rpc_addrs(&self) -> &[SocketAddr] {
        &self.resolved_rpc_addrs
    }

    /// Creates a new `Replica` from a replica protobuf message.
    #[doc(hidden)]
    pub fn from_pb(mut pb: ReplicaPB) -> Result<Replica> {
        let id = try!(TabletServerId::parse_bytes(pb.get_ts_info().get_permanent_uuid()));
        let mut rpc_addrs = Vec::with_capacity(pb.get_ts_info().get_rpc_addresses().len());
        for mut host_port in pb.mut_ts_info().take_rpc_addresses().into_iter() {
            let port = host_port.get_port() as u16;
            rpc_addrs.push((host_port.take_host(), port));
        }
        let resolved_rpc_addrs = dns::resolve_hostports(&rpc_addrs);
        let role = RaftRole::from_pb(pb.get_role());
        let is_local = resolved_rpc_addrs.iter().any(|addr| dns::is_local_addr(&addr.ip()));
        Ok(Replica {
            id: id,
            rpc_addrs: rpc_addrs,
            resolved_rpc_addrs: resolved_rpc_addrs,
            role: role,
            is_local: is_local,
        })
    }
}
