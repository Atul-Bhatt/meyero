mod node;
mod seed;

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use std::error::Error;
use serde_json::{self, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 72];
            socket.read(&mut buffer).await.unwrap();

            // deserialize the buffer
            let v: Value = serde_json::from_slice(&buffer).unwrap();
            println!("{} said {}", v["sender_id"], v["message"]);
        });
    }
    */
    let _dht_client = node::dht_client();
    Ok(())
}
