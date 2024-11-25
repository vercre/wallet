//! Issuance sub-app state.

use std::collections::HashMap;

use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use vercre_holder::credential::ImageData;
use vercre_holder::issuance::{CancelRequest, OfferRequest};
use vercre_holder::{CredentialConfiguration, CredentialOffer, Grants, TxCode};

use crate::config;
use crate::provider::Provider;

/// Configuration and image information for an offered credential.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OfferedCredential {
    /// Credential configuration.
    pub config: CredentialConfiguration,

    /// Logo image data.
    pub logo: Option<ImageData>,

    /// Background image data.
    pub background: Option<ImageData>,
}

/// Application state for the issuance sub-app.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct IssuanceState {
    /// Issuance flow identifier to pass to the vercre-holder crate for state
    /// management.
    pub id: String,

    /// Identifier of the issuer of the credential(s)
    pub issuer: String,

    /// Issuer's name.
    pub issuer_name: String,

    /// Description of the credential(s) offered, keyed by credential
    /// configuration ID.
    pub offered: HashMap<String, OfferedCredential>,

    /// Description of the type of PIN needed to accept the offer.
    pub tx_code: Option<TxCode>,

    /// PIN set by the holder.
    pub pin: Option<String>,
}

/// State change implementation.
impl IssuanceState {
    /// Create an issuance state from a URL-encoded offer.
    pub fn from_offer<Ev>(
        provider: &Provider<Ev>, encoded_offer: &str
    ) -> anyhow::Result<Self>
    where Ev: 'static
    {
        let fields: Vec<(String, String)> = serde_urlencoded::from_str(encoded_offer)?;

        let mut offer = CredentialOffer::default();
        if let Some(credential_issuer) = &fields.iter().find(|(k, _)| k == "credential_issuer") {
            offer.credential_issuer = credential_issuer.1.clone();
        } else {
            return Err(anyhow::anyhow!("credential_issuer not found"));
        }

        if let Some(credential_configuration_ids) = &fields.iter().find(|(k, _)| k == "credential_configuration_ids") {
            offer.credential_configuration_ids = serde_json::from_str::<Vec<String>>(&credential_configuration_ids.1)?;
        } else {
            return Err(anyhow::anyhow!("credential_configuration_ids not found"));
        }

        if let Some(grants) = &fields.iter().find(|(k, _)| k == "grants") {
            offer.grants = Some(serde_json::from_str::<Grants>(&grants.1)?);
        } else {
            offer.grants = None;
        }

        let tx_code = {
            if let Some(grants) = offer.grants.clone() {
                if let Some(pre_auth) = grants.pre_authorized_code {
                    pre_auth.tx_code    
                } else {
                    None
                }
            }
            else { None }
        };

        let offer_req = OfferRequest {
            client_id: config::client_id(),
            subject_id: config::subject_id(),
            offer,
        };
        let offer_response = block_on(vercre_holder::issuance::offer(provider.clone(), &offer_req))?;

        let mut offered = HashMap::new();
        for (config_id, config) in offer_response.offered {
            let (logo, background) = match &config.display {
                Some(display) => {
                    let logo = if let Some(logo_info) = &display[0].logo {
                        if let Some(uri) = &logo_info.uri {
                            Some(block_on(vercre_holder::provider::Issuer::image(provider.clone(), uri))?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let background = if let Some(background_info) = &display[0].background_image {
                        if let Some(uri) = &background_info.uri {
                            Some(block_on(vercre_holder::provider::Issuer::image(provider.clone(), uri))?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    (logo, background)
                }
                None => (None, None),
            };
            offered.insert(config_id.clone(), OfferedCredential { config, logo, background });
        }        

        Ok(Self {
            id: offer_response.issuance_id,
            issuer: offer_response.issuer,
            issuer_name: offer_response.issuer_name,
            offered,
            tx_code,
            pin: None,
        })
    }

    /// Cancel the issuance process.
    pub fn cancel<Ev>(&mut self, provider: &Provider<Ev>) -> anyhow::Result<()>
    where Ev: 'static
    {
        let cancel_request = CancelRequest {
            issuance_id: self.id.clone(),
        };
        block_on(vercre_holder::issuance::cancel(provider.clone(), &cancel_request))?;
        Ok(())
    }
}
