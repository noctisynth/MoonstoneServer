use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LoginModel {
    pub(crate) identity: String,
    pub(crate) password: String,
    pub(crate) unique_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SessionModel {
    pub(crate) session_key: String,
}
