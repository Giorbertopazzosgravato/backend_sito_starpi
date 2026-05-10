use httparse::Header;
use serde::Deserialize;
use crate::server_utils::database::Database;
use crate::server_utils::http_response::{BuildHttpResponse, HttpCodes, HttpResponseDescriptor};

#[derive(Deserialize, Debug)]
pub struct NewsLetterSub{
    pub nome: Option<String>,
    #[serde(rename="e-mail")]
    pub email: Option<String>,
}
impl NewsLetterSub{
    pub async fn new(data: &str, refer: Option<&Header<'_>>, database: &Database) -> Box<dyn BuildHttpResponse> {
        let sub:NewsLetterSub = serde_urlencoded::from_str(data).unwrap_or(NewsLetterSub{ nome: None, email: None });
        let mut response = HttpResponseDescriptor{
            content: refer.unwrap_or(&Header{ name: "", value: "/".as_bytes() }).value.to_owned(),
            content_type: "",
            code: HttpCodes::SeeOtherLocation,
            cookies: None,
        };
        if let Some(nome) = sub.nome && let Some(email) = sub.email {
            match database.insert_entry_newsletter(&email, &nome).await{
                Ok(res) => {
                    if res {
                        response.content_type = ""
                    }
                }
                Err(_) => {}
            }
        }
        Box::new(response)
    }
}