use crate::http::HttpRequest;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

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
        H: TcpRequestHandler,
    {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer);
            let req = String::from_utf8_lossy(&buffer[..]).to_string();
            handler.handle_request(HttpRequest::parse(req).unwrap())
        }
        Ok(())
    }
}

pub trait TcpRequestHandler {
    fn handle_request(&self, http_request: HttpRequest) -> ();
}
