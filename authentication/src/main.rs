use anyhow::Ok;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber;

mod auth;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod state;
mod utils;

use db::init_db_pool;
use handlers::*;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let jwt_exp_minutes = env::var("JWT_EXP_MINUTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(15);
    let refresh_days = env::var("REFRESH_TOKEN_EXP_DAYS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let pool = init_db_pool(&database_url).await?;

    let app_state = AppState {
        db: pool,
        jwt_secret,
        jwt_exp_minutes,
        refresh_token_exp_days: refresh_days,
    };

    let app = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .route("/users/me", get(me))
        .route("/admin/users", get(admin_list_users))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
