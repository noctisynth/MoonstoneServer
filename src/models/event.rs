#![allow(dead_code)]
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(crate) enum Event {
    MessageCreate(Value),
    Close(Value),
    Unknown(Value),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::MessageCreate(value) => f.write_str(&value.to_string()),
            Event::Close(value) => f.write_str(&value.to_string()),
            Event::Unknown(value) => f.write_str(&value.to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct EventModel {
    op: String,
    pub(crate) value: Value,
}

impl Into<Event> for EventModel {
    fn into(self) -> Event {
        match self.op.as_str() {
            "message/create" => Event::MessageCreate(self.value),
            "close" => Event::Close(self.value),
            _ => Event::Unknown(self.value),
        }
    }
}
