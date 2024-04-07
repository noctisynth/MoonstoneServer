pub mod models;
pub mod operations {
    pub mod account;
    pub mod session;
}

use anyhow::{Ok, Result};
use once_cell::sync::Lazy;
use operations::account;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub async fn test() -> Result<()> {
    init().await?;

    let sequence = "sequence";
    let username = "user";
    let tuta_mail = "user@tuta.com";
    let password = "argion$$xxxx";
    let nickname = "苏向夜";
    let accessibility = 1;

    let account = account::create(
        sequence,
        username,
        tuta_mail,
        password,
        nickname,
        accessibility,
    )
    .await?;

    account::delete_by_id(&account.id.id.to_raw()).await?;

    Ok(())
}

pub async fn init() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;
    Ok(())
}
