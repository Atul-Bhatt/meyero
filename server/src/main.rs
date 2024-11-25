mod websocket;
mod routes;
mod models;

use actix_web::{web, App, HttpServer, Responder};
use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;
use sqlx::PgPool;

async fn health_check() -> impl Responder {
    "Hello! Welcome to Meyero."
}

#[actix_web::main]
async fn main() {
    dotenv().ok();

   let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
   sqlx::migrate!().run(&pool).await.unwrap(); 

   let _ = HttpServer::new(|| {
        App::new().route("/", web::get().to(health_check))
    })
    .bind("localhost:8081").unwrap()
    .run()
    .await;

    let url = env::var("WEBSOCKET_URL").expect("Error getting _WEBSOCKET_URL");
    let listener = TcpListener::bind(url).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(websocket::handle_connection(ws_stream));
    }
}
