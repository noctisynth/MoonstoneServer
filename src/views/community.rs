use anyhow::anyhow;
use oblivion::models::render::{BaseResponse, Response};
use oblivion::{oblivion_codegen::async_route, utils::parser::OblivionRequest};
use serde_json::json;

use moonstone_db::operations::{member, message, session};

use crate::models::community::{JoinCommunityModel, MessageModel};
use crate::{
    handlers::community::new_community, models::community::NewCommunityModel,
    utils::model::deserialize,
};

#[async_route]
async fn new_community_handler(mut req: OblivionRequest) -> Response {
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
async fn delete_community_handler(mut _req: OblivionRequest) -> Response {
    todo!()
}

#[async_route]
async fn join_community_handler(mut req: OblivionRequest) -> Response {
    let post_data = match deserialize::<JoinCommunityModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    match member::create(
        &post_data.node,
        &post_data.community_id,
        &post_data.user_id,
        vec!["community::message::read"],
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            return Ok(BaseResponse::JsonResponse(
                json!({"status": false, "msg": e.to_string()}),
                502,
            ))
        }
    }

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": "成功加入社群！"}),
        200,
    ))
}

#[async_route]
async fn new_message_handler(mut req: OblivionRequest) -> Response {
    let post_data = match deserialize::<MessageModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    let user = session::get_by_token(&post_data.token).await?;
    if user.is_none() {
        return Err(anyhow!("用户不存在！"));
    }

    message::create(
        &post_data.message_id,
        &post_data.node,
        &post_data.community_id,
        &user.unwrap().user_id,
        &post_data.text,
    )
    .await?;

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": "success"}),
        200,
    ))
}

#[async_route]
async fn get_message_handler(mut req: OblivionRequest) -> Response {
    let post_data = match deserialize::<MessageModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    let user = session::get_by_token(&post_data.token).await?;
    if user.is_none() {
        return Err(anyhow!("用户不存在！"));
    }

    let messages =
        message::get_all_undelivered_by_user_id(&post_data.node, &user.unwrap().id.id.to_raw())
            .await?;

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": "success", "messages": serde_json::to_value(messages)?}),
        200,
    ))
}
