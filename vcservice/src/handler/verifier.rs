//! # Request handlers for verifier endpoints.

use std::collections::HashMap;
use std::vec;

use anyhow::anyhow;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use vercre_verifier::{
    Constraints, CreateRequestRequest, DeviceFlow, Field, Filter, FilterValue, InputDescriptor,
    RequestObjectRequest, RequestObjectResponse, ResponseRequest, ResponseResponse,
};

use super::{AppError, AppJson};
use crate::AppState;

/// Create authorization request. This is almost a copy of the
/// `CreateRequestRequest` struct from the `vercre_verifier` crate but repeated
/// here to allow `typeshare` to generate the TypeScript equivalent for the
/// sample Verifier application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateRequest {
    /// Purpose of the request.
    pub purpose: String,

    /// Input Descriptors describe the information required from the holder.
    pub input_descriptors: Vec<GenerateInputDescriptor>,
}

/// Input descriptor for the request. Type-generation friendly copy of the
/// `InputDescriptor` struct from the `vercre-diff-exch` crate, omitting any
/// fields that are not applicable to this sample application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateInputDescriptor {
    /// ID of the input descriptor.
    pub id: String,

    /// Contraints specify constraints on data values, and an explanation why a
    /// certain item or set of data is being requested.
    pub constraints: GenerateConstraints,
}

/// Type-generation friendly copy of the `Constraints` struct from the
/// `vercre-diff-exch` crate, omitting any fields that are not applicable to
/// this sample application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateConstraints {
    pub fields: Vec<GenerateField>,
}

/// Type-generation friendly copy of the `Field` struct from the
/// `vercre-diff-exch` crate, omitting any fields that are not applicable to
/// this sample application.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateField {
    /// `JSONPath` expression that selects the target value from the input.
    pub path: Vec<String>,

    /// JSON Schema descriptor used to filter against the values returned from
    /// evaluation of the `JSONPath` expressions in the path array.
    pub filter_value: String,
}

/// Create authorization request response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[typeshare]
pub struct GenerateRequestResponse {
    /// URI to the authorization request.
    pub request_uri: String,

    /// QR code for the request URI.
    pub qr_code: String,
}

// Generate Authorization Request endpoint
#[axum::debug_handler]
pub async fn create_request(
    State(state): State<AppState>, Json(req): Json<GenerateRequest>,
) -> Result<AppJson<GenerateRequestResponse>, AppError> {
    let mut input_descriptors = vec![];
    for in_desc in req.input_descriptors {
        let mut fields = Vec::<Field>::new();
        for f in in_desc.constraints.fields {
            fields.push(Field {
                path: f.path,
                filter: Some(Filter {
                    type_: "string".into(),
                    value: FilterValue::Const(f.filter_value),
                }),
                ..Default::default()
            });
        }
        input_descriptors.push(InputDescriptor {
            id: in_desc.id,
            constraints: Constraints {
                fields: Some(fields),
                ..Default::default()
            },
            name: None,
            purpose: None,
            format: None,
        });
    }

    let request = CreateRequestRequest {
        client_id: state.verifier.to_string(),
        device_flow: DeviceFlow::CrossDevice, // we will get a URI, not a full request object.
        purpose: req.purpose,
        input_descriptors,
        ..Default::default()
    };
    let response =
        vercre_verifier::create_request(state.verifier_provider.clone(), &request).await?;

    if response.request_uri.is_none() {
        return Err(AppError::Status(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow!("no request URI returned").to_string(),
        ));
    }

    let qr_code = response.to_qrcode(None)?;

    let gen_response = GenerateRequestResponse {
        request_uri: response.request_uri,
        qr_code,
    };

    Ok(AppJson(gen_response))
}

// Return an authorization request object.
#[axum::debug_handler]
pub async fn request_object(
    State(state): State<AppState>, Path(object_id): Path<String>,
) -> Result<AppJson<RequestObjectResponse>, AppError> {
    let request = RequestObjectRequest {
        client_id: state.verifier.to_string(),
        id: object_id,
    };
    let response =
        vercre_verifier::request_object(state.verifier_provider.clone(), &request).await?;
    Ok(AppJson(response))
}

// Wallet authorization response (the actual presentation of the credential to
// the verifier).
#[axum::debug_handler]
pub async fn response(
    State(state): State<AppState>, Form(req): Form<HashMap<String, String>>,
) -> Result<AppJson<ResponseResponse>, AppError> {
    let Ok(response_request) = ResponseRequest::form_decode(&req) else {
        return Err(AppError::Status(
            StatusCode::BAD_REQUEST,
            format!("unable to turn HashMap {req:?} into ResponseRequest"),
        ));
    };
    let response =
        vercre_verifier::response(state.verifier_provider.clone(), &response_request).await?;
    Ok(AppJson(response))
}
