#[derive(Debug, PartialEq)]
pub struct HttpStatus {
    pub code: u16,
    description: &'static str,
}

impl HttpStatus {
    pub fn get_description(&self) -> &str {
        self.description
    }

    pub fn ok() -> Self {
        HttpStatus {
            code: 200,
            description: "OK",
        }
    }

    pub fn not_found() -> Self {
        HttpStatus {
            code: 404,
            description: "NOT FOUND",
        }
    }

    pub fn bad_request() -> Self {
        HttpStatus {
            code: 400,
            description: "BAD REQUEST",
        }
    }
}
