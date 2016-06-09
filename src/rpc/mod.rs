use std::any::Any;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{SyncSender, SendError};
use std::time::Instant;

use protobuf::Message;

pub use rpc::messenger::Messenger;

use Error;
use Result;

mod connection;
pub mod master;
mod messenger;
pub mod tablet_server;

/// A callback that will be executed when the RPC is complete. If the RPC succeeds, the result will
/// be `Ok`, and the RPC will contain the response and sidecars. Othewise, the result will contain
/// the failure.
pub trait Callback: Send {
    fn callback(self: Box<Self>, result: Result<()>, rpc: Rpc);
}

impl<F> Callback for F where F: FnOnce(Result<()>, Rpc) + Send {
    fn callback(self: Box<F>, result: Result<()>, rpc: Rpc) {
        (*self)(result, rpc)
    }
}

/// Returns a callback which will send the result and RPC to a sync channel on completion. The
/// caller should ensure that the channel has sufficient capcity for the callback, otherwise the
/// RPC I/O thread will be blocked.
pub fn channel_callback(sender: SyncSender<(Result<()>, Rpc)>) -> Box<Callback> {
    Box::new(move |result, rpc| {
        if let Err(SendError((result, rpc))) = sender.send((result, rpc)) {
            warn!("callback channel disconnected, result: {:?}, rpc: {:?}", result, rpc)
        }
    })
}

pub struct Rpc {
    pub addr: SocketAddr,
    pub service_name: &'static str,
    pub method_name: &'static str,
    pub deadline: Instant,
    pub required_feature_flags: Vec<u32>,
    pub request: Box<Message>,
    pub response: Box<Message>,
    pub sidecars: Vec<Vec<u8>>,
    pub callback: Option<Box<Callback>>,
    pub cancel: Option<Arc<AtomicBool>>,
}

impl Rpc {
    fn complete(mut self) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Ok(()), self)
        }
    }

    fn fail(mut self, error: Error) {
        if let Some(callback) = self.callback.take() {
            callback.callback(Err(error), self)
        }
    }

    pub fn response<T>(&self) -> &T where T: Any {
        self.response.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn mut_response<T>(&mut self) -> &mut T where T: Any {
        self.response.as_any_mut().downcast_mut::<T>().unwrap()
    }

    /// Returns `true` if this RPC has been requested to be cancelled.
    pub fn cancelled(&self) -> bool {
        self.cancel.as_ref().map(|b| b.load(Ordering::Relaxed)).unwrap_or(false)
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    pub fn timed_out(&self, now: Instant) -> bool {
        self.deadline <= now
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rpc {{ {}::{}, deadline: {:?} }}",
               self.service_name, self.method_name, self.deadline)
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use mini_cluster::{MiniCluster, MiniClusterConfig};
    use super::*;

    use env_logger;
    use kudu_pb;

    #[test]
    fn test_master_ping() {
        let _ = env_logger::init();
        let cluster = MiniCluster::new(MiniClusterConfig::default()
                                                         .num_tservers(0)
                                                         .log_rpc_negotiation_trace(true)
                                                         .log_rpc_trace(true));
        let messenger = Messenger::new().unwrap();
        let rpc = master::ping(cluster.master_addrs()[0],
                               Instant::now() + Duration::from_secs(5),
                               kudu_pb::master::PingRequestPB::new());

        let (result, _rpc) = messenger.send_sync(rpc);
        result.unwrap();
    }
}
