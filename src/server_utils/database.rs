use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::PgRow;
use crate::server_utils::env::EnvGetter;

pub struct Database{
    pub connection: Pool<Postgres>
}
impl Database{
    pub async fn new(env: &str)->anyhow::Result<Self>{
        let env = EnvGetter::get_environment_variables(env)?;
        let connection = sqlx::postgres::PgPool::connect(
            &format!("postgres://{}:{}@{}:{}/{}",
                    env.get("username").unwrap(),
                    env.get("password").unwrap(),
                    env.get("database_url").unwrap(),
                    env.get("database_port").unwrap(),
                    env.get("database_name").unwrap()
            ))
            .await?;
        Ok(Self{
            connection,
        })
    }
    pub async fn get_from_db(&mut self, request: &str) -> Result<Vec<u8>, Vec<u8>>{
        match request{
            "please_server_I_need_this_my_news_is_kinda_homeless"=>{
                match sqlx::query("SELECT COALESCE(
                            json_agg(
                                json_build_object(
                                    'title', news.titolo,
                                    'image', news.path_immagine
                                ) ORDER BY news.id
                        )::text,
                        '[]'
       ) as json_string FROM news").fetch_one(&self.connection).await {
                    Ok(result) => {Ok(result.get::<String, &str>("json_string").into_bytes())}
                    Err(error) => {
                        println!("{:?}", error);
                        Err("[{}]".to_string().into_bytes())
                    }
                }
            }
            "please_server_send_me_newds"=>{
                match sqlx::query("SELECT COALESCE(
               json_agg(
                       json_build_object(
                               'titolo', n.titolo,
                               'descrizione', n.descrizione,
                               'imageURL', n.path_immagine,
                               'categoria', c.nome
                       ) ORDER BY  n.id
               )::text,
               '[]'
       ) AS json_string
FROM news n
         JOIN category c ON n.categoria_id = c.id;").fetch_one(&self.connection).await {
                    Ok(result) => {Ok(result.get::<String, &str>("json_string").into_bytes())}
                    Err(error) => {
                        println!("{:?}", error);
                        Err("[{}]".to_string().into_bytes())
                    }
                }
            }
            _=>{
               Err("peanits".to_string().into_bytes())
            }
        }
    }

}