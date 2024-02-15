use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}