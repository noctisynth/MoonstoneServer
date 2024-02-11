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
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::{json, Value};

pub async fn register(
    username: &str,
    tuta_mail: &str,
    password: &str,
    nickname: &str,
    db: &DatabaseConnection,
) -> Result<AccountModel, MoonstoneException> {
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

pub async fn login(
    identity: &str,
    password: &str,
    unique_id: &str,
    db: &DatabaseConnection,
) -> Result<String, MoonstoneException> {
    let user = match Account::find()
        .filter(
            AccountColumn::Username
                .eq(identity)
                .or(AccountColumn::TutaMail.eq(identity))
                .or(AccountColumn::Sequence.eq(identity)),
        )
        .one(db)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err(MoonstoneException::ColumnNotFound {
                    table: "account".to_string(),
                    field: "标识".to_string(),
                    data: identity.to_owned(),
                });
            }
        },
        Err(error) => return Err(MoonstoneException::DatabaseError { error }),
    };

    if !verify_password(password, &user.password) {
        return Err(MoonstoneException::AuthenticationFailed {
            sequence: user.sequence,
            password: password.to_owned(),
        });
    };

    let session_key = match Session::find()
        .filter(SessionColumn::UniqueId.eq(unique_id))
        .filter(SessionColumn::UserId.eq(user.id))
        .one(db)
        .await
        .unwrap()
    {
        Some(session) => {
            let expire_time =
                DateTime::parse_from_str(&session.expire_time, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();

            if Local::now() > expire_time {
                let session_key = hash_password(unique_id)?;
                let mut new_session_model: SessionActiveModel = session.into();
                new_session_model.expire_time =
                    sea_orm::ActiveValue::Set((Local::now() + Duration::days(31)).to_string());
                new_session_model.session_key = sea_orm::ActiveValue::Set(session_key.clone());
                new_session_model.update(db).await.unwrap();
                session_key
            } else {
                session.session_key
            }
        }
        None => {
            let session_key = hash_password(unique_id)?;
            let new_session_model = SessionActiveModel {
                session_key: sea_orm::ActiveValue::Set(session_key.clone()),
                user_id: sea_orm::ActiveValue::Set(user.id),
                unique_id: sea_orm::ActiveValue::Set(unique_id.to_string()),
                expire_time: sea_orm::ActiveValue::Set(
                    (Local::now() + Duration::days(31)).to_string(),
                ),
                ..Default::default()
            };
            new_session_model.insert(db).await.unwrap();
            session_key
        }
    };

    Ok(session_key)
}

pub async fn session(session_key: &str, db: &DatabaseConnection) -> bool {
    match Session::find()
        .filter(SessionColumn::SessionKey.eq(session_key))
        .one(db)
        .await
        .unwrap()
    {
        Some(session) => {
            let expire_time =
                DateTime::parse_from_str(&session.expire_time, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();

            if Local::now() > expire_time {
                false
            } else {
                true
            }
        }
        None => false,
    }
}

pub async fn account(
    session_key: &str,
    db: &DatabaseConnection,
) -> Result<Value, MoonstoneException> {
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
