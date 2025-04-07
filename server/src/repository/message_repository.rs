use crate::models::message_model::MessageChannel;
use std::io::Error;
use crate::AppState;
use actix_web::web;
use anyhow::Result;

pub async fn channel_exists(data: &web::Data<AppState>, from_user: String, to_user: String) -> Result<String, Error> {
    let query_result = sqlx::query_as!(
        MessageChannel,
        "Select message From messages Where from_user = $1 And to_user = $2",
        from_user,
        to_user
        )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(msg) => Ok(msg.message),
        Err(e) => Err(e),
    }
}

pub async fn create_message_channel(data: &web::Data<AppState>, from_user: String, to_user: String) -> Result<()> {
    let _ = sqlx::query_as!(
        MessageChannel,
        "Insert Into message (from_user, to_user) VALUES ($1, $2)",
        from_user,
        to_user,
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
