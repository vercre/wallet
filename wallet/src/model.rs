//! Model for the wallet application state.

pub mod credential;
mod issuance;

pub use credential::CredentialState;
pub use issuance::{IssuanceState, OfferedCredential};

use super::Aspect;
use crate::capabilities::store::StoreEntry;
use crate::provider::Provider;

/// State for the wallet application.
#[derive(Default)]
pub struct Model {
    /// Which aspect of the application is currently active.
    pub active_view: Aspect,

    /// Credential state.
    pub credential: CredentialState,

    /// Issuance state.
    pub issuance: Option<IssuanceState>,

    /// Error state.
    pub error: Option<String>,
}

/// Methods to set the application state based on events.
impl Model {
    /// An error has occurred. Set the error state.
    pub fn error(&mut self, error: String) {
        self.active_view = Aspect::Error;
        self.error = Some(error);
    }

    /// Set up the model with an initial state.
    pub fn ready(&mut self) {
        self.active_view = Aspect::CredentialList;
        self.credential = CredentialState::init();
    }

    /// The user has selected a credential in their wallet to view.
    pub fn select_credential(&mut self, id: String) {
        self.active_view = Aspect::CredentialDetail;
        self.credential.id = Some(id);
    }

    /// The credentials have been retrieved from the wallet's store.
    pub fn credentials_loaded(&mut self, entries: Vec<StoreEntry>) {
        self.active_view = Aspect::CredentialList;
        self.credential.set_credentials(entries);
    }

    /// The user has deleted a credential from their wallet.
    pub fn delete_credential(&mut self) {
        self.credential.id = None;
    }

    /// The user wants to scan an issuance offer QR code.
    pub fn scan_issuance_offer(&mut self) {
        self.active_view = Aspect::IssuanceScan;
        self.issuance = None;
    }

    /// The user has scanned an issuance offer QR code so we can initiate a
    /// pre-authorized issuance flow.
    pub fn issuance_offer(&mut self, encoded_offer: &str) {
        match IssuanceState::from_offer(encoded_offer) {
            Ok(issuance_state) => self.issuance = Some(issuance_state),
            Err(e) => self.error(e.to_string()),
        }
    }

    /// The user has decided to no longer go ahead with the issuance process.
    pub fn cancel_issuance<Ev>(&mut self, provider: &Provider<Ev>)
    where
        Ev: 'static
    {
        self.active_view = Aspect::CredentialList;
        if let Some(issuance) = &mut self.issuance {
            match issuance.cancel(provider) {
                Ok(_) => self.issuance = None,
                Err(e) => self.error(e.to_string()),
            }
        }
    }
}
