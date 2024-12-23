//! # Request handlers for verifier endpoints.

use std::vec;

use anyhow::anyhow;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use vercre_verifier::{Constraints, CreateRequestRequest, DeviceFlow, Field, Filter, FilterValue, InputDescriptor};

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
}

// Generate Authorization Request endpoint
#[axum::debug_handler]
pub async fn create_request(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
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
        input_descriptors.push(InputDescriptor{
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
    let response = vercre_verifier::create_request(state.verifier_provider.clone(), &request).await?;

    let Some(request_uri) = response.request_uri else {
        return Err(anyhow!("No request URI returned").into());
    };

    let gen_response = GenerateRequestResponse {
        request_uri,
    };

    Ok(AppJson(gen_response))
}
