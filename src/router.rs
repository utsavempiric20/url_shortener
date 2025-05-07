use axum::{ routing::{ get, post }, Router };
use tower_http::cors::CorsLayer;
use crate::{
    handlers::{ create_short_link, delete_link, get_clicks, health, redirect_slug },
    store::mongo::MongoStore,
};

pub fn build_router(store: MongoStore, cors: CorsLayer) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/links", post(create_short_link))
        .route("/{slug}", get(redirect_slug))
        .route("/api/getcount/{slug}", get(get_clicks))
        .route("/api/delete/{slug}", get(delete_link))
        .with_state(store)
        .layer(cors)
}
