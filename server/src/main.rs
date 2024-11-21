use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(handle_connection(ws_stream));
    }
}

async fn handle_connection(mut stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
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