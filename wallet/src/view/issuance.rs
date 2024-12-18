//! Issuance flow view models.

use serde::{Deserialize, Serialize};

use super::credential::Credential;
use crate::model::IssuanceState;

/// View-friendly representation of a transaction code specification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TxCode {
    /// The type of characters expected. Will be "numeric" or "text".
    pub input_mode: String,

    /// The number of characters expected. Zero if not applicable.
    pub length: i32,

    /// Helper text to display to the user.
    pub description: String,
}

impl Default for TxCode {
    fn default() -> Self {
        Self {
            input_mode: "numeric".into(),
            length: 0,
            description: "".into(),
        }
    }
}

impl From<Option<vercre_holder::TxCode>> for TxCode {
    fn from(tx_code: Option<vercre_holder::TxCode>) -> Self {
        match tx_code {
            Some(tx_code) => Self {
                input_mode: tx_code.input_mode.unwrap_or("numeric".into()),
                length: tx_code.length.unwrap_or_default(),
                description: tx_code.description.unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}

/// View model for an issuance flow.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct IssuanceView {
    /// The credential issuer's identifier.
    pub issuer: String,

    /// The credential issuer's name.
    pub issuer_name: String,

    /// Credential being offered.
    pub offered: Credential,

    /// PIN requirements.
    pub tx_code: TxCode,
}

impl From<Option<IssuanceState>> for IssuanceView {
    fn from(state: Option<IssuanceState>) -> Self {
        match state {
            Some(_state) => {
                todo!()
            }
            None => Self::default(),
        }
    }
}
