//! # Example Issuer Service
//!
//! Simple, hard-coded issuer service useful for demonstrating the Vercre
//! wallet. Assumes pre-authorized, issuer-initiated flow only.

mod handler;
mod provider;

use std::env;

use axum::extract::{rejection::JsonRejection, FromRequest};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use provider::Provider;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use url::Url;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber).expect("set default subscriber");

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);
    let router = Router::new()
        .route("/create_offer", post(handler::create_offer))
        .route("/.well-known/openid-credential-issuer", get(handler::metadata))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store"),
        ))
        .with_state(Provider::new());

    let http_addr = env::var("VERCRE_HTTP_ADDR").unwrap_or_else(|_| "http://0.0.0.0:8080".into());
    let parsed = Url::parse(&http_addr).expect("VERCRE_HTTP_ADDR should be a valid URL");
    let addr = format!("{}:{}", parsed.host_str().unwrap(), parsed.port().unwrap_or(8080));
    let listener = TcpListener::bind(addr).await.expect("should bind to address");
    tracing::info!("listening on {}", listener.local_addr().expect("listener should have address"));
    axum::serve(listener, router).await.expect("server should run");
}

// Custom JSON extractor to enable overriding the rejection and create our own
/// error response.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    T: Serialize,
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

/// Custom application errors.
pub enum AppError {
    /// The request body contained invalid JSON.
    InvalidJson(JsonRejection),

    /// Unspecified application error.
    Other(anyhow::Error),
}

/// Error response.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::InvalidJson(rejection) => (rejection.status(), rejection.body_text()),
            Self::Other(error) => {
                // Log the error but don't expose it to the client.
                tracing::error!("Internal server error: {}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into())
            }
        };
        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::InvalidJson(rejection)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(error)
    }
}

impl From<vercre_issuer::Error> for AppError {
    fn from(error: vercre_issuer::Error) -> Self {
        Self::Other(error.into())
    }
}