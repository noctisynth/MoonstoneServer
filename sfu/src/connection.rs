#![allow(dead_code)]
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct Pool {
    senders: HashMap<String, UnboundedSender<Value>>,
    receivers: HashMap<String, UnboundedReceiver<Value>>,
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            senders: Default::default(),
            receivers: Default::default(),
        }
    }
}

impl Pool {
    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
            receivers: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: String) {
        let (tx, rx) = mpsc::unbounded_channel::<Value>();
        self.senders.insert(id.clone(), tx);
        self.receivers.insert(id, rx);
    }

    pub fn unregister(&mut self, id: String) {
        self.receivers.remove(&id);
        self.senders.remove(&id);
    }

    pub fn send(&mut self, id: String, value: Value) -> Result<()> {
        let tx = match self.get_sender(id) {
            Some(tx) => tx,
            None => return Err(anyhow!("会话不存在！")),
        };
        tx.send(value)?;
        Ok(())
    }

    pub fn get_sender(&self, id: String) -> Option<&UnboundedSender<Value>> {
        self.senders.get(&id)
    }

    pub fn get_receiver(&mut self, id: String) -> Option<&mut UnboundedReceiver<Value>> {
        self.receivers.get_mut(&id)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use serde_json::json;

    #[tokio::test]
    async fn pool() -> Result<()> {
        let mut pool = Pool::new();
        pool.register("12".to_string());
        let tx = pool.get_sender("12".to_string()).unwrap();
        tx.send(json!({}))?;
        let rx = pool.get_receiver("12".to_string()).unwrap();
        let data = rx.recv().await.unwrap();
        assert_eq!(data, json!({}));
        Ok(())
    }
}
