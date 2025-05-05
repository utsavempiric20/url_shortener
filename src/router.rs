use axum::{ routing::{ get, post }, Router };
use crate::handlers;

pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/links", post(handlers::create_link))
        .route("/api/{slug}", get(handlers::redirect_slug))
}
