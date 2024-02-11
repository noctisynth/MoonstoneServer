use oblivion::{models::render::BaseResponse, utils::parser::OblivionRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn deserialize<T>(req: &mut OblivionRequest) -> Result<T, BaseResponse>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    let post_data: T = match serde_json::from_value(req.get_post()) {
        Ok(result) => match result {
            Some(post_data) => post_data,
            None => {
                return Err(BaseResponse::JsonResponse(
                    json!({"status": false, "msg": "参数模型不符合月光石协议, 拒绝访问!"}),
                    403,
                ));
            }
        },
        Err(_) => {
            return Err(BaseResponse::JsonResponse(
                json!({"status": false, "msg": "参数模型不符合月光石协议, 拒绝访问!"}),
                403,
            ));
        }
    };
    Ok(post_data)
}
