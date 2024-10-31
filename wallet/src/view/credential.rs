//! Credential view models

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use vercre_holder::credential::{
    Credential as CredentialModel, ImageData, SubjectClaims as SubjectClaimsModel,
};
use vercre_holder::CredentialDisplay;

use crate::model::CredentialState;

/// View model for a set of claims associated with a subject (holder).
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct SubjectClaims {
    /// The subject's unique identifier. Empty string if not applicable.
    pub id: String,

    /// The subject's claims.
    pub claims: HashMap<String, String>,
}

impl From<SubjectClaimsModel> for SubjectClaims {
    fn from(subject: SubjectClaimsModel) -> Self {
        let mut claims = HashMap::new();
        for (key, value) in subject.claims {
            claims.insert(key, value.to_string());
        }
        Self {
            id: subject.id.unwrap_or_default(),
            claims,
        }
    }
}

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

    /// Claims
    pub subject_claims: Vec<SubjectClaims>,

    /// The date the credential was issued as an RFC3339 string.
    pub issuance_date: String,

    /// The date the credential is valid from as an RFC3339 string. Empty string
    /// if not applicable.
    pub valid_from: String,

    /// The date the credential is valid until (expiry) as an RFC3339 string.
    /// Empty string if not applicable.
    pub valid_until: String,

    /// Display information from the issuer's metadata for this credential.
    /// Empty vector if not applicable.
    pub display: Vec<CredentialDisplay>,

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
        let subject_claims = credential
            .subject_claims
            .into_iter()
            .map(SubjectClaims::from)
            .collect();
        Self {
            id: credential.id,
            issuer: credential.issuer,
            issued: credential.issued,
            type_: credential.type_,
            format: credential.format,
            subject_claims,
            issuance_date: credential.issuance_date.to_rfc3339(),
            valid_from: credential.valid_from.map_or_else(String::new, |date| date.to_rfc3339()),
            valid_until: credential.valid_until.map_or_else(String::new, |date| date.to_rfc3339()),
            display: credential.display.unwrap_or_else(Vec::new),
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
