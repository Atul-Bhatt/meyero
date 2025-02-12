use crate::models::user_model::{User, UpdateUser, Session};
use std::io::Error;
use crate::AppState;
use actix_web::web;
use uuid::Uuid;
use anyhow::Result;
use chrono::Utc;

pub async fn get_all_users(data: &web::Data<AppState>) -> Result<Vec<User>> {
    let query_result = sqlx::query_as!(
        User,
        "Select * From users"
    )
    .fetch_all(&data.db)
    .await?;

    // if query_result.is_err() {
    //     let message = String::from("Error Fetching Users");
    //     return Error(message)
    // }
    Ok(query_result)
}

pub async fn get_user_by_id(data: &web::Data<AppState>, user_id: &Uuid) -> Result<User> {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await?;

    Ok(query_result)
}

pub async fn get_user_by_username(data: &web::Data<AppState>, username: &String) -> Result<User> {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(&data.db)
        .await?;
    
    Ok(query_result)
}

pub async fn update_user(data: &web::Data<AppState>, body: web::Json<UpdateUser>, user: User) -> Result<User> {
    let query_result = sqlx::query_as!(
        User,
        "UPDATE users SET username = $1, email = $2, name = $3, updated_at = $4 WHERE id = $5 RETURNING *",
        body.username.to_owned().unwrap_or(user.username),
        body.email.to_owned().unwrap_or(user.email),
        body.name.to_owned().unwrap_or(user.name),
        Utc::now(),
        user.id 
    )
    .fetch_one(&data.db)
    .await?;
    
    Ok(query_result)
}

pub async fn delete_user(data: &web::Data<AppState>, user_id: &Uuid) -> u64 {
    let rows_affected = sqlx::query!("DELETE FROM users  WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();
    rows_affected
}

pub async fn add_user(data: &web::Data<AppState>, user: &User, hashed_pass: String) -> Result<User> {
    let query_result = sqlx::query_as!(
        User,
        "Insert Into users (username, email, name, password) VALUES ($1, $2, $3, $4) Returning *",
        user.username.to_string(),
        user.email.to_string(),
        user.name.to_string(),
        hashed_pass, 
    )
    .fetch_one(&data.db)
    .await?;

    Ok(query_result)
}

pub async fn add_token(data: &web::Data<AppState>, session_id: &Uuid, token: &String) -> Result<()> {
    let _ = sqlx::query_as!(
        Token,
        "Insert Into token (session_id, token) VALUES ($1, $2)",
        session_id,
        token
    )
    .execute(&data.db)
    .await?;

    Ok(())
} 

pub async fn create_session(data: &web::Data<AppState>, user_id: &Uuid, user_agent: &String) -> Result<Uuid> {
    let query_result = sqlx::query!(
        "Insert Into session_table (user_id, user_agent) VALUES ($1, $2) returning id",
        user_id,
        user_agent 
    )
    .fetch_one(&data.db)
    .await?;

    Ok(query_result.id)
} 

pub async fn search_username_substring(data: &web::Data<AppState>, username_substring: &String) -> Result<Vec<User>> {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE username ilike $1", username_substring)
        .fetch_all(&data.db)
        .await;
    
    Ok(query_result.unwrap())
}
