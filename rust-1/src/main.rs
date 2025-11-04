use std::sync::Arc;
use axum::{
    Router,
    routing::{get, post},
    extract::Extension,
    response::Html,
    http::Request,

    middleware::Next,

};
mod handlers;
mod state;
mod models;
mod errors;
use handlers::*;
use state::AppState;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState::default());

    //build app router
    let app = Router::new()
    .route("/", get(root))
    .route("/hello/:name", get(hello))
    .route("/items", post(create_item).get(list_items))
    .layer(axum::middleware::from_fn(|req: Request<_>, next: Next| async move {
        // simple inline middleware example (pass-through)
        next.run(req).await
    }))
    .layer(tower_http::trace::TraceLayer::new_for_http())
.layer(Extension(state));
// start server
let listener = TcpListener::bind("127.0.0.1:3000").await?;
tracing::info!("listening on {}", listener.local_addr()?);
axum::serve(listener,app).await?;
Ok(())

}