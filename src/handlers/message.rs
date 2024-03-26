use sea_orm::DatabaseConnection;
use serde_json::{json, Value};

use crate::utils::session::find_and_verify_session;

pub(crate) async fn _new_message(
    session_key: &str,
    _text: &str,
    _dist: &str,
    db: &DatabaseConnection,
) -> Result<(), Value> {
    let _session = match find_and_verify_session(session_key, db).await {
        Some(session) => session,
        None => return Err(json!({"status": false, "msg": "窗口已过期或不存在!"})),
    };

    Ok(())
}
