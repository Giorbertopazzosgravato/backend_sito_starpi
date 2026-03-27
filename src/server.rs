use std::env::current_dir;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path};
use crate::server_utils::database::Database;
use crate::server_utils::file_handler::FileHandler;
pub const HTTP_BAD_REQUEST_DEFAULT_MESSAGE: &str = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: 71\r\n{\"error\": \"Bad request\",\"message\": \"Request body could not be read properly.\",}";
pub struct Server{
    listener: TcpListener,
    db: Database,
}
impl Server{
    pub async fn new(addr: &str) -> anyhow::Result<Self>{
        println!("{:?}", current_dir());
        let listener = TcpListener::bind(addr)?;
        let database = Database::new("./database/db.env").await?;
        Ok(Self{
            listener,
            db: database,
        })
    }
    pub async fn start(&mut self){
        while let Ok((mut stream, _socket_address)) = self.listener.accept(){
            let db = self.db.clone();

            tokio::spawn(async move {
                let database = db;
                let mut buffer: [u8; 1024] = [0; 1024];

                stream.read(&mut buffer).expect("zamn");
                let request_string = String::from_utf8_lossy(&buffer);
                let lines = request_string.split(" ").collect::<Vec<_>>();

                let response = Self::handle_get_request(database, lines.get(1)).await;
                stream.write_all(&response).unwrap();
            });
        }
    }
    async fn handle_get_request(database: Database, request: Option<&&str>) -> Vec<u8>{
        match request{
            None => { HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned() }
            Some(line) => {
                let line = *line;

                if line == "/" || Path::new(line).extension() == None { //cuz spaghetti code is good 👍
                    if line.starts_with("/database/"){
                        database.get(line).await.build_http_response()
                    } else {
                        FileHandler::get_file("index.html").build_http_response()
                    }
                } else {
                    FileHandler::get_file(line.trim_start_matches("/").replace("%20", " ")).build_http_response()
                }
            }
        }
    }
}