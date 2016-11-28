use std::mem;
use std::net::SocketAddr;
use std::time::Instant;

use futures::{Async, Future, Poll, Sink, Stream};
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};
use kudu_pb::master::{
    ListMastersResponsePB,
    ListMastersRequestPB,
};
use tokio_timer;

use Error;
use backoff::Backoff;
use error::{MasterError, MasterErrorCode};
use io::Io;
use rpc::master;
use util;

pub enum ListMastersResponse {
    Leader(SocketAddr, Vec<SocketAddr>),
    Replicas(Vec<SocketAddr>),
}
fn list_masters(io: &Io,
                addr: SocketAddr,
                deadline: Instant)
                -> Box<Future<Item=ListMastersResponse, Error=Error> + Send> {
    let messenger = io.messenger.clone();
    let resolver = io.resolver.clone();
    let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
    let future = rpc.future();
    messenger.send(rpc)
             .then(move |_| future)
             .map_err(|error| error.error)
             .and_then(move |mut rpc| {
                 let response = rpc.mut_response::<ListMastersResponsePB>();
                 if response.has_error() {
                     // This can happen when the master is not yet initialized, e.g.:
                     // code: CATALOG_MANAGER_NOT_INITIALIZED
                     // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
                     return Err(Error::Master(MasterError::new(MasterErrorCode::UnknownError,
                                                               response.take_error().into())));
                 }
                 debug!("ListMasters RPC to master {} response: {:?}", addr, response);

                 let mut responses = Vec::with_capacity(response.get_masters().len());
                 for server_entry in response.mut_masters().iter_mut() {
                     if server_entry.has_error()  {
                         continue;
                     }
                     let role = server_entry.get_role();
                     let response = resolver.resolve(server_entry.mut_registration()
                                                                 .take_rpc_addresses()
                                                                 .into_vec())
                                            .map(move |addrs| -> ListMastersResponse {
                                                if role == Role::LEADER && addrs.contains(&addr) {
                                                    ListMastersResponse::Leader(addr, addrs)
                                                } else {
                                                    ListMastersResponse::Replicas(addrs)
                                                }
                                            });
                     responses.push(response);
                 }

                 Ok(CombineResponses::new(responses).map_err(|_| panic!()))
             }).flatten().boxed()
}

/// Combines multiple futures containing `ListMastersResponse` into one.
struct CombineResponses<F> where F: Future<Item=ListMastersResponse> {
    responses: util::SelectStream<F>,
    replicas: Vec<SocketAddr>,
}
impl <F> CombineResponses<F> where F: Future<Item=ListMastersResponse> {
    fn new(responses: Vec<F>) -> CombineResponses<F> {
        CombineResponses {
            responses: util::select_stream(responses),
            replicas: Vec::new(),
        }
    }
}
impl <F> Future for CombineResponses<F> where F: Future<Item=ListMastersResponse> {
    type Item = ListMastersResponse;
    type Error = F::Error;
    fn poll(&mut self) -> Poll<ListMastersResponse, F::Error> {
        match try_ready!(self.responses.poll()) {
            Some(ListMastersResponse::Replicas(replicas)) => {
                self.replicas.extend_from_slice(&replicas);
                self.replicas.sort_by(util::cmp_socket_addrs);
                self.replicas.dedup();
                Ok(Async::NotReady)
            },
            Some(leader) => Ok(Async::Ready(leader)),
            None => {
                let replicas = mem::replace(&mut self.replicas, Vec::new());
                Ok(Async::Ready(ListMastersResponse::Replicas(replicas)))
            }
        }
    }
}

struct FindLeaderMaster {
    io: Io,
    replicas: Vec<SocketAddr>,
    list_masters: util::SelectStream<Box<Future<Item=ListMastersResponse, Error=Error> + Send>>,
    retry: tokio_timer::Sleep,
    backoff: Backoff,
}

impl Future for FindLeaderMaster {
    type Item = SocketAddr;
    type Error = ();
    fn poll(&mut self) -> Poll<SocketAddr, ()> {
        unimplemented!()
    }
}
