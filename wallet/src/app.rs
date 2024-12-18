//! This module contains the core application fabric for the wallet, including
//! the model, events, and effects that drive the application.

use crux_core::render::Render;
use crux_http::{Http, HttpError};
use crux_kv::KeyValue;
use serde::{Deserialize, Serialize};
use vercre_holder::credential::{self, Credential, ImageData};
use vercre_holder::{Issuer, TokenResponse};

use crate::capabilities::key::KeyStore;
use crate::capabilities::sse::ServerSentEvents;
use crate::capabilities::store::{Catalog, Store, StoreEntry, StoreError};
use crate::model::Model;
use crate::provider::Provider;
use crate::view::ViewModel;

/// Aspect of the application.
///
/// This allows the UI navigation to be reactive: controlled in response to the
/// user's actions.
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Aspect {
    /// Display and deletion of credentials stored in the wallet.
    #[default]
    CredentialList,

    /// Display of a single credential.
    CredentialDetail,

    /// Trigger a credential issuance using an offer QR code.
    IssuanceScan,

    /// View the offer details to decide whether or not to proceed with
    /// issuance.
    IssuanceOffer,

    /// Trigger a credential verification using a presentation request QR code.
    PresentationScan,

    /// The application is in an error state.
    Error,
}

/// Events that can be sent to the wallet application.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    /// Error event is emitted by the core when an error occurs.
    #[serde(skip)]
    Error(String),

    //--- Credential events ----------------------------------------------------
    /// Event emitted by the shell when the app first loads.
    Ready,

    /// Event emitted by the shell to select a credential from the list of
    /// stored credentials for detailed display.
    SelectCredential(String),

    /// Event emitted by the shell to delete a credential from the wallet.
    DeleteCredential(String),

    /// Event emitted by the core when the store capability has loaded
    /// credentials.
    #[serde(skip)]
    CredentialsLoaded(Result<Vec<StoreEntry>, StoreError>),

    /// Event emitted by the core when the store capability has stored a
    /// credential.
    #[serde(skip)]
    CredentialStored(Result<(), StoreError>),

    /// Event emitted by the core when the store capability has deleted a
    /// credential.
    #[serde(skip)]
    CredentialDeleted(Result<(), StoreError>),

    //--- Issuance events ------------------------------------------------------
    /// Event emitted by the shell when the user wants to scan an issuance offer
    /// QR code.
    ScanIssuanceOffer,

    /// Event emitted by the shell when the user scans an offer QR code.
    IssuanceOffer(String),

    /// Event emitted by the core when issuer metadata has been received.
    #[serde(skip)]
    IssuanceIssuer(Result<Issuer, HttpError>),

    /// Event emitted by the core when an offered credential's logo has been
    /// fetched.
    #[serde(skip)]
    IssuanceLogo(Result<(String, ImageData), HttpError>),

    /// Event emitted by the core when an offered credential's background image
    /// has been fetched.
    #[serde(skip)]
    IssuanceBackground(Result<(String, ImageData), HttpError>),

    /// Event emitted by the shell when the user has accepted an issuance offer.
    IssuanceAccepted,

    /// Event emitted by the shell when the user has entered a PIN.
    IssuancePin(String),

    /// Event emitted by the core when an access token has been received.
    #[serde(skip)]
    IssuanceToken(Result<TokenResponse, HttpError>),

    /// Event emitted by the core when a proof has been created.
    #[serde(skip)]
    IssuanceProof(Result<String, String>),

    /// Event emitted by the core when a credential has been received.
    #[serde(skip)]
    IssuanceCredential(Result<(String, Credential), HttpError>),

    /// Event emitted by the core when a credential has been stored.
    #[serde(skip)]
    IssuanceStored(Result<String, StoreError>),

    /// Event emitted by the shell to cancel an issuance.
    CancelIssuance,
}

/// Set of capabilities available to the application.
#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub http: Http<Event>,
    pub key_store: KeyStore<Event>,
    pub kv: KeyValue<Event>,
    pub sse: ServerSentEvents<Event>,
    pub store: Store<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Capabilities = Capabilities;
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        let provider = Provider::new(
            caps.http.clone(),
            caps.key_store.clone(),
            caps.kv.clone(),
            caps.store.clone(),
        );
        match msg {
            Event::Error(e) => {
                model.error(e);
                caps.render.render();
            }
            Event::Ready => {
                model.ready();
                caps.store.list("credential", Event::CredentialsLoaded);
                caps.render.render();
            }
            Event::SelectCredential(id) => {
                model.select_credential(id);
                caps.render.render();
            }
            Event::DeleteCredential(id) => {
                caps.store.delete("credential", id, Event::CredentialDeleted);
            }
            Event::CredentialsLoaded(Ok(entries)) => {
                model.credentials_loaded(entries);

                // TODO: Remove this try-out code
                // if entries.len() < 1 {
                //     let json = include_bytes!("model/credentials.json");
                //     let credentials: Vec<Credential> =
                //         serde_json::from_slice(json).expect("should deserialize");
                //     let credential = credentials[entries.len()].clone();
                //     let data = serde_json::to_vec(&credential).expect("should serialize");
                //     caps.store.save("credential".into(), credential.id.clone(), data,
                // Event::CredentialStored); }
                // -------------------------------

                caps.render.render();
            }
            Event::CredentialStored(Ok(())) => {
                caps.store.list(Catalog::Credential.to_string(), Event::CredentialsLoaded);
            }
            Event::CredentialDeleted(Ok(())) => {
                model.delete_credential();
                caps.store.list(Catalog::Credential.to_string(), Event::CredentialsLoaded);
                caps.render.render();
            }
            Event::ScanIssuanceOffer => {
                model.scan_issuance_offer();
                caps.render.render();
            }
            Event::IssuanceOffer(encoded_offer) => {
                model.issuance_offer(&encoded_offer);
            }
            Event::IssuanceIssuer(Ok(issuer)) => {
                todo!();
            }
            Event::IssuanceLogo(Ok((config_id, image))) => {
                todo!();
            }
            Event::IssuanceBackground(Ok((config_id, image))) => {
                todo!();
            }
            Event::IssuanceAccepted => {
                todo!();
            }
            Event::IssuancePin(pin) => {
                todo!();
            }
            Event::IssuanceToken(Ok(token)) => {
                todo!();
            }
            Event::IssuanceProof(Ok(jwt)) => {
                todo!();
            }
            Event::IssuanceCredential(Ok((config_id, credential))) => {
                todo!();
            }
            Event::IssuanceStored(Ok(config_id)) => {
                todo!();
            }
            Event::CancelIssuance => {
                model.cancel_issuance(&provider);
                caps.render.render();
            }
            // Store errors
            Event::CredentialsLoaded(Err(error))
            | Event::CredentialStored(Err(error))
            | Event::CredentialDeleted(Err(error))
            | Event::IssuanceStored(Err(error)) => {
                model.error(error.to_string());
                caps.render.render();
            }
            // HTTP errors
            Event::IssuanceIssuer(Err(error))
            | Event::IssuanceLogo(Err(error))
            | Event::IssuanceBackground(Err(error))
            | Event::IssuanceToken(Err(error))
            | Event::IssuanceCredential(Err(error)) => {
                model.error(error.to_string());
                caps.render.render();
            }
            // Error as string
            Event::IssuanceProof(Err(error)) => {
                model.error(error);
                caps.render.render();
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel {
            active_view: model.active_view.clone(),
            credential_view: model.credential.clone().into(),
            issuance_view: model.issuance.clone().into(),
            error: model.error.clone().unwrap_or_default(),
        }
    }
}
