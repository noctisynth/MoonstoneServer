use anyhow::Result;
use surrealdb::sql::Datetime;

use crate::models::{Session, SessionModel};
use crate::DB;

pub async fn create(token: &str, user_id: &str, device_id: &str) -> Result<Option<Session>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let old_session: Option<Session> = DB.select(("session", device_id)).await?;
    if !old_session.is_none() {
        return Ok(old_session);
    }

    let session: Option<Session> = DB
        .create(("session", device_id))
        .content(SessionModel {
            token,
            user_id,
            device_id,
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(session)
}

pub async fn update(token: &str, user_id: &str, device_id: &str) -> Result<Option<Session>> {
    DB.use_ns("moonstone").use_db("sessions").await?;

    let old_session: Option<Session> = DB.select(("session", device_id)).await?;
    if !old_session.is_none() {
        return Ok(old_session);
    }

    let session: Option<Session> = DB
        .update(("session", device_id))
        .content(SessionModel {
            token,
            user_id,
            device_id,
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(session)
}

pub async fn get_by_id(id: &str) -> Result<Option<Session>> {
    DB.use_ns("moonstone").use_db("sessions").await?;
    let session: Option<Session> = DB.select(("session", id)).await?;
    Ok(session)
}

pub async fn get_by_user_id(user_id: &str) -> Result<Option<Session>> {
    DB.use_ns("moonstone").use_db("sessions").await?;
    let mut res = DB
        .query("SELECT * from session WHERE user_id = $user_id")
        .bind(("user_id", user_id))
        .await?;
    let session: Option<Session> = res.take(0)?;
    Ok(session)
}

pub async fn get_by_token(token: &str) -> Result<Option<Session>> {
    DB.use_ns("moonstone").use_db("sessions").await?;
    let mut res = DB
        .query("SELECT * from session WHERE token = $token_string")
        .bind(("token_string", token))
        .await?;
    let session: Option<Session> = res.take(0)?;
    Ok(session)
}
