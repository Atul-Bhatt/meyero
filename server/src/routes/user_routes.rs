use crate::models::user_model::{User, UpdateUser, UserLogin, SearchUser};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use rand_core::OsRng;
use crate::AppState;
use crate::service;
use crate::repository;
use serde_json;

use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};

#[get("/list")]
async fn get_all_users(
    data: web::Data<AppState>,
) -> impl Responder {
    let result =  repository::user_repository::get_all_users(&data).await;
    if result.is_err() {
        let message = "Error occured while fetching all users";
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": message}));
    }

    let users = result.unwrap();
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

    let result = repository::user_repository::get_user_by_id(&data, &user_id).await;
    if result.is_err() {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }
    let db_user= result.unwrap();

    let result = repository::user_repository::update_user(&data, body, db_user).await;
    match result {
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
    let rows_affected = repository::user_repository::delete_user(&data, &user_id).await;
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
    let result = repository::user_repository::get_user_by_username(&data, &user.username).await;
    if result.is_err() {
        let message = format!("User with username: {} not found", user.username);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let db_user = result.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(user.password.as_bytes(), &salt).unwrap();

    let compare_pass = Argon2::default().verify_password(db_user.password.as_bytes(), &hash);

    match compare_pass {
        Ok(_) => {
            // generate jwt token
            let token = service::user_service::generate_jwt_token(db_user.id);
            // add jwt token to db
            let result = repository::user_repository::add_token(&data, &db_user.id, &token).await;
            match result {
                Err(err) => {
                    let message = format!("Error: {:?}", err);
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"status": "error","message": message}));
                }
                _ => ()
            }

            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user":db_user,
                "token": token,
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

    let result = repository::user_repository::add_user(&data, &user, hashed_pass.unwrap().to_string()).await;
    match result {
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

#[get("/search")]
async fn search_user(
    username: web::Json<SearchUser>,
    data: web::Data<AppState>,
) -> impl Responder {
   // search all users where substring matches 
   let result = repository::user_repository::search_username_substring(&data, &username.name).await; 
    match result {
        Ok(result) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "users": result
            })});
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
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
