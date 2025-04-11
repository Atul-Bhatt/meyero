use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use actix_web::web;
use crate::{AppState, repository, models::message_model};
use crate::models::message_model::MessageChannel;
use uuid::Uuid;
use chrono::Utc;

pub async fn handle_connection(
    mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    data: web::Data<AppState> 
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    let message = MessageChannel {
                        id: Uuid::parse_str("dff68241-6ae4-4924-acbf-60cb954f76f8").unwrap(),
                        from_user: Uuid::parse_str("6add00c0-fbce-4846-95a1-f403f3f55fc3").unwrap(), // butters
                        to_user: Uuid::parse_str("e4f9c408-a6f7-46a2-bfea-e883e1a9a676").unwrap(), // cartman
                        message: Some(text.clone()),
                        created_at: Utc::now(),
                        updated_at: Utc::now()
                    };
                    println!("Received: {}", text);
                    // Todo: make update_message fallible
                    repository::message_repository::update_message(&data, &message, text.to_string()).await;
                    let received_message = repository::message_repository::fetch_message(&data, message.to_user, message.from_user).await;
                    stream.send(Message::Text(received_message)).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
