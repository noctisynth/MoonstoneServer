use crate::{
    handlers::account::{account, register},
    models::account::RegisterModel,
    settings::DATABASE_URL,
    utils::{email::EMAIL_VALIDATOR, model::deserialize},
};
use entity::account::{Column as AccountColumn, Entity as Account};
use futures::future::{BoxFuture, FutureExt};
use oblivion::{
    models::render::BaseResponse, oblivion_codegen::async_route, utils::parser::OblivionRequest,
};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde_json::json;

#[async_route]
async fn register_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post_data = match deserialize::<RegisterModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return res,
    };

    if !EMAIL_VALIDATOR.validate(&post_data.tuta_mail) {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "邮箱不是合法的Tuta邮箱!"}),
            403,
        );
    };

    let account_find = Account::find()
        .filter(AccountColumn::Username.eq(&post_data.username))
        .filter(AccountColumn::TutaMail.eq(&post_data.tuta_mail))
        .filter(AccountColumn::Nickname.eq(&post_data.nickname))
        .one(&db)
        .await
        .unwrap();

    if !account_find.is_none() {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "Tuta邮箱或用户名已存在!"}),
            403,
        );
    };

    let account = register(
        &post_data.username,
        &post_data.tuta_mail,
        &post_data.password,
        &post_data.nickname,
        &db,
    )
    .await
    .unwrap();

    BaseResponse::JsonResponse(
        json!({"status": true, "msg": format!("用户[{}]创建成功!", account.username)}),
        200,
    )
}

#[async_route]
async fn account_handler(mut req: OblivionRequest) -> BaseResponse {
    let post = req.get_post();

    let session_key = match post["session_key"].as_str() {
        Some(session_key) => session_key,
        None => {
            return BaseResponse::JsonResponse(json!({"status": false, "msg": "参数异常!"}), 403);
        }
    };

    let db = Database::connect(DATABASE_URL).await.unwrap();

    BaseResponse::JsonResponse(account(session_key, &db).await.unwrap(), 200)
}
