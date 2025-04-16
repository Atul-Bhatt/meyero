use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use futures_util::{StreamExt, SinkExt, stream::SplitSink};
use futures::stream::SplitStream;
use actix_web::web;
use crate::{AppState, repository::message_repository, models::message_model};
use crate::models::message_model::{MessageChannel, WSMessage};
use uuid::Uuid;
use chrono::Utc;
use tokio::time::{sleep, Duration};


pub async fn handle_read_connection(
    mut read: SplitStream<WebSocketStream<tokio::net::TcpStream>>,
    data: web::Data<AppState>,
    ws_data: message_model::WebSocketData 
) {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    let json_message: WSMessage = serde_json::from_str(&text).unwrap();
                    let message_channel = MessageChannel {
                        id: Uuid::nil(),
                        from_user: ws_data.token.clone().unwrap().user_id,
                        to_user: Uuid::parse_str(ws_data.to_user.as_str()).unwrap(),
                        message: Some(json_message.message),
                        created_at: Utc::now(),
                        updated_at: Utc::now()
                    };
                    println!("Received: {}", text);
                    // Todo: make update_message fallible
                    message_repository::update_message(&data, &message_channel).await;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

pub async fn handle_write_connection(
    mut write: SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>,
    data: web::Data<AppState>,
    ws_data: message_model::WebSocketData 
) {
    let mut last_message = String::from("");
    loop {
        let received_message = message_repository::fetch_message(
            &data,
            Uuid::parse_str(ws_data.to_user.as_str()).unwrap(),
            ws_data.token.clone().unwrap().user_id
        ).await;

        if last_message != received_message {
            last_message = received_message.clone();
            let json_msg = serde_json::json!({ "message": received_message }).to_string();
            write.send(Message::Text(json_msg)).await.unwrap();
        }

        sleep(Duration::from_secs(1)).await;
    }
}