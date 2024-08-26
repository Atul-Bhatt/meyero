use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let message = json!({
        "sender_id": "Node A",
        "recipient_id": "Node B",
        "message": "Hello, Node B",
    });

    let message_str = message.to_string();
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    stream.write_all(message_str.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
