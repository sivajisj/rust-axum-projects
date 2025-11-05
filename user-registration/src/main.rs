use anyhow::Ok;
use axum::{
    Router,
    extract::Extension,
    routing::{get, post},
};

use std::sync::Arc;

mod errors;
mod handlers;
mod models;
mod responses;
mod state;
use tokio::net::TcpListener;

use handlers::*;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", get(root))
        .route("/register/json", post(register_json))
        .route("/register/json/strict", post(register_json_unique))
        .route("/register/form", post(register_form))
        .route("/users", get(list_users))
        .layer(Extension(state));

    // start server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
