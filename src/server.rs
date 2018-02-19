use std::time::Duration;

use url::Url;

use HostPort;
use MasterId;
use RaftRole;
use Result;
use TabletServerId;
use pb::ExpectField;
use pb::master::list_tablet_servers_response_pb::{Entry as TabletServerEntryPb};
use pb::{ServerEntryPb as MasterEntryPb};
use util;

/// Master metadata.
///
/// This information should be considered 'point-in-time', and may change as the cluster topology
/// changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MasterInfo {
    id: MasterId,
    rpc_addrs: Vec<HostPort>,
    http_addrs: Vec<Url>,
    software_version: String,
    seqno: i64,
    role: RaftRole,
}

impl MasterInfo {

    /// The master unique server ID.
    pub fn id(&self) -> &MasterId {
        &self.id
    }

    /// The master RPC addresses.
    pub fn rpc_addrs(&self) -> &[HostPort] {
        &self.rpc_addrs
    }

    /// The master web UI addresses.
    pub fn http_addrs(&self) -> &[Url] {
        &self.http_addrs
    }

    /// The master sequence number.
    pub fn seqno(&self) -> i64 {
        self.seqno
    }

    /// The master Raft role.
    pub fn role(&self) -> RaftRole {
        self.role
    }

    /// Creates a `MasterInfo` from the Protobuf message format.
    pub(crate) fn from_pb(master: MasterEntryPb) -> Result<MasterInfo> {
        static ENTRY: &str = "MasterEntryPb";

        let role = master.role();
        let instance_id = master.instance_id.expect_field(ENTRY, "instance_id")?;

        let id = MasterId::parse_bytes(&instance_id.permanent_uuid)?;
        let seqno = instance_id.instance_seqno;

        let registration = master.registration.expect_field(ENTRY, "registration")?;
        let https_enabled = registration.https_enabled();

        let rpc_addrs = registration.rpc_addresses
                                    .into_iter()
                                    .map(HostPort::from)
                                    .collect::<Vec<_>>();

        let http_addrs = util::urls_from_pb(&registration.http_addresses, https_enabled)?;

        let software_version = registration.software_version
                                           .expect_field(ENTRY, "software_version")?;

        Ok(MasterInfo {
            id,
            rpc_addrs,
            http_addrs,
            software_version,
            seqno,
            role,
        })
    }
}

/// Tablet server metadata.
///
/// This information should be considered 'point-in-time', and may change as the cluster topology
/// changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabletServerInfo {
    id: TabletServerId,
    rpc_addrs: Vec<HostPort>,
    http_addrs: Vec<Url>,
    software_version: String,
    seqno: i64,
    duration_since_heartbeat: Duration,
}

impl TabletServerInfo {

    /// The tablet server unique server ID.
    pub fn id(&self) -> TabletServerId {
        self.id
    }

    /// The tablet server RPC addresses.
    pub fn rpc_addrs(&self) -> &[HostPort] {
        &self.rpc_addrs
    }

    /// The tablet server web UI addresses.
    pub fn http_addrs(&self) -> &[Url] {
        &self.http_addrs
    }

    /// The software version of the tablet server.
    pub fn software_version(&self) -> &str {
        &self.software_version
    }

    /// The tablet server sequence number.
    pub fn seqno(&self) -> i64 {
        self.seqno
    }

    /// Time since the tablet server last heartbeated to the master.
    pub fn duration_since_heartbeat(&self) -> Duration {
        self.duration_since_heartbeat
    }

    /// Creates a `TabletServerInfo` from the Protobuf message format.
    pub(crate) fn from_pb(tablet_server: TabletServerEntryPb) -> Result<TabletServerInfo> {
        static ENTRY: &str = "TabletServerEntryPb";

        let id = TabletServerId::parse_bytes(&tablet_server.instance_id.permanent_uuid)?;
        let seqno = tablet_server.instance_id.instance_seqno;

        let registration = tablet_server.registration.expect_field(ENTRY, "registration")?;
        let https_enabled = registration.https_enabled();

        let rpc_addrs = registration.rpc_addresses
                                    .into_iter()
                                    .map(HostPort::from)
                                    .collect::<Vec<_>>();

        let http_addrs = util::urls_from_pb(&registration.http_addresses, https_enabled)?;

        let software_version = registration.software_version
                                           .expect_field(ENTRY, "software_version")?;
        let millis_since_heartbeat = tablet_server.millis_since_heartbeat
                                                  .expect_field(ENTRY, "millis_since_heartbeat")?;
        Ok(TabletServerInfo {
            id,
            rpc_addrs,
            http_addrs,
            software_version,
            seqno,
            duration_since_heartbeat: Duration::from_millis(millis_since_heartbeat as u64),
        })
    }
}
