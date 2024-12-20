use serde::Serialize;

use crate::{AppError, AppJson};

pub mod issuer;
pub mod verifier;

#[derive(Serialize)]
pub struct GreetingResponse {
    pub message: String,
}

// Hello World handler
#[axum::debug_handler]
pub async fn index() -> Result<AppJson<GreetingResponse>, AppError> {
    Ok(AppJson(GreetingResponse {
        message: "Vercre Demonstration Service".into(),
    }))
}
