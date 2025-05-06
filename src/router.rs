use axum::{ routing::{ get, post }, Router };
use tower_http::cors::CorsLayer;
use crate::{
    handlers::{ create_short_link, get_clicks, health, redirect_slug },
    store::memory::MemoryStore,
};

pub fn build_router(store: MemoryStore, cors: CorsLayer) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/links", post(create_short_link))
        .route("/{slug}", get(redirect_slug))
        .route("/api/getcount/{slug}", get(get_clicks))
        .with_state(store)
        .layer(cors)
}
