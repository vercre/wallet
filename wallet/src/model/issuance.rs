//! Issuance sub-app state.

use std::collections::HashMap;

use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use vercre_holder::issuance::OfferRequest;
use vercre_holder::{CredentialConfiguration, CredentialOffer, TxCode};

use crate::config;
use crate::provider::Provider;

/// Application state for the issuance sub-app.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct IssuanceState {
    /// Issuance flow identifier to pass to the vercre-holder crate for state
    /// management.
    pub id: String,

    /// Identifier of the issuer of the credential(s)
    pub issuer: String,

    /// Issuer's name.
    pub issuer_name: String,

    /// Description of the credential(s) offered, keyed by credential
    /// configuration ID.
    pub offered: HashMap<String, CredentialConfiguration>,

    /// Description of the type of PIN needed to accept the offer.
    pub tx_code: Option<TxCode>,

    /// PIN set by the holder.
    pub pin: Option<String>,
}

/// State change implementation.
impl IssuanceState {
    /// Create an issuance state from a URL-encoded offer.
    pub fn from_offer<Ev>(
        provider: &Provider<Ev>, encoded_offer: &str
    ) -> anyhow::Result<Self>
    where Ev: 'static
    {
        let offer_str = urlencoding::decode(encoded_offer)?;
        let offer: CredentialOffer = serde_json::from_str(&offer_str)?;
        let tx_code = {
            if let Some(grants) = offer.grants.clone() {
                if let Some(pre_auth) = grants.pre_authorized_code {
                    pre_auth.tx_code    
                } else {
                    None
                }
            }
            else { None }
        };
        let offer_req = OfferRequest {
            client_id: config::client_id(),
            subject_id: config::subject_id(),
            offer,
        };
        let offer_response = block_on(vercre_holder::issuance::offer(provider.clone(), &offer_req))?;
        Ok(Self {
            id: offer_response.issuance_id,
            issuer: offer_response.issuer,
            issuer_name: offer_response.issuer_name,
            offered: offer_response.offered,
            tx_code,
            pin: None,
        })
    }
}
