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
                -> Box<Future<Item=ListMastersResponse, Error=Error> + Send> {
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
             .boxed()
}

fn list_masters_with_retry(mut io: Io,
                           addr: SocketAddr,
                           backoff: Backoff) -> Box<Future<Item=ListMastersResponse, Error=()> + Send> {
    let timer = io.timer().clone();
    util::retry_with_backoff(timer, backoff, move |deadline, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => warn!("ListMasters RPC to {:?} timed out", addr),
            util::RetryCause::Err(error) => warn!("ListMasters RPC to {:?} failed: {}", addr, error),
        }
        list_masters(&io, addr, deadline)
    }).boxed()
}

pub fn find_leader_master(mut io: Io, replicas: Vec<SocketAddr>) -> Box<Future<Item=(SocketAddr, Vec<SocketAddr>),
                                                                               Error=Vec<SocketAddr>> + Send> {
    let list_masters = util::select_stream(replicas.iter().cloned().map(|addr| {
        list_masters_with_retry(io.clone(), addr, Backoff::with_duration_range(10, 10000))
    }));

    FindLeaderMaster {
        io: io,
        replicas: replicas,
        list_masters: list_masters,
    }.boxed()
}

struct FindLeaderMaster {
    io: Io,
    replicas: Vec<SocketAddr>,
    list_masters: util::SelectStream<Box<Future<Item=ListMastersResponse, Error=()> + Send>>,
}

impl Future for FindLeaderMaster {
    type Item = (SocketAddr, Vec<SocketAddr>);
    type Error = Vec<SocketAddr>;
    fn poll(&mut self) -> Poll<(SocketAddr, Vec<SocketAddr>), Vec<SocketAddr>> {
        loop {
            match self.list_masters.poll() {
                Ok(Async::Ready(None)) => {
                    let replicas = mem::replace(&mut self.replicas, Vec::new());
                    return Err(replicas);
                },
                Ok(Async::Ready(Some(ListMastersResponse::Leader(addr, replicas)))) => {
                    return Ok(Async::Ready((addr, replicas)));
                },
                Ok(Async::Ready(Some(ListMastersResponse::Replicas(addrs)))) => {
                    for addr in addrs {
                        if !self.replicas.contains(&addr) {
                            self.replicas.push(addr);
                            let stream = list_masters_with_retry(self.io.clone(),
                                                                 addr,
                                                                 Backoff::with_duration_range(10, 10000));
                            self.list_masters.add(stream);
                        }
                    }
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(error) => unreachable!(),
            }
        }
    }
}

pub fn find_leader_master_with_retry(mut io: Io,
                                     backoff: Backoff,
                                     mut replicas: Vec<SocketAddr>)
                                     -> Box<Future<Item=(SocketAddr, Vec<SocketAddr>), Error=()> + Send> {
    let timer = io.timer().clone();
    Box::new(util::retry_with_backoff(timer, backoff, move |deadline, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => trace!("FindLeaderMaster round timed out"),
            util::RetryCause::Err(reps) => {
                warn!("Unable to find a leader master among replicas {:?}", reps);
                replicas = reps;
            },
        }
        find_leader_master(io.clone(), replicas.clone())
    }))
}
