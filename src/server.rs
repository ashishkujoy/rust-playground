
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::io::prelude::*;

pub struct Server {
    host: &'static str,
    port: &'static str,
}

impl Server {
    pub fn new(host: &'static str, port: &'static str) -> Self {
        Server { host, port }
    }

    pub fn start<H>(&self, handler: H) -> Result<(), Box<dyn Error>> where H : TcpRequestHandler{
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        for stream in listener.incoming() {
            handler.handle_request(stream.unwrap())
        }
        Ok(())
    }
}

pub trait TcpRequestHandler {
    fn handle_request(&self, stream: TcpStream) -> ();
}