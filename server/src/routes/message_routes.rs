use crate::{models::message_model::MessageChannel, repository::message_repository};
use crate::AppState;
use crate::websocket;
use crate::auth::AuthUser;
use actix_web::{post, web, Responder, HttpResponse};
use std::env;

#[post("/initiate")]
pub async fn initiate_messaging(
    auth: AuthUser,
    mut message_channel: web::Json<MessageChannel>,
    data: web::Data<AppState>,
) -> impl Responder {
    message_channel.from_user = auth.token.unwrap().user_id;
    let mut send_canvas = String::from("");
    let mut receive_canvas = String::from("");

    // check if a message channel already exists
    let send_channel_exists = message_repository::channel_exists(
        &data,
        message_channel.from_user,
        message_channel.to_user
        )
        .await;
    match send_channel_exists {
        Ok(msg) => {
            send_canvas = msg;
        }
        Err(_) => {
           // create a new channel 
           let create_channel = message_repository::create_message_channel(
               &data,
               message_channel.from_user,
               message_channel.to_user
               )
               .await;
           match create_channel {
               Err(error) => {
                   return HttpResponse::InternalServerError()
                       .json(serde_json::json!({"status": "error", "message": error.to_string()}))
               }
               _ => {}
           }
        }
    }

    // to check receiver channel, switch from_user with to_user
    let receive_channel_exists = message_repository::channel_exists(
        &data,
        message_channel.to_user,
        message_channel.from_user
        )
        .await;
    match receive_channel_exists {
        Ok(msg) => {
            receive_canvas = msg;
        }
        Err(_) => {
           // create a new channel receiver channel, switch from_user with to_user 
           let create_channel = message_repository::create_message_channel(
               &data,
               message_channel.to_user,
               message_channel.from_user
               )
               .await;
           match create_channel {
               Err(error) => {
                   return HttpResponse::InternalServerError()
                       .json(serde_json::json!({"status": "error", "message": error.to_string()}))
               }
               _ => {}
           }
        }
    }


    let json_response = serde_json::json!({
        "send_canvas": send_canvas,
        "receive_canvas": receive_canvas
    });
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/messaging")
        .service(initiate_messaging);

    conf.service(scope);
}
