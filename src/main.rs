mod router;
mod handlers;
mod service;
mod error;
mod store;
mod utils;

use std::env;

use axum::{ http::HeaderValue, serve };
use store::mongo::MongoStore;
use tower_http::cors::{ Any, CorsLayer };
use tracing_subscriber::{ fmt::layer, layer::SubscriberExt, util::SubscriberInitExt };
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5173"))
        .allow_methods(Any)
        .allow_headers(Any);

    tracing_subscriber::registry().with(layer()).init();

    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let db = env::var("MONGODB_DB").unwrap_or_else(|_| "url_shortener".into());

    // let store = MemoryStore::default();
    let store = MongoStore::new(&uri, &db).await?;
    let app = router::build_router(store, cors);
    let addr = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("url shortener connected {:?}", addr);
    serve(addr, app.into_make_service()).await?;

    Ok(())
}
