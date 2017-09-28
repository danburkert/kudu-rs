use std::marker::PhantomData;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use std::any::Any;

use futures::{Async, AsyncSink, Future, Poll, Sink, Stream};
use parking_lot::Mutex;
use pb::master::{
    AlterTableRequestPb, AlterTableResponsePb,
    CreateTableRequestPb, CreateTableResponsePb,
    DeleteTableRequestPb, DeleteTableResponsePb,
    GetTableLocationsRequestPb, GetTableLocationsResponsePb,
    GetTableSchemaRequestPb, GetTableSchemaResponsePb,
    GetTabletLocationsRequestPb, GetTabletLocationsResponsePb,
    IsAlterTableDoneRequestPb, IsAlterTableDoneResponsePb,
    IsCreateTableDoneRequestPb, IsCreateTableDoneResponsePb,
    ListMastersRequestPb, ListMastersResponsePb,
    ListTablesRequestPb, ListTablesResponsePb,
    ListTabletServersRequestPb, ListTabletServersResponsePb,
    PingRequestPb, PingResponsePb,
};
use pb::{ServerEntryPb as MasterEntry};
use futures::sync::{mpsc, oneshot};
use tokio::reactor::Handle;
use tokio_timer;

use backoff::Backoff;
use dns::Resolver;
use list_masters::find_leader_master;
use proto::Message;
use rpc::{
    Connection,
    Rpc,
    master,
};
use util;
use Error;
use MasterError;
use MasterErrorCode;
use MasterId;
use RaftRole;
use Result;
use Status;

pub struct MasterProxy {
    sender: mpsc::Sender<Rpc<RpcState>>,
}

struct RpcState {
    send: oneshot::Sender<Result<Box<Message>>>,

    /// Method-specific function which inspects the RPC response message and
    /// lifts out the master error, if it exists.
    master_error: fn(Box<Message>) -> Result<Box<Message>>,
}

enum Leader {
    /// The known leader.
    Known(Connection<RpcState>, Vec<SocketAddr>),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    Unknown(Box<Future<Item=(Connection<RpcState>, Vec<SocketAddr>), Error=!>>),
}

impl Leader {

    fn poll_cxn(&mut self) -> Poll<&mut Connection<RpcState>, !> {
        loop {
            let (cxn, replicas) = match *self {
                Leader::Known(ref mut cxn, ..) => return Ok(Async::Ready(cxn)),
                Leader::Unknown(ref mut future) => try_ready!(future.poll()),
            };
            *self = Leader::Known(cxn, replicas);
        }
    }

}

struct MasterProxyTask {
    reactor: Handle,
    resolver: Resolver,
    timer: tokio_timer::Timer,
    leader: Leader,
    queue: mpsc::Receiver<Rpc<RpcState>>,
    retry: Vec<Rpc<RpcState>>,
}

/// Returns `true` if the RPC should be retried as a result of the error.
fn should_retry(error: &Error) -> bool {
    match *error {
        Error::Io(..) => true,
        Error::InvalidArgument(..) => false,
        Error::Rpc(ref error) => error.is_fatal(),
        Error::Master(ref error) => error.code() == MasterErrorCode::NotTheLeader ||
                                    error.code() == MasterErrorCode::CatalogManagerNotInitialized,
        Error::Serialization(..) => false,
        Error::TimedOut => false,
        Error::TabletServer(..) |
            Error::NegotiationError(..) |
            Error::NoRangePartition => unreachable!(),
    }
}

/// Returns `true` if the error should cause the leader cache to be reset.
fn should_reset(error: &Error) -> bool {
    match *error {
        Error::Master(ref error) => error.code() == MasterErrorCode::NotTheLeader ||
                                    error.code() == MasterErrorCode::CatalogManagerNotInitialized,
        _ => false,
    }
}

impl MasterProxyTask {

