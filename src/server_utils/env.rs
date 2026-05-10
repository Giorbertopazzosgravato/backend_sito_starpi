use std::{env};

pub struct EnvGetter;
pub struct DatabaseVariables{
    pub username: String,
    pub password: String,
    pub database_url: String,
    pub database_port: String,
    pub database_name: String,
}
impl EnvGetter{
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