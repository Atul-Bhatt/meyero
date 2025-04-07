use crate::models::message_model::MessageChannel;
use std::io::Error;
use crate::AppState;
use actix_web::web;
use anyhow::Result;

pub async fn channel_exists(data: &web::Data<AppState>, message_channel: &MessageChannel) -> Result<bool, Error> {
    let query_result = sqlx::query_scalar("Select count(*) From messages Where from_user = $1 And to_user = $2")
        .bind(&message_channel.from_user)
        .bind(&message_channel.to_user)
        .fetch_one(&data.db)
        .await;

    let count: i64 = query_result.unwrap();
    if count > 1 {
        return Ok(true);
    }
    Ok(false)
}

pub async fn create_message_channel(data: &web::Data<AppState>, message_channel: &MessageChannel) -> Result<()> {
    let _ = sqlx::query_as!(
        MessageChannel,
        "Insert Into message (from_user, to_user, message) VALUES ($1, $2, $3)",
        message_channel.from_user,
        message_channel.to_user,
        message_channel.message
    )
    .execute(&data.db)
    .await?;

    Ok(())
}

pub async fn update_message(data: &web::Data<AppState>, message_channel: &MessageChannel, msg: String) {
    let _ = sqlx::query_as!(
        MessageChannel,
        "Update message message = $1 where from_user =  $2 and to_user =  $3)",
        message_channel.message,
        message_channel.from_user,
        message_channel.to_user
    )
    .execute(&data.db)
    .await?;

    Ok(())
}
