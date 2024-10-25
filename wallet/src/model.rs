//! Model for the wallet application state.

mod credential;
mod issuance;

use chrono::{serde::ts_milliseconds_option::deserialize as ts_milliseconds_option, DateTime, Utc};
pub use credential::CredentialState;
use issuance::IssuanceState;
use serde::{Deserialize, Serialize};

use super::SubApp;

/// State for the wallet application.
#[derive(Default, Serialize)]
pub struct Model {
    /// Which aspect of the application is currently active.
    pub active_view: SubApp,

    /// Credential state.
    pub credential: CredentialState,

    /// Issuance state.
    pub issuance: Option<IssuanceState>,

    /// Error state.
    pub error: Option<String>,

    // TODO: Remove ---
    pub count: Count,
    // ----------------
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Count {
    pub value: isize,
    #[serde(deserialize_with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}
