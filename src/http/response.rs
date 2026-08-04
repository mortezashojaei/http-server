use super::status_code::StatusCode;
use std::io::{Result as IoResult, Write};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    content_type: &'static str,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
            content_type: "text/plain",
        }
    }

    pub fn json(status_code: StatusCode, body: String) -> Self {
        Response {
            status_code,
            body: Some(body),
            content_type: "application/json",
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            self.status_code,
            self.status_code.reason_phrase(),
            body.len()
        )?;

        if !body.is_empty() {
            write!(stream, "Content-Type: {}\r\n", self.content_type)?;
        }

        write!(stream, "\r\n{}", body)?;
        stream.flush()
    }
}
