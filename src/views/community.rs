use anyhow::{anyhow, Result};
use oblivion::models::session::Session;
use oblivion::{oblivion_codegen::async_route, utils::parser::OblivionRequest};
use serde_json::{json, Value};

use moonstone_db::operations::{member, message, session};

use crate::models::community::{GetAllMessagesModel, JoinCommunityModel, MessageModel};
use crate::{
    handlers::community::new_community, models::community::NewCommunityModel,
    utils::model::deserialize,
};

#[async_route]
async fn new_community_handler(sess: Session) -> Result<Value> {
    let post_data = match deserialize::<NewCommunityModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    let user = session::get_by_token(&post_data.session_key).await?;
    if user.is_none() {
        return Ok(json!({"status": false, "msg": "账户不存在！"}));
    }

    let new_community_model = new_community(
        &post_data.name,
        &user.unwrap().id.id.to_raw(),
        post_data.security_level,
        Some(&post_data.token),
        post_data.cross_origin,
    )
    .await?;

    Ok(
        json!({"status": true, "msg": "社群创建成功！", "community_id": new_community_model.id.id.to_raw()}),
    )
}

#[async_route]
async fn delete_community_handler(mut _req: OblivionRequest) -> Result<Value> {
    todo!()
}

#[async_route]
async fn join_community_handler(session: Session) -> Result<Value> {
    let post_data = match deserialize::<JoinCommunityModel>(&session.recv().await?) {
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
        Err(e) => return Ok(json!({"status":false,"msg":e.to_string()})),
    }

    Ok(json!({"status": true, "msg": "成功加入社群！"}))
}

#[async_route]
async fn new_message_handler(sess: Session) -> Result<Value> {
    let post_data = match deserialize::<MessageModel>(&sess.recv().await?) {
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

    Ok(json!({"status": true, "msg": "success"}))
}

#[async_route]
async fn get_message_handler(sess: Session) -> Result<Value> {
    let post_data = match deserialize::<GetAllMessagesModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    let user = session::get_by_token(&post_data.token).await?;
    if user.is_none() {
        return Err(anyhow!("用户不存在！"));
    }

    let messages = message::get_all_undelivered_by_user_id(&user.unwrap().id.id.to_raw()).await?;

    Ok(json!({"status": true, "msg": "success", "messages": serde_json::to_value(messages)?}))
}
