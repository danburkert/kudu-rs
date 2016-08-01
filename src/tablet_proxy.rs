use std::collections::HashMap;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use parking_lot::Mutex;

use rpc::{Messenger, Rpc};
use master::MasterProxy;

/// Maximum number of RPCs to queue in the tablet server proxy during leader discovery. When the
/// queue is full, additional attempts to send RPCs will immediately fail with `RpcError::Backoff`.
const QUEUE_LEN: usize = 32;

/// The leader refresh ListMaster RPCs should have a long enough timeout that non-failed tablet
/// servers can respond, but short enough that the RPCs don't stay queued forever.
const LEADER_REFRESH_TIMEOUT_SECS: u64 = 10;

/// The target replica of an RPC.
enum TargetReplica {
    Leader,
    Closest,
    Exact(SocketAddr),
}

/// The `TabletProxy` tracks the replicas of a tablet, and proxies RPCs to them.
#[derive(Clone)]
pub struct TabletProxy {
    inner: Arc<Mutex<Inner>>,
    messenger: Messenger,
    master_proxy: MasterProxy,
}

enum Leader {
    /// The known leader.
    Known(SocketAddr),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    /// holds the queue of RPCs, and the next registered timeout deadline.
    Unknown {
        queue: Vec<Rpc>,
        deadline: Option<Instant>,
    },
}

impl Leader {
    fn is_known(&self) -> bool {
        match *self {
            Leader::Known(_) => true,
            Leader::Unknown { .. } => false,
        }
    }
}

struct Inner {
    leader: SocketAddr,
    replicas: HashMap<SocketAddr, ReplicaState>,
}

struct ReplicaState {
    circuit_breaker: CircuitBreaker,
}

// TODO: replace with a real implementation
struct CircuitBreaker;
