use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use actix_web::web;
use crate::{AppState, repository, models::message_model};
use crate::models::message_model::{MessageChannel, WSMessage};
use uuid::Uuid;
use chrono::Utc;

pub async fn handle_connection(
    mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    data: web::Data<AppState>,
    to_user: &str 
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    let json_message = serde_json::from_str::<WSMessage>(&text).unwrap();
                    let message_channel = MessageChannel {
                        id: Uuid::parse_str("dff68241-6ae4-4924-acbf-60cb954f76f8").unwrap(),
                        from_user: Uuid::parse_str("6add00c0-fbce-4846-95a1-f403f3f55fc3").unwrap(), // butters
                        to_user: Uuid::parse_str(to_user).unwrap(),
                        message: Some(json_message.message),
                        created_at: Utc::now(),
                        updated_at: Utc::now()
                    };
                    println!("Received: {}", text);
                    // Todo: make update_message fallible
                    repository::message_repository::update_message(&data, &message_channel, text.to_string()).await;
                    let received_message = repository::message_repository::fetch_message(&data, message_channel.to_user, message_channel.from_user).await;
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
