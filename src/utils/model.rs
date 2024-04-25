use oblivion::models::{client::Response, render::BaseResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) fn deserialize<T>(res: &Response) -> Result<T, BaseResponse>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    let invalid = BaseResponse::JsonResponse(
        json!({
            "status": false,
            "msg": "你的请求参数模型不符合月长石通讯协议，\
            拒绝访问！请注意，你的行为已被上报第五议会，\
            如果确认你的行为属于入侵，你将被即刻定位并处决！"
        }),
        403,
    );
    let post_data: T = match serde_json::from_value(match res.json() {
        Ok(json) => json,
        Err(_) => return Err(invalid),
    }) {
        Ok(result) => match result {
            Some(post_data) => post_data,
            None => {
                return Err(invalid);
            }
        },
        Err(_) => {
            return Err(invalid);
        }
    };
    Ok(post_data)
}
