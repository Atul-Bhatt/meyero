mod websocket;

use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("WEBSOCKET_URL").expect("Error getting _WEBSOCKET_URL");
    let listener = TcpListener::bind(url).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(websocket::handle_connection(ws_stream));
    }
}
