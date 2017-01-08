use std::marker::PhantomData;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use futures::{Async, AsyncSink, Future, Poll, Sink};
use parking_lot::Mutex;
use kudu_pb::master::{
    AlterTableRequestPB, AlterTableResponsePB,
    CreateTableRequestPB, CreateTableResponsePB,
    DeleteTableRequestPB, DeleteTableResponsePB,
    GetTableLocationsRequestPB, GetTableLocationsResponsePB,
    GetTableSchemaRequestPB, GetTableSchemaResponsePB,
    GetTabletLocationsRequestPB, GetTabletLocationsResponsePB,
    IsAlterTableDoneRequestPB, IsAlterTableDoneResponsePB,
    IsCreateTableDoneRequestPB, IsCreateTableDoneResponsePB,
    ListMastersRequestPB, ListMastersResponsePB,
    ListTablesRequestPB, ListTablesResponsePB,
    ListTabletServersRequestPB, ListTabletServersResponsePB,
    PingRequestPB, PingResponsePB,
};
use kudu_pb::wire_protocol::{ServerEntryPB as MasterEntry};
use futures::sync::{mpsc, oneshot};
use tokio::reactor::Handle;
use tokio_timer;

use backoff::Backoff;
use dns::Resolver;
use list_masters::find_leader_master;
use protobuf::Message;
use rpc::{
    Connection,
    Rpc,
    FailedRpc,
    RpcFuture,
    RpcResult,
    master
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
    sender: mpsc::Sender<Rpc>,
}

enum Leader {
    /// The known leader.
    Known(Connection, Vec<SocketAddr>),
    /// The leader is unknown, RPCs must be queued until the leader is discovered.
    Unknown(Box<Future<Item=(Connection, Vec<SocketAddr>), Error=!>>),
}

impl Leader {

    fn poll_cxn(&mut self) -> Poll<&mut Connection, !> {
        loop {
            let (cxn, replicas) = match *self {
                Leader::Known(ref mut cxn, ..) => return Ok(Async::Ready(cxn)),
                Leader::Unknown(ref mut future) => try_ready!(future.poll()),
            };
            *self = Leader::Known(cxn, replicas);
        }
    }

    fn reset(&mut self, reactor: &Handle, resolver: &Resolver, timer: &tokio_timer::Timer) {
        let replicas = match *self {
            Leader::Known(_, ref mut replicas) => mem::replace(replicas, Vec::new()),
            Leader::Unknown(..) => return,
        };

        let leader = find_leader_master(reactor.clone(),
                                        resolver.clone(),
                                        timer.clone(),
                                        Backoff::with_duration_range(1_000, 60_000),
                                        replicas);

        *self = Leader::Unknown(Box::new(leader));
    }
}

struct MasterProxyTask {
    reactor: Handle,
    resolver: Resolver,
    timer: tokio_timer::Timer,
    in_flight: util::SelectStream<RpcFuture>,
    recv: mpsc::Receiver<Rpc>,
    leader: Leader,
}

struct InFlightRpc {
    send: oneshot::Sender<RpcResult>,
    recv: oneshot::Receiver<RpcResult>,
    handler: fn(&RpcResult) -> Result<()>,
}

impl MasterProxyTask {

    fn try_poll(&mut self) -> Poll<(), Error> {
        let MasterProxyTask {
            ref reactor,
            ref resolver,
            ref timer,
            ref mut in_flight,
            ref mut recv,
            ref mut leader
        } = *self;

        // Unwrap the leader connection.
        let cxn = match leader.poll_cxn() {
            Ok(Async::Ready(cxn)) => cxn,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(..) => unreachable!(),
        };

        // Step 1: Write queued RPCs to the connection.

        // Step 2: Crank the connection.
        cxn.poll_complete()?;

        // Step 3: Complete finished RPCs.

        //while let Async::Ready(rpc) = in_flight.poll()


        unimplemented!()
    }
}

impl Future for MasterProxyTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {


        unimplemented!()


    }
}



pub trait MasterResponse: Message {
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
