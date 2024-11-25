//use uuid::Uuid;
//use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    //id: Uuid,
    pub username: String,
    pub email: String,
    pub name: String,
    password: String,
    //created_at: DateTime<Utc>,
    //updated_at: DateTime<Utc>
}