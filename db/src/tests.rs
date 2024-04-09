use crate::operations::{account, community, member, message};
use crate::DB;
use anyhow::Result;
use surrealdb::engine::local::RocksDb;
use tokio::test;

#[test]
async fn account() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;

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

    let deleted_account = account::delete_by_id(&account.id.id.to_raw()).await?;
    assert_eq!(account.id, deleted_account.id);

    Ok(())
}

#[test]
async fn member() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;

    let node = "oblivion://127.0.0.1:7076";

    let sequence = "sequence";
    let password = "argion$$xxxx";
    let nickname = "苏向夜";
    let accessibility = 1;

    let account = account::create(
        sequence,
        "user",
        "user@tuta.com",
        password,
        nickname,
        accessibility,
    )
    .await?;
    let account2 = account::create(
        sequence,
        "user2",
        "user2@tuta.com",
        password,
        nickname,
        accessibility,
    )
    .await?;

    let user_id = &account.id.id.to_raw();
    let user_id2 = &account2.id.id.to_raw();

    let community = community::create("测试", user_id, 0, None, true).await?;
    let community_id = &community.id.id.to_raw();

    let member1 = member::create(node, community_id, user_id, vec![]).await?;
    let member2 = member::create(node, community_id, user_id2, vec![]).await?;

    let all_members = member::get_all_by_community_id(community_id).await?;

    account::delete_by_id(&user_id).await?;
    account::delete_by_id(&user_id2).await?;
    member::delete_by_id(&member1.id.id.to_raw()).await?;
    member::delete_by_id(&member2.id.id.to_raw()).await?;
    community::delete_by_id(&community_id).await?;

    assert!(
        vec![member1.clone(), member2.clone()] == all_members
            || vec![member2, member1] == all_members
    );

    Ok(())
}

#[test]
async fn message() -> Result<()> {
    DB.connect::<RocksDb>("database").await?;

    let node = "oblivion://127.0.0.1:7076";
    let text = "苏向夜的消息";

    let sequence = "sequence";
    let password = "argion$$xxxx";
    let nickname = "苏向夜";
    let accessibility = 1;

    let account = account::create(
        sequence,
        "user",
        "user@tuta.com",
        password,
        nickname,
        accessibility,
    )
    .await?;
    let account2 = account::create(
        sequence,
        "user2",
        "user2@tuta.com",
        password,
        nickname,
        accessibility,
    )
    .await?;

    let user_id = &account.id.id.to_raw();
    let user_id2 = &account2.id.id.to_raw();

    let community = community::create("测试", user_id, 0, None, true).await?;
    let community_id = &community.id.id.to_raw();

    let member1 = member::create(node, community_id, user_id, vec![]).await?;
    let member2 = member::create(node, community_id, user_id2, vec![]).await?;

    let message = message::create("fwefwergfrew", node, community_id, user_id, text).await?;

    let unread = message::get_all_undelivered_by_user_id(node, user_id).await?;

    let message2 = message::create("fergfregwe", node, community_id, user_id2, text).await?;

    let unread2 = message::get_all_undelivered_by_user_id(node, user_id).await?;
    let unread3 = message::get_all_undelivered_by_user_id(node, user_id).await?;

    account::delete_by_id(&user_id).await?;
    account::delete_by_id(&user_id2).await?;
    message::delete_by_id(&message.id.id.to_raw()).await?;
    message::delete_by_id(&message2.id.id.to_raw()).await?;
    member::delete_by_id(&member1.id.id.to_raw()).await?;
    member::delete_by_id(&member2.id.id.to_raw()).await?;
    community::delete_by_id(&community_id).await?;

    assert!(unread.is_empty());
    assert!(!unread2.is_empty());
    assert!(unread3.is_empty());
    Ok(())
}
