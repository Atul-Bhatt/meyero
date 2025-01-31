use crate::{models::message_model::MessageChannel, repository};
use crate::AppState;
use actix_web::http::StatusCode;
use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};

#[post("/initiate")]
pub async fn initiate_messaging(
    message_channel: web::Json<MessageChannel>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Start a websocket between two users
    
    // check if a message channle already exists
    let channel_exists = repository::message_repository::channel_exists(&data, &message_channel).await;
    let exists: bool = channel_exists.unwrap();
    if !exists {
       // create a new channel 
       let create_channel = repository::message_repository::create_message_channel(&data, &message_channel).await;
       match create_channel {
           Err(error) => {
               return HttpResponse::InternalServerError()
                   .json(serde_json::json!({"status": "error", "message": error}))
           }
           _ => {}
       }
    }

    let json_response = serde_json::json!({
        "status": "success"
    });
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/messaging")
        .service(initiate_messaging);

    conf.service(scope);
}
