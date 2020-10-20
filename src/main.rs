use std::process;
use quick_bytes::server::Server;


fn main() {
    let server = Server::new("127.0.0.1", "7878");
    match server.start() {
        Ok(_) => {}
        Err(error) => {
            let des = error.to_string();
            eprintln!("Unable to establish connection at port: 7878, {}", des);
            process::exit(1);
        }
    }
}
