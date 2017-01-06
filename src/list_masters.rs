use std::mem;
use std::net::SocketAddr;
use std::time::Instant;

use futures::stream::futures_unordered;
use futures::future::join_all;
use futures::{Async, Future, Poll, Sink, Stream};
use kudu_pb::consensus_metadata::{RaftPeerPB_Role as Role};
use kudu_pb::wire_protocol::ServerEntryPB;
use kudu_pb::master::{
    ListMastersResponsePB,
    ListMastersRequestPB,
};
use tokio::reactor::Handle;
use tokio_timer;

use Error;
use Result;
use backoff::Backoff;
use dns::Resolver;
use error::{MasterError, MasterErrorCode};
use rpc::Connection;
use rpc::ConnectionOptions;
use rpc::master;
use util;
use rpc::Rpc;

pub enum ListMastersResponse {

    /// The connection to the discovered leader master, along with the full set of replica
    /// addresses (including the leader).
    Leader(Connection, Vec<SocketAddr>),

    /// The discovered set of the replica addresses.
    Replicas(Vec<SocketAddr>),
}

fn list_masters(handle: &Handle,
                resolver: Resolver,
                addr: SocketAddr,
                deadline: Instant)
                -> impl Future<Item=ListMastersResponse, Error=Error> + 'static {
    let mut rpc = master::list_masters(addr, deadline, ListMastersRequestPB::new());
    let future = rpc.future().map_err(|error| error.error);
    Box::new(Connection::new(&addr, handle, ConnectionOptions::default())
        .and_then(move |connection| connection.send(rpc))
        .and_then(move |connection| {
            future.map(Rpc::take_response::<ListMastersResponsePB>)
                  // Check if the response contains an error. This can happen when the master is
                  // not yet initialized, e.g.:
                  //     code: CATALOG_MANAGER_NOT_INITIALIZED
                  //     status {code: SERVICE_UNAVAILABLE
                  //             message: "catalog manager has not been initialized"}
                  .and_then(|mut response: ListMastersResponsePB| -> Result<ListMastersResponsePB> {
                      if response.has_error() {
                          Err(Error::Master(MasterError::new(MasterErrorCode::UnknownError,
                                                             response.take_error().into())))
                      } else {
                          Ok(response)
                      }
                  })
                  // Transform the list masters response PB into a future which will resolve to a
                  // vec containing the role and socket addrs for each replica.
                  .and_then(move |mut response: ListMastersResponsePB| {
                      let mut v = Vec::new();

                      for mut master in response.take_masters().into_iter() {
                          if master.has_error() {
                              trace!("ListMastersResponsePB ServerEntryPB error: {:?}",
                                     master.get_error());
                              continue;
                          }

                          let role = master.get_role();
                          let addrs = resolver.resolve_all(master.take_registration()
                                                                 .take_rpc_addresses()
                                                                 .into_vec());
                          v.push(addrs.map(move |addrs| (role, addrs)));
                      }

                      join_all(v)
                  })
                  // Aggregate the replica addresses, and deterimine if this connection is to the
                  // leader.
                  .map(move |masters| {
                      let mut is_leader = false;
                      let mut replicas = Vec::new();

                      for (role, addrs) in masters {
                          if role == Role::LEADER && addrs.contains(&addr) {
                              is_leader = true;
                          }
                          replicas.extend(addrs);
                      }

                      if is_leader {
                          ListMastersResponse::Leader(connection, replicas)
                      } else {
                          ListMastersResponse::Replicas(replicas)
                      }
                  })
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
                          -> Box<Future<Item=(Connection, Vec<SocketAddr>),
                                        Error=Vec<SocketAddr>>> {
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
    type Item = (Connection, Vec<SocketAddr>);
    type Error = Vec<SocketAddr>;
    fn poll(&mut self) -> Poll<(Connection, Vec<SocketAddr>), Vec<SocketAddr>> {
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
                          -> impl Future<Item=(Connection, Vec<SocketAddr>), Error=!> + 'static {
    util::retry_with_backoff(timer.clone(), backoff, move |_, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => trace!("FindLeaderMaster round timed out"),
            util::RetryCause::Err(reps) => {
                warn!("Unable to find a leader master among replicas {:?}", reps);
                replicas = reps;
            },
        }
        try_find_leader_master(reactor.clone(), resolver.clone(), timer.clone(), replicas.clone())
    })
}
