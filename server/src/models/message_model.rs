use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub from_user: String,
    pub to_user: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> 
}
