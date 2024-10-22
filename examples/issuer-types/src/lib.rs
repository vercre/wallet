//! # Issuer Types
//!
//! Simple types that can be shared between the issuer service and the issuer
//! web UI. Uses `typeshare` to generate TypeScript types from Rust types
//! defined here.
//!
//! Use the `cargo make typegen-issuer` command to generate the types.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Create offer request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct CreateOfferRequest {
    /// Credential issuer identifier (URL).
    pub credential_issuer: String,

    /// Issuer's identifier of the intended holder of the credential.
    pub subject_id: String,

    /// The identifier of the type of credential to be issued.
    pub credential_configuration_id: String,

    /// Type of authorization grant to include in the offer.
    pub grant_type: String,

    /// Whether or not a PIN is required to validate requester of the credential
    /// offer is the person accepting the credential.
    pub tx_code_required: bool,
}

/// Create offer response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct CreateOfferResponse {
    /// QR code for the credential offer
    pub qr_code: String,

    /// PIN code required to accept the credential offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_code: Option<String>,
}

/// Error response from the API
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[typeshare]
pub struct ErrorResult {
    /// The error type
    error: String,

    /// The error message
    error_description: String,
}
