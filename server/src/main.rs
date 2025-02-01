mod websocket;
mod routes;
mod models;
mod service;
mod repository;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

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

   let _ = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {db: pool.clone() }))
        .configure(routes::user_routes::config)
        .configure(routes::message_routes::config)
        .wrap(Logger::default())
    })
    .bind("localhost:8081").unwrap()
    .run()
    .await;
}
