use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;


#[derive(Debug)]
pub enum AppError {
    Validation(String),
    Internal(String),
    ImvalidEmail(String)


}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let(status, message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg)=> (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ImvalidEmail(msg)=> (StatusCode::INTERNAL_SERVER_ERROR, msg)

        };
        let body = Json(json!({"error": message}));
        (status, body).into_response()
    }
}