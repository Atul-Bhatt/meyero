use crate::models::group_model::{GroupRequest};
use crate::repository::group_repository;
use crate::AppState;
use crate::auth::AuthUser;
use serde_json;

use actix_web::{post, web, Responder, HttpResponse};

// fetch all users other than current user
#[post("/create")]
async fn create_group(
    auth: AuthUser,
    group: web::Json<GroupRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    // check token
    if auth.token.is_none() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "error", "message": "invalid or missing token"}))
    }

    let result = group_repository::create_group(&data, group).await;
    if result.is_err() {
        let message = "Error occured while creating group";
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": message}));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": "group created successfully"
    });
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/group")
        .service(create_group);

    conf.service(scope);
}