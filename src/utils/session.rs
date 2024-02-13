use chrono::{DateTime, Local};
use entity::session::{Column as SessionColumn, Entity as Session, Model as SessionModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub(crate) async fn find_and_verify_session(
    session_key: &str,
    db: &DatabaseConnection,
) -> Option<SessionModel> {
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
                None
            } else {
                Some(session)
            }
        }
        None => None,
    }
}
