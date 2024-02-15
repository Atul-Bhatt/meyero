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

mod handler;
mod websocket;

type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Serialize)]
struct ApiData {
    status: String,
    data: String,
}

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Token,
    Code
}

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, Error>>>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
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

#[post("publish")]
async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::Text(body.message.clone().into())));
            }
        });

    Ok(HttpResponse::Accepted())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health)
            // .service(web::resource("/ws").route(web::get().to(julia_ws)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
