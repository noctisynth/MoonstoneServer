pub mod exceptions;
pub mod utils;
pub mod views;
pub mod handlers {
    pub mod account;
    pub mod community;
    pub mod session;
}
pub mod models;

use std::sync::Arc;

use crate::views::account::{profile_handler, register_handler};
use crate::views::channel::channel;
// use crate::views::community::{
//     get_message_handler, join_community_handler, new_community_handler, new_message_handler,
// };
use crate::views::session::{login_handler, session_handler};
use anyhow::Result;
use moonstone_db::init;
use oblivion::models::router::Router;
use oblivion::models::server::Server;
use oblivion::path_route;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use utils::connection::Pool;

pub(crate) static POOL: Lazy<Arc<Mutex<Pool>>> =
    Lazy::new(|| Arc::new(Mutex::new(Pool::default())));

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;

    let mut router = Router::new();

    path_route!(router, "/channel" => channel);

    path_route!(router, "/account/new" => register_handler);
    path_route!(router, "/account/profile" => profile_handler);

    path_route!(router, "/session/new" => login_handler);
    path_route!(router, "/session/alive" => session_handler);

    // path_route!(router, "/community/new" => new_community_handler);

    // path_route!(router, "/member/new" => join_community_handler);

    // path_route!(router, "/message/new" => new_message_handler);
    // path_route!(router, "/message/delete" => new_message_handler);
    // path_route!(router, "/message/get" => get_message_handler);

    let server = Server::new("0.0.0.0", 7076, router);
    server.run().await?;

    Ok(())
}
