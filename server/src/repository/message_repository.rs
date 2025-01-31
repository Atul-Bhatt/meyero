use crate::models::message_model::MessageChannel;
use std::io::Error;
use crate::AppState;
use actix_web::web;

pub async fn channel_exists(data: &web::Data<AppState>, message_channel: &MessageChannel) -> Result<bool, Error> {
    let query_result = sqlx::query_scalar("Select count(*) From messages Where from_user = $1 And to_user = $2")
        .bind(message_channel.from_user)
        .bind(message_channel.to_user)
        .fetch_one(&data.db)
        .await;

    let count = query_result.unwrap();
    if count > 1 {
        return Ok(true);
    }
    Ok(false)
}
