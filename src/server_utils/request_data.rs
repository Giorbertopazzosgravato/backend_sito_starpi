use std::collections::HashMap;
use httparse::Header;
use serde::Deserialize;

#[derive(Debug)]
pub struct Cookies {
    pub data: HashMap<String, String>,
}
impl Cookies {
    pub fn new(cookies: Vec<&Header>) -> Self {
        let mut ret = Self{ data: HashMap::new() };
        for cookie in cookies{
            let pair = String::from_utf8_lossy(cookie.value);
            let pair = pair
                .split("=")
                .collect::<Vec<_>>();
            if pair.len() == 2 && let Some(key) = pair.get(0) && let Some(value) = pair.get(1){
                ret.data.insert(key.to_string(), value.to_string());
            }
        }
        ret
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginData{
    pub email: Option<String>,
    pub pwd: Option<String>,
}
