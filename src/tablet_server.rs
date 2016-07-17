use std::time::Duration;

use kudu_pb::master::{ListTabletServersResponsePB_Entry as TabletServerEntry};

use Result;
use TabletServerId;

/// Tablet server metadata.
///
/// This information should be considered 'point-in-time', and may change as the cluster topology
/// changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabletServer {
    id: TabletServerId,
    rpc_addrs: Vec<(String, u16)>,
    http_addrs: Vec<(String, u16)>,
    software_version: String,
    seqno: i64,
    duration_since_heartbeat: Duration,
}

impl TabletServer {
    pub fn id(&self) -> &TabletServerId {
        &self.id
    }

    pub fn rpc_addrs(&self) -> &[(String, u16)] {
        &self.rpc_addrs
    }

    pub fn http_addrs(&self) -> &[(String, u16)] {
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

    #[doc(hidden)]
    pub fn from_pb(mut tablet_server: TabletServerEntry) -> Result<TabletServer> {
        let id = try!(TabletServerId::parse_bytes(tablet_server.get_instance_id().get_permanent_uuid()));
        let seqno = tablet_server.get_instance_id().get_instance_seqno();

        // TODO: check bounds on port casts.
        let rpc_addrs = tablet_server.mut_registration()
                                     .take_rpc_addresses()
                                     .into_iter()
                                     .map(|mut host_port| (host_port.take_host(),
                                                           host_port.get_port() as u16))
                                     .collect::<Vec<_>>();
        let http_addrs = tablet_server.mut_registration()
                                      .take_http_addresses()
                                      .into_iter()
                                      .map(|mut host_port| (host_port.take_host(),
                                                            host_port.get_port() as u16))
                                      .collect::<Vec<_>>();

        let software_version = tablet_server.mut_registration().take_software_version();
        let duration_since_heartbeat = Duration::from_millis(tablet_server.get_millis_since_heartbeat() as u64);
        Ok(TabletServer {
            id: id,
            rpc_addrs: rpc_addrs,
            http_addrs: http_addrs,
            software_version: software_version,
            seqno: seqno,
            duration_since_heartbeat: duration_since_heartbeat,
        })
    }
}
