//! # Example Issuer Service
//!
//! Simple, hard-coded issuer service useful for demonstrating the Vercre
//! wallet. Assumes pre-authorized, issuer-initiated flow only.

mod handler;
mod provider;

use std::env;

use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use provider::Provider;
use serde::Serialize;
use serde_json::json;
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

/// Wrapper for `axum::Response`
pub struct AxResult<T>(vercre_issuer::Result<T>);

impl<T> IntoResponse for AxResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self.0 {
            Ok(v) => (StatusCode::OK, Json(json!(v))),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_json())),
        }
        .into_response()
    }
}

impl<T> From<vercre_issuer::Result<T>> for AxResult<T> {
    fn from(val: vercre_issuer::Result<T>) -> Self {
        Self(val)
    }
}
