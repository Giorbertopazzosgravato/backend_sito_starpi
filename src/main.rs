use crate::server::Server;

mod server;
pub mod server_utils;

#[tokio::main]
async fn main() {
    Server::new("0.0.0.0:7878")
        .await
        .unwrap()
        .start()
        .await;

}
