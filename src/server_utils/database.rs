use std::str::FromStr;
use sqlx::{Pool, Postgres, Row};
use crate::server_utils::env::EnvGetter;
use crate::server_utils::file_handler::{HttpCodes, HttpResponseDescriptor};

#[derive(Clone)]
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
    pub(crate) async fn get(&self, line: &str) -> HttpResponseDescriptor {
        let line = line.strip_prefix("/database/").unwrap_or("");
        match self.get_from_db(line).await{
            Ok(response) => {
                HttpResponseDescriptor{
                    content: response,
                    content_type: "text/json",
                    code: HttpCodes::Ok,
                }
            }
            Err(err) => { HttpResponseDescriptor{
                content: err,
                content_type: "text/json",
                code: HttpCodes::FileNotFound,
            } }
        }
    }
    pub async fn get_from_db(&self, request: &str) -> Result<Vec<u8>, Vec<u8>>{
        println!("request: {}", request);
        let request = request.split("/").collect::<Vec<_>>();
        match request[0]{
            "please_server_I_need_this_my_news_is_kinda_homeless"=>{
                match sqlx::query("SELECT COALESCE(
                            json_agg(
                                json_build_object(
                                    'title', news.titolo,
                                    'image', ('/foto/news/'||news.path_immagine),
                                    'link', ('/news/' || news.link)
                                ) ORDER BY news.id DESC
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
                match sqlx::query("
                    SELECT COALESCE(
                        json_agg(
                            json_build_object(
                               'titolo', n.titolo,
                               'descrizione', n.descrizione,
                               'imageURL', ('/foto/news/'||n.path_immagine),
                               'link', ('/news/' || n.link),
                               'data_rilascio', n.data_rilascio,
                               'categoria', c.nome
                            ) ORDER BY  n.data_rilascio DESC
                        )::text, '[]') AS json_string
                        FROM news n
                        JOIN categoria c
                            ON n.categoria_id = c.id;")

                    .fetch_one(&self.connection).await {
                    Ok(result) => {Ok(result.get::<String, &str>("json_string").into_bytes())}
                    Err(error) => {
                        println!("{:?}", error);
                        Err("[{}]".to_string().into_bytes())
                    }
                }
            }
            "send_me_teams" => {
                // qua una cache sarebbe op
                let year = i32::from_str(*request.get(1).unwrap_or(&"2025")).unwrap_or(2025);
                match sqlx::query("SELECT (coalesce(jsonb_agg(dipartimenti_json), '[]'::jsonb))::text AS json_finale
FROM (
         SELECT
             jsonb_build_object(
                     'dipartimento', d.nome,
                     -- Ora restituisce l'intero array di capi invece del solo primo elemento
                     'capo', coalesce(
                                 jsonb_agg(
                                     jsonb_build_object(
                                             'nome', p.nome,
                                             'cognome', p.cognome,
                                             'link', p.link,
                                             'imgURL', ( '/foto/teams/'||$1||'/' || i.fotourl )
                                     )
                                 ) FILTER (WHERE r.nome_ruolo ILIKE '%Chief%' OR r.nome_ruolo ILIKE '%President%'),
                                 '[]'::jsonb
                             ),
                     -- Aggreghiamo tutti gli altri che NON sono capi
                     'persone', coalesce(
                                     jsonb_agg(
                                         jsonb_build_object(
                                                 'nome', p.nome,
                                                 'cognome', p.cognome,
                                                 'link', p.link,
                                                 'imgURL', ( '/foto/teams/'||$1||'/' || i.fotourl )
                                         )
                                     ) FILTER (WHERE r.nome_ruolo NOT ILIKE '%Chief%' AND r.nome_ruolo NOT ILIKE '%President%'),
                                     '[]'::jsonb
                                )
             ) AS dipartimenti_json
         FROM iscrizione i
                  JOIN ruolo r ON i.ruolo = r.id
                  JOIN persona p ON i.id_persona = p.id
                  JOIN dipartimento d ON i.dipartimento = d.id
         WHERE i.anno = $1
         GROUP BY d.nome
     ) sub;").bind(year).fetch_one(&self.connection).await
                {
                    Ok(row) => {
                        println!("row: {:?}", row);
                        Ok(row.get::<String, &str>("json_finale").into_bytes())}
                    Err(error) => {
                        println!("{:?}", error);
                        Err("[]".to_string().into_bytes())}
                }
            }
            _=>{
               Err("peanits".to_string().into_bytes())
            }
        }
    }

}