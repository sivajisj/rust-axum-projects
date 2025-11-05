use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min=3, message="Name must be atleast three characters length"))]
    pub name: String,

    #[validate(email(message="Invalid email address"))]
    pub email: String,

    #[validate(range(min=18, message="Age must be at least 18"))]
    pub age: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u8
}