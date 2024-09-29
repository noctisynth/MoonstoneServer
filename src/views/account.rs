use crate::{
    handlers::account::{profile, register},
    models::{account::RegisterModel, session::SessionModel},
    utils::{email::EMAIL_VALIDATOR, model::deserialize},
};
use anyhow::Result;
use moonstone_db::operations::account::filter_by_identity;
use oblivion::models::session::Session;
use oblivion::oblivion_codegen::async_route;
use serde_json::{json, Value};

#[async_route]
async fn register_handler(sess: Session) -> Result<Value> {
    let post_data = match deserialize::<RegisterModel>(&mut sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    if !EMAIL_VALIDATOR.validate(&post_data.tuta_mail) {
        return Ok(json!({"status": false, "msg": "邮箱不是合法的Tuta邮箱!"}));
    };

    let account_find = filter_by_identity(vec![&post_data.username, &post_data.tuta_mail]).await?;

    if !account_find.is_none() {
        return Ok(json!({"status": false, "msg": "Tuta邮箱或用户名已存在!"}));
    };

    let account = register(
        &post_data.username,
        &post_data.tuta_mail,
        &post_data.password,
        &post_data.nickname,
    )
    .await?;

    Ok(json!({"status": true, "msg": format!("用户[{}]创建成功!", account.username)}))
}

#[async_route]
async fn profile_handler(sess: Session) -> Result<Value> {
    let session_key = &match deserialize::<SessionModel>(&mut sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    }
    .session_key;

    profile(session_key).await
}
