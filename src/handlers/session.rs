use crate::{
    exceptions::Exception,
    utils::password::{hash_password, verify_password},
};
use chrono::{DateTime, Duration, Local};
use entity::account::{Column as AccountColumn, Entity as Account};
use entity::session::{
    ActiveModel as SessionActiveModel, Column as SessionColumn, Entity as Session,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub(crate) async fn login(
    identity: &str,
    password: &str,
    unique_id: &str,
    db: &DatabaseConnection,
) -> Result<String, Exception> {
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
                return Err(Exception::ColumnNotFound {
                    table: "account".to_string(),
                    field: "标识".to_string(),
                    data: identity.to_owned(),
                });
            }
        },
        Err(error) => return Err(Exception::DatabaseError { error }),
    };

    if !verify_password(password, &user.password) {
        return Err(Exception::AuthenticationFailed {
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

pub(crate) async fn session(session_key: &str, db: &DatabaseConnection) -> bool {
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
