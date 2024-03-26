use futures::future::{BoxFuture, FutureExt};
use oblivion::{
    models::render::BaseResponse, oblivion_codegen::async_route, utils::parser::OblivionRequest,
};
use sea_orm::Database;
use serde_json::json;

use crate::{
    handlers::community::new_community,
    models::community::NewCommunityModel,
    settings::DATABASE_URL,
    utils::{model::deserialize, session::find_and_verify_session},
};

#[async_route]
pub(crate) async fn new_community_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post_data = match deserialize::<NewCommunityModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return res,
    };

    if find_and_verify_session(&post_data.session_key, &db)
        .await
        .is_none()
    {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "登录过期！", "community_id": json!("null")}),
            403,
        );
    };

    let new_community_model = new_community(
        &post_data.name,
        post_data.security_level,
        post_data.cross_origin,
        Some(post_data.token),
        &db,
    )
    .await
    .unwrap();

    BaseResponse::JsonResponse(
        json!({"status": true, "msg": "社群创建成功！", "community_id": new_community_model.id}),
        200,
    )
}

#[async_route]
pub(crate) async fn delete_community_handler(mut _req: OblivionRequest) -> BaseResponse {
    todo!()
}
