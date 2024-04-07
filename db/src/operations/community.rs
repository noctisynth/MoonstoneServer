use anyhow::Result;
use surrealdb::sql::Datetime;

use crate::models::{Community, CommunityModel};
use crate::DB;

pub async fn create(
    name: &str,
    user_id: &str,
    security_level: i32,
    token: Option<&str>,
    cross_origin: bool,
) -> Result<Community> {
    DB.use_ns("moonstone").use_db("community").await?;

    let community: Vec<Community> = DB
        .create("community")
        .content(CommunityModel {
            name,
            user_id,
            security_level,
            token,
            cross_origin,
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(community.first().unwrap().to_owned())
}
