extern crate redis;
pub mod exceptions;
pub mod settings;
pub mod views {
    pub mod account;
    pub mod community;
    pub mod message;
    pub mod session;
}
pub mod utils {
    pub mod email;
    pub mod model;
    pub mod password;
    pub mod sequence;
    pub mod session;
}
pub mod handlers {
    pub mod account;
    pub mod community;
    pub mod message;
    pub mod session;
}
pub mod models {
    pub mod account;
    pub mod community;
    pub mod message;
    pub mod session;
}

use oblivion::models::router::Router;
use oblivion::models::server::Server;
use oblivion::path_route;
use views::account::{account_handler, register_handler};
use views::session::{login_handler, session_handler};

use crate::views::community::new_community_handler;

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    path_route!(&mut router, "/account/new" => register_handler);
    path_route!(&mut router, "/account/info" => account_handler);

    path_route!(&mut router, "/session/new" => login_handler);
    path_route!(&mut router, "/session/alive" => session_handler);

    path_route!(&mut router, "/community/new" => new_community_handler);

    let mut server = Server::new("0.0.0.0", 7076, router);
    server.run().await;
}
