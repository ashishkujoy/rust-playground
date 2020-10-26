use crate::response::HttpResponse;
use crate::server::HttpRequestHandler;
use crate::{
    http_status_code::HttpStatus,
    request::{HttpMethod, HttpRequest},
};
use std::collections::HashMap;

type RequestHandler = Box<dyn Fn(&HttpRequest, &mut HttpResponse)>;

pub struct App {
    get_routes: HashMap<String, RequestHandler>,
    post_routes: HashMap<String, RequestHandler>,
}

impl App {
    pub fn new(initial_capacity: usize) -> Self {
        App {
            get_routes: HashMap::with_capacity(initial_capacity / 2),
            post_routes: HashMap::with_capacity(initial_capacity / 2),
        }
    }

    pub fn get(&mut self, path: String, h: RequestHandler) {
        self.get_routes.insert(path, h);
    }

    pub fn post(&mut self, path: String, h: RequestHandler) {
        self.post_routes.insert(path, h);
    }
}

impl HttpRequestHandler for App {
    fn handle_request(&self, req: &HttpRequest, res: &mut HttpResponse) {
        let handler = if req.method == HttpMethod::POST {
            self.post_routes.get(&req.path)
        } else {
            self.get_routes.get(&req.path)
        };


        if let Some(handler) = handler {
            handler(req, res);
            
        } else {
            res.write("{\"body\": \"Requested resource not found\"}".to_string());
            res.set_header("content-type", "application/json");
            res.set_status(HttpStatus::not_found())
        }
    }
}