    fn try_poll(&mut self) -> Poll<(), ()> {
        let MasterProxyTask {
            ref reactor,
            ref resolver,
            ref timer,
            ref mut leader,
            ref mut queue,
            ref mut retry,
        } = *self;

        // Unwrap the leader connection.
        let cxn = try_ready!(leader.poll_cxn().map_err(|_| ()));

        // Step 1: Complete in-flight RPCs.
        loop {
            match cxn.poll().unwrap() {
                Async::Ready(Some(mut rpc)) => {
                    let state = rpc.take_state().unwrap();
                    let result = rpc.take_result().and_then(state.master_error);
                    match result {
                        Ok(msg) => state.send.complete(Ok(msg)),
                        Err(error) => {
                            if cxn.is_shutdown {
                                // The connection was shutdown as a result of the error.
                                rpc.set_state(state);
                                self.retries.push(rpc);
                            } else if should_reset(&error) {
                                cxn.shutdown();
                                rpc.set_state(state);
                                self.retries.push(rpc);
                            } else {
                                state.send.complete(Err(error));
                            }
                        },
                    }
                },
                Async::Ready(None) => return Err(()),
                Async::NotReady => break,
            }
        }

        // Step 2: Send queued RPCs to the connection.
        /*
        while let Some(rpc) = queue.poll().unwrap() {
            if let AsyncSink::NotReady(rpc) = cxn.start_send(rpc)? {
                assert!(retry.clone().start_send(rpc).unwrap().is_ready());
                break;
            }
        }
        */

        // Step 3: Flush sent RPCs.
        /*
        cxn.poll_complete()
        */
        unimplemented!()
    }
}

impl Future for MasterProxyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        /*
        if let Err(error) = self.try_poll() {
            // Reset the leader connection.
            let replicas = match self.leader {
                Leader::Known(ref mut cxn, ref mut replicas) => {
                    cxn.shutdown();
                    while let Ok(Async::Ready(Some(mut rpc))) = cxn.poll() {
                        let state = rpc.take_state().unwrap();
                        let result = rpc.take_result().and_then(state.master_error);

                        match result {
                            Ok(msg) => state.send.complete(Ok(msg)),
                            Err(error) => if should_reset(&error) {
                                assert!(retry.clone().start_send(rpc).unwrap().is_ready());
                            } else {
                                state.send.complete(Err(error));
                            },
                        }
                    }
                    mem::replace(replicas, Vec::new());
                },
                Leader::Unknown(..) => return,
            };

            let leader = find_leader_master(reactor.clone(),
                                            resolver.clone(),
                                            timer.clone(),
                                            Backoff::with_duration_range(1_000, 60_000),
                                            replicas);
            self.leader = leader;
        }
        */
        Ok(Async::NotReady)
    }
}

pub trait MasterResponse: Message + Any {
    fn error(&mut self) -> Option<MasterError>;
}
macro_rules! impl_master_response {
    ($response_type:ident) => {
        impl MasterResponse for $response_type {
            fn error(&mut self) -> Option<MasterError> {
                if self.has_error() { Some(MasterError::from(self.take_error())) } else { None }
            }
        }
    };
}
impl_master_response!(AlterTableResponsePB);
impl_master_response!(CreateTableResponsePB);
impl_master_response!(DeleteTableResponsePB);
impl_master_response!(GetTableLocationsResponsePB);
impl_master_response!(GetTableSchemaResponsePB);
impl_master_response!(GetTabletLocationsResponsePB);
impl_master_response!(IsAlterTableDoneResponsePB);
impl_master_response!(IsCreateTableDoneResponsePB);
impl_master_response!(ListTablesResponsePB);
impl_master_response!(ListTabletServersResponsePB);
impl MasterResponse for PingResponsePB {
    fn error(&mut self) -> Option<MasterError> {
        None
    }
}
impl MasterResponse for ListMastersResponsePB {
    fn error(&mut self) -> Option<MasterError> {
        if self.has_error() {
            Some(MasterError::new(MasterErrorCode::UnknownError, Status::from(self.take_error())))
        } else { None }
    }
}
