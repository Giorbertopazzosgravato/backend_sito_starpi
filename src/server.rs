use std::env::current_dir;
use std::io::{Read, Write};
use std::net::TcpListener;
use httparse::Header;
use crate::server_utils::database::{Database, UuidTypes};
use crate::server_utils::http_response::{BuildHttpResponse, HttpCodes, HttpResponseDescriptor, PrivateHttpCodes, PrivateResponseDescriptor};
use crate::server_utils::news_letter_substription::NewsLetterSub;
use crate::server_utils::request_data::{Cookies, LoginData};

pub const HTTP_BAD_REQUEST_DEFAULT_MESSAGE: &str = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: 71\r\n{\"error\": \"Bad request\",\"message\": \"Request body could not be read properly.\",}";
pub struct Server{
    listener: TcpListener,
    db: Database,
}
impl Server{
    pub async fn new(addr: &str) -> anyhow::Result<Self>{
        println!("{:?}", current_dir());
        let listener = TcpListener::bind(addr)?;
        let database = Database::new().await?;
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
                let mut headers = [httparse::EMPTY_HEADER; 64];
                let mut parsed = httparse::Request::new(&mut headers);
                let status = parsed.parse(request_string.as_bytes());
                println!("status: {:?}", status);
                println!("parsed: {:?}", parsed.headers);
                println!("parsed: {:?}", parsed.path);
                println!("parsed: {:?}", parsed.method);
                println!("request: {:?}", request_string);
                let body =
                    if let Ok(status) = status &&
                        let Some(len_header) = parsed.headers.iter().find(|h| h.name=="Content-Length"){
                    let len: usize = std::str::from_utf8(len_header.value).unwrap_or("0").parse().unwrap_or(0usize);
                        &request_string[0..len]
                } else { "" };
                let response = Self::handle_request(parsed.method, parsed.path, Vec::from(parsed.headers), body, database).await;
                // println!("response: {:?}", String::from_utf8_lossy(&response));
                stream.write_all(&response).unwrap();
            });
            let db = self.db.clone();
            // famo partire il thread che ogni 12 ore pulisce le sessioni attive :3
            tokio::spawn(async move{
                db.pulisci_sessioni().await;
            });
        }
    }
    async fn handle_request(request_type: Option<&str>, path: Option<&str>, headers: Vec<Header<'_>>, body: &str, database: Database) -> Vec<u8>{
        if let Some(request_type) = request_type{
            match request_type{
                "GET" => {Self::handle_get_request(database, path, headers).await}
                "POST" => {
                    Self::handle_post_requests(headers, path, body, database).await }
                &_ => {
                    HttpResponseDescriptor{
                        content: "what the fucking kind of protocol is this".to_string().into_bytes(),
                        content_type: "text/html",
                        code: HttpCodes::PermissionDenied,
                        cookies: None,
                    }.build_http_response()
                }
            }
        } else{
            HttpResponseDescriptor{
                content: "what the fucking kind of request is this".to_string().into_bytes(),
                content_type: "text/html",
                code: HttpCodes::PermissionDenied,
                cookies: None,
            }.build_http_response()
        }
    }
    async fn handle_post_requests(headers: Vec<Header<'_>>, path: Option<&str>, body: &str, database: Database) -> Vec<u8>{
        if let Some(path) = path{
            match path{
                "/IscrizioneGiornale" => { // change this shit
                    NewsLetterSub::new(&body, headers.iter().find(|h| h.name == "Refer"), &database)
                        .await
                        .build_http_response()
                }
                "/troncami_dentro" => {
                    // zstd\r\n\r\nemail=nigga%40gmail.com&pwd=nigga\0\0\0\0\0...
                    let data = serde_urlencoded::from_str(body).unwrap_or(LoginData { email: None, pwd: None });
                    database.login(
                        data.email.unwrap_or("".to_string()).to_string(),
                        data.pwd.unwrap_or("".to_string()).to_string()
                    ).await.build_http_response()
                }
                &_ => {
                    println!("what is this diddy blud asking me");
                    HttpResponseDescriptor{
                        content: vec![],
                        content_type: "/",
                        code: HttpCodes::SeeOtherLocation,
                        cookies: None,
                    }.build_http_response()
                }
            }
        } else{
            println!("no path");
            HttpResponseDescriptor{
                content: vec![],
                content_type: "/",
                code: HttpCodes::SeeOtherLocation,
                cookies: None,
            }.build_http_response()
        }
    }
    async fn handle_get_request(database: Database, path: Option<&str>, headers: Vec<Header<'_>>) -> Vec<u8>{
        match path {
            None => { HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned() }
            Some(line) => {
                if line.starts_with("/database/"){
                    database.get(line).await.build_http_response()
                }
                else if line.starts_with("/AreaPrivata/"){
                    let path_requested = line.trim_start_matches("/AreaPrivata/");
                    let cookies = headers.iter().rev().filter(|h| h.name == "Cookie").collect::<Vec<_>>();
                    let cookies = Cookies::new(cookies);
                    println!("cookies (handle get request): {:?}", cookies);
                    match database.is_uuid_valid(cookies.data.get("uuid")).await{
                        UuidTypes::NotValid => {
                            HttpResponseDescriptor{
                                content: vec![],
                                content_type: "/login/",
                                code: HttpCodes::SeeOtherLocation,
                                cookies: None,
                            }.build_http_response()
                        }
                        UuidTypes::Admin => {
                            let path = if path_requested != "" {
                                path_requested.to_string()
                            } else {
                                "index.html".to_string()
                            };

                            PrivateResponseDescriptor{
                                path,
                                code: PrivateHttpCodes::PrivateResponseOkAdmin,
                            }.build_http_response()
                        }
                        UuidTypes::User => {
                            let path = if path_requested != "" {
                                path_requested.to_string()
                            } else {
                                "index.html".to_string()
                            };
                            PrivateResponseDescriptor{
                                path,
                                code: PrivateHttpCodes::PrivateResponseOkUser,
                            }.build_http_response()
                        }
                    }
                } else{
                    HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned()
                }
            }
        }
    }
}