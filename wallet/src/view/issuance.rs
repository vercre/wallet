//! Issuance flow view models.

use serde::{Deserialize, Serialize};
use vercre_holder::{Claim, CredentialConfiguration, Format};

use super::capitalize;
use crate::model::IssuanceState;

/// View-friendly representation of a credential configuration.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CredentialSummary {
    /// Credential configuration identifier.
    pub config_id: String,

    /// Name from issuer's metadata.
    pub name: String,

    /// Description from issuer's metadata.
    pub description: String,

    /// Claims the credential contains. The entries are the claim names. For
    /// nested claims, the claim name is prefixed with the parent claim name.
    pub claims: Vec<String>,

    /// Background color from issuer's metadata. Empty string if not applicable.
    pub background_color: String,

    /// Text color from issuer's metadata. Empty string if not applicable.
    pub text_color: String,

    /// URL for a logo image.
    ///
    /// Empty string if not applicable.
    pub logo_url: String,

    /// URL for a background image.
    ///
    /// Empty string if not applicable.
    pub background_url: String,
}

impl CredentialSummary {
    /// Convert a CredentialConfiguration to a CredentialSummary.
    pub fn from_configuration(
        config_id: impl Into<String>, config: &CredentialConfiguration,
    ) -> Self {
        let (name, description, background_color, text_color, logo_url, background_url) =
            match &config.display {
                Some(display_list) => {
                    // TODO: Locale support.
                    let display = display_list
                        .clone()
                        .into_iter()
                        .find(|display| display.locale.is_none())
                        .unwrap_or_else(|| display_list[0].clone());
                    let logo_url = match display.logo {
                        Some(logo) => logo.uri.unwrap_or_default(),
                        None => String::new(),
                    };
                    let background_url = match display.background_image {
                        Some(background) => background.uri.unwrap_or_default(),
                        None => String::new(),
                    };
                    (
                        display.name,
                        display.description.unwrap_or_default(),
                        display.background_color.unwrap_or_default(),
                        display.text_color.unwrap_or_default(),
                        logo_url,
                        background_url,
                    )
                }
                None => (
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ),
            };
        Self {
            config_id: config_id.into(),
            name,
            description,
            claims: claim_labels(&config.format),
            background_color,
            text_color,
            logo_url,
            background_url,
        }
    }
}

/// View-friendly representation of a transaction code specification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TxCode {
    /// The type of characters expected. Will be "numeric" or "text".
    pub input_mode: String,

    /// The number of characters expected. Zero if not applicable.
    pub length: i32,

    /// Helper text to display to the user.
    pub description: String,
}

impl Default for TxCode {
    fn default() -> Self {
        Self {
            input_mode: "numeric".into(),
            length: 0,
            description: "".into(),
        }
    }
}

impl From<Option<vercre_holder::TxCode>> for TxCode {
    fn from(tx_code: Option<vercre_holder::TxCode>) -> Self {
        match tx_code {
            Some(tx_code) => Self {
                input_mode: tx_code.input_mode.unwrap_or("numeric".into()),
                length: tx_code.length.unwrap_or_default(),
                description: tx_code.description.unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}

/// View model for an issuance flow.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct IssuanceView {
    /// The credential issuer's identifier.
    pub issuer: String,

    /// The credential issuer's name.
    pub issuer_name: String,

    /// Credential being offered.
    pub offered: CredentialSummary,

    /// PIN requirements.
    pub tx_code: TxCode,
}

impl From<Option<IssuanceState>> for IssuanceView {
    fn from(state: Option<IssuanceState>) -> Self {
        match state {
            Some(state) => {
                // Pick the first credential in the issuance state to display. Since the
                // app is intended to allow the holder to accept or reject the
                // credential and/or claims one at a time.
                if state.offered.is_empty() {
                    return Self::default();
                }
                let Some(next_config) = state.offered.iter().next() else {
                    return Self::default();
                };
                Self {
                    issuer: state.issuer.clone(),
                    issuer_name: state.issuer_name.clone(),
                    tx_code: state.tx_code.into(),
                    offered: CredentialSummary::from_configuration(next_config.0, next_config.1),
                }
            }
            None => Self::default(),
        }
    }
}

// Extract display labels for claims from a credential format.
fn claim_labels(format: &Format) -> Vec<String> {
    let mut labels = Vec::<String>::new();
    match format {
        Format::JwtVcJson(profile) | Format::LdpVc(profile) | Format::JwtVcJsonLd(profile) => {
            if let Some(cs) = &profile.credential_definition.credential_subject {
                for (name, claim) in cs.into_iter() {
                    let mut label = "".to_string();
                    label_from_def(&mut label, &name, &claim);
                    labels.push(label);
                }
            } else {
                return labels;
            }
        }
        Format::IsoMdl(_) => {
            // TODO: Implement this.
        }
        Format::VcSdJwt(_) => {
            // TODO: Implement this.
        }
    }
    labels
}

// Recursively build claim labels from nested claim definitions.
// TODO: Use locales.
fn label_from_def(label: &mut String, key: &str, claim: &Claim) {
    match claim {
        Claim::Entry(def) => {
            if let Some(def_display) = &def.display {
                let locale_display = def_display
                    .clone()
                    .into_iter()
                    .find(|d| d.locale.is_none())
                    .unwrap_or_else(|| def_display[0].clone());
                label.push_str(&locale_display.name);
            } else {
                label.push_str(&capitalize(&key));
            }
        }
        Claim::Set(set) => {
            if !label.is_empty() {
                label.push_str(".");
            }
            for (name, claim) in set.into_iter() {
                label_from_def(label, name, claim);
            }
        }
    }
}
