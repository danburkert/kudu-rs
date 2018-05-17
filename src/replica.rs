#![allow(unused_imports)]

use std::collections::{
    VecDeque,
    HashMap,
};
use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::{Duration, Instant};

use bytes::Bytes;
use futures::{
    Async,
    Future,
    Poll,
    Stream,
};
use futures::stream::FuturesUnordered;
use krpc::{
    Call,
    Proxy,
    RpcFuture,
};
use prost::Message;
use tokio_timer::Delay;

use Error;
use MasterError;
use MasterErrorCode;
use TabletServerError;
use TabletServerErrorCode;
use backoff::Backoff;
use retry::Retriable;
use util::ContextFuture;

/// The strategy for speculative execution.
pub enum Speculation {
    Full,
    Staggered(Duration),
}

/// The policy for replica selection.
#[derive(PartialEq)]
pub(crate) enum Selection {
    Leader,
    Closest,
}

impl Selection {

    /// Sorts a set of replicas into a prioritized queue of offsets.
    fn prioritize<R>(&self, replicas: &[R], backoff: Backoff) -> VecDeque<ReplicaState> where R: Replica {
        let mut queue = VecDeque::with_capacity(replicas.len());
        match self {
            Selection::Leader => for (idx, replica) in replicas.iter().enumerate() {
                if replica.is_stale() {
                    continue;
                } else if replica.is_leader() {
                    queue.push_front(ReplicaState::new(idx, backoff.clone(), replica.proxy()));
                } else {
                    queue.push_back(ReplicaState::new(idx, backoff.clone(), replica.proxy()));
                }
            },
            // TODO: Figure out how to find who's closest.
            Selection::Closest => for (idx, replica) in replicas.iter().enumerate() {
                if replica.is_stale() {
                    continue;
                } else {
                    queue.push_back(ReplicaState::new(idx, backoff.clone(), replica.proxy()));
                }
            },
        }
        queue
    }
}

pub(crate) trait Replica {

    /// Returns a KRPC proxy to the tablet server hosting the replica.
    fn proxy(&self) -> Proxy;

    /// Returns true if the replica is a leader.
    fn is_leader(&self) -> bool;

    /// Sets the replica state to leader.
    fn mark_leader(&self);

    /// Sets the replica state to follower.
    fn mark_follower(&self);

    /// Returns true if the replica location is stale.
    fn is_stale(&self) -> bool;

    /// Marks the replica location as stale.
    fn mark_stale(&self);
}

pub(crate) trait ReplicaSet {
    type Replica: Replica;

    fn replicas(&self) -> &[Self::Replica];
}

/// A container holding the internal, intermediate state of a replica.
struct ReplicaState {
    index: usize,
    proxy: Proxy,
    backoff: Backoff,
    failure: Option<Error>,
}

impl ReplicaState {
    fn new(index: usize, backoff: Backoff, proxy: Proxy) -> ReplicaState {
        ReplicaState { index, backoff, proxy, failure: None }
    }
}

impl fmt::Debug for ReplicaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReplicaState")
            .field("index", &self.index)
            .field("backoff", &self.backoff)
            .field("failure", &self.failure)
            .finish()
    }
}

#[must_use = "futures do nothing unless polled"]
pub(crate) struct ReplicaRpc<Set, Req, Resp>
where Set: ReplicaSet,
      Req: Message + 'static,
      Resp: Retriable {
    replica_set: Set,
    call: Call<Req, Resp>,
    speculation: Speculation,
    selection: Selection,

    speculative_timer: Option<Delay>,

    /// Holds replicas which have not yet been attempted.
    queue: VecDeque<ReplicaState>,

    /// Holds replicas which have in-flight RPCs.
    in_flight: FuturesUnordered<ContextFuture<RpcFuture<Resp>, ReplicaState>>,

    /// Holds replicas which have failed with a retriable error, and are scheduled to be retried
    /// after a backoff period.
    backoff: FuturesUnordered<ContextFuture<Delay, ReplicaState>>,

    /// Holds replicas which have failed with non-retriable errors.
    failures: Vec<ReplicaState>,
}

