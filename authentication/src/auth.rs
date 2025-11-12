use anyhow::{Result, anyhow};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub role: String,
    pub exp: i64,
}

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if parsed_hash.is_err() {
        return false;
    }

    let parsed = parsed_hash.unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

pub fn hash_refresh_token(token: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let token_hash = argon2
        .hash_password(token.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash refresh token: {}", e))?;

    Ok(token_hash.to_string())
}

pub fn verify_refresh_token_hash(hash: &str, token: &str) -> bool {
    let parsed = PasswordHash::new(hash);
    if parsed.is_err() {
        return false;
    }

    Argon2::default()
        .verify_password(token.as_bytes(), &parsed.unwrap())
        .is_ok()
}

pub fn generate_jwt(secret: &str, user_id: &str, role: &str, minutes: i64) -> Result<String> {
    let exp = (OffsetDateTime::now_utc() + Duration::minutes(minutes)).unix_timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!("Failed to generate JWT: {}", e))?;

    Ok(token)
}

pub fn verify_jwt(secret: &str, token: &str) -> Result<TokenData<Claims>> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| anyhow!("Invalid JWT: {}", e))?;

    Ok(data)
}
