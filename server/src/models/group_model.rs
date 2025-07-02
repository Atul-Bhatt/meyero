use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::user_model::User;

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_by: Uuid,
    pub updated_at: DateTime<Utc>
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct GroupUser {
    pub group_id: Uuid,
    pub username: Uuid,
    pub added_by: Uuid,
    pub added_at: DateTime<Utc>,
    pub removed_by: Uuid,
    pub removed_at: DateTime<Utc>
}

#[derive(Deserialize, Serialize)]
pub struct GroupRequest {
    pub group: Group,
    pub users: Vec<User>
}