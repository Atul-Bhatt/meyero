use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchUser {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_agent: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: Uuid,
    pub token: String, 
    pub session_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked: DateTime<Utc>
}