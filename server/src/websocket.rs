use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use actix_web::web;
use crate::{AppState, repository, models::message_model};

pub async fn handle_connection(
    mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    message_channel: message_model::MessageChannel,
    data: web::Data<AppState>
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    println!("Received: {}", text);
                    // stream.send(Message::Text(text)).await.unwrap();
                    // Todo: make update_message fallible
                    repository::message_repository::update_message(&data, &message_channel, text.to_string()).await;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
