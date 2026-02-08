use axum::{routing::{get, post, delete}, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod routes;
mod db;
mod error;
mod models;

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // CORS configuration for frontend development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health::check))
        // Whitelist API
        .route("/api/whitelist", get(routes::whitelist::list))
        .route("/api/whitelist", post(routes::whitelist::create))
        .route("/api/whitelist/:id", get(routes::whitelist::get))
        .route("/api/whitelist/:id", delete(routes::whitelist::delete))
        // Middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
