//! # View Model
//! 
pub mod credential;
pub mod issuance;

use credential::CredentialView;
use issuance::IssuanceView;
use serde::{Deserialize, Serialize};

use super::Aspect;

/// View model for the wallet application.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ViewModel {
    /// Which aspect of the application is currently active.
    pub active_view: Aspect,

    /// Credential view model.
    pub credential_view: CredentialView,

    /// Issuance view model.
    pub issuance_view: IssuanceView,

    /// Error message.
    pub error: String,
}
