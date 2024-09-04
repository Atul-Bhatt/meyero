use actix_web::{get, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;

#[get("/health_check")]
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
