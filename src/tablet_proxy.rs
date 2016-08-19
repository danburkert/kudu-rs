use std::collections::HashSet;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use kudu_pb::master::{
    GetTabletLocationsRequestPB, GetTabletLocationsResponsePB,
};

use backoff::Backoff;
use Error;
use master::MasterProxy;
use queue_map::QueueMap;
use Result;
use rpc::{master, Messenger, Rpc};
use Tablet;
use TabletId;

/// Maximum number of RPCs to queue in the tablet server proxy during leader discovery. When the
/// queue is full, additional attempts to send RPCs will immediately fail with `RpcError::Backoff`.
const QUEUE_LEN: usize = 32;

/// The leader refresh ListMaster RPCs should have a long enough timeout that non-failed masters
/// can respond, but short enough that the RPCs don't stay queued forever.
const LEADER_REFRESH_TIMEOUT_SECS: u64 = 60;

/// The target replica of an RPC.
enum TargetReplica {
    /// Send the RPC to the tablet leader.
    Leader,

    /// Send the RPC to the closest replica.
    Closest,
}

enum Leader {
    /// The known leader.
    Known(SocketAddr),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    /// holds the queue of RPCs, and the next registered timeout deadline.
    Unknown(QueueMap<Rpc>),
}

impl Leader {
    fn is_known(&self) -> bool {
        match *self {
            Leader::Known(_) => true,
            Leader::Unknown(_) => false,
        }
    }
}

/// The `TabletProxy` tracks the replicas of a tablet, and proxies RPCs to them.
#[derive(Clone)]
pub struct TabletProxy {
    id: TabletId,
    inner: Arc<Mutex<Inner>>,
    messenger: Messenger,
    master_proxy: MasterProxy,
}

struct Inner {
    tablet: Tablet,
    leader: Leader,
    replicas: Vec<SocketAddr>,
    refresh_in_progress: bool,
}

impl TabletProxy {

    /// Sends the RPC if the leader replica is known, otherwise queues the RPC to be sent when the
    /// leader is discovered.
    fn send_to_leader(&self, rpc: Rpc) {
        /// Returns the queue index if the RPC is queued.
        fn inner(tablet_proxy: &TabletProxy, mut rpc: Rpc) -> Option<usize> {
            // Keep critical section short by performing the send outside the lock.
            let leader_addr = {
                let mut inner = tablet_proxy.inner.lock();
                match inner.leader {
                    Leader::Known(addr) => Some(addr),
                    Leader::Unknown(ref mut queue) => {
                        if queue.len() < QUEUE_LEN {
                            return Some(queue.push(rpc));
                        }
                        None
                    },
                }
            };
            if let Some(leader_addr) = leader_addr {
                rpc.addr = leader_addr;
                tablet_proxy.messenger.send(rpc);
            } else {
                rpc.fail(Error::Backoff);
            }
            None
        }

        // This control flow is a bit funky to keep the critical section short and still appease the
        // borrow checker.

        let now = Instant::now();
        // Make sure that the duration_since call below doesn't panic.
        if rpc.timed_out(now) {
            rpc.fail(Error::TimedOut);
            return;
        }
        let duration = rpc.deadline.duration_since(now);

        if let Some(queue_idx) = inner(self, rpc) {
            let tablet_proxy = self.clone();
            self.messenger.timer(duration, Box::new(move || {
                tablet_proxy.timeout_queued_rpc(queue_idx)
            }));
        }
    }

    /// Times out the queued RPC with the given index.
    fn timeout_queued_rpc(&self, queue_idx: usize) {
        // Keep the critical section short.
        let mut rpc = None;
        if let Leader::Unknown(ref mut queue) = self.inner.lock().leader {
            rpc = queue.remove(queue_idx);
        }
        if let Some(rpc) = rpc {
            // Warning: extreme hack. We can get 'false positive' timeout callbacks when we
            // transition from Unknown -> Known -> Unknown, because the queue resets the idx
            // counter back to 0, so we may get the callback from a previous Unkown era. To
            // properly fix this we either need to keep the timeout handle with the queued RPCs,
            // like in Connection, or keep a transition counter or something. Easier than all that
            // is to just check if the RPC is actually timed out.
            if rpc.timed_out(Instant::now()) {
                rpc.fail(Error::TimedOut);
            } else {
                self.send_to_leader(rpc);
            }
        }
    }

    /// Clears the leader cache if the currently cached leader matches the provided address.
    /// If the cache is cleared, a refresh is initiated.
    fn reset_inner(&self, stale_leader: SocketAddr) {
        let queue = QueueMap::with_capacity(QUEUE_LEN);
        {
            let mut inner = self.inner.lock();
            match inner.leader {
                // Do nothing if the cached leader has already been refreshed.
                Leader::Unknown(_) => return,
                Leader::Known(leader) if leader != stale_leader => return,
                // Otherwise fall through to cache eviction and refresh.
                _ => (),
            }
            inner.leader = Leader::Unknown(queue);

            if inner.refresh_in_progress { return; }
            inner.refresh_in_progress = true;
        }
        self.refresh_tablet_locations();
    }

    /// Asynchronously refreshes the cached tablet locations.
    fn refresh_tablet_locations(&self) {
        debug_assert!(self.inner.lock().refresh_in_progress);
        let cancel = Arc::new(AtomicBool::new(false));
        debug!("refreshing locations for tablet {:?}", self.id);

        // The real leader address will be filled in by `send_to_leader`.
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let deadline = Instant::now() + Duration::from_secs(LEADER_REFRESH_TIMEOUT_SECS);
        let mut request = GetTabletLocationsRequestPB::new();
        request.mut_tablet_ids().push(self.id.as_bytes().as_ref().to_owned());
        let mut rpc = master::get_tablet_locations(addr, deadline, request);
        let proxy = self.clone();

        // Backoff that manages backoffs between retries.
        let backoff = Backoff::with_duration_range(10, 30_000);

        rpc.callback = Some(Box::new(move |result, rpc| {
            thread::spawn(move || proxy.handle_get_tablet_locations_response(result, rpc, backoff));
        }));

        self.master_proxy.send_to_leader(rpc);
    }

    fn handle_get_tablet_locations_response(self,
                                            result: Result<()>,
                                            mut rpc: Rpc,
                                            backoff: Backoff) {

        let mut replicas: HashSet<SocketAddr> = HashSet::new();

        let mut leader: HashSet<SocketAddr> = HashSet::new();

        if let Err(error) = result {
            info!("{:?}: GetTabletLocations RPC failed: {}", self, error);
            // Fall through to retry.
        } else {
            let response = rpc.mut_response::<GetTabletLocationsResponsePB>();
            if response.has_error() {

            } else {
            }
        }

    }
}

impl fmt::Debug for TabletProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabletProxy {{ id: {} }}", self.id)
    }
}
