//! REST API routes (Axum)

use axum::{
    routing::{get, post},
    Router,
};

mod handlers;

/// Create the REST API router
pub fn create_router() -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // API v1 routes
        .nest("/api/v1", api_v1_routes())
}

fn api_v1_routes() -> Router {
    Router::new()
        .route("/price", post(handlers::price_instrument))
        .route("/price/batch", post(handlers::price_portfolio))
        .route("/calibrate", post(handlers::calibrate))
        .route("/exposure", post(handlers::calculate_exposure))
}
