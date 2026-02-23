use crate::server::Server;

mod server;
mod cached_page;
mod test_server;

fn main() {
    Server::new("127.0.0.1:7878")
        .unwrap()
        .start();
    // test_server::TestServer::start()
}
