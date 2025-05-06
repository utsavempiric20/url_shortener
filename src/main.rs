mod router;
mod handlers;
mod service;
mod error;
mod store;
mod utils;

use axum::{ http::HeaderValue, serve };
use store::memory::MemoryStore;
use tower_http::cors::{ Any, CorsLayer };
use tracing_subscriber::{ fmt::layer, layer::SubscriberExt, util::SubscriberInitExt };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(Any)
        .allow_headers(Any);

    tracing_subscriber::registry().with(layer()).init();
    let store = MemoryStore::default();
    let app = router::build_router(store, cors);
    let addr = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("url shortener connected {:?}", addr);
    serve(addr, app.into_make_service()).await?;

    Ok(())
}
