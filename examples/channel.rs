use anyhow::Result;
use oblivion::models::client::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // let register = Client::connect("oblivion://127.0.0.1:7076/account/new").await?;
    // register.send_json(
    //     json!({
    //         "username": "username",
    //         "tuta_mail": "user@tutanota.com",
    //         "password": "test",
    //         "nickname": "苏向夜"
    //     }),
    //     200,
    // ).await?;
    // println!("{}", register.recv().await?.text()?);

    let register = Client::connect("oblivion://127.0.0.1:7076/session/new").await?;
    register
        .send_json(
            json!({
                "identity": "username",
                "password": "test",
                "unique_id": "my_device"
            }),
            200,
        )
        .await?;
    let res = register.recv().await?.json()?;

    let client = Client::connect("oblivion://127.0.0.1:7076/channel").await?;

    client
        .send_json(json!({"session_key": res.get("session_key").unwrap()}), 200)
        .await?;
    let status = client.recv().await?.json()?;
    println!("status: {}", &status);
    assert_eq!(status["status"], json!(true));
    client.listen().await?;
    println!("2");
    // client.send_json(json!({"op": "unknown", "value": {}}), 200).await?;
    println!("3");
    client.send_json(json!({"op": "message/create", "value": {}}), 200).await?;
    client.send_json(json!({"op": "message/create", "value": {}}), 200).await?;
    client.send_json(json!({"op": "message/create", "value": {}}), 200).await?;
    client.send_json(json!({"op": "message/create", "value": {}}), 200).await?;
    client.send_json(json!({"op": "message/create", "value": {}}), 200).await?;
    println!("4");
    client.send_json(json!({"op": "close", "value": {}}), 200).await?;
    println!("5");

    println!("{}", client.pop().await.unwrap().text()?);
    Ok(())
}
