use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use actix_web::web;
use crate::{AppState, repository, models::message_model};

pub async fn handle_connection(
    mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    println!("Received: {}", text);
                    // Todo: make update_message fallible
                    //repository::message_repository::update_message(&data, &message_channel, text.to_string()).await;
                    //let received_message = repository::message_repository::fetch_message(&data, message_channel.to_user, message_channel.from_user).await;
                    //stream.send(Message::Text(received_message)).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
