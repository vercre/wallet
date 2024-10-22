//! # Request handlers

use std::vec;

use anyhow::anyhow;
use axum::extract::State;
use axum::response::Result;
use axum::Json;
use issuer_types::{CreateOfferRequest, CreateOfferResponse};
use vercre_issuer::{OfferType, SendType};

use super::{AppError, AppJson};
use crate::provider::Provider;

// Create a credential offer
#[axum::debug_handler]
pub async fn create_offer(
    State(provider): State<Provider>, Json(req): Json<CreateOfferRequest>,
) -> Result<AppJson<CreateOfferResponse>, AppError> {
    let gt = format!("\"{}\"", req.grant_type);
    let Ok(grant_type) = serde_json::from_str(&gt) else {
        return Err(anyhow!("invalid grant type: {}", req.grant_type).into());
    };

    let request = vercre_issuer::CreateOfferRequest {
        credential_issuer: req.credential_issuer,
        subject_id: Some(req.subject_id),
        credential_configuration_ids: vec![req.credential_configuration_id.clone()],
        grant_types: Some(vec![grant_type]),
        tx_code_required: req.tx_code_required,
        send_type: SendType::ByVal,
    };

    let response: vercre_issuer::CreateOfferResponse =
        vercre_issuer::create_offer(provider, request).await?;
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
    let Some(grants) = offer.grants else {
        return Err(anyhow!("expected grants").into());
    };
    let issuer_state = match grants.authorization_code {
        Some(auth_code_grant) => auth_code_grant.issuer_state,
        None => None,
    };
    let pre_authorized_code = match grants.pre_authorized_code {
        Some(pre_auth_grant) => Some(pre_auth_grant.pre_authorized_code),
        None => None,
    };

    let rsp = CreateOfferResponse {
        credential_issuer: offer.credential_issuer,
        credential_configuration_id: offer.credential_configuration_ids[0].clone(),
        tx_code: response.tx_code,
        issuer_state,
        pre_authorized_code,
    };

    Ok(AppJson(rsp))
}
