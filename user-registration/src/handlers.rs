use crate::{
    errors::AppError,
    models::{RegisterUser, User},
    responses::ApiResponse,
    state::AppState,
};
use axum::{
    extract::{Extension, Form, Json},
    response::{Html, Json as AxumJson},
};
use std::{iter, sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
}};
use validator::Validate;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub async fn root() -> Html<&'static str> {
    Html("<h1>Welcome tot he Form+Json Validation Service</h2>")
}

pub async fn register_json(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterUser>,
) -> Result<AxumJson<ApiResponse<User>>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = User {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };

    let mut users = state.users.lock().unwrap();
    users.push(user.clone());

    Ok(AxumJson(ApiResponse {
        state: "success".into(),
        data: Some(user),
    }))
}


pub  async fn register_json_unique(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterUser>) ->Result<AxumJson<ApiResponse<User>>, AppError> {
        payload
        .validate()
        .map_err(|e| AppError::Validation((e.to_string())))?;



    let mut users = state.users.lock()
    .map_err(|_| AppError::Internal("Failed to acquire lock".to_string()))?;

    if users.iter().any(|u| u.email.eq_ignore_ascii_case(&payload.email)) {
        return Err(AppError::Internal("Email already registered".to_string()));
    }
    let user = User{
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        name: payload.name,
        email: payload.email,
        age: payload.age
    };
    users.push(user.clone());

    Ok(AxumJson(ApiResponse {
        state: "success".into(),
        data: Some(user),
    }))
    }


pub async fn register_form(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterUser>,
) -> Result<AxumJson<ApiResponse<User>>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation((e.to_string())))?;

    let user = User {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };

    let mut users = state.users.lock().unwrap();
    users.push(user.clone());

    Ok(AxumJson(ApiResponse {
        state: "success".into(),
        data: Some(user),
    }))
}

pub async fn list_users(
    Extension(state): Extension<Arc<AppState>>,
) -> AxumJson<ApiResponse<Vec<User>>> {
    let users = state.users.lock().unwrap();
    AxumJson(ApiResponse {
        state: "ok".into(),
        data: Some(users.clone()),
    })
}
