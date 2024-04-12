use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewCommunityModel {
    pub(crate) session_key: String,
    pub(crate) name: String,
    pub(crate) security_level: i32,
    pub(crate) token: String,
    pub(crate) cross_origin: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JoinCommunityModel {
    pub(crate) token: String,
    pub(crate) node: String,
    pub(crate) user_id: String,
    pub(crate) community_id: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MessageModel {
    pub(crate) token: String,
    pub(crate) node: String,
    pub(crate) community_id: String,
    pub(crate) message_id: String,
    pub(crate) text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GetAllMessagesModel {
    pub(crate) token: String,
}