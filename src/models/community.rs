use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewCommunityModel {
    pub(crate) session_key: String,
    pub(crate) name: String,
    pub(crate) security_level: i32,
    pub(crate) token: String,
    pub(crate) cross_origin: bool,
}
