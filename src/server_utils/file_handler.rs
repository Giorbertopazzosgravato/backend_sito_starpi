use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const DIST: &str = "dist/";
const FOTO: &str = "foto/";
pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const HTTP_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden";
pub enum HttpCodes{
    Ok = 200,
    PermissionDenied = 403,
    FileNotFound = 404,
}
pub struct FileHandler;
pub struct HttpResponseDescriptor {
    pub content: Vec<u8>,
    pub content_type: &'static str,
    pub code: HttpCodes
}

impl FileHandler{
    pub fn get_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> HttpResponseDescriptor {
        if let Ok(path) = Self::is_path_safe(path) {
            match fs::File::open(&path) {
                Ok(mut file_content) => {
                    let content_type = Self::get_content_type(path.extension());
                    let mut buffer = vec![];
                    let bytes_read = file_content.read_to_end(&mut buffer).unwrap_or(0);
                    if bytes_read > 0 {
                        HttpResponseDescriptor {
                            content: buffer,
                            content_type,
                            code: HttpCodes::Ok,
                        }
                    } else {
                        println!("error reading file: {:?}", path);
                        HttpResponseDescriptor {
                            content: "fuck, couldn't read file".as_bytes().to_owned(),
                            content_type: "text/html",
                            code: HttpCodes::FileNotFound,
                        }
                    }
                }
                Err(what) => {
                    println!("error while opening file: {what}");
                    HttpResponseDescriptor {
                            content: Self::get_error_page(),
                            content_type: "text/html",
                            code: HttpCodes::FileNotFound,
                    }
                }
            }
        } else {
            HttpResponseDescriptor {
                content: Self::get_error_page(), // todo : mandare a fare in culo l'utente
                content_type: "text/html",
                code: HttpCodes::PermissionDenied,
            }
        }
    }
    fn is_path_safe<P: AsRef<Path> + std::fmt::Debug>(user_input: P) -> Result<PathBuf, bool> {
        let user_input = user_input.as_ref();
        let (base_path, combined_path) = if user_input.starts_with(FOTO){
            (
                PathBuf::from(FOTO).canonicalize(),
                PathBuf::from(FOTO).join(user_input.strip_prefix(FOTO).unwrap_or(user_input)).canonicalize()
            )
        } else {
            (
                PathBuf::from(DIST).canonicalize(),
                PathBuf::from(DIST).join(user_input).canonicalize()
            )
        };
        match combined_path {
            Ok(resolved_path) => {
                if let Ok(base_path) = base_path{
                    if resolved_path.starts_with(base_path){ Ok(resolved_path) } else {Err(true)}
                } else{
                    println!("error while looking for base path");
                    Err(false)
                }
            }
            Err(what) => {
                println!("error while looking for user path: {what}");
                Err(false)
            }
        }
    }
    fn get_error_page() -> Vec<u8>{
        match fs::read_to_string("404.html"){
            Ok(file_content) => {file_content.into()}
            Err(what) => {
                println!("error while getting error page: {what}");
                //possibly log the error, what the hell
                "<h1>server so broken I couldn't even find a 404 page</h1>".as_bytes().to_owned()
            }
        }
    }

    fn get_content_type(extension: Option<&OsStr>) -> &'static str {
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
}

impl HttpResponseDescriptor {
    pub fn build_http_response(&self) -> Vec<u8>{
        match self.code{
            HttpCodes::Ok => {
                let mut final_response = format!("{HTTP_OK}\r\nContent-type: {}\r\nContent-Length:{}\r\n\r\n", self.content_type, self.content.len()).into_bytes();
                final_response.extend(&self.content);
                final_response
            }
            HttpCodes::PermissionDenied => {
                let mut final_response = format!("{HTTP_FORBIDDEN}\r\nContent-type: {}\r\nContent-Length:{}\r\n\r\n", self.content_type, self.content.len()).into_bytes();
                final_response.extend(&self.content);
                final_response
            }
            HttpCodes::FileNotFound => {
                let mut final_response = format!("{HTTP_BAD_REQUEST}\r\nContent-Length:{}\r\n\r\n", self.content.len()).into_bytes();
                final_response.extend(&self.content);
                final_response
            }
        }
    }
}