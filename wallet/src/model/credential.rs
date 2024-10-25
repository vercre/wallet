//! Credential sub-app state.

use serde::{Deserialize, Serialize};
use vercre_holder::Credential;

/// Application state for the credential sub-app.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct CredentialState {
    /// Currently selected credential.
    pub id: Option<String>,

    /// Credentials stored in the wallet.
    pub credentials: Vec<Credential>,
}
