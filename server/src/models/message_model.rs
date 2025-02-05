use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct MessageChannel {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> 
}

