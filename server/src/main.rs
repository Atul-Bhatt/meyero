use std::collections::HashMap;

use actix_web::{App, HttpServer, HttpRequest, HttpResponse, Error, get, web, Responder, Result, post};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_rt;
use env_logger::Env;
use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc, RwLock};
use actix_ws::Message;
use std::sync::Arc;

use std::{io, net, net::TcpListener, thread};
use tungstenite::{accept, handshake, Message};

#[derive(Serialize)]
struct ApiData {
    status: String,
    data: String,
}

#[get("/health")]
async fn health() -> Result<impl Responder> {
    // let obj = ApiData {
    //     status: "healthy".to_string(),
    //     data: "Welcome to Meyero".to_string(),
    // };
    // Ok(web::Json(obj))
    Ok("Meyero is live...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let stream = stream?;
        // Spawn a new thread for each connection
        thread::spawn(move || {
            handle_connection(stream).unwrap_or_else(|err| eprintln!("Error: {}", err));
        });
    }

    Ok(())

    // HttpServer::new(|| {
    //     let cors = Cors::permissive();

    //     App::new()
    //         .wrap(cors)
    //         .wrap(Logger::default())
    //         .wrap(Logger::new("%a %{User-Agent}i"))
    //         .service(health)
    //         // .service(web::resource("/ws").route(web::get().to(julia_ws)))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
}

async fn handle_connection(stream: net::TcpStream) -> io::Result<()> {
    let mut websocket = accept(stream)?;

    loop {
        let msg = websocket.read_message()?;

        match msg {
            Message::Text(text) => {
                println!("Received text message: {}", text);
                websocket.write_message(Message::Text(text))?;
            }
            Message::Binary(data) => {
                println!("Received binary message: {:?}", data);
                websocket.write_message(Message::Binary(data))?;
            }
            Message::Close(code) => {
                println!("Client closed connection with code: {}", code);
                return Ok(());
            }
            Message::Ping(data) => {
                println!("Received ping message: {:?}", data);
                websocket.write_message(Message::Pong(data))?;
            }
            Message::Pong(_) => {
                // Ignore pong messages
            }
            _ => {}
        }
    }
}