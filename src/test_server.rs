use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use crate::server::HTTP_OK;

pub struct TestServer{}
impl TestServer{
    pub fn start(){
        let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in tcp_listener.incoming(){
            let mut stream = stream.unwrap();
            let mut buffer = [0u8; 1024];
            stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer).to_string();
            println!("{request}");

            let request = request.split(" ").collect::<Vec<_>>()[1];

            let response = if request == "/"{
                let file_content = fs::read_to_string("test.html").unwrap();
                println!("{file_content}");
                format!("{HTTP_OK}\r\nContent-length:{}\r\n\r\n{}", file_content.len(), file_content).as_bytes().to_owned()
            } else if request == "/favicon.ico" {
                "HTTP/1.1 204 No Content\r\nCache-Control: max-age=31536000\r\nConnection: keep-alive".as_bytes().to_owned()
            } else if request == "/.well-known/appspecific/com.chrome.devtools.json"{
                "{vaffanculo: true}".as_bytes().to_owned()
            } else {
                let mut buffer = vec![];
                let mut file_content = fs::File::open("../black_cock.jpg").unwrap();
                let file_content = file_content.read_to_end(&mut buffer).unwrap();

                let mut partial = format!("{HTTP_OK}\r\nContent-length:{}\r\n\r\n", file_content).as_bytes().to_owned();
                partial.extend(buffer);
                partial
            };
            stream.write_all(&response).unwrap();
            println!("finito")
        }
    }
}