//! # View Model
//! 
pub mod credential;

use credential::CredentialView;
use serde::{Deserialize, Serialize};

use super::Aspect;

/// View model for the wallet application.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ViewModel {
    /// Which aspect of the application is currently active.
    pub active_view: Aspect,

    /// Credential view model.
    pub credential_view: CredentialView,

    // --- TODO: Remove ---
    pub text: String,
    pub confirmed: bool,
    // --------------------
}
