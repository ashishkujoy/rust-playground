use crate::http_status_code::HttpStatus;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

pub struct Server {
    host: &'static str,
    port: &'static str,
}

impl Server {
    pub fn new(host: &'static str, port: &'static str) -> Self {
        Server { host, port }
    }

    pub fn start<H>(&self, handler: H) -> Result<(), Box<dyn Error>>
    where
        H: HttpRequestHandler,
    {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer);
            let req = String::from_utf8_lossy(&buffer[..]);
            let request = HttpRequest::parse(req.trim_end()).unwrap();
            let mut response = HttpResponse::new(HttpStatus::not_found());
            handler.handle_request(&request, &mut response);
            let res = format!(
                "HTTP/1.1 {} {}\r\n\r\n",
                response.status.code,
                response.status.get_description()
            );
            stream.write(res.as_bytes());
        }
        Ok(())
    }
}

pub trait HttpRequestHandler {
    fn handle_request(&self, http_request: &HttpRequest, http_response: &mut HttpResponse) -> ();
}
