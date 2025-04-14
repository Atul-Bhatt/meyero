use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct MessageChannel {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WSMessage {
    pub message: String
}