use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use bytes::Bytes;
use futures::sync::oneshot;

use Error;
use RequestBody;
use RpcResult;

/// An in-flight remote procedure call.
///
/// `Rpc` is the type-erased version of `Call`, with the addition of the completion handle. `Rpc`
/// is used internally to track an individual in-flight RPCs.
pub(crate) struct Rpc {
    pub service: &'static str,
    pub method: &'static str,
    pub required_feature_flags: &'static [u32],
    pub timestamp: Instant,
    pub deadline: Instant,
    pub request: Arc<RequestBody>,
    pub completer: oneshot::Sender<RpcResult>,
}

impl Rpc {
    /// Returns `true` if the RPC has been canceled by the caller.
    pub fn is_canceled(&self) -> bool {
        self.completer.is_canceled()
    }

    /// Returns `true` if the RPC is timed out.
    pub fn is_timed_out(&self, now: Instant) -> bool {
        self.deadline <= now
    }

    /// Completes the RPC.
    pub fn complete(self, body: Bytes, sidecars: Vec<Bytes>) {
        let _ = self.completer.send(Ok((body, sidecars)));
    }

    /// Fails the RPC.
    pub fn fail(self, error: Error) {
        let _ = self.completer.send(Err(error));
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dbg = f.debug_struct("Rpc");
        dbg.field("service", &self.service);
        dbg.field("method", &self.method);
        if !self.required_feature_flags.is_empty() {
            dbg.field("required_feature_flags", &self.required_feature_flags);
        }
        dbg.field("timestamp", &self.timestamp);
        dbg.field("deadline", &self.deadline);
        dbg.finish()
    }
}
