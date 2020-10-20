
use std::net::TcpListener;
use std::error::Error;

pub struct Server {
    host: &'static str,
    port: &'static str,
}

impl Server {
    pub fn new(host: &'static str, port: &'static str) -> Self {
        Server { host, port }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        Ok(())
    }
}
