use std::{fs};
use std::env::current_dir;
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::server_utils::database::Database;
use crate::server_utils::thread_pool::ThreadPool;

pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const HTTP_BAD_REQUEST_DEFAULT_MESSAGE: &str = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: 71\r\n{\"error\": \"Bad request\",\"message\": \"Request body could not be read properly.\",}";
pub struct Server{
    listener: TcpListener,
    db: Database,
    thread_pool: ThreadPool,
    website_path: PathBuf,
    photo_path: PathBuf,
}
impl Server{
    pub async fn new(addr: &str) -> anyhow::Result<Self>{
        println!("{:?}", current_dir());
        let listener = TcpListener::bind(addr)?;
        let database = Database::new("./database/db.env").await?;
        let thread_pool  = ThreadPool::new(10);
        Ok(Self{
            listener,
            website_path: Path::new("./dist/").to_owned(),
            photo_path: Path::new("./foto/").to_owned(),
            db: database,
            thread_pool
        })
    }
    pub async fn start(&mut self){
        while let Ok((mut stream, socket_address)) = self.listener.accept(){
            let db = self.db.clone();
            self.thread_pool.execute( async move ||{
                let database = db;
                let mut buffer: [u8; 1024] = [0; 1024];

                stream.read(&mut buffer).expect("zamn");
                let request_string = String::from_utf8_lossy(&buffer);
                let lines = request_string.split(" ").collect::<Vec<_>>();

                let response = Self::handle_get_request(self, lines.get(1)).await;
                stream.write_all(&response).unwrap();
            });

        }
    }
    async fn handle_get_request(database: Database, request: Option<&&str>) -> Vec<u8>{
        match request{
            None => { HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned() }
            Some(line) => {
                let line = *line;

                match if line == "/"
                || Path::new(line).extension() == None { //cuz spaghetti code is good 👍
                    if line.starts_with("/database/"){
                        database.get_from_database(line).await
                    } else {
                        Self::get_file_content(/* &server::Server */, "index.html")
                    }
                } else {
                    Self::get_file_content(line.trim_start_matches("/").replace("%20", " "))
                }
                {
                    Ok((response, content_type) ) => {
                        let mut final_response = format!("{HTTP_OK}\r\nContent-type: {}\r\nContent-Length:{}\r\n\r\n", content_type, response.len()).into_bytes();
                        final_response.extend(response);
                        final_response
                    }
                    Err(response ) => {
                        println!("errore dio terrone {}", String::from_utf8_lossy(&response));
                        let mut final_response = format!("{HTTP_BAD_REQUEST}\r\nContent-Length:{}\r\n\r\n", response.len()).into_bytes();
                        final_response.extend(response);
                        final_response
                    }
                }
            }
        }
    }
    fn get_file_content<P: AsRef<Path> + std::fmt::Debug>(&self, path: P) -> Result<(Vec<u8>, &'static str), Vec<u8>>{
        if let Ok(resolved_path) =  self.is_path_safe(path) {
            match fs::File::open(&resolved_path){
                Ok(mut file_content) => {
                    let content_type = Self::get_content_type(resolved_path.extension());
                    let mut buffer = vec![];
                    let bytes_read = file_content.read_to_end(&mut buffer).unwrap_or(0);
                    if bytes_read > 0{ Ok((buffer, content_type)) } else{ println!("fuck"); Err("fuck".as_bytes().to_owned()) }
                }
                Err(_) => { Err(Self::get_error_page()) }
            }
        } else {
            Err(Self::get_error_page())
        }
    }
    fn get_content_type(extension: Option<&OsStr>) -> &'static str{
        match extension{
            None => {"*/*"}
            Some(extension) => {
                if let Some(extension) = extension.to_str(){
                    println!("{extension}");
                    match extension{
                        "html" => { "text/html" }
                        "css" => { "text/css" }

                        "js" => { "application/javascript" }

                        "png" => { "image/png" }
                        "jpeg" => { "image/jpeg" }
                        "jpg" => { "image/jpeg" }
                        "webp" => { "image/webp" }
                        "gif" => { "image/webp" }
                        "heic" => { "image/heic" }
                        "heif" => { "image/heif" }
                        &_ => { "*/*" }
                    }
                } else {
                    "*/*"
                }

            }
        }
    }
    fn get_error_page()->Vec<u8>{
        match fs::read_to_string("404.html"){
            Ok(file_content) => {file_content.into()}
            Err(_) => {
                println!("zsdqweweorfinedroiugnseopg");
                //possibly log the error, what the hell
                "<h1>server so broken I couldn't even find a 404 page</h1>".as_bytes().to_owned()
            }
        }
    }
    fn is_path_safe<P: AsRef<Path> + std::fmt::Debug>(&self, user_input: P) -> Result<PathBuf, bool> {
        let user_input = user_input.as_ref();
        let combined_path = if user_input.starts_with("foto/"){
            self.photo_path.join(user_input.strip_prefix("foto/").unwrap_or(user_input))
        } else {
            self.website_path.join(user_input)
        };

        println!("{:?}", combined_path);
        let resolved_path = match combined_path.canonicalize() {
            Ok(path) => path,
            Err(_) => return Err(false), // Path doesn't exist or permission denied
        };
        let website_path = match self.website_path.canonicalize() {
            Ok(path) => path,
            Err(_) => return Err(false),
        };
        let foto_path = match self.photo_path.canonicalize(){
            Ok(path) => path,
            Err(_) => return Err(false),
        };

        if resolved_path.starts_with(website_path) || resolved_path.starts_with(foto_path){ Ok(resolved_path) } else { Err(true) }
    }
    async fn get_from_database(database: &mut Database, line: &str) -> Result<(Vec<u8>, &'static str), Vec<u8>>{
        let line = line.strip_prefix("/database/").unwrap_or("");
        match database.get_from_db(line).await{
            Ok(response) => { Ok((response, "text/json")) }
            Err(err) => { Err(err) }
        }
    }
}