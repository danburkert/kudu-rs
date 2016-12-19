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
use tokio::reactor::Handle;
use tokio_timer;

use Error;
use Result;
use backoff::Backoff;
use error::{MasterError, MasterErrorCode};
use rpc::master;
use util;
use dns::Resolver;
use rpc::Connection;
use rpc::ConnectionOptions;

pub enum ListMastersResponse {
    Leader(SocketAddr, Vec<SocketAddr>),
    Replicas(Vec<SocketAddr>),
}

fn list_masters(handle: &Handle,
                resolver: Resolver,
                addr: SocketAddr,
                deadline: Instant)
                -> Box<Future<Item=ListMastersResponse, Error=Error>> {
    let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
    let future = rpc.future();
    Box::new(Connection::new(&addr, handle, ConnectionOptions::default())
               .and_then(|connection| connection.send(rpc))
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
               }))
}

fn list_masters_with_retry(handle: Handle,
                           resolver: Resolver,
                           timer: tokio_timer::Timer,
                           addr: SocketAddr,
                           backoff: Backoff) -> Box<Future<Item=ListMastersResponse, Error=!>> {
    Box::new(util::retry_with_backoff(timer, backoff, move |deadline, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => warn!("ListMasters RPC to {:?} timed out", addr),
            util::RetryCause::Err(error) => warn!("ListMasters RPC to {:?} failed: {}", addr, error),
        }
        list_masters(&handle, resolver.clone(), addr, deadline)
    }))
}

fn try_find_leader_master(handle: Handle,
                          resolver: Resolver,
                          timer: tokio_timer::Timer,
                          replicas: Vec<SocketAddr>)
                          -> Box<Future<Item=(SocketAddr, Vec<SocketAddr>), Error=Vec<SocketAddr>>> {
    let list_masters = util::select_stream(replicas.iter().cloned().map(|addr| {
        list_masters_with_retry(handle.clone(),
                                resolver.clone(),
                                timer.clone(),
                                addr,
                                Backoff::with_duration_range(20, 10000))
    }));

    Box::new(FindLeaderMaster {
        handle: handle,
        resolver: resolver,
        timer: timer,
        replicas: replicas,
        list_masters: list_masters,
    })
}

struct FindLeaderMaster {
    handle: Handle,
    resolver: Resolver,
    timer: tokio_timer::Timer,
    replicas: Vec<SocketAddr>,
    list_masters: util::SelectStream<Box<Future<Item=ListMastersResponse, Error=!>>>,
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
                            let stream = list_masters_with_retry(self.handle.clone(),
                                                                 self.resolver.clone(),
                                                                 self.timer.clone(),
                                                                 addr,
                                                                 Backoff::with_duration_range(10, 10000));
                            self.list_masters.add(stream);
                        }
                    }
                },
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(_) => unreachable!(),
            }
        }
    }
}

pub fn find_leader_master(reactor: Handle,
                          resolver: Resolver,
                          timer: tokio_timer::Timer,
                          backoff: Backoff,
                          mut replicas: Vec<SocketAddr>)
                          -> Box<Future<Item=(SocketAddr, Vec<SocketAddr>), Error=!>> {
    Box::new(util::retry_with_backoff(timer.clone(), backoff, move |_, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => trace!("FindLeaderMaster round timed out"),
            util::RetryCause::Err(reps) => {
                warn!("Unable to find a leader master among replicas {:?}", reps);
                replicas = reps;
            },
        }
        try_find_leader_master(reactor.clone(), resolver.clone(), timer.clone(), replicas.clone())
    }))
}
