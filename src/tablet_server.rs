use std::time::Duration;

use pb::ExpectField;
use pb::master::list_tablet_servers_response_pb::{Entry as TabletServerEntry};

use HostPort;
use Result;
use TabletServerId;

/// Tablet server metadata.
///
/// This information should be considered 'point-in-time', and may change as the cluster topology
/// changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabletServer {
    id: TabletServerId,
    rpc_addrs: Vec<HostPort>,
    http_addrs: Vec<HostPort>,
    software_version: String,
    seqno: i64,
    duration_since_heartbeat: Duration,
}

impl TabletServer {
    pub fn id(&self) -> TabletServerId {
        self.id
    }

    pub fn rpc_addrs(&self) -> &[HostPort] {
        &self.rpc_addrs
    }

    pub fn http_addrs(&self) -> &[HostPort] {
        &self.http_addrs
    }

    pub fn software_version(&self) -> &str {
        &self.software_version
    }

    pub fn seqno(&self) -> i64 {
        self.seqno
    }

    pub fn duration_since_heartbeat(&self) -> Duration {
        self.duration_since_heartbeat
    }

    pub(crate) fn from_pb(tablet_server: TabletServerEntry) -> Result<TabletServer> {
        static ENTRY: &str = "TabletServerEntry";

        let id = TabletServerId::parse_bytes(&tablet_server.instance_id.permanent_uuid)?;
        let seqno = tablet_server.instance_id.instance_seqno;

        let registration = tablet_server.registration.expect_field(ENTRY, "registration")?;

        let rpc_addrs = registration.rpc_addresses
                                    .into_iter()
                                    .map(HostPort::from)
                                    .collect::<Vec<_>>();

        let http_addrs = registration.http_addresses
                                     .into_iter()
                                     .map(HostPort::from)
                                     .collect::<Vec<_>>();

        let software_version = registration.software_version
                                           .expect_field(ENTRY, "software_version")?;
        let millis_since_heartbeat = tablet_server.millis_since_heartbeat
                                                  .expect_field(ENTRY, "millis_since_heartbeat")?;
        Ok(TabletServer {
            id: id,
            rpc_addrs: rpc_addrs,
            http_addrs: http_addrs,
            software_version,
            seqno: seqno,
            duration_since_heartbeat: Duration::from_millis(millis_since_heartbeat as u64),
        })
    }
}
