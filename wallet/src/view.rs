//! # View Model
//! 
mod credential;

use credential::{CredentialSummary, CredentialDetail};
use serde::{Deserialize, Serialize};

use super::SubApp;

/// View model for the wallet application.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ViewModel {
    /// Which aspect of the application is currently active.
    pub active_view: SubApp,

    /// List of loaded credentials.
    pub credentials: Vec<CredentialSummary>,

    /// Currently selected credential.
    pub credential: Option<CredentialDetail>,

    // --- TODO: Remove ---
    pub text: String,
    pub confirmed: bool,
    // --------------------
}
