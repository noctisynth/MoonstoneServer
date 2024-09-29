use crate::{
    handlers::session::{login, session},
    models::session::{LoginModel, SessionModel},
    utils::model::deserialize,
};
use anyhow::Result;
use oblivion::models::session::Session;
use oblivion::oblivion_codegen::async_route;
use serde_json::{json, Value};

#[async_route]
async fn login_handler(sess: Session) -> Result<Value> {
    let post_data = match deserialize::<LoginModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    Ok(
        json!({"status": true, "msg": "身份验证成功", "session_key": match login(&post_data.identity, &post_data.password, &post_data.unique_id).await {
            Ok(session_key) => session_key,
            Err(error) => {
                return Ok(json!({"status": false, "msg": error.to_string()}));
            },
        }}),
    )
}

#[async_route]
async fn session_handler(sess: Session) -> Result<Value> {
    let session_key = &match deserialize::<SessionModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    }
    .session_key;

    let status = session(session_key).await?;

    Ok(json!({"status": status}))
}
