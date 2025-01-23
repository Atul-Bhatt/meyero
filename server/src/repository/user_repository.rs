use crate::models::user_model::{User, UpdateUser};
use std::io::Error;
use crate::AppState;
use actix_web::web;
use uuid::Uuid;

pub async fn get_all_users(data: &web::Data<AppState>) -> Result<Vec<User>, Error> {
    let query_result = sqlx::query_as!(
        User,
        "Select * From users"
    )
    .fetch_all(data.db)
    .await;

    query_result.unwrap()
}

pub async fn get_user_by_id(data: &web::Data<AppState>, user_id: &Uuid) -> Result<User, Error> {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(data.db)
        .await;

    return query_result.unwrap
}

pub async fn get_user_by_username(data: &web::Data<AppState>, username: &String) -> Result<User, Error> {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(&data.db)
        .await;
    
    query_result.unwrap()
}

pub async fn update_user(data: &web::Data<AppState>, body: web::Json<UpdateUser>) -> Result<User, Error> {
    let query_result = sqlx::query_as!(
        User,
        "UPDATE users SET username = $1, email = $2, name = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.username.to_owned().unwrap_or(user.username),
        body.email.to_owned().unwrap_or(user.email),
        body.name.to_owned().unwrap_or(user.name),
        now,
        user_id 
    )
    .fetch_one(&data.db)
    .await;
    
    query_result.unwrap()
}

pub async fn delete_user(data: &web::Data<AppState>, user_id: &Uuid) -> i32 {
    let rows_affected = sqlx::query!("DELETE FROM users  WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();
    rows_affected
}

pub async fn add_user(data: &web::Data<AppState>, user: &User, hashed_pass: String) -> Result<User, Error> {
    let query_result = sqlx::query_as!(
        User,
        "Insert Into users (username, email, name, password) VALUES ($1, $2, $3, $4) Returning *",
        user.username.to_string(),
        user.email.to_string(),
        user.name.to_string(),
        hashed_pass, 
    )
    .fetch_one(&data.db)
    .await;

    query_result.unwrap()
}