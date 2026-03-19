use crate::server::Server;

mod server;
mod cached_page;
mod test_server;
pub mod server_utils;

#[tokio::main]
async fn main() {
    Server::new("127.0.0.1:7878")
        .await
        .unwrap()
        .start()
        .await;
}
