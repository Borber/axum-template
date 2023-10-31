mod common;
mod config;
mod controller;
mod database;
mod handler;
mod service;

use anyhow::Result;
use axum::{routing::get, Router};
use config::log;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::config::CONFIG;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = log::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    info!("Server listened at {}", CONFIG.server_url);
    axum::Server::bind(&CONFIG.server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
