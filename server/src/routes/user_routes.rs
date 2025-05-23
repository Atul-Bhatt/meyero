use crate::models::user_model::{User, UpdateUser, UserLogin, SearchUserParams};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::SaltString, PasswordHash};
use rand_core::OsRng;
use crate::AppState;
use crate::auth;
use crate::repository::user_repository;
use crate::auth::AuthUser;
use serde_json;

use actix_web::{get, post, patch, delete, web, Responder, HttpResponse, HttpRequest};

// fetch all users other than current user
#[get("/list")]
async fn get_all_users(
    auth: AuthUser,
    data: web::Data<AppState>,
) -> impl Responder {
    // check token
    if auth.token.is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "invalid or missing token"}))
    }

    let result = user_repository::get_all_users(&data, auth.token.unwrap().user_id).await;
    if result.is_err() {
        let message = "Error occured while fetching all users";
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": message}));
    }

    let users = result.unwrap();
    let json_response = serde_json::json!({
        "status": "success",
        "results": users.len(),
        "user": users
    });
    HttpResponse::Ok().json(json_response)
}


#[patch("/{id}")]
async fn edit_user(
    auth: AuthUser,
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    // check token
    if auth.token.is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "invalid or missing token"}))
    }
    
    let user_id = path.into_inner();

    let result = user_repository::get_user_by_id(&data, &user_id).await;
    if result.is_err() {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }
    let db_user= result.unwrap();

    let result = user_repository::update_user(&data, body, db_user).await;
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
    auth: AuthUser,
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    // check token
    if auth.token.is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "invalid or missing token"}))
    }

    let user_id = path.into_inner();
    let rows_affected = user_repository::delete_user(&data, &user_id).await;
    if rows_affected == 0 {
        let message = format!("User with ID: {} not found", user_id);
        return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

#[post("/login")]
async fn login(
   request: HttpRequest,
   user: web::Json<UserLogin>,
   data: web::Data<AppState>, 
) -> impl Responder {

    // get User-Agent from request
    let user_agent = request.headers().get("user-agent").unwrap().to_str().unwrap();

    let result = user_repository::get_user_by_username(&data, &user.username).await;
    if result.is_err() {
        let message = format!("User with username: {} not found", user.username);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let db_user = result.unwrap();
    let hash = PasswordHash::new(&db_user.password).unwrap();

    let compare_pass = Argon2::default().verify_password(&user.password.as_bytes(), &hash);

    match compare_pass {
        Ok(_) => {
            // create session
            let session_result = user_repository::create_session(&data, &db_user.id, &user_agent.to_string()).await;
            match session_result {
                Err(err) => {
                    let message = format!("Error: {:?}", err);
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"status": "error", "message": message}));
                }
                _ => ()
            }
            let session_id = session_result.unwrap();

            // generate jwt token
            let token = auth::generate_jwt_token(db_user.id);
            // add jwt token to db
            let result = user_repository::add_token(&data, &session_id, &token).await;
            match result {
                Err(err) => {
                    let message = format!("Error: {:?}", err);
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"status": "error","message": message}));
                }
                _ => ()
            }

            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user_name":db_user.name,
                "user_id": db_user.id,
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

    let result = user_repository::add_user(&data, &user, hashed_pass.unwrap().to_string()).await;
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
    auth: AuthUser,
    req: HttpRequest, 
    data: web::Data<AppState>,
) -> impl Responder {
    // check token
    if auth.token.is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "invalid or missing token"}))
    }

    // search all users where substring matches 
    let query_params = web::Query::<SearchUserParams>::from_query(req.query_string());
    if query_params.is_err() {
        return HttpResponse::BadRequest().json(serde_json::json!({"message": "missing username query param"}));
    } else if query_params.as_ref().unwrap().username.is_empty() {
        return HttpResponse::Ok().json(serde_json::json!({"status": "success", "data": serde_json::json!({"users": []})}));
    }

    let result = user_repository::search_username_substring(&data, &query_params.unwrap().username).await; 
    match result {
        Ok(result) => {
            let response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "users": result
                })
            });
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
        .service(signup)
        .service(search_user);

    conf.service(scope);
}
