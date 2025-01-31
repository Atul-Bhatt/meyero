use crate::{models::message_model::MessageChannel, repository};
use crate::AppState;
use actix_web::{get, post, patch, delete, web, Responder, HttpResponse};

#[post("/initiate")]
pub async fn initiate_messaging(
    message_channel: web::Json<MessageChannel>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Start a websocket between two users
    
    // check if a message channle already exists
    let result = repository::message_repository::channel_exists(&data, &message_channel);
    

    // insert empty message between users
    
   String::from("") 
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/messaging")
        .service(initiate_messaging);

    conf.service(scope);
}
