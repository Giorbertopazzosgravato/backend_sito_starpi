use std::collections::HashMap;

pub struct PostRequestData{
    pub data: HashMap<String, String>,
}
impl PostRequestData{
    
    pub fn new(data: &str) -> Self {
        let mut post = PostRequestData{data: HashMap::new()};
        let new_data = data
            .trim_end_matches("\0")
            .split("\r\n\r\n")
            .collect::<Vec<_>>()
            .get(1)
            .unwrap_or(&"")
            .split("&")
            .collect::<Vec<_>>();
        for entry in new_data {
            let splitted = entry.split("=").collect::<Vec<_>>();
            if splitted.len() >= 2 {
                let first_entry =splitted.get(0).unwrap();
                let data = splitted.get(1).unwrap().replace("%40", "@").to_string();
                    post.data.insert(first_entry.to_string(), data.to_string());
                }
            }
        post
    }
}