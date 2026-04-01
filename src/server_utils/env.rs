use std::collections::HashMap;
use std::{env, fs};

pub struct EnvGetter;
pub struct DatabaseVariables{
    pub username: String,
    pub password: String,
    pub database_url: String,
    pub database_port: String,
    pub database_name: String,
}
impl EnvGetter{
    pub fn get_environment_variables_from_file(path_to_file: &str) -> anyhow::Result<HashMap<String, String>>{
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
    pub fn get_environment_variables() -> anyhow::Result<DatabaseVariables>{
        let username = env::var("username").expect("Variabile username mancante");
        let password = env::var("password").expect("Variabile password mancante");
        let database_url = env::var("database_url").expect("Variabile database_url mancante");
        let database_port = env::var("database_port").unwrap_or_else(|_| "5432".to_string());
        let database_name = env::var("database_name").expect("Variabile database_name mancante");
        Ok(DatabaseVariables{
            username,
            password,
            database_url,
            database_port,
            database_name,
        })
    }

}