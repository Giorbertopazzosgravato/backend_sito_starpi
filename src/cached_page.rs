use crate::server::HTTP_OK;

pub struct CachedPage{
    pub length: usize,
    pub content: Vec<u8>,
    pub file_type: String,
}
impl CachedPage{
    pub fn build_response(&self) -> Vec<u8> {
        let mut ret = format!("{HTTP_OK}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", self.file_type ,self.length, ).into_bytes();
        ret.extend(&self.content);
        ret

    }
}