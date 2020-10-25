use crate::http_status_code::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        HttpResponse { status, body: None }
    }

    pub fn write(&mut self, body: String) {
        self.body = Some(body)
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn set_header(&mut self, name: &str, value: &str) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_update_status_code() {
        let mut http_response = HttpResponse::new(HttpStatus::ok());
        http_response.set_status(HttpStatus::not_found());

        assert_eq!(http_response.status, HttpStatus::not_found());
    }
}
