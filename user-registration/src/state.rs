use crate::models::User;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState{
    pub users: Mutex<Vec<User>>
}