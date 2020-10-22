use quick_bytes::{http_status_code::HttpStatus, response::HttpResponse};
use quick_bytes::{
    request::HttpRequest,
    server::{HttpRequestHandler, Server},
};
use std::process;

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
impl HttpRequestHandler for DummyHandler {
    fn handle_request(&self, http_request: &HttpRequest, http_response: &mut HttpResponse) -> () {
        println!("{:?}", http_request);
        if http_request.path == "/bad" {
            http_response.set_status(HttpStatus::not_found());
        } else {
            http_response.set_status(HttpStatus::ok());
        }
    }
}
