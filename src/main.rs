use quick_bytes::http_status_code::HttpStatus;
use quick_bytes::server::Server;
use quick_bytes::app::App;
use std::process;

fn main() {
    let mut app = App::new(4);

    app.get("/".to_string(), Box::new(|_req, res| {
        res.write("{\"hello\": \"\"}".to_string());
        res.set_header("content-type", "application/json");
        res.set_status(HttpStatus::ok())
    }));

    let server = Server::new("127.0.0.1", "7878");
    match server.start(app) {
        Ok(_) => {}
        Err(error) => {
            let des = error.to_string();
            eprintln!("Unable to establish connection at port: 7878, {}", des);
            process::exit(1);
        }
    }
}
