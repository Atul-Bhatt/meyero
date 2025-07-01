use crate::models::group_model::{Group};
use crate::AppState;
use actix_web::web;
use anyhow::Result;

pub async fn create_group(data: &web::Data<AppState>, group: web::Json<Group>) -> Result<()> {
    let _ = sqlx::query_as!(
        Group,
        "Insert Into groups (name, created_by, updated_by) VALUES ($1, $2, $2)",
        group.name.to_string(),
        group.created_by.to_string(),
    )
    .fetch_one(&data.db)
    .await?;
    
    Ok(())
}