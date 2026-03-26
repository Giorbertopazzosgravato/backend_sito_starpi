use std::fs;
use std::path::Path;

const DIST: &str = "dist/";
const FOTO: &str = "foto/";

pub struct FileHandler;
impl FileHandler{
    pub fn get_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Vec<u8>, Vec<u8>>{
        if let Some(path) = is_path_safe(path){
            match fs::File::open(&path){
                Ok(mut file_content) => {
                    let content_type = Self::get_content_type(path.extension());
                    let mut buffer = vec![];
                    let bytes_read = file_content.read_to_end(&mut buffer).unwrap_or(0);
                    if bytes_read > 0{ Ok((buffer, content_type)) } else{ println!("fuck"); Err("fuck".as_bytes().to_owned()) }
                }
                Err(_) => { Err(Self::get_error_page()) }
            }
        } else{
            get_404_page()
        }


        Ok("ciao".to_string().into_bytes())
    }
}