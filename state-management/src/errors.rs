use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    Database(String),
    Internal(String),
}

impl IntoResponse for AppError{
    fn into_response(self)-> Response{
        let (status, msg) = match self{
            Self::NotFound(m) => (StatusCode::NOT_FOUND,m),
            Self::Conflict(m) => (StatusCode::CONFLICT, m),
            Self::Database(m) => (StatusCode::INTERNAL_SERVER_ERROR,m),
            Self::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),

        };
        (status, Json(json!({ "error": msg }))).into_response()

    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}