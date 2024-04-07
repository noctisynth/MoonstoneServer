use oblivion::models::render::{BaseResponse, Response};
use oblivion::{oblivion_codegen::async_route, utils::parser::OblivionRequest};
use serde_json::json;

use moonstone_db::operations::session;

use crate::{
    handlers::community::new_community, models::community::NewCommunityModel,
    utils::model::deserialize,
};

#[async_route]
pub(crate) async fn new_community_handler(mut req: OblivionRequest) -> Response {
    let post_data = match deserialize::<NewCommunityModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    let user = session::get_by_token(&post_data.session_key).await?;
    if user.is_none() {
        return Ok(BaseResponse::JsonResponse(
            json!({"status":false,"msg":"账户不存在！"}),
            403,
        ));
    }

    let new_community_model = new_community(
        &post_data.name,
        &user.unwrap().id.id.to_raw(),
        post_data.security_level,
        Some(&post_data.token),
        post_data.cross_origin,
    )
    .await?;

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": "社群创建成功！", "community_id": new_community_model.id.id.to_raw()}),
        200,
    ))
}

#[async_route]
pub(crate) async fn delete_community_handler(mut _req: OblivionRequest) -> Response {
    todo!()
}
