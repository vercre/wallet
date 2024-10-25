//! Credential view models

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use vercre_holder::credential::{self, Credential};
use vercre_holder::Quota;

/// Summary view for a verifiable credential
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialSummary {
    /// Credential ID
    pub id: String,

    /// CSS color to use for the background of a credential display
    pub background_color: Option<String>,

    /// CSS color to use for the text of a credential display
    pub color: Option<String>,

    /// Label to display on the credential to indicate the issuer
    pub issuer: Option<String>,

    /// Logo to display on the credential
    pub logo: Option<Image>,

    /// URL of the original source of the logo
    pub logo_url: Option<String>,

    /// Background image to display on the credential
    pub background: Option<Image>,

    /// URL of the original source of the background image
    pub background_url: Option<String>,

    /// Name of the credential
    pub name: Option<String>,
}

impl From<Credential> for CredentialSummary {
    fn from(credential: Credential) -> Self {
        let displays = credential.display.clone().unwrap_or_default();
        // TODO: locale
        let display = displays[0].clone();
        Self {
            id: credential.id.clone(),
            background_color: display.background_color.clone(),
            color: display.text_color.clone(),
            issuer: Some(credential.issuer.clone()),
            logo: credential.logo.as_ref().map(|logo| logo.clone().into()),
            logo_url: match display.logo {
                Some(image) => image.uri,
                None => None,
            },
            background: credential.background.as_ref().map(|bg| bg.clone().into()),
            background_url: match display.background_image {
                Some(image) => image.uri,
                None => None,
            },
            name: Some(display.name),
        }
    }
}

/// Image information
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Image {
    /// Base64 encoded image
    pub data: String,

    /// Image media type
    pub media_type: String,
}

impl From<credential::Image> for Image {
    fn from(img: credential::Image) -> Self {
        Self {
            data: img.image,
            media_type: img.media_type,
        }
    }
}
/// Detail view for a verifiable credential
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialDetail {
    /// Display information
    display: CredentialSummary,

    /// Start of validity period
    valid_from: Option<String>,

    /// End of validity period (expiry)
    valid_until: Option<String>,
    
    /// Description
    description: Option<String>,
    
    /// Claims
    claims: HashMap<String, Value>,
}

impl From<Credential> for CredentialDetail {
    fn from(credential: Credential) -> Self {
        let displays = credential.display.clone().unwrap_or_default();
        // TODO: locale
        let display = displays[0].clone();
        let vc = credential.vc.clone();
        let mut claims = HashMap::new();

        let subjects = match &vc.credential_subject {
            Quota::One(sub) => vec![sub.clone()],
            Quota::Many(subs) => subs.clone(),
        };

        for subject in subjects {
            let claims_map = subject.claims;
            for (key, value) in claims_map {
                let val = serde_json::to_value(&value).unwrap_or_default();
                claims.insert(key.clone(), val);
            }
        }

        Self {
            display: credential.into(),
            valid_from: vc.valid_from.map(|d| d.to_rfc2822()),
            valid_until: vc.valid_until.map(|d| d.to_rfc2822()),
            description: display.description,
            claims,
        }
    }
}
