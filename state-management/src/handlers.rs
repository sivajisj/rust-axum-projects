use axum::{
    extract::{Path, State},
    response::Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{CreateUser, UpdateUser, User},
    state::AppState,
};

pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(Json(users))
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(AppError::from)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound("User not found".into())),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user: User =
        sqlx::query_as::<_, User>("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
            .bind(&payload.name)
            .bind(&payload.email)
            .fetch_one(&state.db)
            .await
            .map_err(AppError::from)?;

    Ok(Json(user))
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 RETURNING *",
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::from)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound("User not found".into())),
    }
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let rows = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(AppError::from)?
        .rows_affected();

    if rows == 0 {
        Err(AppError::NotFound("User not found".into()))
    } else {
        Ok(Json(json!({ "status": "deleted" })))
    }
}
