use crate::models::Item;
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, Path},
    response::{Html, Json as AxumJson},
};
use std::sync::Arc;

use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub async fn root() -> Html<&'static str> {
    Html("<h1>Welcome to Hello Axum</h1>")
}

pub async fn hello(Path(name): Path<String>) -> AxumJson<serde_json::Value> {
    AxumJson(json!({"message": format!("Hello, {}", name)}))
}

#[derive(serde::Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_item(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateItem>,
) -> AxumJson<serde_json::Value> {

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let item = Item{
        id,
        name: payload.name,
        description: payload.description,
    };
    let mut items = state.items.lock().unwrap();
    items.push(item);
    AxumJson(json!({"status": "created", "id": id}))
}

pub async fn list_items(Extension(state): Extension<Arc<AppState>>) -> AxumJson<serde_json::Value>{
    let items = state.items.lock().unwrap();
    AxumJson(serde_json::to_value(items.clone()).unwrap())
}