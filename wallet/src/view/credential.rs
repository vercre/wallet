//! Credential view models

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use vercre_holder::credential::{Credential as CredentialModel, ImageData};
use vercre_holder::Claim;

use crate::model::CredentialState;

/// View model for a verifiable credential.
///
/// Matches the `Credential` model from the `vercre-holder` crate as closely as
/// possible but caters for ease of code generation for shells.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Credential {
    /// Credential `id` is the credential's unique identifier
    /// (from Verifiable Credential `id` or generated if credential has no
    /// `id`).
    pub id: String,

    /// The credential issuer.
    pub issuer: String,

    /// The Verifiable Credential as issued, for use in Presentation
    /// Submissions. This could be a base64-encoded JWT or 'stringified'
    /// JSON.
    pub issued: String,

    /// The credential type. Used to determine whether a credential matches a
    /// presentation request.
    #[serde(rename = "type")]
    pub type_: Vec<String>,

    /// Credential format. Information on how the encoded credential is
    /// formatted.
    pub format: String,

    /// Claims display. The key is the subject ID and the value is the claims
    /// displayed as a string with a claim on each line and nested claims
    /// indented.
    pub claims: HashMap<String, String>,

    /// The date the credential was issued as an RFC3339 string.
    pub issuance_date: String,

    /// The date the credential is valid from as an RFC3339 string. Empty string
    /// if not applicable.
    pub valid_from: String,

    /// The date the credential is valid until (expiry) as an RFC3339 string.
    /// Empty string if not applicable.
    pub valid_until: String,

    /// Name from issuer's metadata.
    pub name: String,

    /// Description from issuer's metadata.
    pub description: String,

    /// Background color from issuer's metadata. Empty string if not applicable.
    pub background_color: String,

    /// A base64-encoded logo image for the credential ingested from the logo
    /// url in the display section of the metadata.
    ///
    /// The elements of `ImageData` will be empty strings if the logo has not
    /// been set.
    pub logo: ImageData,

    /// A base64-encoded background image for the credential ingested from the
    /// url in the display section of the metadata.
    ///
    /// The elements of `ImageData` will be empty strings if the background has
    /// not been set.
    pub background: ImageData,
}

impl From<CredentialModel> for Credential {
    fn from(credential: CredentialModel) -> Self {
        let mut claims = HashMap::new();
        for claim_set in &credential.subject_claims {
            let mut display = String::new();
            claims_display(
                &mut display,
                claim_set.claims.clone(),
                credential.claim_definitions.clone().unwrap_or_default(),
                0,
            );
            claims.insert(claim_set.id.clone().unwrap_or_default(), display);
        }
        let (name, description, background_color) = match credential.display {
            Some(display_list) => {
                // TODO: Locale support.
                let display = display_list
                    .clone()
                    .into_iter()
                    .find(|display| display.locale.is_none())
                    .unwrap_or_else(|| display_list[0].clone());
                (
                    display.name,
                    display.description.unwrap_or_default(),
                    display.background_color.unwrap_or_default(),
                )
            }
            None => (String::new(), String::new(), String::new()),
        };
        Self {
            id: credential.id,
            issuer: credential.issuer,
            issued: credential.issued,
            type_: credential.type_,
            format: credential.format,
            claims: HashMap::new(), // TODO: Implement claims_display
            issuance_date: credential.issuance_date.to_rfc3339(),
            valid_from: credential.valid_from.map_or_else(String::new, |date| date.to_rfc3339()),
            valid_until: credential.valid_until.map_or_else(String::new, |date| date.to_rfc3339()),
            name,
            description,
            background_color,
            logo: credential.logo.unwrap_or_default(),
            background: credential.background.unwrap_or_default(),
        }
    }
}

/// View for the verifiable credential sub-app
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialView {
    /// Currently active credential ID
    pub id: Option<String>,

    /// List of stored credentials
    pub credentials: Vec<Credential>,
}

impl From<CredentialState> for CredentialView {
    fn from(state: CredentialState) -> Self {
        Self {
            id: state.id,
            credentials: state.credentials.into_iter().map(Credential::from).collect(),
        }
    }
}

// Convert the `SubjectClaims` model to a string representation, including
// using the claim definitions to find the claim labels.
// TODO: Use locales.
#[allow(dead_code)]
fn claims_display(
    display: &mut String, claims: Map<String, Value>, definitions: HashMap<String, Claim>,
    indent_level: usize,
) {
    let indent = "  ".repeat(indent_level);
    for (key, value) in claims {
        // Find a label if possible, otherwise use the key.
        if let Some(definition) = definitions.get(&key) {
            match definition {
                Claim::Entry(def) => {
                    if let Some(def_display) = def.display.clone() {
                        let locale_display = def_display
                            .clone()
                            .into_iter()
                            .find(|d| d.locale.is_none())
                            .unwrap_or_else(|| def_display[0].clone());
                        display.push_str(&format!("{}{}: ", indent, locale_display.name));
                    } else {
                        let label = capitalize(&key);
                        display.push_str(&format!("{}{}: ", indent, label));
                    };
                }
                Claim::Set(_) => {
                    let label = capitalize(&key);
                    display.push_str(&format!("{}{}: ", indent, label));
                }
            }
        } else {
            let label = capitalize(&key);
            display.push_str(&format!("{}{}: ", indent, label));
        }
        // And now the value. Might need to recurse if it's a nested claim.
        match value {
            Value::Object(map) => {
                display.push('\n');
                claims_display(display, map, definitions.clone(), indent_level + 1);
            }
            _ => {
                display.push_str(&format!("{}\n", value));
            }
        }
    }
    *display = display.replace("\"", "");
}

// Capitalize the first letter of a string.
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn test_claims_display() {
        let json = include_bytes!("../model/credentials.json");
        let credentials: Vec<CredentialModel> =
            serde_json::from_slice(json).expect("should deserialize");
        assert_eq!(credentials.len(), 2);

        let claim_sets = credentials[0].subject_claims.clone();
        assert_eq!(claim_sets.len(), 1);
        let claims = claim_sets[0].claims.clone();

        let Some(defs) = credentials[0].claim_definitions.clone() else {
            panic!("No claim definitions found");
        };

        let display = &mut String::new();
        claims_display(display, claims, defs, 0);
        assert_yaml_snapshot!(display);
    }
}
