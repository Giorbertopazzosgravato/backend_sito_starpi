use std::env::current_dir;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path};
use crate::server_utils::database::Database;
use crate::server_utils::file_handler::{FileHandler, HttpResponseDescriptor};
use crate::server_utils::news_letter_substription::NewsLetterSub;
use crate::server_utils::post_request_data::PostRequestData;

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

                stream.read(&mut buffer).unwrap_or(0);
                let request_string = String::from_utf8_lossy(&buffer);
                println!("request: {:?}", request_string);
                let lines = request_string.split(" ").collect::<Vec<_>>();
                let response = Self::handle_request(lines, database).await;
                println!("response: {:?}", String::from_utf8_lossy(&response));
                stream.write_all(&response).unwrap();
            });
        }
    }
    async fn handle_request(body: Vec<&str>, database: Database) -> Vec<u8>{
        let request_type = body.get(0);
        if let Some(request_type) = request_type{
            match request_type{
                &"GET" => {Self::handle_get_request(database, body.get(1)).await}
                &"POST" => {
                    println!("{:?}", body);
                    Self::handle_post_requests(body.get(body.len()-1), body.get(1), database).await }
                &_ => {
                    HttpResponseDescriptor{
                        content: "what the fucking kind of protocol is this".to_string().into_bytes(),
                        content_type: "text/html",
                        code: crate::server_utils::file_handler::HttpCodes::PermissionDenied,
                        cookies: None,
                    }.build_http_response()
                }
            }
        } else{
            HttpResponseDescriptor{
                content: "what the fucking kind of request is this".to_string().into_bytes(),
                content_type: "text/html",
                code: crate::server_utils::file_handler::HttpCodes::PermissionDenied,
                cookies: None,
            }.build_http_response()
        }
    }
    async fn handle_post_requests(data: Option<&&str>, path: Option<&&str>, database: Database) -> Vec<u8>{
        if let Some(path) = path{
            match path{
                &"/IscrizioneGiornale" => { // change this shit
                    match NewsLetterSub::new(&data){
                        None => {
                            println!("error creating user {:?}", data);
                        }
                        Some(user) => {
                            tokio::spawn(async move{
                                user.write_on_file("/usr/src/app/data/utenti_newsletter.txt").await;
                                println!("wrote on file");
                            });
                        }
                    }
                    HttpResponseDescriptor{
                        content: vec![],
                        content_type: "/",
                        code: crate::server_utils::file_handler::HttpCodes::SeeOtherLocation,
                        cookies: None,
                    }.build_http_response()
                }
                &"/troncami_dentro" => {
                    // zstd\r\n\r\nemail=nigga%40gmail.com&pwd=nigga\0\0\0\0\0...
                    let data = PostRequestData::new(data.unwrap_or(&""));
                    database.login(
                        data.data.get("email").unwrap_or(&"".to_string()).to_string(),
                        data.data.get("pwd").unwrap_or(&"".to_string()).to_string()
                    ).await.build_http_response()
                }
                &_ => {
                    println!("what is this diddy blud asking me");
                    HttpResponseDescriptor{
                        content: vec![],
                        content_type: "/",
                        code: crate::server_utils::file_handler::HttpCodes::SeeOtherLocation,
                        cookies: None,
                    }.build_http_response()
                }
            }
        } else{
            println!("no path");
            HttpResponseDescriptor{
                content: vec![],
                content_type: "/",
                code: crate::server_utils::file_handler::HttpCodes::SeeOtherLocation,
                cookies: None,
            }.build_http_response()
        }
        
        // il server non gestisce piu' i file statici 😭😭😭

    }
    async fn handle_get_request(database: Database, request: Option<&&str>) -> Vec<u8>{
        match request{
            None => { HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned() }
            Some(line) => {
                let line = *line;
                if line.starts_with("/database/"){
                    database.get(line).await.build_http_response()
                } else if line.starts_with("/AreaPrivata/"){
                    database.login("gino".to_string(), "pino".to_string()).await.build_http_response()
                } else{
                    HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned()
                }
            }
        }
    }
}