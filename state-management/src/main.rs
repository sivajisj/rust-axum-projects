use axum::{Router, routing::{get, post, put, delete}};
use tracing_subscriber;
use dotenvy::dotenv;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
mod db;
mod state;
mod handlers;
mod models;
mod errors;
mod responses;

use db::init_db_pool;
use state::AppState;
use handlers::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL")?;
    let pool = init_db_pool(&db_url).await?;
    let shared_state = AppState { db: pool };

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(shared_state);
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

   
    Ok(())
}
