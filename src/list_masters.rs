use std::mem;
use std::net::SocketAddr;
use std::time::Instant;

use futures::{Async, Future, Poll, Sink, Stream};
use futures::future::join_all;
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};
use kudu_pb::master::{
    ListMastersResponsePB,
    ListMastersRequestPB,
};
use tokio_timer;

use Error;
use Result;
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
                -> impl Future<Item=ListMastersResponse, Error=Error> + Send {
    let messenger = io.messenger.clone();
    let resolver = io.resolver.clone();
    let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
    let future = rpc.future();
    messenger.send(rpc)
             .then(move |_| future)
             .map_err(|error| error.error)
             .then(|rpc| -> Result<ListMastersResponsePB> {
                 // Transform the RPC into a ListMastersResponsePB.
                 let mut response = rpc?.take_response::<ListMastersResponsePB>();
                 if response.has_error() {
                     // This can happen when the master is not yet initialized, e.g.:
                     // code: CATALOG_MANAGER_NOT_INITIALIZED
                     // status {code: SERVICE_UNAVAILABLE message: "catalog manager has not been initialized"}
                     return Err(Error::Master(MasterError::new(MasterErrorCode::UnknownError,
                                                               response.take_error().into())));
                 }
                 Ok(response)
             })
             .map(move |mut response| {
                 // Transform the ListMastersResponsePB into an iterator of futures containing the
                 // role and resolved addresses of each replica.
                 response.take_masters()
                         .into_iter()
                         .filter(|server_entry| !server_entry.has_error())
                         .map(|mut server_entry| {
                             let role = server_entry.get_role();
                             let host_ports = server_entry.mut_registration()
                                                          .take_rpc_addresses()
                                                          .into_vec();
                             (role, host_ports)
                         })
                        .map(move |(role, host_ports)| resolver.resolve(host_ports)
                                                               .map(move |addrs| (role, addrs)))
                        // TODO: collect isn't strictly necessary, but without it rustc hits the
                        // monomorphization limit.
                        .collect::<Vec<_>>()
             })
             .and_then(join_all) // Wait for all of the DNS resolutions.
             .map(move |replicas| {
                 // Transform the iterator of host, addrs pairs into a ListMastersResult.
                 let mut leader = false;
                 let mut replica_addrs: Vec<SocketAddr> = Vec::new();
                 for (role, addrs) in replicas {
                     if role == Role::LEADER && addrs.contains(&addr) {
                         leader = true;
                     }
                     replica_addrs.extend(addrs);
                 }
                 if leader {
                     ListMastersResponse::Leader(addr, replica_addrs)
                 } else {
                     ListMastersResponse::Replicas(replica_addrs)
                 }
             })
}

fn list_masters_with_retry(backoff: Backoff, timer: tokio_timer::Timer) {

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
    backoff_stream: util::BackoffStream,
}

impl Future for FindLeaderMaster {
    type Item = SocketAddr;
    type Error = ();
    fn poll(&mut self) -> Poll<SocketAddr, ()> {
        unimplemented!()
    }
}
