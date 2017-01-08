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
/// When the `Rpc` completes (either succesfully or unsuccessfully), the included oneshot future is
/// fired.
///
/// `Rpc`s impose a few restrictions to ensure eventual termination: all `Rpc`s have a timeout
/// deadline, and `Rpc`s may optionally be cancelled by the caller if they are no longer necessary.
/// Timing out or cancelling an `Rpc` should cause it to be completed with the corresponding error
/// message as soon as possible.
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
///
/// `Rpc`s may be marked as 'fail-fast' to indicate that if any interruption in the connection to
/// the remote server occurs, the RPC should be immediately failed. If an `Rpc` is not marked as
/// fail-fast, it will be retried to the same destination until it either fails, or it times out.
/// Fail-fast RPCs are useful for situations where the RPC could be retried against an alternate
/// remote server.
#[must_use = "rpcs must not be dropped"]
pub struct Rpc {
    service: &'static str,
    method: &'static str,
    required_feature_flags: &'static [u32],
    deadline: Instant,
    request: Box<Message>,
    response: Box<Message>,
    sidecars: Vec<Vec<u8>>,
    error: Option<Error>,
}

impl Rpc {

    pub fn new(service: &'static str,
               method: &'static str,
               required_feature_flags: &'static [u32],
               deadline: Instant,
               request: Box<Message>,
               response: Box<Message>) -> Rpc {
        Rpc {
            service: service,
            method: method,
            required_feature_flags: required_feature_flags,
            deadline: deadline,
            request: request,
            response: response,
            sidecars: Vec::new(),
            error: None,
        }
    }

    fn fail(&mut self, error: Error) {
        self.error = Some(error);
    }

    fn clear(&mut self) {
        self.error = None;
        self.response.clear();
        self.sidecars.clear();
    }

    pub fn result<T>(&self) -> Result<&T, &Error> where T: Any {
        if let Some(ref error) = self.error {
            Err(error)
        } else {
            Ok(self.response.as_any().downcast_ref::<T>().unwrap())
        }
    }

    pub fn take_result<T>(mut self) -> Result<T, Error> where T: Any {
        if let Some(error) = self.error.take() {
            Err(error)
        } else {
            Ok(*self.response.into_any().downcast::<T>().unwrap())
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

    fn response_mut(&mut self) -> &mut Message {
        &mut *self.response
    }
}

impl fmt::Debug for Rpc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rpc {{ {}::{}, deadline: {:?} }}",
               self.service, self.method, self.deadline)
    }
}
