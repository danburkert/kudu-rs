use std::ascii::AsciiExt;
use std::collections::HashSet;
use std::time::Instant;

use futures::{
    Async,
    Future,
};
use prost::Message;

use pb::rpc::{
    AuthenticationTypePb,
    ConnectionContextPb,
    NegotiatePb,
    RpcFeatureFlag,
    authentication_type_pb,
};
use pb::rpc::negotiate_pb::{
    NegotiateStep,
    SaslMechanism as SaslMechanismPb,
};
use transport::{
    Response,
    Transport,
};
use Error;
use RequestBody;

const NEGOTIATION_CALL_ID: i32 = -33;
const CONNECTION_CONTEXT_CALL_ID: i32 = -3;
const SASL: authentication_type_pb::Type = authentication_type_pb::Type::Sasl(authentication_type_pb::Sasl {});

#[derive(Debug)]
enum AuthenticationType {
    Sasl(SaslMechanism),
    Token,
    Certificate,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum SaslMechanism {
    Plain,
    Gssapi,
}

impl SaslMechanism {

    fn from_pb(pb: &SaslMechanismPb) -> Option<SaslMechanism> {
        if pb.mechanism.eq_ignore_ascii_case("PLAIN") {
            Some(SaslMechanism::Plain)
        } else if pb.mechanism.eq_ignore_ascii_case("GSSAPI") {
            Some(SaslMechanism::Gssapi)
        } else {
            None
        }
    }

    fn to_pb(&self) -> SaslMechanismPb {
        match *self {
            SaslMechanism::Plain => SaslMechanismPb { mechanism: "PLAIN".to_string() },
            SaslMechanism::Gssapi => SaslMechanismPb { mechanism: "GSSAPI".to_string() },
        }
    }
}

pub struct Inner {
    pb: NegotiatePb,
    transport: Transport,
    deadline: Instant,
    /// The authentication type used by the connection. Filled in during negotiation.
    authentication: Option<AuthenticationType>,
    /// The features supported by the server. Filled in during negotiation.
    supported_features: Vec<RpcFeatureFlag>,
}

impl Inner {

    fn poll(&mut self) -> Result<Async<()>, Error> {
        match self.pb.step() {
            NegotiateStep::Unknown => self.do_initial_step(),
            NegotiateStep::Negotiate => self.do_negotiate_step(),
            NegotiateStep::SaslInitiate => self.do_sasl_plain_step(),
            _ => unreachable!(),
        }
    }

    fn send_negotiate_pb(&mut self) -> Result<(), Error> {
        self.transport.send(NEGOTIATION_CALL_ID, "", "", &[], &self.pb, self.deadline)
    }

    fn recv_negotiate_pb(&mut self) -> Result<Async<()>, Error> {
        match try_ready!(self.transport.poll()) {
            Response::Ok { call_id, body, sidecars } => {
                if call_id != NEGOTIATION_CALL_ID {
                    return Err(Error::Negotiation(format!(
                                "Received illegal call-id during negotiation: {}",
                                call_id)));
                }
                if !sidecars.is_empty() {
                    return Err(Error::Negotiation(
                            "Received illegal RPC sidecars during negotiation".to_string()));
                }
                self.pb.clear();
                self.pb.merge_length_delimited(body)?;
                Ok(Async::Ready(()))
            },
            Response::Err { call_id, error } => Err(error.into()),
        }
    }

    fn do_initial_step(&mut self) -> Result<Async<()>, Error> {
        self.pb.clear();
        self.pb.supported_features.push(RpcFeatureFlag::ApplicationFeatureFlags as i32);
        self.pb.step = NegotiateStep::Negotiate as i32;
        self.pb.sasl_mechanisms.push(SaslMechanism::Plain.to_pb());
        self.pb.authn_types.push(AuthenticationTypePb {
            type_: Some(SASL),
        });

        self.send_negotiate_pb()?;
        self.do_negotiate_step()
    }

    fn do_negotiate_step(&mut self) -> Result<Async<()>, Error> {
        try_ready!(self.recv_negotiate_pb());

        if self.pb.step() != NegotiateStep::Negotiate {
            return Err(Error::Negotiation(format!("expected Negotiate step, received: {:?}", self.pb.step())));
        }

        // Save the server's feature flags.
        self.supported_features.extend(self.pb.supported_features());

        debug_assert!(self.pb.authn_types.len() <= 1);
        // If the server doesn't send back an authentication type, default
        // to SASL in order to maintain backwards compatibility.
        let authn_type = self.pb
                             .authn_types
                             .first_mut()
                             .and_then(|authn_type| authn_type.type_.take())
                             .unwrap_or(SASL);
        match authn_type {
            authentication_type_pb::Type::Sasl(..) => self.do_sasl_initiate(),
            authentication_type_pb::Type::Token(..) => {
                Err(Error::Negotiation("TOKEN authentication is not supported".to_string()))
            },
            authentication_type_pb::Type::Certificate(..) => {
                Err(Error::Negotiation("CERTIFICATE authentication is not supported".to_string()))
            },
        }
    }

    fn do_sasl_initiate(&mut self) -> Result<Async<()>, Error> {
        // Determine which mechanism to use.
        let server_mechs = self.pb
                               .sasl_mechanisms
                               .iter()
                               .flat_map(SaslMechanism::from_pb)
                               .collect::<HashSet<_>>();

        if server_mechs.is_empty() {
            return Err(Error::Negotiation("server does not support any SASL mechanisms".to_string()));
        } else if !server_mechs.contains(&SaslMechanism::Plain) {
            return Err(Error::Negotiation("SASL GSSAPI mechanism is not supported".to_string()));
        }

        self.authentication = Some(AuthenticationType::Sasl(SaslMechanism::Plain));
        self.do_sasl_plain_initiate()
    }

    fn do_sasl_plain_initiate(&mut self) -> Result<Async<()>, Error> {
        self.pb.step = NegotiateStep::SaslInitiate as i32;
        self.pb.token = Some(b"\0kudu-rs-user\0".to_vec());
        self.pb.sasl_mechanisms = vec![SaslMechanism::Plain.to_pb()];
        self.send_negotiate_pb()?;
        self.do_sasl_plain_step()
    }

    fn do_sasl_plain_step(&mut self) -> Result<Async<()>, Error> {
        try_ready!(self.recv_negotiate_pb());

        if self.pb.step() != NegotiateStep::SaslSuccess {
            return Err(Error::Negotiation(format!("expected SaslSuccess step, received: {:?}",
                                                  self.pb.step())));
        }

        self.send_connection_context()?;
        Ok(Async::Ready(()))
    }

    fn send_connection_context(&mut self) -> Result<(), Error> {
        let context = ConnectionContextPb::default();
        self.transport.send(CONNECTION_CONTEXT_CALL_ID, "", "", &[], &self.pb, self.deadline)
    }
}

pub struct Negotiator {
    inner: Option<Inner>,
}

impl Negotiator {
    fn new(transport: Transport, deadline: Instant) -> Inner {
        Inner {
            pb: NegotiatePb::default(),
            transport,
            deadline,
            authentication: None,
            supported_features: Vec::new(),
        }
    }
}

impl Future for Negotiator {
    type Item = Transport;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Transport>, Error> {
        match self.inner {
            Some(ref mut inner) => try_ready!(inner.poll()),
            None => return Ok(Async::NotReady),
        }

        let Inner { transport, ..  } = self.inner.take().unwrap();
        Ok(Async::Ready(transport))
    }
}
