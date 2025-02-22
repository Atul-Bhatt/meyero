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
    // Start a websocket between two users
    dotenv().ok();

    let url = env::var("WEBSOCKET_URL").expect("Error getting WEBSOCKET_URL");
    let listener = TcpListener::bind(url).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(websocket::handle_connection(ws_stream));
    }
    
    // check if a message channel already exists
    let channel_exists = repository::message_repository::channel_exists(&data, &message_channel).await;
    let exists: bool = channel_exists.unwrap();
    if !exists {
       // create a new channel 
       let create_channel = repository::message_repository::create_message_channel(&data, &message_channel).await;
       match create_channel {
           Err(error) => {
               return HttpResponse::InternalServerError()
                   .json(serde_json::json!({"status": "error", "message": error.to_string()}))
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
