pub mod models;
pub mod operations {
    pub mod account;
    pub mod community;
    pub mod member;
    pub mod message;
    pub mod session;
}

use anyhow::Result;
use once_cell::sync::Lazy;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub async fn init() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;
    Ok(())
}

#[cfg(test)]
mod tests;
