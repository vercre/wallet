//! Credential sub-app state.

use serde::{Deserialize, Serialize};
use vercre_holder::credential::Credential;

/// Application state for the credential sub-app.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct CredentialState {
    /// Currently selected credential.
    pub id: Option<String>,

    /// Credentials stored in the wallet.
    pub credentials: Vec<Credential>,
}

impl CredentialState {
    /// Create a new credential state.
    #[must_use]
    pub fn init() -> Self {
        let json = include_bytes!("credentials.json");
        let credentials: Vec<Credential> = serde_json::from_slice(json).expect("should deserialize");
        Self {
            id: None,
            credentials,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let state = CredentialState::init();
        assert_eq!(state.credentials.len(), 2);
    }
}
