mod router;
mod handlers;
mod service;
mod error;
mod store;

use axum::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = router::build_router();
    let addr = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {:?}", addr);
    serve(addr, app.into_make_service()).await?;

    Ok(())
}
