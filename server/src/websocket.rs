use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};

pub async fn handle_connection(mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    println!("Received: {}", text);

                    stream.send(Message::Text(text)).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}