use anyhow::Result;
use futures::lock::Mutex;
use moonstone_db::operations::session;
use oblivion::types::server::Response;
use oblivion::{
    models::{render::BaseResponse, session::Session},
    oblivion_codegen::async_route,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

use crate::{
    models::{
        event::{Event, EventModel},
        session::SessionModel,
    },
    utils::model::deserialize,
    POOL,
};

async fn send_forever(user_id: String, status: Arc<Mutex<bool>>, sess: Arc<Session>) -> Result<()> {
    let pool_arc = Arc::clone(&POOL);
    let mut pool = pool_arc.lock().await;
    let rx = pool.get_receiver(user_id).unwrap();
    loop {
        if !*status.lock().await {
            break Ok(());
        }
        if let Ok(Some(value)) = timeout(Duration::from_secs(2), rx.recv()).await {
            if let Err(err) = sess.send(value.to_string().into_bytes(), 200).await {
                break Err(err);
            }
        }
    }
}

#[async_route]
async fn channel(sess: Session) -> Response {
    let res = sess.recv().await?;
    let peer_data = match deserialize::<SessionModel>(&res) {
        Ok(model) => model,
        Err(res) => return Ok(res),
    };
    let user_id = match session::get_by_token(&peer_data.session_key).await? {
        Some(sess) => sess.user_id,
        None => {
            return Ok(BaseResponse::JsonResponse(
                json!({"status": false, "msg": "会话不存在或已经过期！"}),
                403,
            ))
        }
    };
    sess.send_json(json!({"status": true, "msg": "登录成功"}), 200)
        .await?;

    let pool_arc = Arc::clone(&POOL);
    pool_arc.lock().await.register(user_id.clone());
    let sess = Arc::new(sess);

    let status = Arc::new(Mutex::new(true));
    let sender = tokio::spawn(send_forever(
        user_id.clone(),
        Arc::clone(&status),
        Arc::clone(&sess),
    ));
    loop {
        if let Ok(res) = sess.recv().await {
            let event: Event = match deserialize::<EventModel>(&res) {
                Ok(res) => res.into(),
                Err(res) => {
                    sess.response(res).await?;
                    break;
                }
            };
            println!("event: {}", event);
            match event {
                Event::Close(_) => break,
                Event::MessageCreate(_) => {}
                Event::Unknown(_) => {
                    sess.send_json(
                        json!({"status": false, "msg": "客户端推送了未知事件！"}),
                        403,
                    )
                    .await?
                }
            }
        }
    }
    sender.abort();
    pool_arc.lock().await.unregister(user_id);

    Ok(BaseResponse::JsonResponse(
        json!({"status": false, "msg": "会话结束"}),
        200,
    ))
}