impl <Set, Req, Resp> ReplicaRpc<Set, Req, Resp>
where Set: ReplicaSet,
      Req: Message + 'static,
      Resp: Retriable {

    pub(crate) fn new(replica_set: Set,
                      call: Call<Req, Resp>,
                      speculation: Speculation,
                      selection: Selection,
                      backoff: Backoff)
                      -> ReplicaRpc<Set, Req, Resp> {

        let queue = selection.prioritize(replica_set.replicas(), backoff);
        ReplicaRpc {
            replica_set,
            call,
            speculation,
            selection,
            queue,
            in_flight: FuturesUnordered::new(),
            backoff: FuturesUnordered::new(),
            speculative_timer: None,
            failures: Vec::new(),
        }
    }

    fn speculation_timer_is_ready(&mut self) -> bool {
        self.speculative_timer.as_mut().map_or(true, |sleep| {
            sleep.poll().expect("timer failed").is_ready()
        })
    }

    fn reset_speculation_timer(&mut self, duration: Duration) {
        let mut sleep = Delay::new(Instant::now() + duration);
        // Poll the timer in order to schedule this task for wakeup on expiry.
        // TODO: can the timer be initially complete?
        assert!(sleep.poll().expect("timer failed").is_not_ready());
        self.speculative_timer = Some(sleep);
    }

    /// Dispatches queued RPCs according to the speculative execution policy.
    /// Returns true if RPCs have been dispatched.
    fn dispatch_rpcs(&mut self) -> bool {
        let in_flight_count = self.in_flight.len();
        if !self.queue.is_empty() {
            match self.speculation {
                Speculation::Full => for mut replica in self.queue.drain(..) {
                    // Completely drain the queue and issue RPCs against all replicas.
                    let rpc = replica.proxy.send(self.call.clone());
                    let context = ContextFuture::new(rpc, replica);
                    self.in_flight.push(context);
                },
                Speculation::Staggered(duration) => {
                    if self.speculation_timer_is_ready() {
                        let mut replica = self.queue.pop_front().unwrap();
                        let rpc = replica.proxy.send(self.call.clone());
                        let context = ContextFuture::new(rpc, replica);
                        self.in_flight.push(context);
                        self.reset_speculation_timer(duration);
                    }
                },
            }
        }

        while let Async::Ready(Some((_, mut replica))) = self.backoff.poll().unwrap() {
            let rpc = replica.proxy.send(self.call.clone());
            let context = ContextFuture::new(rpc, replica);
            self.in_flight.push(context);
        }
        in_flight_count != self.in_flight.len()
    }

    fn poll_in_flight(&mut self) -> Poll<(Proxy, Resp, Vec<Bytes>), Error> {
        loop {
            let (response, mut replica) = match self.in_flight.poll() {
                Ok(Async::Ready(None)) | Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(Some(((resp, sidecars), replica)))) => {
                    let response = resp.into_result().map(|resp| (resp, sidecars));
                    (response, replica)
                },
                Err((error, replica)) => (Err(error.into()), replica),
            };

            match response {
                Ok((response, sidecars)) => {
                    if self.selection == Selection::Leader {
                        self.replica_set.replicas()[replica.index].mark_leader();
                    }
                    return Ok(Async::Ready((replica.proxy, response, sidecars)))
                },

                | Err(error@Error::TabletServer(TabletServerError { code: TabletServerErrorCode::TabletNotFound, .. }))
                | Err(error@Error::TabletServer(TabletServerError { code: TabletServerErrorCode::TabletFailed, .. })) => {
                    self.replica_set.replicas()[replica.index].mark_stale();
                    match self.selection {
                        Selection::Leader => {
                            // If a tablet replica has failed and this is a leader-only RPC then
                            // it's not guaranteed that our view of the tablet config includes the
                            // leader. In order to make progress we must fail fast, and in a
                            // higher-level componenet re-request the tablet config from the meta
                            // cache and retry.
                            //
                            // TODO: implement the higher level retry mechanism mentioned above.
                            return Err(error);
                        },
                        Selection::Closest => {
                            // If we aren't relying on finding the leader then we can continue
                            // trying the RPC at other replicas.
                            replica.failure = Some(error);
                            self.failures.push(replica);
                        }
                    }
                },

                | Err(error@Error::Master(MasterError { code: MasterErrorCode::NotTheLeader, .. }))
                | Err(error@Error::Master(MasterError { code: MasterErrorCode::CatalogManagerNotInitialized, .. }))
                | Err(error@Error::TabletServer(TabletServerError { code: TabletServerErrorCode::NotTheLeader, .. }))
                | Err(error@Error::TabletServer(TabletServerError { code: TabletServerErrorCode::TabletNotRunning, .. }))
                => {
                    self.replica_set.replicas()[replica.index].mark_follower();
                    // Retry the RPC after a backoff period.
                    replica.failure = Some(error);
                    let mut backoff = Delay::new(Instant::now() + replica.backoff.next_backoff());
                    let context = ContextFuture::new(backoff, replica);
                    self.backoff.push(context);
                }

                | Err(error@Error::Io(..)) => {
                    // IO errors are non-retriable, however they are not fatal.
                    replica.failure = Some(error);
                    self.failures.push(replica);
                },

                // TODO: handle timed out error.  Probably by gathering up all of the errors that
                // have been encountered and creating a compound error.

                // All other error kinds are non-retriable application errors. Fail fast.
                Err(error) => return Err(error),
            }
        }
    }
}

