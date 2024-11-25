//! # Wallet Holder Capability
//!
//! The holder capability provides for interactions between `vercre-holder` and
//! capabilities provided by the shell in order to implement the callbacks
//! needed. This unpacks the standard `vercre-holder` API into a more granular
//! set of helper functions that can work with Crux's event and effects system.
use std::fmt::{self, Debug};

use crux_core::capability::{CapabilityContext, Operation};
use crux_core::Capability;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use vercre_holder::issuance::{OfferRequest, OfferResponse};

/// Errors that can be returned by the Holder capability.
#[derive(Clone, Debug, Deserialize, Serialize, Error, PartialEq, Eq)]
pub enum HolderError {
    /// Invalid request.
    #[error("invalid store request {message}")]
    InvalidRequest { message: String },

    /// The response from the shell capability was invalid.
    #[error("invalid store response {message}")]
    InvalidResponse { message: String },
}

/// Supported operations for the Holder capability.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HolderOperation {
    /// Process a credential offer from an issuer.
    Offer { request: OfferRequest },
}

impl Debug for HolderOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolderOperation::Offer { request } => {
                f.debug_struct("HolderOperation::Offer").field("request", request).finish()
            }
        }
    }
}

/// The result of an operation on the Holder capability.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum HolderResult {
    /// The operation was successful.
    Ok { response: HolderResponse },

    /// The operation failed.
    Err { error: HolderError },
}

impl HolderResult {
    fn unwrap_offer(self) -> Result<OfferResponse, HolderError> {
        match self {
            HolderResult::Ok { response } => match response {
                HolderResponse::Offer { offer_response } => Ok(offer_response),
                // _ => {
                //     panic!("unexpected response type for Offer operation: {:?}", response);
                // }
            },
            HolderResult::Err { error } => Err(error.clone()),
        }
    }
}

/// The possible responses from the Holder capability.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HolderResponse {
    /// Response to an offer request.
    Offer { offer_response: OfferResponse },
}

impl Operation for HolderOperation {
    type Output = HolderResult;
}

/// Capability type.
pub struct Holder<Ev> {
    context: CapabilityContext<HolderOperation, Ev>,
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

    #[cfg(feature = "typegen")]
    fn register_types(generator: &mut crux_core::typegen::TypeGen) -> crux_core::typegen::Result {
        generator.register_type::<HolderResponse>()?;
        generator.register_type::<HolderError>()?;
        generator.register_type::<Self::Operation>()?;
        generator.register_type::<<Self::Operation as Operation>::Output>()?;
        Ok(())
    }
}

impl<Ev> Clone for Holder<Ev> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
        }
    }
}

impl<Ev> Holder<Ev>
where
    Ev: 'static,
{
    /// Creates a new Holder capability instance.
    pub fn new(context: CapabilityContext<HolderOperation, Ev>) -> Self {
        Self { context }
    }

    /// Process a credential offer from an issuer.
    pub fn offer<F>(&self, request: OfferRequest, make_event: F)
    where
        F: FnOnce(Result<OfferResponse, HolderError>) -> Ev + Send + Sync + 'static,
    {
        let context = self.context.clone();
        self.context.spawn({
            async move {
                let response = offer(&context, request).await;
                context.update_app(make_event(response))
            }
        });
    }
}

async fn offer<Ev: 'static>(
    context: &CapabilityContext<HolderOperation, Ev>, request: OfferRequest,
) -> Result<OfferResponse, HolderError> {
    context.request_from_shell(HolderOperation::Offer {
        request,
    })
    .await
    .unwrap_offer()
}
