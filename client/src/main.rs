use std::{io, net::TcpStream};
use tungstenite::{connect, Message};

fn main() -> io::Result<()> {
    let (mut socket, _) = connect("ws://localhost:8080")?;

    socket.write_message(Message::Text("Hello from client!".to_string()))?;

    loop {
        let msg = socket.read_message()?;

        match msg {
            Message::Text(text) => println!("Server: {}", text),
            Message::Binary(data) => println!("Server: {:?}", data),
            _ => println!("Unexpected message type!"),
        }
    }
}