impl <Set, Req, Resp> Future for ReplicaRpc<Set, Req, Resp>
where Set: ReplicaSet,
      Req: Message + 'static,
      Resp: Retriable {

    type Item = (Proxy, Resp, Vec<Bytes>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Error> {
        loop {
            // Poll in-flight RPCs for completion.
            if let Async::Ready(item) = self.poll_in_flight()? {
                return Ok(Async::Ready(item));
            }

            // Dispatch RPCs if possible. If any RPCs are dispatched, go back to step 1 in order to
            // poll them.
            if self.dispatch_rpcs() {
                continue;
            }

            // Check if all RPCs are failed.

            if self.queue.is_empty() && self.in_flight.is_empty() && self.backoff.is_empty() {
                debug_assert!(self.failures.len() > 0);
                if self.failures.len() == 1 {
                    return Err(self.failures.pop().unwrap().failure.unwrap());
                }

                let errors = self.failures
                                 .iter_mut()
                                 .map(|failure| failure.failure.take().unwrap())
                                 .collect::<Vec<_>>();

                return Err(Error::Compound("failed replicated RPC".to_string(), errors));
            }

            return Ok(Async::NotReady);
        }
    }
}

// Proxy is it's own Replica/ReplicaSet. This allows Proxy to reuse the same retry logic, without
// actually having different replicas.

impl ReplicaSet for Proxy {
    type Replica = Proxy;
    fn replicas(&self) -> &[Proxy] {
        // TODO(rust-lang/rust#45703): replace with slice::from_ref.
        unsafe {
            slice::from_raw_parts(self, 1)
        }
    }
}

impl Replica for Proxy {
    fn proxy(&self) -> Proxy {
        self.clone()
    }

    fn is_leader(&self) -> bool {
        unimplemented!()
    }

    fn mark_leader(&self) {
        unimplemented!()
    }

    fn mark_follower(&self) {
        unimplemented!()
    }

    fn is_stale(&self) -> bool {
        unimplemented!()
    }

    fn mark_stale(&self) {
        unimplemented!()
    }
}
