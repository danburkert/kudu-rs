use kudu_pb::master::{TabletLocationsPB, TabletLocationsPB_ReplicaPB as ReplicaPB};

use RaftRole;
use TabletId;
use TabletServerId;
use Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tablet {
    id: TabletId,
    partition_lower_bound: Vec<u8>,
    partition_upper_bound: Vec<u8>,
    replicas: Vec<Replica>,
}

impl Tablet {

    pub fn id(&self) -> &TabletId {
        &self.id
    }

    pub fn partition_lower_bound(&self) -> &[u8] {
        &self.partition_lower_bound
    }

    pub fn partition_upper_bound(&self) -> &[u8] {
        &self.partition_upper_bound
    }

    pub fn replicas(&self) -> &[Replica] {
        &self.replicas
    }

    /// Creates a new `Tablet` from a tablet locations protobuf message.
    pub fn from_pb(mut pb: TabletLocationsPB) -> Result<Tablet> {
        let id = try!(TabletId::parse_bytes(pb.get_tablet_id()));
        let partition_lower_bound = pb.mut_partition().take_partition_key_start();
        let partition_upper_bound = pb.mut_partition().take_partition_key_end();
        let mut replicas = Vec::with_capacity(pb.get_replicas().len());
        for replica in pb.take_replicas().into_iter() {
            replicas.push(try!(Replica::from_pb(replica)));
        }
        Ok(Tablet {
            id: id,
            partition_lower_bound: partition_lower_bound,
            partition_upper_bound: partition_upper_bound,
            replicas: replicas,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replica {
    tablet_server_id: TabletServerId,
    tablet_server_addrs: Vec<(String, u16)>,
    role: RaftRole,
}

impl Replica {
    pub fn tablet_server_id(&self) -> &TabletServerId {
        &self.tablet_server_id
    }

    pub fn tablet_server_addrs(&self) -> &[(String, u16)] {
        &self.tablet_server_addrs
    }

    /// Creates a new `Replica` from a replica protobuf message.
    pub fn from_pb(mut pb: ReplicaPB) -> Result<Replica> {
        let ts_id = try!(TabletServerId::parse_bytes(pb.get_ts_info().get_permanent_uuid()));
        let mut ts_addrs = Vec::with_capacity(pb.get_ts_info().get_rpc_addresses().len());
        for mut host_port in pb.mut_ts_info().take_rpc_addresses().into_iter() {
            let port = host_port.get_port() as u16;
            ts_addrs.push((host_port.take_host(), port));
        }
        let role = RaftRole::from_pb(pb.get_role());
        Ok(Replica {
            tablet_server_id: ts_id,
            tablet_server_addrs: ts_addrs,
            role: role,
        })
    }
}
