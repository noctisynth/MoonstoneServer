use crate::{
    handlers::account::{login, session},
    models::account::LoginModel,
    settings::DATABASE_URL,
    utils::model::deserialize,
};
use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::Database;
use serde_json::json;

#[async_route]
async fn login_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post_data = match deserialize::<LoginModel>(&mut req) {
        Ok(model) => model,
        Err(res) => return res,
    };

    BaseResponse::JsonResponse(
        json!({"status": true, "msg": "身份验证成功", "session_key": match login(&post_data.identity, &post_data.password, &post_data.unique_id, &db).await {
            Ok(session_key) => session_key,
            Err(error) => {
                return BaseResponse::JsonResponse(json!({"status": false, "msg": error.to_string()}), 403);
            },
        }}),
        200,
    )
}

#[async_route]
async fn session_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post = req.get_post();
    let session_key = match post["session_key"].as_str() {
        Some(session_key) => session_key,
        None => return BaseResponse::JsonResponse(json!({"status": false}), 403),
    };

    let status = session(session_key, &db).await;
    let status_code = if status { 200 } else { 403 };

    BaseResponse::JsonResponse(json!({"status": status}), status_code)
}
