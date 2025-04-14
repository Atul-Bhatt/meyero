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
use std::sync::{Arc, Mutex};

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
   //let mut to_user = "e4f9c408-a6f7-46a2-bfea-e883e1a9a676";
   let user_id_holder = Arc::new(Mutex::new(None));
   let user_id_holder_clone = user_id_holder.clone();

   // listen for websocket connections
   tokio::spawn(async move{
        let url = env::var("WEBSOCKET_URL").expect("Error getting WEBSOCKET_URL");
        let listener = TcpListener::bind(url).await.unwrap();
        while let Ok((stream, _)) = listener.accept().await {
            let callback = |req: &Request, res: Response| {
                let mut web_socket_data = models::message_model::WebSocketData {token: None, to_user: "".to_string()};
                if let Some(path) = req.uri().path().strip_prefix("/ws/user/") {
                    web_socket_data.to_user = path.to_string();
                    println!("User {} connected", path);
                }
                if let Some(token) = req.uri().query() {
                    web_socket_data.token = Some(auth::decode_token(&token[6..]).unwrap());
                }
                *user_id_holder_clone.lock().unwrap() = Some(web_socket_data);
                Ok(res)
            };

            let ws_stream= accept_hdr_async(stream, callback).await.unwrap();
            let ws_data = user_id_holder.lock().unwrap().clone().unwrap();
            tokio::spawn(websocket::handle_connection(ws_stream, db_pool.clone(), ws_data));
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
