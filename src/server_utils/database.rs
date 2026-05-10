use std::str::FromStr;
use std::time::Duration;
use sqlx::{query, Error, Pool, Postgres, Row};
use sqlx::postgres::PgQueryResult;
use tokio::time;
use uuid::Uuid;
use crate::server_utils::env::EnvGetter;
use crate::server_utils::http_response::{BuildHttpResponse, Cookie, HttpCodes, HttpResponseDescriptor, PrivateHttpCodes, PrivateResponseDescriptor};

#[derive(Clone)]
pub struct Database{
    pub connection: Pool<Postgres>
}
pub enum UuidTypes{
    NotValid,
    Admin,
    User,
}
impl Database{
    pub async fn new()->anyhow::Result<Self>{
        let env = EnvGetter::get_environment_variables()?;
        let connection = sqlx::postgres::PgPool::connect(
            &format!("postgres://{}:{}@{}:{}/{}",
                    env.username,
                    env.password,
                    env.database_url,
                    env.database_port,
                    env.database_name
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
                    cookies: None,
                }
            }
            Err(err) => { HttpResponseDescriptor{
                content: err,
                content_type: "text/json",
                code: HttpCodes::FileNotFound,
                cookies: None,
            } }
        }
    }
    pub async fn get_from_db(&self, request: &str) -> Result<Vec<u8>, Vec<u8>>{
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
                                             'imgURL', ( '/foto/teams/'||$1||'/' || i.fotourl ),
                                             'quote', i.quote
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
         GROUP BY d.nome, d.id
         ORDER BY d.id DESC
     ) sub;").bind(year).fetch_one(&self.connection).await
                {
                    Ok(row) => {
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
    pub async fn insert_entry_newsletter(&self, email: &str, nome: &str) -> anyhow::Result<bool> {
        let result = sqlx::query("insert into newsLetter values($1, $2)").bind(email).bind(nome).execute(&self.connection).await;
        match result {
            Ok(res) => { if res.rows_affected() == 0 { Ok(false) } else { Ok(true) } }
            Err(error) => { println!("error while inserting person in newsLetter: {:?}", error); Err(anyhow::Error::from(error)) }
        }
    }
    pub async fn login(&self, email: String, password: String) -> Box<dyn BuildHttpResponse> {
        match sqlx::query(
            "with utente_valido as (
    select id, livello
    from credenziali
    where
        email = $1 and
        password = $2
),
nuova_sessione as (
    insert into sessioni (uuid, date_generated, credenziali)
        select gen_random_uuid(), now(), id
           from utente_valido
    on conflict (credenziali) do update
        set uuid = excluded.uuid, date_generated = now()
    returning uuid
)
select livello, uuid from utente_valido, nuova_sessione;"
        )
            .bind(email)
            .bind(password)
            .fetch_one(&self.connection)
            .await {
            Ok(row) => {
                println!("{:?}", row);
                if row.is_empty(){
                    Box::new(HttpResponseDescriptor{
                        content: "wrong email or password".as_bytes().to_owned(),
                        content_type: "text/text",
                        code: HttpCodes::PermissionDenied,
                        cookies: None,
                    })
                } else {
                    let user_level: i32 = row.get("livello");
                    let uuid: Uuid = row.get("uuid");
                    if user_level == 1{
                        Box::new(HttpResponseDescriptor{
                            content: "".as_bytes().to_owned(),
                            content_type: "/private/",
                            code: HttpCodes::SeeOtherLocation,
                            cookies: Some(vec![
                                Cookie{
                                    name: "uuid".to_string(),
                                    value: uuid.to_string(),
                                    options: Some("Max-Age=86400".to_string()),
                            }]),
                        })
                    } else if user_level == 2 {
                        Box::new(PrivateResponseDescriptor{
                            path: "".to_string(),
                            code: PrivateHttpCodes::PrivateResponseOkUser,
                        })
                    } else{
                        Box::new(HttpResponseDescriptor{
                            content: "".as_bytes().to_owned(),
                            content_type: "",
                            code: HttpCodes::PermissionDenied,
                            cookies: None,
                        })
                    }
                }
            }
            Err(error) => {
                println!("error while accessing database login: {:?}", error);
                Box::new(HttpResponseDescriptor{
                    content: "error while accessing database".as_bytes().to_owned(),
                    content_type: "text/json",
                    code: HttpCodes::PermissionDenied,
                    cookies: None,
                })
            }
        }
    }
    pub async fn is_uuid_valid(&self, uuid: Option<&String>) -> UuidTypes {
        if let Some(uuid) = uuid {
            let uuid = Uuid::parse_str(&uuid).unwrap_or(Uuid::nil());
            let result = sqlx::query("
            select c.livello from
                credenziali c join sessioni s
                    on s.credenziali = c.id
            where uuid = $1 and s.date_generated > now() - interval '24 hours'")
                .bind(uuid)
                .fetch_one(&self.connection)
                .await;
            return match result {
                Ok(res) => { 
                    if !res.is_empty() { 
                        if res.get::<i32, &str>("livello") == 1 {
                            UuidTypes::Admin
                        } else {
                            UuidTypes::User
                        }  /* todo: genera un nuovo uuid per ogni richiesta, di modo da evitare che l'utente perda l'accesso */ 
                    } else {
                        UuidTypes::NotValid
                    } 
                }
                Err(err) => {
                    println!("errore mentre controllo l'uuid: {:?}", err);
                    UuidTypes::NotValid
                }
            }
        }
        println!("no uuid");
        UuidTypes::NotValid
    }
    pub async fn pulisci_sessioni(&self){
        let mut interval = time::interval(Duration::from_secs(60*60*24));
        loop{
            interval.tick().await;
            println!("inizio pulizia sessioni");
            let risultato = query("delete from sessioni where date_generated < now() - interval '12 hours'")
                .execute(&self.connection)
                .await;
            match risultato {
                Ok(res) => {println!("pulizia ok, eliminate {} sessioni", res.rows_affected());}
                Err(e) => {println!("errore durante pulizia sessioni: {}", e)}
            }
        }
    }
}