use crate::models::user_model::{User, UpdateUser, UserLogin};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use rand_core::OsRng;
use crate::AppState;
use serde_json;
use chrono::Utc;

use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};

#[get("/list")]
async fn get_all_users(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "Select * From users"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Error occured while fetching all users";
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": message}));
    }

    let users = query_result.unwrap();
    let json_response = serde_json::json!({
        "status": "success",
        "results": users.len(),
        "uses": users
    });
    HttpResponse::Ok().json(json_response)
}


#[patch("/{id}")]
async fn edit_user(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let now = Utc::now();
    let user = query_result.unwrap();

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
    .await
    ;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user":user 
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/{id}")]
async fn delete_user(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM users  WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

#[post("/login")]
async fn login(
   user: web::Json<UserLogin>,
   data: web::Data<AppState>, 
) -> impl Responder {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", user.username)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("User with username: {} not found", user.username);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let db_user = query_result.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(user.password.as_bytes(), &salt).unwrap();

    let compare_pass = Argon2::default().verify_password(db_user.password.as_bytes(), &hash);

    match compare_pass {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user":user 
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
    
}

#[post("/signup")]
async fn signup(
    user: web::Json<User>,
    data: web::Data<AppState>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_pass = Argon2::default().hash_password(user.password.clone().as_bytes(), &salt);

    let query_result = sqlx::query_as!(
        User,
        "Insert Into users (username, email, name, password) VALUES ($1, $2, $3, $4) Returning *",
        user.username.to_string(),
        user.email.to_string(),
        user.name.to_string(),
        hashed_pass.unwrap().to_string(), 
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(result) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": result
            })});
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
                {
                    return HttpResponse::BadRequest()
                        .json(serde_json::json!({"status": "fail", "message": "User with that username alread exists"}));
                }

                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .service(get_all_users)
        .service(edit_user)
        .service(delete_user)
        .service(login)
        .service(signup);

    conf.service(scope);
}
