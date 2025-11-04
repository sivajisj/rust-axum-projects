use std::sync::Mutex;
use crate::models::Item;

#[derive(Default)]
pub struct AppState {
    pub items: Mutex<Vec<Item>>,
}