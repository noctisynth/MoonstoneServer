use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RegisterModel {
    pub(crate) username: String,
    pub(crate) tuta_mail: String,
    pub(crate) password: String,
    pub(crate) nickname: String,
}
