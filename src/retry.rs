use std::mem;
use std::sync::Arc;
use std::time::Instant;

use bytes::Bytes;
use futures::{
    Async,
    Future,
    Poll,
};
use krpc::{
    Descriptor,
    Proxy,
    RpcFuture,
};
use prost::Message;
use timer::{
    Sleep,
    Timer,
};

use Error;
use MasterError;
use TabletServerError;
use backoff::Backoff;
use pb::{master, tserver};

pub trait Retriable : Message + Default {
    fn into_result(self) -> Result<Self, Error> where Self: Sized;
}

pub trait RetryProxy {

    fn send_retriable<Req, Resp>(self,
                                 descriptor: Descriptor<Req, Resp>,
                                 request: Arc<Req>,
                                 timer: Timer)
                                 -> RetryFuture<Req, Resp>
    where Req: Message + 'static,
          Resp: Retriable;
}

impl RetryProxy for Proxy {
    pub fn send_retriable<Req, Resp>(self,
                                     descriptor: Descriptor<Req, Resp>,
                                     request: Arc<Req>,
                                     timer: Timer)
                                     -> RetryFuture<Req, Resp>
    where Req: Message + 'static,
          Resp: Retriable {
        let mut backoff = Backoff::default();
        let sleep = timer.sleep(backoff.next_backoff());
        let state = State::InFlight(self.send(descriptor, request));

        RetryFuture {
            backoff,
            proxy: self,
            timer,
            request,
            descriptor,
            state,
            errors: Vec::new(),
        }
    }
}

enum State<Resp> where Resp: Retriable {
    InFlight(RpcFuture<Resp>),
    Waiting,
}

/// A future which wraps an in-flight Kudu RPC, and retries it after a backoff period if it fails
/// with a retriable error.
#[must_use = "futures do nothing unless polled"]
pub(crate) struct RetryFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    backoff: Backoff,
    proxy: Proxy,
    timer: Timer,
    descriptor: Descriptor<Req, Resp>,
    request: Arc<Req>,

    sleep: Sleep,
    state: State<Resp>,
    errors: Vec<Error>,
}

impl <Req, Resp> RetryFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {

    pub fn send(proxy: Proxy,
                timer: Timer,
                descriptor: Descriptor<Req, Resp>,
                request: Arc<Req>) {
        let mut backoff = Backoff::default();
        let sleep = timer.sleep(backoff.next_backoff());
        let state = State::InFlight(proxy.send(descriptor, request));

        RetryFuture {
            backoff,
            proxy,
            timer,
            request,
            descriptor,
            state,
            errors: Vec::new(),
        }
    }

    fn fail(&mut self) -> Result<(), Error> {
        let errors = mem::replace(&mut self.errors, Vec::new());
        let description = format!("{}.{} RPC failed", self.descriptor.service, self.descriptor.method);
        Err(Error::Compound(description, errors))
    }
}

impl <Req, Resp> Future for RetryFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    type Item = (Resp, Vec<Bytes>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            // Poll the in-flight RPC to see if it's complete.
            match self.state {
                State::InFlight(mut response) => {
                    let error = match response.poll() {
                        Ok(Async::Ready((response, sidecars))) => match response.into_result() {
                            Ok(response) => return Ok(Async::Ready((response, sidecars))),
                            Err(error) => error,
                        },
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(error) => error.into(),
                    };

                    // The in-flight RPC failed. Check to see if it's retriable.
                    let is_retriable = error.is_retriable();
                    self.errors.push(error);
                    if !is_retriable {
                        self.fail()?;
                    }
                    *self.state = State::Waiting;
                },
                State::Waiting =>  {
                    try_ready!(self.sleep.poll().map_err(|_| -> Error { unreachable!() }));
                    let backoff = self.backoff.next_backoff();
                    if request.deadline < Instant::now() + backoff {
                        self.errors.push(Error::TimedOut);
                        self.fail()?;
                    }

                    let response = self.proxy.as_mut().unwrap().send(self.descriptor.clone(), self.request.clone());
                    let sleep = self.timer.sleep(backoff);
                    self.sleep = sleep;
                    self.state = State::InFlight(response);
                    // Fall through for another trip through the loop.
                },
            };
        }
    }
}

macro_rules! infallible_response {
    ($type:ty) => {
        impl Retriable for $type {
            fn into_result(self) -> Result<$type, Error> { Ok(self) }
        }
    };
}

infallible_response!(master::PingResponsePb);
infallible_response!(tserver::PingResponsePb);

macro_rules! master_response {
    ($type:ident) => {
        impl Retriable for master::$type {
            fn into_result(self) -> Result<master::$type, Error> {
                match self.error {
                    Some(error) => Err(MasterError::from(error).into()),
                    None => Ok(self),
                }
            }
        }
    };
}

master_response!(AlterTableResponsePb);
master_response!(ConnectToMasterResponsePb);
master_response!(CreateTableResponsePb);
master_response!(DeleteTableResponsePb);
master_response!(GetTableLocationsResponsePb);
master_response!(GetTableSchemaResponsePb);
master_response!(GetTabletLocationsResponsePb);
master_response!(IsAlterTableDoneResponsePb);
master_response!(IsCreateTableDoneResponsePb);
master_response!(ListMastersResponsePb);
master_response!(ListTablesResponsePb);
master_response!(ListTabletServersResponsePb);

macro_rules! tserver_response {
    ($type:ident) => {
        impl Retriable for tserver::$type {
            fn into_result(self) -> Result<tserver::$type, Error> {
                match self.error {
                    Some(error) => Err(TabletServerError::from(error).into()),
                    None => Ok(self),
                }
            }
        }
    };
}

tserver_response!(WriteResponsePb);
