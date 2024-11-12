//! Model for the wallet application state.

pub mod credential;
mod issuance;

pub use credential::CredentialState;
use issuance::IssuanceState;
use serde::Serialize;

use super::Aspect;
use crate::capabilities::store::{Store, StoreEntry};

/// State for the wallet application.
#[derive(Default, Serialize)]
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
    pub fn issuance_offer<Ev>(
        &mut self, encoded_offer: &str, http: crux_http::Http<Ev>, store: Store<Ev>,
    ) {
        self.active_view = Aspect::IssuanceOffer;
        match IssuanceState::from_offer(encoded_offer, http, store) {
            Ok(issuance_state) => self.issuance = Some(issuance_state),
            Err(e) => self.error(e.to_string()),
        }
    }
}
