use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("malformed url")] InvalidUrl,
    #[error("slug collision")] SlugExists,
    #[error("slug not found")] NotFound,
    #[error("internal error : {0}")] Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = match self {
            AppError::InvalidUrl => StatusCode::BAD_GATEWAY,
            AppError::SlugExists => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let msg = self.to_string();
        (code, Json(ErrBody { error: msg })).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
