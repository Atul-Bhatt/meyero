use actix_web::{App, HttpServer, HttpRequest, HttpResponse, Error, get, web, Responder, Result};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_rt;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(get_julia_image)
            .service(web::resource("/ws").route(web::get().to(julia_ws)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
