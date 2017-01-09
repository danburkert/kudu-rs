use std::any::Any;
use std::fmt;
use std::time::Instant;

use protobuf::Message;

//pub use rpc::messenger::Messenger;
pub use rpc::connection::ConnectionOptions;
pub use rpc::connection::Connection;

use Error;

mod connection;
//mod messenger;
pub mod master;
pub mod tablet_server;

/// An `Rpc` contains all the state necessary to execute and retry an operation on a remote Kudu
/// server.
///
/// `Rpc`s have a timeout deadline that is passed to the remote server.
///
/// The [Kudu RPC protocol](https://github.com/cloudera/kudu/blob/master/docs/design-docs/rpc.md)
/// uses Protobuf as a standard serialization format. Each RPC to a Kudu server consists of a
/// request header message and a body message. The request header includes the service name and
/// method name, which together identify the type of the request and response body types. The
/// response includes a response header, a response body, and optional sidecars.
///
/// The `Rpc` struct holds both the request Protobuf message as well as the response. When the RPC
/// response is received from the wire, it is deserialized into the response message, and the `Rpc`
/// is completed.
#[must_use = "rpcs must not be dropped"]
pub struct Rpc {
    service: &'static str,
    method: &'static str,
    required_feature_flags: &'static [u32],
    response: fn() -> Box<Message>,
    request: Box<Message>,
    deadline: Instant,
    result: Option<Result<Box<Message>, Error>>,
}

impl Rpc {

    pub fn new(service: &'static str,
               method: &'static str,
               required_feature_flags: &'static [u32],
               response: fn() -> Box<Message>,
               deadline: Instant,
               request: Box<Message>) -> Rpc {
        Rpc {
            service: service,
            method: method,
            required_feature_flags: required_feature_flags,
            response: response,
            deadline: deadline,
            request: request,
            result: None,
        }
    }

    fn fail(&mut self, error: Error) {
        self.result = Some(Err(error));
    }

    fn complete(&mut self) -> &mut Message {
        assert!(self.result.is_none());
        self.result = Some(Ok((self.response)()));
        match self.result {
            Some(Ok(ref mut response)) => &mut **response,
            _ => unreachable!(),
        }
    }


    pub fn result<T>(&self) -> Result<&T, &Error> where T: Any {
        match *self.result.as_ref().take().expect("RPC not complete") {
            Ok(ref msg) => Ok(msg.as_any().downcast_ref::<T>().unwrap()),
            Err(ref error) => Err(error),
        }
    }

    pub fn take_result<T>(mut self) -> Result<T, Error> where T: Any {
        match self.result.take().expect("RPC not complete") {
            Ok(msg) => Ok(*msg.into_any().downcast::<T>().unwrap()),
            Err(error) => Err(error),
        }
    }

    /// Returns `true` if the provided instant is greater than or equal to this RPC's deadline.
    fn timed_out(&self, now: Instant) -> bool {
        self.deadline <= now
    }

    pub fn service(&self) -> &'static str {
        self.service
    }

    pub fn method(&self) -> &'static str {
        self.method
    }

    pub fn required_feature_flags(&self) -> &'static [u32] {
        self.required_feature_flags
    }

    pub fn deadline(&self) -> Instant {
        self.deadline
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rpc {{ {}::{}, deadline: {:?} }}",
               self.service, self.method, self.deadline)
    }
}
