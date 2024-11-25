use crate::models::user_model::User;

use actix_web::{get, post, web, Result};

#[get("/users")]
async fn get_all_users() -> Result<String> {
    Ok(format!("Not implemented yet!"))
}

#[post("/user")]
async fn insert_user(user: web::Json<User>) -> Result<String> {
    Ok(format!("Welcome {}", user.username))
}