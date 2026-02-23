use std::{fs, io};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const HTTP_BAD_REQUEST_DEFAULT_MESSAGE: &str = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: 71\r\n{\"error\": \"Bad request\",\"message\": \"Request body could not be read properly.\",}";
pub struct Server{
    listener: TcpListener,
    base_path: PathBuf,
}
impl Server{
    pub fn new(addr: &str) -> io::Result<Self>{
        let listener = TcpListener::bind(addr)?;
        Ok(Self{
            listener,
            base_path: Path::new("./dist/").to_owned(),
        })
    }
    pub fn start(&mut self){
        // Todo: make a thread pool, for now let's just thank god we have this
        for stream in self.listener.incoming(){
            let mut stream= stream.unwrap();
            let mut buffer: [u8; 1024] = [0; 1024];

            stream.read(&mut buffer).expect("zamn");
            let request_string = String::from_utf8_lossy(&buffer);
            let lines = request_string.split(" ").collect::<Vec<_>>();
            println!("{:?}", lines);
            let response = Self::handle_get_request(self, lines.get(1));
            stream.write_all(&response).unwrap();

        }
    }
    fn handle_get_request(&self, request: Option<&&str>) -> Vec<u8>{
        match request{
            None => { HTTP_BAD_REQUEST_DEFAULT_MESSAGE.as_bytes().to_owned() }
            Some(line) => {
                let line = *line;
                match if line == "/"{ self.get_file_content("index.html") } else{ self.get_file_content(line.trim_start_matches("/")) }
                {
                    Ok(response ) => {
                        let mut final_response = format!("{HTTP_OK}\r\nContent-Length:{}\r\n\r\n", response.len()).into_bytes();
                        final_response.extend(response);
                        final_response
                    }
                    Err(response ) => {
                        let mut final_response = format!("{HTTP_BAD_REQUEST}\r\nContent-Length:{}\r\n\r\n", response.len()).into_bytes();
                        final_response.extend(response);
                        final_response
                    }
                }
            }
        }
    }
    fn get_file_content<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, Vec<u8>>{
        if let Ok(resolved_path) =  self.is_path_safe(path) {
            println!("path: {:?}", resolved_path);
            match fs::File::open(resolved_path){
                Ok(mut file_content) => {
                    let mut buffer = vec![];
                    let bytes_read = file_content.read_to_end(&mut buffer).unwrap_or(0);
                    if bytes_read > 0{ Ok(buffer) } else{ println!("fuck"); Err("fuck".as_bytes().to_owned()) }
                }
                Err(_) => { Err(Self::get_error_page()) }
            }
        } else {
            Err(Self::get_error_page())
        }
    }
    fn get_error_page()->Vec<u8>{
        match fs::read_to_string("/dist/404.html"){
            Ok(file_content) => {file_content.into()}
            Err(_) => {
                "<h1>server so broken I couldn't even find a 404 page</h1>".as_bytes().to_owned()
            }
        }
    }
    fn is_path_safe<P: AsRef<Path>>(&self, user_input: P) -> Result<PathBuf, bool> {
        let combined_path = self.base_path.join(user_input);
        println!("{:?}", combined_path);
        let resolved_path = match combined_path.canonicalize() {
            Ok(path) => path,
            Err(_) => return Err(false), // Path doesn't exist or permission denied
        };
        let canonical_base = match self.base_path.canonicalize() {
            Ok(path) => path,
            Err(_) => return Err(false),
        };

        if resolved_path.starts_with(canonical_base){ Ok(resolved_path) } else { Err(true) }
    }
}