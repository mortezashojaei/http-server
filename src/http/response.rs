use std::fmt;
use std::io::{Result as IoResult, Write};

use crate::http::status_code::StatusCode;

#[derive(Debug, Clone)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");
        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code.as_u16(),
            self.status_code.reason_phrase(),
            body.len(),
            body
        )
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.body {
            Some(body) => write!(
                f,
                "HTTP/1.1 {} {}\r\n\r\n{}",
                self.status_code.as_u16(),
                self.status_code.reason_phrase(),
                body
            ),
            None => write!(
                f,
                "HTTP/1.1 {} {}\r\n\r\n",
                self.status_code.as_u16(),
                self.status_code.reason_phrase()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_valid_http_response() {
        let response = Response::new(StatusCode::Ok, Some("hello".into()));
        assert_eq!(response.status_code(), StatusCode::Ok);
        assert_eq!(response.body(), Some("hello"));

        let mut buffer = Vec::new();
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.starts_with("HTTP/1.1 200 OK"));
        assert!(output.contains("Content-Length: 5"));
        assert!(output.ends_with("hello"));
    }
}
