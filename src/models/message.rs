use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MessageModel {
    pub(crate) session_key: String,
    pub(crate) text: String,
    pub(crate) dist: String,
}
