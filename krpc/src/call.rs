use std::sync::Arc;
use std::time::Instant;
use std::marker::PhantomData;
use std::fmt;

use prost::Message;

/// A remote procedure call.
///
/// `Call` describes a remote procedure call: the remote service, the method, the required feature
/// flags, the deadline, the request, and the response type. `Call` instances are dispatched to a
/// remote server using `Proxy::send`, which returns the response future.
pub struct Call<Req, Resp> where Req: Message + 'static, Resp: Message + Default {
    pub(crate) service: &'static str,
    pub(crate) method: &'static str,
    pub(crate) required_feature_flags: &'static [u32],
    pub(crate) deadline: Instant,
    pub(crate) request: Arc<Req>,
    _marker: PhantomData<Resp>,
}

impl <Req, Resp> Call<Req, Resp> where Req: Message + 'static, Resp: Message + Default {

    /// Creates a new `Call` instance.
    pub fn new(service: &'static str,
               method: &'static str,
               request: Arc<Req>,
               deadline: Instant) -> Call<Req, Resp> {
        Call {
            service,
            method,
            required_feature_flags: &[],
            deadline,
            request: request,
            _marker: PhantomData::default(),
        }
    }

    /// Returns the call's remote service.
    pub fn service(&self) -> &'static str {
        self.service
    }

    /// Returns the call's remote method.
    pub fn method(&self) -> &'static str {
        self.method
    }

    /// Returns the call's deadline.
    pub fn deadline(&self) -> Instant {
        self.deadline
    }

    /// Retrieves the required feature flags of the call.
    pub fn required_feature_flags(&self) -> &'static [u32] {
        self.required_feature_flags
    }

    pub fn set_deadline(&mut self, deadline: Instant) -> &mut Call<Req, Resp> {
        self.deadline = deadline;
        self
    }

    /// Sets the required feature flags of the call.
    ///
    /// If not set, no feature flags are sent with the call.
    pub fn set_required_feature_flags(&mut self, required_feature_flags: &'static [u32]) -> &mut Call<Req, Resp> {
        self.required_feature_flags = required_feature_flags;
        self
    }
}

impl <Req, Resp> fmt::Debug for Call<Req, Resp> where Req: Message + 'static, Resp: Message + Default {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dbg = f.debug_struct("Call");
        dbg.field("service", &self.service);
        dbg.field("method", &self.method);
        if !self.required_feature_flags.is_empty() {
            dbg.field("required_feature_flags", &self.required_feature_flags);
        }
        dbg.field("deadline", &self.deadline);
        dbg.finish()
    }
}

impl <Req, Resp> Clone for Call<Req, Resp> where Req: Message + 'static, Resp: Message + Default {
    fn clone(&self) -> Call<Req, Resp> {
        Call {
            service: self.service,
            method: self.method,
            required_feature_flags: self.required_feature_flags,
            deadline: self.deadline,
            request: self.request.clone(),
            _marker: PhantomData::default(),
        }
    }
}
