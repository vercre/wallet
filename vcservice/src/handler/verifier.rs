//! # Request handlers for verifier endpoints.

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{AppError, AppJson};
use crate::AppState;

/// Create authorization request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateRequest {}

/// Create authorization request response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateRequestResponse {}

// Generate Authorization Request endpoint
#[axum::debug_handler]
async fn create_request(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<AppJson<GenerateRequestResponse>, AppError> {

    let request = vercre_verifier::CreateRequestRequest {
        client_id: state.verifier.to_string(),
    };

    request.client_id = format!("http://{host}");
    vercre_verifier::create_request(state.verifier_provider.clone(), &request).await?
}
