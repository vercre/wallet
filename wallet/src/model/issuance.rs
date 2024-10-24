//! Issuance sub-app state.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use vercre_holder::{CredentialConfiguration, TxCode};

/// Application state for the issuance sub-app.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct IssuanceState {
    /// Issuance flow identifier to pass to the vercre-holder crate for state
    /// management.
    pub id: String,

    /// Issuer of the credential(s)
    pub issuer: String,

    /// Description of the credential(s) offered, keyed by credential
    /// configuration ID.
    pub offered: HashMap<String, CredentialConfiguration>,

    /// Description of the type of PIN needed to accept the offer.
    pub tx_code: Option<TxCode>,

    /// PIN set by the holder.
    pub pin: Option<String>,
}

//// State change implementation.
// impl IssuanceState {
//     /// Create an issuance state from a URL-encoded offer.
//     pub fn from_offer(encoded_offer: &str) -> anyhow::Result<Self> {
//         let offer_str = urlencoding::decode(encoded_offer)?;
//         let offer: CredentialOffer = serde_json::from_str(&offer_str)?;
//         let request = OfferRequest {
//             client_id: config::client_id(),
//         };

//         todo!()
//     }
// }
