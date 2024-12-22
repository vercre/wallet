//! # Request handlers for issuer endpoints.

use std::vec;

use anyhow::anyhow;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Result;
use axum::Json;
use axum_extra::headers::Host;
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use vercre_issuer::{MetadataResponse, OfferType, SendType};

use crate::AppState;

use super::{AppError, AppJson};

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

// Create a credential offer
#[axum::debug_handler]
pub async fn create_offer(
    State(state): State<AppState>, TypedHeader(host): TypedHeader<Host>,
    Json(req): Json<CreateOfferRequest>,
) -> Result<AppJson<CreateOfferResponse>, AppError> {
    let gt = format!("\"{}\"", req.grant_type);
    let Ok(grant_type) = serde_json::from_str(&gt) else {
        return Err(anyhow!("invalid grant type: {}", req.grant_type).into());
    };

    let request = vercre_issuer::CreateOfferRequest {
        credential_issuer: format!("http://{host}"),
        subject_id: Some(req.subject_id),
        credential_configuration_ids: vec![req.credential_configuration_id.clone()],
        grant_types: Some(vec![grant_type]),
        tx_code_required: req.tx_code_required,
        send_type: SendType::ByVal,
    };

    let response: vercre_issuer::CreateOfferResponse =
        vercre_issuer::create_offer(state.issuer_provider, request).await?;
    let offer = match response.offer_type {
        OfferType::Object(offer) => offer,
        OfferType::Uri(s) => return Err(anyhow!("unexpected URI offer {s}").into()),
    };
    if offer.credential_configuration_ids.len() != 1 {
        return Err(anyhow!("expected 1 credential configuration ID").into());
    }
    if offer.credential_configuration_ids[0] != req.credential_configuration_id {
        return Err(anyhow!("unexpected credential configuration ID").into());
    }
    let qr_code = offer.to_qrcode("openid-credential-offer://credential_offer=")?;

    let rsp = CreateOfferResponse {
        qr_code,
        tx_code: response.tx_code,
    };

    Ok(AppJson(rsp))
}

// Metadata endpoint
#[axum::debug_handler]
pub async fn metadata(
    headers: HeaderMap, State(state): State<AppState>, TypedHeader(host): TypedHeader<Host>,
) -> Result<AppJson<MetadataResponse>, AppError> {
    let request = vercre_issuer::MetadataRequest {
        credential_issuer: format!("http://{host}"),
        languages: headers
            .get("accept-language")
            .and_then(|v| v.to_str().ok())
            .map(ToString::to_string),
    };
    let response = vercre_issuer::metadata(state.issuer_provider.clone(), request).await?;
    Ok(AppJson(response))
}
