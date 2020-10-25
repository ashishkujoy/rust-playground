use crate::response::HttpResponse;
use crate::server::HttpRequestHandler;
use crate::{
    http_status_code::HttpStatus,
    request::{HttpMethod, HttpRequest},
};
use std::collections::HashMap;

struct App<H> {
    get_routes: Box<HashMap<String, H>>,
    post_routes: Box<HashMap<String, H>>,
}

impl<H> App<H>
where
    H: HttpRequestHandler,
{
    pub fn new() -> Self {
        App {
            get_routes: Box::new(HashMap::default()),
            post_routes: Box::new(HashMap::default()),
        }
    }

    pub fn get(&mut self, path: String, h: H) {
        self.get_routes.insert(path, h);
    }

    pub fn post(&mut self, path: String, h: H) {
        self.post_routes.insert(path, h);
    }
}

impl<H> HttpRequestHandler for App<H>
where
    H: HttpRequestHandler,
{
    fn handle_request(&self, req: &HttpRequest, res: &mut HttpResponse) {
        let handler = if req.method == HttpMethod::POST {
            self.post_routes.get(&req.path)
        } else {
            self.get_routes.get(&req.path)
        };

        if let Some(handler) = handler {
            handler.handle_request(req, res)
        } else {
            res.write("{\"body\": \"Requested resource not found\"}".to_string());
            res.set_header("content-type", "application/json");
            res.set_status(HttpStatus::not_found())
        }
    }
}
