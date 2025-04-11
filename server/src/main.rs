mod websocket;
mod routes;
mod models;
mod repository;
mod auth;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors;
use tokio_tungstenite::{accept_hdr_async, tungstenite::handshake::server::{Request, Response}};
use tokio::net::TcpListener;

use sqlx::{Pool, Postgres, PgPool};
use std::env;
use dotenv::dotenv;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
   let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
   sqlx::migrate!().run(&pool).await.unwrap(); 

   let db_pool = web::Data::new(AppState {db: pool.clone() });

   // listen for websocket connections
   tokio::spawn(async move {
        let url = env::var("WEBSOCKET_URL").expect("Error getting WEBSOCKET_URL");
        let listener = TcpListener::bind(url).await.unwrap();
        while let Ok((stream, _)) = listener.accept().await {
            let callback = |req: &Request, mut res: Response| {
                if let Some(path) = req.uri().path().strip_prefix("/ws/user/") {
                    println!("User {} connected", path);
                    // You can store user ID or do routing here.
                }
                Ok(res)
            };

            let ws_stream = accept_hdr_async(stream, callback).await.unwrap();
            tokio::spawn(websocket::handle_connection(ws_stream, db_pool.clone()));
        }
   });

   let _ = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {db: pool.clone() }))
        .configure(routes::user_routes::config)
        .configure(routes::message_routes::config)
        .wrap(middleware::Logger::default())
        .wrap(
            actix_cors::Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                    actix_web::http::header::CONTENT_TYPE
                    ])
                .max_age(3600),
        )
    })
    .bind("0.0.0.0:8081").unwrap()
    .run()
    .await;

}
