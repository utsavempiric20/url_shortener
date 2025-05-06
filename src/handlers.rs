use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ IntoResponse, Redirect },
    Json,
};
use crate::{
    error::Result,
    service::{ create_link, lookup },
    store::{ memory::MemoryStore, LinkStore },
};

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

pub async fn health() -> impl IntoResponse {
    Json(Health { status: "ok" })
}

#[derive(Deserialize)]
pub struct CreateReq {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateResp {
    pub slug: String,
    pub short_url: String,
    pub long_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: u64,
}
pub async fn create_short_link(
    State(store): State<MemoryStore>,
    Json(body): Json<CreateReq>
) -> Result<impl IntoResponse> {
    let link = create_link(&store, &body.url)?;
    let short_url = format!("http://localhost:8080/{}", link.slug);
    Ok((
        StatusCode::CREATED,
        Json(CreateResp {
            slug: link.slug,
            short_url,
            long_url: link.long_url,
            clicks: link.clicks,
            created_at: link.created_at,
        }),
    ))
}

pub async fn get_clicks(
    State(store): State<MemoryStore>,
    Path(slug): Path<String>
) -> Result<impl IntoResponse> {
    let link = lookup(&store, &slug)?;
    Ok((
        StatusCode::OK,
        Json(CreateResp {
            slug: link.slug,
            short_url: format!("http://localhost:8080/{}", slug),
            long_url: link.long_url,
            clicks: link.clicks,
            created_at: link.created_at,
        }),
    ))
}

pub async fn redirect_slug(
    State(store): State<MemoryStore>,
    Path(slug): Path<String>
) -> Result<impl IntoResponse> {
    let link = lookup(&store, &slug)?;
    store.increment_click(&slug);
    Ok(Redirect::temporary(&link.long_url))
}
