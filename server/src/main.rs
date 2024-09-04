use server::run;
use std::net::TcpListener;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port");
    println!("Running on port: {}", listener.local_addr().unwrap().port());

    run(listener)?.await
}
