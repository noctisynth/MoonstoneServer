use crate::{
    exceptions::Exception,
    utils::{password::hash_password, sequence::generate_sequence},
};
use chrono::{DateTime, Local};
use entity::account::{
    ActiveModel as AccountActiveModel, Column as AccountColumn, Entity as Account,
    Model as AccountModel,
};
use entity::session::{Column as SessionColumn, Entity as Session};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::{json, Value};

pub(crate) async fn register(
    username: &str,
    tuta_mail: &str,
    password: &str,
    nickname: &str,
    db: &DatabaseConnection,
) -> Result<AccountModel, Exception> {
    let password = hash_password(password)?;

    let new_user_model = AccountActiveModel {
        sequence: sea_orm::ActiveValue::Set(generate_sequence(db).await.to_string()),
        username: sea_orm::ActiveValue::Set(username.to_string()),
        tuta_mail: sea_orm::ActiveValue::Set(tuta_mail.to_string()),
        password: sea_orm::ActiveValue::Set(password),
        nickname: sea_orm::ActiveValue::Set(nickname.to_string()),
        ..Default::default()
    };

    let new_user = new_user_model.insert(db).await.unwrap();

    Ok(new_user)
}

pub(crate) async fn account(
    session_key: &str,
    db: &DatabaseConnection,
) -> Result<Value, Exception> {
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
                return Ok(json!({"status": false, "msg": "登录已经过期，请重新登录！"}));
            } else {
                session
            }
        }
        None => return Ok(json!({"status": false, "msg": "窗口不存在！"})),
    };

    let account = Account::find()
        .filter(AccountColumn::Id.eq(session.user_id))
        .one(db)
        .await
        .unwrap()
        .unwrap();

    Ok(json!(
        {
            "username": account.username,
            "nickname": account.nickname,
            "favicon": account.favorites_icon,
        }
    )
    .to_owned())
}
