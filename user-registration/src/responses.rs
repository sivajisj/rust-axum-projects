use serde::Serialize;



#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub state: String,
    pub data: Option<T>
}