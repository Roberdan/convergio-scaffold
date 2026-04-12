//! HTTP API routes for convergio-scaffold.

use axum::Router;

/// Returns the router for this crate's API endpoints.
pub fn routes() -> Router {
    Router::new()
    // .route("/api/scaffold/health", get(health))
}
