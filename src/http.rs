use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
}

impl HttpRequest {
    fn new(method: HttpMethod, path: String) -> Self {
        HttpRequest { method, path }
    }

    pub(crate) fn parse(request: String) -> Result<HttpRequest, HttpParseError> {
        let request_line = request.split("\r\n").take(1).next().unwrap();
        let mut tokens = request_line.split(" ");
        
        let method = match tokens.next() {
            Some("GET") => HttpMethod::GET,
            Some("POST") => HttpMethod::POST,
            _ => {
                return Err(HttpParseError {
                    cause: HttpParseErrorCause::MethodNotFound,
                })
            }
        };

        let path = match tokens.next() {
            Some(path) => path,
            None => {
                return Err(HttpParseError {
                    cause: HttpParseErrorCause::PathNotPresent,
                })
            }
        };

        Ok(HttpRequest {
            method,
            path: path.to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpParseError {
    cause: HttpParseErrorCause,
}

#[derive(Debug, PartialEq)]
enum HttpParseErrorCause {
    MethodNotFound,
    PathNotPresent,
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_request_method_and_path() {
        let raw_request = "POST /helloworld HTTP/1.1\n\r";
        let request = HttpRequest::parse(raw_request.to_string()).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path, "/helloworld".to_string());
    }

    #[test]
    fn give_error_for_unknown_method() {
        let raw_request = "FOO /helloworld HTTP/1.1\r\n";
        let request = HttpRequest::parse(raw_request.to_string());

        assert_eq!(
            request,
            Err(HttpParseError {
                cause: HttpParseErrorCause::MethodNotFound
            })
        );
    }

    #[test]
    fn give_error_for_missing_http_method() {
        let raw_request = "Request:\r\n";
        let request = HttpRequest::parse(raw_request.to_string());

        assert_eq!(
            request,
            Err(HttpParseError {
                cause: HttpParseErrorCause::MethodNotFound
            })
        );
    }

    #[test]
    fn give_error_for_missing_request_uri() {
        let raw_request = "POST\r\n";
        let request = HttpRequest::parse(raw_request.to_string());

        assert_eq!(
            request,
            Err(HttpParseError {
                cause: HttpParseErrorCause::PathNotPresent
            })
        );
    }
}
