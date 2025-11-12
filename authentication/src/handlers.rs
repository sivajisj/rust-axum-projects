use axum::{
    extract::{State, Path},
    response::Json,
    Json as AxumJson,
};
use crate::{
    state::AppState,
    models::{RegisterPayload, LoginPayload, User, AuthResponse},
    errors::AppError,
    auth::{
        hash_password, verify_password, generate_jwt,
        hash_refresh_token, verify_refresh_token_hash,
    },
    utils::generate_refresh_token,
};
use uuid::Uuid;
use serde_json::json;

pub async fn register(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<RegisterPayload>,
) -> Result<AxumJson<serde_json::Value>, AppError> {
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest("Password too short (min 8)".into()));
    }

    let password_hash = hash_password(&payload.password)
        .map_err(|e| AppError::Internal(e.into()))?;

    

    let rec = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind("user")
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.message().to_lowercase().contains("unique") {
                return AppError::Conflict("Email already exists".into());
            }
        }
        AppError::Internal(e.into())
    })?;

    Ok(AxumJson(json!({
        "id": rec.id,
        "email": rec.email
    })))
}

pub async fn login(
    State(state): State<AppState>,
    AxumJson(payload): AxumJson<LoginPayload>,
) -> Result<AxumJson<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::Unauthorized)?;

    if !verify_password(&user.password_hash, &payload.password) {
        return Err(AppError::Unauthorized);
    }

    let access_token = generate_jwt(
        &state.jwt_secret,
        &user.id.to_string(),
        &user.role,
        state.jwt_exp_minutes,
    )
    .map_err(|e| AppError::Internal(e.into()))?;

    let refresh_raw = generate_refresh_token();
    let refresh_hash = hash_refresh_token(&refresh_raw)
        .map_err(|e| AppError::Internal(e.into()))?;

    sqlx::query("UPDATE users SET refresh_token_hash = $1 WHERE id = $2")
        .bind(&refresh_hash)
        .bind(user.id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(AxumJson(AuthResponse {
        access_token,
        refresh_token: refresh_raw,
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    AxumJson(body): AxumJson<serde_json::Value>,
) -> Result<AxumJson<AuthResponse>, AppError> {
    let refresh = body
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("Missing refresh_token".into()))?;

    let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE refresh_token_hash IS NOT NULL")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    for u in users {
        if let Some(hash) = &u.refresh_token_hash {
            if verify_refresh_token_hash(hash, refresh) {
                let new_refresh = generate_refresh_token();
                let new_hash = hash_refresh_token(&new_refresh)
                    .map_err(|e| AppError::Internal(e.into()))?;

                sqlx::query("UPDATE users SET refresh_token_hash = $1 WHERE id = $2")
                    .bind(&new_hash)
                    .bind(&u.id)
                    .execute(&state.db)
                    .await
                    .map_err(|e| AppError::Internal(e.into()))?;

                let access = generate_jwt(
                    &state.jwt_secret,
                    &u.id.to_string(),
                    &u.role,
                    state.jwt_exp_minutes,
                )
                .map_err(|e| AppError::Internal(e.into()))?;

                return Ok(AxumJson(AuthResponse {
                    access_token: access,
                    refresh_token: new_refresh,
                }));
            }
        }
    }

    Err(AppError::Unauthorized)
}

use crate::middleware::AuthUser;

pub async fn me(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<AxumJson<serde_json::Value>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, role, refresh_token_hash, created_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?
    .ok_or(AppError::NotFound("User not found".into()))?;

    Ok(AxumJson(json!({
        "id": user.id,
        "name": user.name,
        "email": user.email,
        "role": user.role,
        "created_at": user.created_at
    })))
}

pub async fn admin_list_users(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<AxumJson<serde_json::Value>, AppError> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized);
    }

    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password_hash, role, refresh_token_hash, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(AxumJson(json!(users)))
}
