use crate::utils::{password::hash_password, sequence::generate_sequence};
use anyhow::{Ok, Result};
use moonstone_db::models::Account;
use moonstone_db::operations::{account, session};
use serde_json::{json, Value};

pub(crate) async fn register(
    username: &str,
    tuta_mail: &str,
    password: &str,
    nickname: &str,
) -> Result<Account> {
    let password = hash_password(password)?;

    let new_account = account::create(
        &generate_sequence().await?,
        username,
        tuta_mail,
        &password,
        nickname,
        1,
    )
    .await?;

    Ok(new_account)
}

pub(crate) async fn profile(token: &str) -> Result<Value> {
    let session = match session::get_by_token(token).await? {
        Some(session) => session,
        None => return Ok(json!({"status": false, "msg": "窗口不存在！"})),
    };

    let account_info = account::get_by_id(&session.user_id).await?;

    if account_info.is_none() {
        return Ok(json!({"status": false}));
    }

    Ok(serde_json::to_value(account_info.unwrap().profile)?)
}
