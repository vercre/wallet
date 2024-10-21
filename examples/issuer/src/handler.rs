//! # Request handlers

use axum::{extract::State, Json};
use vercre_issuer::{CreateOfferRequest, CreateOfferResponse};

use super::AxResult;
use crate::provider::Provider;

// Create a credential offer
#[axum::debug_handler]
pub async fn create_offer(
    State(provider): State<Provider>, Json(req): Json<CreateOfferRequest>,
) -> AxResult<CreateOfferResponse> {
    vercre_issuer::create_offer(provider, req).await.into()
}