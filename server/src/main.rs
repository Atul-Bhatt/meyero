use actix_web::{App, HttpServer, HttpRequest, HttpResponse, Error, get, web, Responder, Result};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_rt;
use env_logger::Env;
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use actix_ws::Message;

mod handler;
mod websocket;

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
