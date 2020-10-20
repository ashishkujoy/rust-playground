use quick_bytes::{
    http::HttpRequest,
    server::{Server, TcpRequestHandler},
};
use std::{net::TcpStream, process};

fn main() {
    let server = Server::new("127.0.0.1", "7878");
    match server.start(DummyHandler {}) {
        Ok(_) => {}
        Err(error) => {
            let des = error.to_string();
            eprintln!("Unable to establish connection at port: 7878, {}", des);
            process::exit(1);
        }
    }
}

struct DummyHandler {}
impl TcpRequestHandler for DummyHandler {
    fn handle_request(&self, http_request: HttpRequest) -> () {
        println!("{:?}", http_request)
    }
}
