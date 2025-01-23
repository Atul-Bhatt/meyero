use crate::models::user_model::User;
use std::io::Error;
use crate::AppState;
use actix_web::web;
use uuid::Uuid;

pub async fn get_all_users(data: web::Data<AppState>) -> Result<Vec<User>, Error> {
    let query_result = sqlx::query_as!(
        User,
        "Select * From users"
    )
    .fetch_all(&data.db)
    .await;

    query_result.unwrap()
}