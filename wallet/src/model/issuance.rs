//! Issuance sub-app state.
use std::collections::HashMap;

use anyhow::bail;
use vercre_holder::credential::ImageData;
use vercre_holder::issuance::{
    Accepted, IssuanceFlow, NotAccepted, PreAuthorized, WithOffer, WithToken, WithoutToken,
};
use vercre_holder::{CredentialConfiguration, CredentialOffer, PreAuthorizedCodeGrant};

use crate::provider::Provider;

/// Configuration and image information for an offered credential.
#[derive(Clone, Debug, Default)]
pub struct OfferedCredential {
    /// Credential configuration.
    pub config: CredentialConfiguration,

    /// Logo image data.
    pub logo: Option<ImageData>,

    /// Background image data.
    pub background: Option<ImageData>,

    /// Credential has been received.
    pub received: bool,

    /// Credential has been stored.
    pub stored: bool,
}

impl OfferedCredential {
    /// Determine if the credential logo needs to be fetched.
    pub fn needs_logo(&self) -> bool {
        if self.logo.is_some() {
            return false;
        }
        if let Some(display) = &self.config.display {
            if let Some(logo) = &display[0].logo {
                if logo.uri.is_some() {
                    return true;
                }
            }
        }
        false
    }

    /// Determine if the credential background needs to be fetched.
    pub fn needs_background(&self) -> bool {
        if self.background.is_some() {
            return false;
        }
        if let Some(display) = &self.config.display {
            if let Some(background) = &display[0].background_image {
                if background.uri.is_some() {
                    return true;
                }
            }
        }
        false
    }
}

/// Application state for the issuance sub-app.
#[derive(Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::large_enum_variant)]
pub enum IssuanceState {
    /// No issuance is in progress.
    #[default]
    Inactive,

    /// An offer has been received
    Offered { offer: CredentialOffer, grant: PreAuthorizedCodeGrant },

    /// Issuer metadata has been received. Can use this state to keep updating
    /// the offered credentials' logo and background images.
    IssuerMetadata {
        flow: IssuanceFlow<WithOffer, PreAuthorized, NotAccepted, WithoutToken>,
        offerred: HashMap<String, OfferedCredential>,
    },

    /// The offer has been accepted by the user. Can use this state to update
    /// the PIN number if needed.
    Accepted {
        flow: IssuanceFlow<WithOffer, PreAuthorized, Accepted, WithoutToken>,
        offerred: HashMap<String, OfferedCredential>,
    },

    /// An access token has been received.
    Token {
        flow: IssuanceFlow<WithOffer, PreAuthorized, Accepted, WithToken>,
        offerred: HashMap<String, OfferedCredential>,
    },

    /// A proof has been created. Can use this state to receive credentials and
    /// update the offered list to keep track of outstanding credentials. Can
    /// also use it to keep track of the credentials stored.
    Proof {
        flow: IssuanceFlow<WithOffer, PreAuthorized, Accepted, WithToken>,
        offerred: HashMap<String, OfferedCredential>,
        proof: String,
    },
}

/// State change implementation.
impl IssuanceState {
    /// Create an issuance state from a URL-encoded offer.
    pub fn from_offer(encoded_offer: &str) -> anyhow::Result<Self> {
        let offer_str = urlencoding::decode(encoded_offer)?;
        let offer = serde_json::from_str::<CredentialOffer>(&offer_str)?;

        // Check the offer has a pre-authorized grant. This is the only flow
        // type supported by this wallet (for now).
        let Some(pre_auth_code_grant) = offer.pre_authorized_code() else {
            bail!("grant other than pre-authorized code is not supported");
        };

        Ok(Self::Offered {
            offer,
            grant: pre_auth_code_grant,
        })
    }

    /// Cancel the issuance process.
    pub fn cancel<Ev>(&mut self, _provider: &Provider<Ev>) -> anyhow::Result<()>
    where
        Ev: 'static,
    {
        // TODO: Reset state
        Ok(())
    }
}
