use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct Item {
    pub id: u64,
    pub(crate)  name: String,
    pub description: Option<String>
}