use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, json!({ "error": m })),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, json!({ "error": "unauthorized" })),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, json!({ "error": m })),
            AppError::Conflict(m) => (StatusCode::CONFLICT, json!({ "error": m })),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": format!("{}", e) })),
        };
        (status, Json(body)).into_response()
    }
}
