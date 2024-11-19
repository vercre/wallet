//! # Holder Capability. Wrapper for `vercre-holder` endpoints to allow for
//! asynchronous processing via the Crux event and side-effect system.
use std::fmt::{self, Debug, Display};

use crux_core::capability::{CapabilityContext, Operation};
use crux_core::Capability;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use vercre_holder::issuance::{OfferRequest, OfferResponse};

use crate::provider::Provider;

/// Errors that can be returned by the Holder capability.
#[derive(Clone, Debug, Deserialize, Serialize, Error, PartialEq, Eq)]
pub enum HolderError {
    /// Invalid request.
    #[error("invalid holder request {message}")]
    InvalidRequest { message: String },

    /// The response from the shell capability was invalid.
    #[error("invalid holder response {message}")]
    InvalidResponse { message: String },
}

/// Supported operations for the Holder capability.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HolderOperation {
    GetOffer {
        offer_request: OfferRequest,
    },
}

impl Debug for HolderOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolderOperation::GetOffer { offer_request } => {
                write!(f, "GetOffer {{ offer_request: {:?} }}", offer_request)
            }
        }
    }
}

/// The result of an operation on the `vercre-holder` API.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum HolderResult {
    /// The operation was successful.
    Ok { response: HolderResponse },

    /// The operation failed.
    Err { error: HolderError },
}

impl HolderResult {
    fn unwrap_get_offer(self) -> Result<OfferResponse, HolderError> {
        match self {
            HolderResult::Ok { response } => match response {
                HolderResponse::Offer{ offer_response } => Ok(offer_response),
                _ => {
                    panic!("unexpected response for GetOffer operation: {:?}", response);
                },
            },
            HolderResult::Err { error } => Err(error.clone()),
        }
    }
}

/// The possible responses from the Holder capability.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum HolderResponse {
    /// The response from getting offer details from an issuer.
    Offer {
        offer_response: OfferResponse,
    }
}

impl Operation for HolderOperation {
    type Output = HolderResult;
}

/// The type used to implement the Holder capability.
pub struct Holder<Ev> {
    context: CapabilityContext<HolderOperation, Ev>,
    provider: Provider<Ev>,
}

impl<Ev> Capability<Ev> for Holder<Ev> {
    type MappedSelf<MappedEv> = Holder<MappedEv>;
    type Operation = HolderOperation;

    fn map_event<F, NewEv>(&self, f: F) -> Self::MappedSelf<NewEv>
    where
        F: Fn(NewEv) -> Ev + Send + Sync + 'static,
        Ev: 'static,
        NewEv: 'static + Send,
    {
        Holder::new(self.context.map_event(f))
    }
}

impl<Ev> Clone for Holder<Ev> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            provider: self.provider.clone(),
        }
    }
}

impl<Ev> Holder<Ev>
where 
    Ev: 'static,
{
    /// Create a new instance of the Holder capability.
    pub fn new(context: CapabilityContext<HolderOperation, Ev>) -> Self {
        context.
        Self {
            context,
            provider: Provider::new(context.clone()),
        }
    }
}