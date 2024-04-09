use anyhow::{anyhow, Result};
use surrealdb::sql::Datetime;

use crate::models::{Member, MemberModel};
use crate::DB;

pub async fn create(
    node: &str,
    community_id: &str,
    user_id: &str,
    permissions: Vec<&str>,
) -> Result<Member> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query("SELECT * from member WHERE node = $node AND community_id = $community_id AND user_id = $user_id")
        .bind(("node", node))
        .bind(("community_id", community_id))
        .bind(("user_id", user_id))
        .await?;
    let queried: Option<Member> = res.take(0)?;
    if !queried.is_none() {
        return Err(anyhow!("用户已经是此群成员了。"));
    }

    let member: Vec<Member> = DB
        .create("member")
        .content(MemberModel {
            node,
            community_id,
            user_id,
            permissions,
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(member.first().unwrap().to_owned())
}

pub async fn update_permissions(
    node: &str,
    community_id: &str,
    user_id: &str,
    permissions: Vec<&str>,
) -> Result<Member> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query("SELECT * from member WHERE node = $node AND community_id = $community_id AND user_id = $user_id")
        .bind(("node", node))
        .bind(("community_id", community_id))
        .bind(("user_id", user_id))
        .await?;
    let queried: Option<Member> = res.take(0)?;
    if queried.is_none() {
        return Err(anyhow!("用户还不是此群成员。"));
    }

    let member: Option<Member> = DB
        .update(("member", queried.unwrap().id.id.to_raw()))
        .content(MemberModel {
            node,
            community_id,
            user_id,
            permissions,
            create_at: Datetime::default(),
            update_at: Datetime::default(),
        })
        .await?;

    Ok(member.unwrap())
}

pub async fn get_all_by_user_id(node: &str, user_id: &str) -> Result<Vec<Member>> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query("SELECT * from member WHERE node = $node AND user_id = $user_id")
        .bind(("node", node))
        .bind(("user_id", user_id))
        .await?;
    let queried: Vec<Member> = res.take(0)?;

    Ok(queried)
}

pub async fn get_all_by_community_id(community_id: &str) -> Result<Vec<Member>> {
    DB.use_ns("moonstone").use_db("community").await?;

    let mut res = DB
        .query("SELECT * from member WHERE community_id = $community_id")
        .bind(("community_id", community_id))
        .await?;
    let queried: Vec<Member> = res.take(0)?;

    Ok(queried)
}

pub async fn delete_by_id(id: &str) -> Result<Member> {
    DB.use_ns("moonstone").use_db("community").await?;

    let member: Option<Member> = DB.delete(("member", id)).await?;
    if member.is_none() {
        return Err(anyhow!("用户尚未成为此社群成员！"));
    }

    Ok(member.unwrap())
}
