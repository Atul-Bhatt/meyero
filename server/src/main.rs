mod websocket;
mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;
use sqlx::{PgPool, Postgres, Pool};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() {
    dotenv().ok();

   let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
   sqlx::migrate!().run(&pool).await.unwrap(); 

   let _ = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {db: pool.clone() }))
        .configure(routes::user_routes::config)
        .wrap(Logger::default())
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
