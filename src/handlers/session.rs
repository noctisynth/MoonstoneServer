use crate::{
    exceptions::Exception,
    utils::password::{hash_password, verify_password},
};
use anyhow::{Error, Result};
use moonstone_db::operations::{account, session};

pub(crate) async fn login(identity: &str, password: &str, unique_id: &str) -> Result<String> {
    let user = match account::get_by_identity(identity).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err(Error::from(Exception::ColumnNotFound {
                    table: "account".to_string(),
                    field: "标识".to_string(),
                    data: identity.to_owned(),
                }));
            }
        },
        Err(error) => return Err(Error::from(error)),
    };

    if !verify_password(password, &user.password) {
        return Err(Error::from(Exception::AuthenticationFailed {
            sequence: user.sequence,
            password: password.to_owned(),
        }));
    };

    let session_key = match session::get_by_id(unique_id).await? {
        Some(session) => session.token,
        None => {
            let token = hash_password(unique_id)?;
            session::create(&token, &user.id.id.to_raw(), unique_id).await?;
            token
        }
    };

    Ok(session_key)
}

pub(crate) async fn session(token: &str) -> Result<bool> {
    match session::get_by_token(token).await? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
