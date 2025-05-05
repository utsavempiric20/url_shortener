use serde::Serialize;
use axum::{ Json, response::IntoResponse, http::StatusCode };

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

pub async fn health() -> impl IntoResponse {
    Json(Health { status: "ok" })
}

pub async fn create_link() -> impl IntoResponse {
    Json(Health { status: StatusCode::NOT_IMPLEMENTED.as_str() })
}

pub async fn redirect_slug() -> impl IntoResponse {
    Json(Health { status: StatusCode::NOT_IMPLEMENTED.as_str() })
}
