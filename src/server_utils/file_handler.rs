use std::ffi::OsStr;
use std::fmt::format;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::server_utils::file_handler::CookieOptions::{Domain, MaxAge};

const DIST: &str = "dist/";
const FOTO: &str = "foto/";
const NEWS: &str = "news/";
pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const HTTP_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden";
pub const HTTP_SEE_OTHER_LOCATIONS: &str = "HTTP/1.1 303 See Other";
pub enum HttpCodes{
    Ok = 200,
    SeeOtherLocation = 303,
    PermissionDenied = 403,
    FileNotFound = 404,
}
pub enum CookieOptions{
    Domain (String),
    Expires(Date),
    HttpOnly,
    MaxAge(u128),
    Partitioned,
    Path(String),
    Secure,
    SameSiteStrict,
    SameSiteLax,
    SameSiteNone,
}
pub struct Date{
    day: u8,
    month: String, // Jan, Feb, Mar...
    year: u32,
}
pub struct FileHandler;
pub struct Cookie{
    name: String,
    value: String,
    options: Option<String>
}
pub struct HttpResponseDescriptor {
    pub content: Vec<u8>,
    pub content_type: &'static str,
    pub code: HttpCodes,
    pub cookies: Option<Vec<Cookie>>
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
                            cookies: None,
                        }
                    } else {
                        println!("error reading file: {:?}", path);
                        HttpResponseDescriptor {
                            content: "fuck, couldn't read file".as_bytes().to_owned(),
                            content_type: "text/html",
                            code: HttpCodes::FileNotFound,
                            cookies: None,
                        }
                    }
                }
                Err(what) => {
                    println!("error while opening file: {what}");
                    HttpResponseDescriptor {
                            content: Self::get_error_page(),
                            content_type: "text/html",
                            code: HttpCodes::FileNotFound,
                        cookies: None,
                    }
                }
            }
        } else {
            HttpResponseDescriptor {
                content: Self::get_error_page(), // todo : mandare a fare in culo l'utente
                content_type: "text/html",
                code: HttpCodes::PermissionDenied,
                cookies: None,
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
        } else if user_input.starts_with(NEWS) {
            (
                PathBuf::from(NEWS).canonicalize(),
                PathBuf::from(NEWS).join(user_input.strip_prefix(NEWS).unwrap_or(user_input)).canonicalize()
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
                println!("error while looking for user path: {:?} {what}", user_input);
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

    pub fn get_area_privata_html() -> HttpResponseDescriptor {
        let mut response = Self::get_file("AreaPrivata.html");
        response.cookies = Some(vec![
            Cookie {
                name: "uuid".to_string(),
                value: Uuid::new_v4().to_string(),
                options: Some("; Domain = starpi.eu; Max-Age = 62400; SameSite=Lax".to_string()),
            }]);
        response
    }
}

impl HttpResponseDescriptor {
    pub fn build_http_response(&self) -> Vec<u8>{
        match self.code{
            HttpCodes::Ok => {
                let mut cookies_vec = vec![];
                if let Some(cookies) = &self.cookies{
                    for cookie in cookies{
                        cookies_vec.push(
                            format!(
                                "Set-Cookie: {}={}{}\r\n",
                                cookie.name,
                                cookie.value,
                                cookie.options.as_ref().unwrap_or(&"".to_string())
                            ));
                    }
                }
                let mut cookie_string = "".to_string();
                for cookie in cookies_vec{
                    cookie_string += &cookie;
                }
                let mut final_response = format!(
                    "{HTTP_OK}\r\nContent-type: {}\r\nContent-Length:{}\r\n{}\r\n",
                    self.content_type,
                    self.content.len(),
                    cookie_string,
                ).into_bytes();
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
            HttpCodes::SeeOtherLocation => {
                format!("{HTTP_SEE_OTHER_LOCATIONS}\r\nLocation: {}\r\n\r\n", self.content_type).into_bytes()
            }
        }
    }
}