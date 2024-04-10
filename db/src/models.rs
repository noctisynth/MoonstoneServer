use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize)]
pub(crate) struct AccessibilityModel<'a> {
    pub name: &'a str,
    pub level: i32,
    pub desc: &'a str,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct Accessibility {
//     pub id: Thing,
//     pub name: String,
//     pub level: i32,
//     pub desc: String,
//     pub create_at: Datetime,
//     pub update_at: Datetime,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountProfile {
    pub nickname: String,
    pub sex: bool,
    pub country: Option<String>,
    pub favorites_icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AccountModel<'a> {
    pub sequence: &'a str,
    pub username: &'a str,
    pub tuta_mail: &'a str,
    pub password: &'a str,
    pub accessibility: i32,
    pub profile: AccountProfile,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Thing,
    pub sequence: String,
    pub username: String,
    pub tuta_mail: String,
    pub password: String,
    pub accessibility: i32,
    pub profile: AccountProfile,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize)]
pub(crate) struct SessionModel<'a> {
    pub token: &'a str,
    pub user_id: &'a str,
    pub device_id: &'a str,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Deserialize)]
pub struct Session {
    pub id: Thing,
    pub token: String,
    pub user_id: String,
    pub device_id: String,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize)]
pub(crate) struct CommunityModel<'a> {
    pub name: &'a str,
    pub user_id: &'a str,
    pub security_level: i32,
    pub token: Option<&'a str>,
    pub cross_origin: bool,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Community {
    pub id: Thing,
    pub name: String,
    pub user_id: String,
    pub security_level: i32,
    pub token: Option<String>,
    pub cross_origin: bool,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize)]
pub(crate) struct MemberModel<'a> {
    pub node: &'a str,
    pub community_id: &'a str,
    pub user_id: &'a str,
    pub permissions: Vec<&'a str>,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Member {
    pub id: Thing,
    pub node: String,
    pub community_id: String,
    pub user_id: String,
    pub permissions: Vec<String>,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize)]
pub(crate) struct MessageModel<'a> {
    pub community_id: &'a str,
    pub node: &'a str,
    pub user_id: &'a str,
    pub text: &'a str,
    pub undelivered: Vec<String>,
    pub create_at: Datetime,
    pub update_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Thing,
    pub community_id: String,
    pub node: String,
    pub user_id: String,
    pub text: String,
    pub undelivered: Vec<String>,
    pub create_at: Datetime,
    pub update_at: Datetime,
}
