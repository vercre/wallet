//! # Request handlers for issuer endpoints.

use std::collections::HashMap;
use std::vec;

use anyhow::anyhow;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Result;
use axum::{Form, Json};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use vercre_issuer::{
    CredentialRequest, CredentialResponse, MetadataResponse, OfferType, SendType, TokenRequest,
    TokenResponse,
};

use super::{AppError, AppJson};
use crate::AppState;

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

// Create a credential offer
#[axum::debug_handler]
pub async fn create_offer(
    State(state): State<AppState>, Json(req): Json<CreateOfferRequest>,
) -> Result<AppJson<CreateOfferResponse>, AppError> {
    let gt = format!("\"{}\"", req.grant_type);
    let Ok(grant_type) = serde_json::from_str(&gt) else {
        return Err(anyhow!("invalid grant type: {}", req.grant_type).into());
    };

    let request = vercre_issuer::CreateOfferRequest {
        credential_issuer: state.issuer.to_string(),
        subject_id: Some(req.subject_id),
        credential_configuration_ids: vec![req.credential_configuration_id.clone()],
        grant_types: Some(vec![grant_type]),
        tx_code_required: req.tx_code_required,
        send_type: SendType::ByVal,
    };

    let response: vercre_issuer::CreateOfferResponse =
        vercre_issuer::create_offer(state.issuer_provider, request).await?;
    let mut offer = match response.offer_type {
        OfferType::Object(offer) => offer,
        OfferType::Uri(s) => return Err(anyhow!("unexpected URI offer {s}").into()),
    };
    if offer.credential_configuration_ids.len() != 1 {
        return Err(anyhow!("expected 1 credential configuration ID").into());
    }
    if offer.credential_configuration_ids[0] != req.credential_configuration_id {
        return Err(anyhow!("unexpected credential configuration ID").into());
    }

    // Override the issuer's identifier with the environment variable if it
    // exists so our hardcoded data can work with our hosting location.
    offer.credential_issuer = state.external_address.to_string();

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
    headers: HeaderMap, State(state): State<AppState>,
) -> Result<AppJson<MetadataResponse>, AppError> {
    let request = vercre_issuer::MetadataRequest {
        credential_issuer: state.issuer.to_string(),
        languages: headers
            .get("accept-language")
            .and_then(|v| v.to_str().ok())
            .map(ToString::to_string),
    };
    let mut response = vercre_issuer::metadata(state.issuer_provider.clone(), request).await?;

    // Override the issuer's endpoint information with the environment variable
    // if it exists so our hardcoded data can work with our hosting location.
    response.credential_issuer.credential_issuer = state.external_address.to_string();
    response.credential_issuer.credential_endpoint =
        format!("{}/credential", state.external_address);
    response.credential_issuer.deferred_credential_endpoint =
        Some(format!("{}/deferred", state.external_address));

    Ok(AppJson(response))
}

// Token endpoint
#[axum::debug_handler]
pub async fn token(
    State(state): State<AppState>, Form(req): Form<HashMap<String, String>>,
) -> Result<AppJson<TokenResponse>, AppError> {
    let Ok(mut token_request) = TokenRequest::form_decode(&req) else {
        return Err(AppError::Status(
            StatusCode::BAD_REQUEST,
            format!("unable to turn HashMap {req:?} into TokenRequest"),
        ));
    };
    token_request.credential_issuer = state.issuer.to_string();

    let response = vercre_issuer::token(state.issuer_provider.clone(), token_request).await?;
    Ok(AppJson(response))
}

// Credential endpoint
#[axum::debug_handler]
pub async fn credential(
    State(state): State<AppState>, TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(mut req): Json<CredentialRequest>,
) -> Result<AppJson<CredentialResponse>, AppError> {
    req.credential_issuer = state.issuer.to_string();
    req.access_token = auth.token().to_string();

    let response = vercre_issuer::credential(state.issuer_provider.clone(), req).await?;
    Ok(AppJson(response))
}
