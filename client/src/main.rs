mod node;
mod seed;

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            socket.read(&mut buffer).await.unwrap();
            println!("{:?}", buffer)
        });
    }
}
