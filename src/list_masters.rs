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
use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend};
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};
use kudu_pb::master::{ListMastersRequestPB, ListMastersResponsePB};
use parking_lot::Mutex;
use tokio_timer;
use take_mut;

use Error;
use MasterError;
use MasterErrorCode;
use MasterId;
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

pub struct ListMasters {
    io: Io,
    state: State,
    backoff: Backoff,
}

enum State {
    StartSend(RpcFuture, Rpc),
    /// The `ListMasters` request is being sent to the master.
    Send(RpcFuture),
    /// The `ListMasters` response hosts are being resolved.
    Resolve(ListMastersFuture),
    /// The `ListMasters` request will be retried after a backoff period.
    Retry(tokio_timer::Sleep, Rpc),
}

impl State {
    fn send(&mut self) -> &mut RpcFuture {
        match *self {
            State::Send(ref mut send) => send,
            _ => unreachable!(),
        }
    }
    fn resolve(&mut self) -> &mut ListMastersFuture {
        match *self {
            State::Resolve(ref mut resolve) => resolve,
            _ => unreachable!(),
        }
    }
    fn retry(&mut self) -> &mut tokio_timer::Sleep {
        match *self {
            State::Retry(ref mut sleep, ..) => sleep,
            _ => unreachable!(),
        }
    }
    fn kind(&self) -> StateKind {
        match *self {
            State::StartSend(..) => StateKind::StartSend,
            State::Send(..) => StateKind::Send,
            State::Resolve(..) => StateKind::Resolve,
            State::Retry(..) => StateKind::Retry,
        }
    }
}

enum StateKind {
    StartSend,
    Send,
    Resolve,
    Retry,
}

pub enum ListMastersResponse {
    Leader(SocketAddr, Vec<SocketAddr>),
    Replicas(Vec<SocketAddr>),
}

pub type ListMastersFuture = Box<Future<Item=ListMastersResponse, Error=()>>;

impl ListMasters {
    fn retry(&mut self, rpc: Rpc) -> Poll<ListMastersResponse, ()> {
        let sleep = self.io.timer().sleep(self.backoff.next_backoff());
        self.state = State::Retry(sleep, rpc);
        self.poll_retry()
    }

    fn poll_start_send(&mut self) -> Poll<ListMastersResponse, ()> {
        {
            let ListMasters { ref mut io, ref mut state, .. } = *self;
            take_mut::take(state, |state| {
                if let State::StartSend(rpc_future, rpc) = state {
                    match io.messenger().start_send(rpc).unwrap() {
                        AsyncSink::Ready => State::Send(rpc_future),
                        AsyncSink::NotReady(rpc) => State::StartSend(rpc_future, rpc),
                    }
                } else { unreachable!() }
            });
        }
        match self.state.kind() {
            StateKind::StartSend => Ok(Async::NotReady),
            StateKind::Send => self.poll_send(),
            _ => unreachable!()
        }
    }

    fn poll_send(&mut self) -> Poll<ListMastersResponse, ()> {
        match self.state.send().poll() {
            Ok(Async::Ready(mut rpc)) => {
                let addr = rpc.addr;
                {
                    let response = rpc.mut_response::<ListMastersResponsePB>();
                    if response.has_error() {
                        // This can happen when the master is not yet initialized, e.g.:
                        // code: CATALOG_MANAGER_NOT_INITIALIZED
                        // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
                        info!("ListMasters RPC to master {} failed: {:?}", addr, response.get_error());
                        // Fall through to retry.
                    } else {
                        debug!("ListMasters RPC to master {} response: {:?}", addr, response);

                        let mut responses = Vec::with_capacity(response.get_masters().len());
                        for server_entry in response.mut_masters().iter_mut() {
                            if server_entry.has_error()  {
                                continue;
                            }
                            let role = server_entry.get_role();
                            let resp = self.io.resolver().resolve(server_entry.mut_registration()
                                                                              .take_rpc_addresses()
                                                                              .into_vec())
                                           .map(move |addrs| {
                                               if role == Role::LEADER && addrs.contains(&addr) {
                                                   ListMastersResponse::Leader(addr, addrs)
                                               } else {
                                                   ListMastersResponse::Replicas(addrs)
                                               }
                                           });
                            responses.push(resp);
                        }
                        unimplemented!()
                    }
                }
                self.retry(rpc)
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(RpcError { rpc, error }) => {
                info!("ListMasters RPC to master {} failed: {}", rpc.addr, error);
                self.retry(rpc)
            }
        }
    }

    fn poll_retry(&mut self) -> Poll<ListMastersResponse, ()> {
        {
            let ListMasters { ref mut state, .. } = *self;
            try_ready!(state.retry().poll());
            take_mut::take(state, |state| {
                if let State::Retry(_, mut rpc) = state {
                    let rpc_future = rpc.future();
                    State::StartSend(rpc_future, rpc)
                } else { unreachable!() }
            });
        }
        self.poll_start_send()
    }

    fn poll_resolve(&mut self) -> Poll<ListMastersResponse, ()> {
        self.state.resolve().poll()
    }
}

impl Future for ListMasters {
    type Item = ListMastersResponse;
    type Error = ();
    fn poll(&mut self) -> Poll<ListMastersResponse, ()> {
        match self.state.kind() {
            StateKind::StartSend => self.poll_start_send(),
            StateKind::Send => self.poll_send(),
            StateKind::Resolve => self.poll_resolve(),
            StateKind::Retry => self.poll_resolve(),

        }
    }
}
