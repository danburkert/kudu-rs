use std::mem;
use std::net::SocketAddr;
use std::time::Instant;

use futures::future::join_all;
use futures::{Async, Future, IntoFuture, Poll, Sink, Stream};
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
use Role;

pub enum ListMastersResponse<S> where S: Send {

    /// The connection to the discovered leader master, along with the full set of replica
    /// addresses (including the leader).
    Leader(Connection<S>, Vec<SocketAddr>),

    /// The discovered set of the replica addresses.
    Replicas(Vec<SocketAddr>),
}

fn list_masters<S>(handle: &Handle,
                   resolver: Resolver,
                   addr: SocketAddr,
                   deadline: Instant)
                   -> Box<Future<Item=ListMastersResponse<S>, Error=Error> + 'static>
where S: Send + 'static
{
    let rpc = master::list_masters(deadline, ListMastersRequestPB::new());
    Box::new(Connection::new(&addr, handle, ConnectionOptions::default())
        .and_then(move |connection| connection.send(rpc))
        .and_then(move |connection| connection.into_future().map_err(|_| unreachable!()))
        .and_then(move |(rpc, connection)| {
            rpc.unwrap()
               .take_result()
               .map(|msg| *msg.into_any().downcast::<ListMastersResponsePB>().unwrap())
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
               .map(move |mut response: ListMastersResponsePB| {
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
               // Result<Future<..>> -> Future<..>
               .into_future()
               .flatten()
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
        })
    )
}

fn list_masters_with_retry<S>(handle: Handle,
                              resolver: Resolver,
                              timer: tokio_timer::Timer,
                              addr: SocketAddr,
                              backoff: Backoff) -> Box<Future<Item=ListMastersResponse<S>, Error=!>>
where S: Send + 'static
{
    Box::new(util::retry_with_backoff(timer, backoff, move |deadline, cause| {
        match cause {
            util::RetryCause::Initial => (),
            util::RetryCause::TimedOut => warn!("ListMasters RPC to {:?} timed out", addr),
            util::RetryCause::Err(error) => warn!("ListMasters RPC to {:?} failed: {}", addr, error),
        }
        list_masters(&handle, resolver.clone(), addr, deadline)
    }))
}

fn try_find_leader_master<S>(handle: Handle,
                             resolver: Resolver,
                             timer: tokio_timer::Timer,
                             replicas: Vec<SocketAddr>)
                             -> Box<Future<Item=(Connection<S>, Vec<SocketAddr>),
                                           Error=Vec<SocketAddr>>>
where S: Send + 'static
{
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

struct FindLeaderMaster<S> where S: Send + 'static {
    handle: Handle,
    resolver: Resolver,
    timer: tokio_timer::Timer,
    replicas: Vec<SocketAddr>,
    list_masters: util::SelectStream<Box<Future<Item=ListMastersResponse<S>, Error=!>>>,
}

impl <S> Future for FindLeaderMaster<S> where S: Send {
    type Item = (Connection<S>, Vec<SocketAddr>);
    type Error = Vec<SocketAddr>;
    fn poll(&mut self) -> Poll<(Connection<S>, Vec<SocketAddr>), Vec<SocketAddr>> {
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

pub fn find_leader_master<S>(reactor: Handle,
                             resolver: Resolver,
                             timer: tokio_timer::Timer,
                             backoff: Backoff,
                             mut replicas: Vec<SocketAddr>)
                             -> Box<Future<Item=(Connection<S>, Vec<SocketAddr>), Error=!>>
where S: Send + 'static
{
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
