use crate::error::Result;
use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use super::api;

const INDEX_HTML: &str = include_str!("../../static/index.html");

pub async fn start_server(port: u16) -> Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        // Static files
        .route("/", get(index_handler))
        // API routes
        .route("/api/status", get(api::status))
        .route("/api/keys", get(api::list_keys))
        .route("/api/keys/generate", post(api::generate_key))
        .route("/api/keys/rotate", post(api::rotate_key))
        .route("/api/keys/archived", get(api::list_archived_keys))
        .route("/api/encrypt/message", post(api::encrypt_message))
        .route("/api/decrypt/message", post(api::decrypt_message))
        .route("/api/encrypt/file", post(api::encrypt_file))
        .route("/api/decrypt/file", post(api::decrypt_file))
        .route("/api/sign", post(api::sign_data))
        .route("/api/verify", post(api::verify_signature))
        .route("/api/stego/capacity", post(api::stego_capacity))
        .route("/api/config", get(api::get_config))
        .layer(cors)
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)); // 100MB max

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Starting Hermes Web UI on http://{}", addr);
    println!("Press Ctrl+C to stop the server");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| crate::error::HermesError::ConfigError(e.to_string()))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| crate::error::HermesError::ConfigError(e.to_string()))?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}
