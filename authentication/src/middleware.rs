use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};

use crate::{auth::verify_jwt, state::AppState, errors::AppError};

pub struct AuthUser(pub crate::auth::Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ✅ Clone AppState to drop immutable borrow
        let state = parts
            .extensions
            .get::<AppState>()
            .cloned()
            .ok_or(AppError::Unauthorized)?;

        // ✅ Use &state instead of state.as_ref()
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        // ✅ Verify JWT
        let token_data =
            verify_jwt(&state.jwt_secret, bearer.token()).map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser(token_data.claims))
    }
}
