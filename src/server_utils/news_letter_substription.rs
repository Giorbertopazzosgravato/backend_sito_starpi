use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct NewsLetterSub{
    pub nome: Option<String>,
    pub email: Option<String>,
}
impl NewsLetterSub{
    pub fn new(data: &Option<&&str>) -> Option<Self>{
        let mut ret = NewsLetterSub{
            nome: None,
            email: None,
        };
        if let Some(data) = data{
            // zstd\r\n\r\nnome=gino&e-mail=il_postino%40gmail.com\0\0\0\0\0\0...
            // =>
            // [(nome=gino), (e-mail=il_postino%40gmail.com)]
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
                    if first_entry == &"nome"{
                        ret.nome = Some(splitted.get(1).unwrap().replace("%40", "@").to_string());
                    } else if first_entry == &"e-mail"{
                        ret.email = Some(splitted.get(1).unwrap().replace("%40", "@").to_string());
                    }
                }
            }
        }

        if ret.email.is_some() && ret.nome.is_some(){
             Some(ret)
        } else {
            None
        }
    }
    pub async fn write_on_file(&self, path: &str){
        let file = fs::File::options().create(true).append(true).open(path).await;
        if let Ok(mut file) = file{
            let result = file.write(format!("{}, {}\n", self.email.as_ref().unwrap(), self.nome.as_ref().unwrap()).as_bytes(), ).await;
            if let Err(result) = result{ println!("error while writing the file {:?} \nfor user{:?}", result, self) }
        } else {
            println!("error while opening the file: {path}");
        }
    }
}