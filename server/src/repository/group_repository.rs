use crate::models::group_model::{Group, GroupUser, GroupRequest};
use crate::AppState;
use actix_web::web;
use anyhow::Result;

pub async fn create_group(data: &web::Data<AppState>, group_req: web::Json<GroupRequest>) -> Result<()> {
    let query_result = sqlx::query_as!(
        Group,
        "Insert Into groups (name, created_by, updated_by) Values ($1, $2, $2) Returning *",
        group_req.group.name.to_string(),
        group_req.group.created_by.to_string(),
    )
    .fetch_one(&data.db)
    .await?;

    let group_id = query_result.id;

    for user in group_req.users {
        let _ = sqlx::query_as!(
            GroupUser,
            "Insert Into group_user (group_id, username) Values ($1, $2)",
            group_id,
            user.id,
        )
        .fetch_one(&data.db)
        .await?;
    }
    
    Ok(())
}