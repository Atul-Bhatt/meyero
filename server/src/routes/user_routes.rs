use crate::models::user_model::User;
use crate::AppState;
use serde_json;

use actix_web::{get, post, web, Result, Responder, HttpResponse};

#[get("/list")]
async fn get_all_users() -> Result<String> {
    Ok(format!("Not implemented yet!"))
}

#[post("/insert")]
async fn insert_user(
    user: web::Json<User>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "Insert Into users (username, email, name, password) VALUES ($1, $2, $3, $4) Returning *",
        user.username.to_string(),
        user.email.to_string(),
        user.name.to_string(),
        user.password.to_string(), // saving without hashing for now.
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
        .service(insert_user);

    conf.service(scope);
}