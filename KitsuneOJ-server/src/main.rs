use crate::api::v0::routes::routes::api_routes;
use crate::config::db_config::DbConfig;
use crate::connection::database::establish_connection;
use crate::connection::http::create_http_client;
use crate::connection::redis::establish_redis_connection;
use crate::middleware::cors::cors_layer;
use crate::state::AppState;
use crate::utils::logger::init_tracing;
use axum::Router;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::error;

mod api;
mod config;
mod connection;
mod dto;
mod errors;
mod middleware;
mod service;
mod state;
mod utils;

pub async fn run_server() -> anyhow::Result<()> {
    let conn = establish_connection().await;

    let redis = establish_redis_connection().await.map_err(|e| {
        error!("Failed to establish redis connection: {}", e);
        anyhow::anyhow!("Redis connection failed: {}", e)
    })?;
    let http_client = create_http_client().await.map_err(|e| {
        error!("Failed to create HTTP client: {}", e);
        anyhow::anyhow!("HTTP client creation failed: {}", e)
    })?;

    let server_url = format!(
        "{}:{}",
        &DbConfig::get().server_host,
        &DbConfig::get().server_port
    );

    let state = AppState {
        conn,
        redis,
        http_client,
    };

    let app = Router::new()
        .merge(api_routes(state.clone()))
        .layer(CookieManagerLayer::new())
        .layer(cors_layer())
        .with_state(state);

    println!("Starting server at: {}", server_url);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // tracing 초기화
    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
