use crate::http_status_code::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        HttpResponse { status }
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }
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
