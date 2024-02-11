use crate::{
    exceptions::MoonstoneException,
    utils::{
        password::{hash_password, verify_password},
        sequence::generate_sequence,
    },
};
use chrono::{DateTime, Duration, Local};
use entity::account::{
    ActiveModel as AccountActiveModel, Column as AccountColumn, Entity as Account,
    Model as AccountModel,
};
use entity::session::{
    ActiveModel as SessionActiveModel, Column as SessionColumn, Entity as Session,
};
use oblivion::models::render::BaseResponse;
use redis::{Client, Commands};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::{json, Value};

pub async fn new_message(
    session_key: &str,
    text: &str,
    dist: &str,
    db: &DatabaseConnection,
) -> Result<(), Value> {
    let session = match Session::find()
        .filter(SessionColumn::SessionKey.eq(session_key))
        .one(db)
        .await
        .unwrap()
    {
        Some(session) => {
            let expire_time =
                DateTime::parse_from_str(&session.expire_time, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();

            if Local::now() > expire_time {
                return Err(json!({"status": false, "msg": "登录已经过期，请重新登录！"}));
            } else {
                session
            }
        }
        None => return Err(json!({"status": false, "msg": "窗口不存在！"})),
    };

    let client = Client::open("redis://60.204.187.79:5900/").unwrap();
    let mut con = client.get_connection().unwrap();

    // con.set(key, value)

    session.user_id;

    Ok(())
}
