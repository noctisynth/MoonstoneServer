use anyhow::{anyhow, Result};
use surrealdb::sql::Datetime;

use crate::models::{Message, MessageModel};
use crate::DB;

pub async fn create(
    message_id: &str,
    node: &str,
    community_id: &str,
    user_id: &str,
    text: &str,
) -> Result<Message> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query(
            "SELECT VALUE user_id from member WHERE community_id = $community_id AND user_id != $user_id",
        )
        .bind(("community_id", community_id))
        .bind(("user_id", user_id))
        .await?;
    let send_to: Vec<String> = res.take(0)?;

    println!("sender_id: {}", user_id);
    println!("send_to: {:?}", send_to);

    let message: Option<Message> = DB
        .create(("message", message_id))
        .content(MessageModel {
            community_id,
            node,
            user_id,
            text,
            send_to,
            delivered: vec![],
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(message.unwrap())
}

pub async fn get_all_undelivered_by_user_id(node: &str, user_id: &str) -> Result<Vec<Message>> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query("SELECT * from message WHERE node = $node AND send_to CONTAINS $user_id AND delivered CONTAINSNOT $user_id")
        .bind(("node", node))
        .bind(("user_id", user_id))
        .await?;

    let messages: Vec<Message> = res.take(0)?;

    Ok(messages)
}

pub async fn delete_by_id(id: &str) -> Result<Message> {
    DB.use_ns("moonstone").use_db("community").await?;

    let message: Option<Message> = DB.delete(("message", id)).await?;
    if message.is_none() {
        return Err(anyhow!("用户不存在！"));
    }

    Ok(message.unwrap())
}
