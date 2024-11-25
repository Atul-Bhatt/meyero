use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: Uuid,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}