use crate::{models::message_model::MessageChannel, repository};
use crate::AppState;
use crate::websocket;
use actix_web::{post, web, Responder, HttpResponse};
use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;

#[post("/initiate")]
pub async fn initiate_messaging(
    message_channel: web::Json<MessageChannel>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut send_canvas = String::from("");
    let mut receive_canvas = String::from("");

    // check if a message channel already exists
    let send_channel_exists = repository::message_repository::channel_exists(&data, message_channel.from_user.to_string(), message_channel.to_user.to_string()).await;
    match send_channel_exists {
        Ok(msg) => {
            send_canvas = msg;
        }
        Err(_) => {
           // create a new channel 
           let create_channel = repository::message_repository::create_message_channel(&data, message_channel.from_user.to_string(), message_channel.to_user.to_string()).await;
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
    let receive_channel_exists = repository::message_repository::channel_exists(&data, message_channel.to_user.to_string(), message_channel.from_user.to_string()).await;
    match receive_channel_exists {
        Ok(msg) => {
            receive_canvas = msg;
        }
        Err(_) => {
           // create a new channel receiver channel, switch from_user with to_user 
           let create_channel = repository::message_repository::create_message_channel(&data, message_channel.to_user.to_string(), message_channel.from_user.to_string()).await;
           match create_channel {
               Err(error) => {
                   return HttpResponse::InternalServerError()
                       .json(serde_json::json!({"status": "error", "message": error.to_string()}))
               }
               _ => {}
           }
        }
    }

    // start a websocket connection
    let url = env::var("WEBSOCKET_URL").expect("Error getting WEBSOCKET_URL");
    let listener = TcpListener::bind(url).await.unwrap();

    if let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(websocket::handle_connection(ws_stream, message_channel.clone(), data.clone()));
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
