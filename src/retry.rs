use std::mem;
use std::time::Instant;

use bytes::Bytes;
use futures::{
    Async,
    Future,
    Poll,
};
use krpc::{
    Call,
    Proxy,
    RpcFuture,
};
use prost::Message;
use tokio_timer::Delay;

use Error;
use MasterError;
use TabletServerError;
use backoff::Backoff;
use pb::{master, tserver};

/// TODO: rename to KuduResponse.
pub(crate) trait Retriable : Message + Default {
    fn into_result(self) -> Result<Self, Error> where Self: Sized;
}

pub(crate) trait RetryProxy {
    fn send_retriable<Req, Resp>(self, call: Call<Req, Resp>) -> RetryFuture<Req, Resp>
    where Req: Message + 'static,
          Resp: Retriable;
}

impl RetryProxy for Proxy {
    fn send_retriable<Req, Resp>(mut self, call: Call<Req, Resp>) -> RetryFuture<Req, Resp>
    where Req: Message + 'static,
          Resp: Retriable {
        let mut backoff = Backoff::default();
        let sleep = Delay::new(Instant::now() + backoff.next_backoff());
        let state = State::InFlight(self.send(call.clone()));

        RetryFuture {
            backoff,
            proxy: self,
            call,
            sleep,
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
    call: Call<Req, Resp>,
    sleep: Delay,
    state: State<Resp>,
    errors: Vec<Error>,
}

fn fail<Req, Resp>(errors: &mut Vec<Error>, call: &Call<Req, Resp>) -> Result<(), Error>
where Req: Message + 'static,
      Resp: Retriable {
    let errors = mem::replace(errors, Vec::new());
    let description = format!("RPC failed: {:?}", call);
    Err(Error::Compound(description, errors))
}

impl <Req, Resp> Future for RetryFuture<Req, Resp> where Req: Message + 'static, Resp: Retriable {
    type Item = (Resp, Vec<Bytes>);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            // Poll the in-flight RPC to see if it's complete.
            // NLL hack.
            let state = match self.state {
                State::InFlight(ref mut response) => {
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
                        fail(&mut self.errors, &self.call)?;
                    }
                    State::Waiting
                },
                State::Waiting =>  {
                    try_ready!(self.sleep.poll().map_err(|_| -> Error { unreachable!() }));
                    let backoff = Instant::now() + self.backoff.next_backoff();
                    if self.call.deadline() < backoff {
                        self.errors.push(Error::TimedOut);
                        fail(&mut self.errors, &self.call)?;
                    }

                    let response = self.proxy.send(self.call.clone());
                    let sleep = Delay::new(backoff);
                    self.sleep = sleep;
                    State::InFlight(response)
                },
            };
            self.state = state;
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
tserver_response!(ScanResponsePb);
