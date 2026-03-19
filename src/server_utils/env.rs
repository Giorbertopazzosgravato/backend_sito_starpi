use std::collections::HashMap;
use std::fs;

pub struct EnvGetter;
impl EnvGetter{
    pub fn get_environment_variables(path_to_file: &str) -> anyhow::Result<HashMap<String, String>>{
        let file_content = fs::read_to_string(path_to_file)?;
        let file_content = file_content.split("\n").collect::<Vec<_>>();
        let mut hm = HashMap::new();
        for value in file_content{
            let key_val = value.split("=").collect::<Vec<_>>();
            if let Some(key) = key_val.get(0){
                if let Some(value) = key_val.get(1){
                    let key = key.replace(" ", "");
                    let value = value.replace(" ", "");
                    hm.insert(key, value);
                }
            }
        }
        Ok(hm)
    }
}