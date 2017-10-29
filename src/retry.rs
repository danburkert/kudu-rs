use std::mem;
use std::time::Instant;

use bytes::Bytes;
use futures::{
    Async,
    Future,
    Poll,
};
use krpc::{
    Proxy,
    Request,
    Response,
};
use prost::Message;
use timer::{
    Sleep,
    Timer,
};

use Error;
use MasterError;
use backoff::Backoff;
use pb::master::*;

pub trait KuduResponse : Message + Default {
    fn into_result(self) -> Result<Self, Error> where Self: Sized;
}

enum State<R> where R: KuduResponse {
    InFlight(Response<R>),
    Waiting(Option<Request>),
}

/// A future which wraps an in-flight Kudu RPC, and retries it after a backoff period if it fails
/// with a retriable error.
#[must_use = "futures do nothing unless polled"]
pub(crate) struct Retry<R> where R: KuduResponse {
    backoff: Backoff,
    proxy: Option<Proxy>,
    timer: Timer,

    sleep: Sleep,
    state: State<R>,
    errors: Vec<Error>,
}

impl <R> Retry<R> where R: KuduResponse {
    pub fn wrap(response: Response<R>, proxy: Proxy, timer: Timer) -> Retry<R> {
        let mut backoff = Backoff::default();
        let sleep = timer.sleep(backoff.next_backoff());

        Retry {
            backoff,
            proxy: Some(proxy),
            timer,
            sleep,
            state: State::InFlight(response),
            errors: Vec::new(),
        }
    }

    fn fail(&mut self, request: &Request) -> Result<(), Error> {
        let errors = mem::replace(&mut self.errors, Vec::new());
        let description = format!("{}.{} RPC failed", request.service, request.method);
        Err(Error::Compound(description, errors))
    }
}

impl <R> Future for Retry<R> where R: KuduResponse {
    type Item = (R, Vec<Bytes>, Proxy);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            // Extra-raunchy NLL hack. The state field *must* be replaced before returning
            // NotReady, otherwise the unreachable! branch below can be hit.
            let state = mem::replace(&mut self.state, State::Waiting(None));

            // Poll the in-fligh RPC to see if it's complete.
            let request = match state {
                State::InFlight(mut response) => {
                    let (error, request) = match response.poll() {
                        Ok(Async::Ready((response, sidecars, request))) => match response.into_result() {
                            Ok(response) => return Ok(Async::Ready((response, sidecars, self.proxy.take().unwrap()))),
                            Err(error) => (error, request),
                        },
                        Ok(Async::NotReady) => {
                            self.state = State::InFlight(response);
                            return Ok(Async::NotReady);
                        },
                        Err((error, request)) => (error.into(), request),
                    };

                    // The in-flight RPC failed. Check to see if it's retriable.
                    let is_retriable = error.is_retriable();
                    self.errors.push(error);
                    if !is_retriable {
                        self.fail(&request)?;
                    }

                    request
                },
                State::Waiting(Some(request)) => request,
                State::Waiting(None) => unreachable!(),
            };

            // There's no in-flight RPC. Check the timer to see if it's time to make another attempt.
            match self.sleep.poll().expect("Sleep::poll failed") {
                Async::Ready(()) => {
                    let backoff = self.backoff.next_backoff();
                    if request.deadline < Instant::now() + backoff {
                        self.errors.push(Error::TimedOut);
                        self.fail(&request)?;
                    }

                    let response = self.proxy.as_mut().unwrap().send(request);
                    let sleep = self.timer.sleep(backoff);
                    self.sleep = sleep;
                    self.state = State::InFlight(response);
                    // Fall through for another trip through the loop.
                },
                Async::NotReady => {
                    self.state = State::Waiting(Some(request));
                    return Ok(Async::NotReady);
                },
            }
        }
    }
}

macro_rules! infallible_response {
    ($type:ident) => {
        impl KuduResponse for $type {
            fn into_result(self) -> Result<$type, Error> { Ok(self) }
        }
    };
}

infallible_response!(PingResponsePb);

macro_rules! master_response {
    ($type:ident) => {
        impl KuduResponse for $type {
            fn into_result(self) -> Result<$type, Error> {
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
