//! # Example Issuer and Verifier Service
//!
//! Simple, hard-coded service useful for demonstrating the Vercre wallet.
//! Assumes pre-authorized, issuer-initiated flow only.

mod handler;
mod provider;

use std::borrow::Cow;
use std::env;

use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use typeshare::typeshare;
use url::Url;

use handler::{issuer, verifier};

/// Application state.
#[derive(Clone)]
pub struct AppState {
    external_address: Cow<'static, str>,
    issuer: Cow<'static, str>,
    verifier: Cow<'static, str>,
    issuer_provider: provider::issuer::Provider,
    verifier_provider: provider::verifier::Provider,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();
    tracing::subscriber::set_global_default(subscriber).expect("set default subscriber");
    let external_address =
        env::var("VERCRE_HTTP_ADDR").unwrap_or_else(|_| "http://0.0.0.0:8080".into());
    let issuer = env::var("VERCRE_ISSUER").unwrap_or_else(|_| "http://vercre.io".into());
    let verifier = env::var("VERCRE_VERIFIER").unwrap_or_else(|_| "http://vercre.io".into());

    let app_state = AppState {
        external_address: external_address.into(),
        issuer: issuer.into(),
        verifier: verifier.into(),
        issuer_provider: provider::issuer::Provider::new(),
        verifier_provider: provider::verifier::Provider::new(),
    };

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);
    let router = Router::new()
        .route("/", get(handler::index))
        .route("/create_offer", post(issuer::create_offer))
        .route("/.well-known/openid-credential-issuer", get(issuer::metadata))
        .route("/token", post(issuer::token))
        .route("/credential", post(issuer::credential))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store"),
        ))
        .with_state(app_state);

    let http_addr = Url::parse("http://0.0.0.0:8080").expect("http_addr should be a valid URL");
    let addr = format!("{}:{}", http_addr.host_str().unwrap(), http_addr.port().unwrap_or(8080));
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

    /// Status code and message error.
    Status(StatusCode, String),

    /// Unspecified application error.
    Other(anyhow::Error),
}

/// Error response.
#[derive(Debug, Default, Deserialize, Serialize)]
#[typeshare]
pub struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::InvalidJson(rejection) => (rejection.status(), rejection.body_text()),
            Self::Status(status, message) => {
                tracing::error!("status error: {status} {message}");
                (status, message)
            }
            Self::Other(error) => {
                tracing::error!("internal server error: {}", error);
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
