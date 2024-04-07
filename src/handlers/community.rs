use anyhow::Result;
use moonstone_db::{models::Community, operations::community};

pub(crate) async fn new_community(
    name: &str,
    user_id: &str,
    security_level: i32,
    token: Option<&str>,
    cross_origin: bool,
) -> Result<Community> {
    let community_model =
        community::create(name, user_id, security_level, token, cross_origin).await?;

    Ok(community_model)
}
