use server::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port");
    println!("Running on port: {}", listener.local_addr().unwrap().port());

    run(listener)?.await
}
