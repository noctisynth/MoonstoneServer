use entity::community::{ActiveModel as CommunityActiveModel, Model as CommunityModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

pub(crate) async fn new_community(
    name: &str,
    security_level: i32,
    cross_origin: bool,
    token: Option<String>,
    db: &DatabaseConnection,
) -> Result<CommunityModel, ()> {
    let new_community_model = CommunityActiveModel {
        name: sea_orm::ActiveValue::set(name.to_string()),
        security_level: sea_orm::ActiveValue::set(security_level),
        cross_origin: sea_orm::ActiveValue::set(cross_origin),
        token: sea_orm::ActiveValue::set(token),
        ..Default::default()
    };

    let community_model = new_community_model.insert(db).await.unwrap();

    Ok(community_model)
}
