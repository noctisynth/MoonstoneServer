use anyhow::{anyhow, Result};
use surrealdb::sql::Datetime;

use crate::models::{Account, AccountModel, AccountProfile};
use crate::DB;

pub async fn create(
    sequence: &str,
    username: &str,
    tuta_mail: &str,
    password: &str,
    nickname: &str,
    accessibility: i32,
) -> Result<Account> {
    DB.use_ns("moonstone").use_db("accounts").await?;

    let mut queried = DB
        .query("SELECT * FROM account WHERE username = $username OR tuta_mail = $tuta_mail")
        .bind(("username", username))
        .bind(("tuta_mail", tuta_mail))
        .await?;
    let exist_account: Option<Account> = queried.take(0)?;
    if !exist_account.is_none() {
        return Err(anyhow!("使用此用户名或Tuta邮箱的账户已存在！"));
    }

    let account: Vec<Account> = DB
        .create("account")
        .content(AccountModel {
            sequence,
            username,
            tuta_mail,
            password,
            accessibility,
            profile: AccountProfile {
                nickname: nickname.to_string(),
                sex: false,
                country: None,
                favorites_icon: None,
            },
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(account.first().unwrap().to_owned())
}

pub async fn delete_by_id(id: &str) -> Result<Account> {
    DB.use_ns("moonstone").use_db("accounts").await?;

    let account: Option<Account> = DB.select(("account", id)).await?;
    if account.is_none() {
        return Err(anyhow!("用户不存在！"));
    }

    let account: Option<Account> = DB.delete(("account", id)).await?;

    Ok(account.unwrap())
}

pub async fn get_by_id(id: &str) -> Result<Option<Account>> {
    DB.use_ns("moonstone").use_db("accounts").await?;
    let account: Option<Account> = DB.select(("account", id)).await?;
    Ok(account)
}

pub async fn get_by_sequence(sequence: &str) -> Result<Option<Account>> {
    DB.use_ns("moonstone").use_db("accounts").await?;

    let mut res = DB
        .query("SELECT * from account WHERE sequence = $sequence")
        .bind(("sequence", sequence))
        .await?;

    let account: Option<Account> = res.take(0)?;

    Ok(account)
}

pub async fn get_by_identity(identity: &str) -> Result<Option<Account>> {
    DB.use_ns("moonstone").use_db("accounts").await?;

    let mut res = DB
        .query("SELECT * from account WHERE username = $identity OR tuta_mail = $identity")
        .bind(("identity", identity))
        .await?;

    let account: Option<Account> = res.take(0)?;

    Ok(account)
}

pub async fn filter_by_identity(identities: Vec<&str>) -> Result<Option<Account>> {
    DB.use_ns("moonstone").use_db("accounts").await?;

    let mut res = DB
        .query("SELECT * from account WHERE username IN $identities OR tuta_mail IN $identities")
        .bind(("identities", identities))
        .await?;

    let account: Option<Account> = res.take(0)?;

    Ok(account)
}
