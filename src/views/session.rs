use crate::{
    handlers::session::{login, session},
    models::session::{LoginModel, SessionModel},
    utils::model::deserialize,
};
use oblivion::models::{render::BaseResponse, session::Session};
use oblivion::oblivion_codegen::async_route;
use oblivion::types::server::Response;
use serde_json::json;

#[async_route]
async fn login_handler(sess: Session) -> Response {
    let post_data = match deserialize::<LoginModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": "身份验证成功", "session_key": match login(&post_data.identity, &post_data.password, &post_data.unique_id).await {
            Ok(session_key) => session_key,
            Err(error) => {
                return Ok(BaseResponse::JsonResponse(json!({"status": false, "msg": error.to_string()}), 403));
            },
        }}),
        200,
    ))
}

#[async_route]
async fn session_handler(sess: Session) -> Response {
    let session_key = &match deserialize::<SessionModel>(&sess.recv().await?) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    }
    .session_key;

    let status = session(session_key).await?;
    let status_code = if status { 200 } else { 403 };

    Ok(BaseResponse::JsonResponse(
        json!({"status": status}),
        status_code,
    ))
}
