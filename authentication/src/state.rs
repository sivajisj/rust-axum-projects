use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState{
    pub db: DbPool,
    pub jwt_secret: String,
    pub jwt_exp_minutes: i64,
    pub refresh_token_exp_days: i64,
}