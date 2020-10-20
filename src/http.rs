pub struct HttpRequest {
    method: HttpMethod,
    path: String
}

impl HttpRequest {
    fn new(method: HttpMethod, path: String) -> Self {
        HttpRequest {
            method,
            path
        }
    }
}


pub enum HttpMethod {
    GET,
    POST,
}
