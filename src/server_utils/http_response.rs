use std::ffi::OsStr;
use std::fmt::format;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::server_utils::http_response::CookieOptions::{Domain, MaxAge};

const AREA_PRIVATA: &str = "/private/";
pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const HTTP_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden";
pub const HTTP_SEE_OTHER_LOCATIONS: &str = "HTTP/1.1 303 See Other";
pub enum HttpCodes{
    Ok = 200,
    SeeOtherLocation = 303,
    PermissionDenied = 403,
    FileNotFound = 404,
    PrivateResponseOkAdmin = 9999,
    PrivateResponseOkUser = 9998,
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
pub struct Cookie{
    pub name: String,
    pub value: String,
    pub options: Option<String>
}
pub struct HttpResponseDescriptor {
    pub content: Vec<u8>,
    pub content_type: &'static str,
    pub code: HttpCodes,
    pub cookies: Option<Vec<Cookie>>
}

impl HttpResponseDescriptor {
    fn build_cookies(&self) -> String {
        let mut cookies_vec = vec![];
        if let Some(cookies) = &self.cookies {
            for cookie in cookies {
                cookies_vec.push(
                    format!(
                        "Set-Cookie: {}={}; {}\r\n",
                        cookie.name,
                        cookie.value,
                        cookie.options.as_ref().unwrap_or(&"".to_string())
                    ));
            }
        }
        let mut cookie_string = "".to_string();
        for cookie in cookies_vec {
            cookie_string += &cookie;
        }
        cookie_string
    }
    pub fn build_http_response(&self) -> Vec<u8> {
        match self.code {
            HttpCodes::Ok => {
                let cookie_string = self.build_cookies();
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
                format!("{HTTP_SEE_OTHER_LOCATIONS}\r\nLocation: {}\r\n{}\r\n", self.content_type, self.build_cookies()).into_bytes()
            }
            HttpCodes::PrivateResponseOkAdmin => {
                "HTTP/1.1 200 OK\r\nX-Accel-Redirect: /admin/index.html\r\nContent-Length: 0\r\n\r\n".as_bytes().to_owned()
            }
            HttpCodes::PrivateResponseOkUser => {
                "HTTP/1.1 200 OK\r\nX-Accel-Redirect: /utente/index.html\r\nContent-Length: 0\r\n\r\n".as_bytes().to_owned()
            }
        }
    }
}