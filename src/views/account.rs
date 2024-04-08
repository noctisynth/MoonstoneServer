use crate::{
    handlers::account::{profile, register},
    models::{account::RegisterModel, session::SessionModel},
    utils::{email::EMAIL_VALIDATOR, model::deserialize},
};
use moonstone_db::operations::account::filter_by_identity;
use oblivion::models::render::{BaseResponse, Response};
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use serde_json::json;

#[async_route]
async fn register_handler(mut req: OblivionRequest) -> Response {
    let post_data = match deserialize::<RegisterModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };

    if !EMAIL_VALIDATOR.validate(&post_data.tuta_mail) {
        return Ok(BaseResponse::JsonResponse(
            json!({"status": false, "msg": "邮箱不是合法的Tuta邮箱!"}),
            403,
        ));
    };

    let account_find = filter_by_identity(vec![&post_data.username, &post_data.tuta_mail]).await?;

    if !account_find.is_none() {
        return Ok(BaseResponse::JsonResponse(
            json!({"status": false, "msg": "Tuta邮箱或用户名已存在!"}),
            403,
        ));
    };

    let account = register(
        &post_data.username,
        &post_data.tuta_mail,
        &post_data.password,
        &post_data.nickname,
    )
    .await
    .unwrap();

    Ok(BaseResponse::JsonResponse(
        json!({"status": true, "msg": format!("用户[{}]创建成功!", account.username)}),
        200,
    ))
}

#[async_route]
async fn profile_handler(mut req: OblivionRequest) -> Response {
    let session_key = &match deserialize::<SessionModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    }
    .session_key;

    Ok(BaseResponse::JsonResponse(
        profile(session_key).await.unwrap(),
        200,
    ))
}
