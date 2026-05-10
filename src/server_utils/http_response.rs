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
pub enum PrivateHttpCodes{
    PrivateResponseOkAdmin,
    PrivateResponseOkUser,
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
pub struct PrivateResponseDescriptor{
    pub path: String,
    pub code: PrivateHttpCodes,
}

pub trait BuildHttpResponse{
    fn build_http_response(&self) -> Vec<u8>;
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
}
impl BuildHttpResponse for HttpResponseDescriptor{
    fn build_http_response(&self) -> Vec<u8> {
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
                format!("{HTTP_SEE_OTHER_LOCATIONS}\r\nLocation: {}\r\n{}\r\n", String::from_utf8_lossy(&self.content), self.build_cookies()).into_bytes()
            }
        }
    }
}
impl BuildHttpResponse for PrivateResponseDescriptor {
    fn build_http_response(&self) -> Vec<u8> {
        match self.code {
            PrivateHttpCodes::PrivateResponseOkAdmin => {
                format!("HTTP/1.1 200 OK\r\nX-Accel-Redirect: /admin/{}\r\nContent-Length: 0\r\n\r\n", self.path).as_bytes().to_owned()
            }
            PrivateHttpCodes::PrivateResponseOkUser => {
                format!("HTTP/1.1 200 OK\r\nX-Accel-Redirect: /user/{}\r\nContent-Length: 0\r\n\r\n", self.path).as_bytes().to_owned()
            }
        }
    }
}