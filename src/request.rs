use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    fn new(method: HttpMethod, path: String, headers: HashMap<String, String>) -> Self {
        HttpRequest {
            method,
            path,
            headers,
        }
    }

    pub(crate) fn parse(request: &str) -> Result<HttpRequest, HttpParseError> {
        let mut lines = request.split("\r\n");
        let request_line = lines.next().unwrap();
        let mut tokens = request_line.split(" ");

        let method = Self::parse_request_method(tokens.next())?;
        let path = Self::parse_request_path(tokens.next())?;

        let mut headers: HashMap<String, String> = HashMap::new();
        for header_line in lines {
            let mut tokens = header_line.split(": ");
            let header_name = tokens.next();
            let header_value = tokens.next();
            match (header_name, header_value) {
                (Some(name), Some(value)) => {
                    headers.insert(name.to_string(), value.to_string());
                }
                _ => {}
            }
        }

        Ok(HttpRequest {
            method,
            path: path.to_string(),
            headers: headers,
        })
    }

    fn parse_request_method(method_token: Option<&str>) -> Result<HttpMethod, HttpParseError> {
        match method_token {
            Some("GET") => Ok(HttpMethod::GET),
            Some("POST") => Ok(HttpMethod::POST),
            _ => {
                return Err(HttpParseError {
                    cause: HttpParseErrorCause::MethodNotFound,
                })
            }
        }
    }

    fn parse_request_path(path_token: Option<&str>) -> Result<&str, HttpParseError> {
        match path_token {
            Some(path) => Ok(path),
            None => {
                return Err(HttpParseError {
                    cause: HttpParseErrorCause::PathNotPresent,
                })
            }
        }
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
        let request = HttpRequest::parse(raw_request).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path, "/helloworld".to_string());
    }

    #[test]
    fn give_error_for_unknown_method() {
        let raw_request = "FOO /helloworld HTTP/1.1\r\n";
        let request = HttpRequest::parse(raw_request);

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
        let request = HttpRequest::parse(raw_request);

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
        let request = HttpRequest::parse(raw_request);

        assert_eq!(
            request,
            Err(HttpParseError {
                cause: HttpParseErrorCause::PathNotPresent
            })
        );
    }

    #[test]
    fn parse_headers() {
        let raw_request = "GET /helloworld HTTP/1.1\r\nHost: localhost:7878\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("User-Agent".to_string(), "curl/7.64.1".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Host".to_string(), "localhost:7878".to_string());

        let request = HttpRequest::parse(raw_request).unwrap();

        assert_eq!(request.headers, headers)
    }
}
