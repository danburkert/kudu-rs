use std::fmt;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use futures::sync::oneshot;
use futures::{Async, Future, Poll, Sink, StartSend};
use parking_lot::Mutex;
use tokio_timer;

use Error;
use MasterError;
use MasterErrorCode;
use MasterId;
use RaftRole;
use Result;
use Status;
use backoff::Backoff;
use dns;
use io::Io;
use itertools::Itertools;
use protobuf::Message;
use queue_map::QueueMap;
use rpc::{
    Messenger,
    Rpc,
    RpcError,
    RpcFuture,
    RpcResult,
    master
};
use util;

pub struct MasterProxy {
    inner: Arc<Mutex<Leader>>,
    io: Io,
}

/// Container for master metadata.
struct Inner {
    leader: Leader,
    replicas: HashSet<SocketAddr>,
}

enum Leader {
    /// The known leader.
    Known(SocketAddr),
    /// The leader is unknown.
    Unknown,
}


impl MasterProxy {

    /*
    fn send_list_masters(&self, addr: SocketAddr, deadline: Instant) -> Box<Future<Item=SocketAddr>> {
        unimplemented!()

    }
    */
}

impl Sink for MasterProxy {
    type SinkItem = Rpc;
    type SinkError = ();

    fn start_send(&mut self, mut rpc: Rpc) -> StartSend<Rpc, ()> {
        info!("{:?}: start_send, rpc: {:?}", self, rpc);
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), ()> {
        Ok(Async::Ready(()))
    }
}

impl fmt::Debug for MasterProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MasterProxy {{ }}")
    }
}

/*
    /// Sends a `ListMasters` RPC to a master with the provided deadline and cancellation token.
    /// The completion handler is set to `handle_list_masters_response`.
    fn send_list_masters(&self, addr: SocketAddr, deadline: Instant, cancel: Arc<AtomicBool>) {
        trace!("sending ListMasters RPC to {}", addr);

        // Backoff that manages backoffs between retries.
        let backoff = Backoff::with_duration_range(10, 30_000);

        let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
        rpc.cancel = Some(cancel);
        // This RPC can't be retried at another replica, so the RPC layer should retry it
        // automatically.
        rpc.fail_fast = false;
        let proxy = self.clone();
        rpc.callback = Some(Box::new(move |result, rpc: Rpc| {
            thread::spawn(move || proxy.handle_list_masters_response(result, rpc, backoff));
        }));
        self.messenger.send(rpc);
    }
*/
