//! Issuance sub-app state.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use vercre_holder::{CredentialConfiguration, TxCode};

// Application state for the issuance sub-app.
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
