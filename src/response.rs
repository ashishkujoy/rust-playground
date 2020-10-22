pub struct HttpResponse {
    pub status_code: u16,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> Self {
        HttpResponse { status_code }
    }

    pub fn set_status(&mut self, status_code: u16) {
        self.status_code = status_code;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_update_status_code() {
        let mut http_response = HttpResponse::new(200);
        http_response.set_status(404);

        assert_eq!(http_response.status_code, 404);
    }
}
