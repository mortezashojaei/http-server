use super::method::{Method, MethodError};
use super::query_string::QueryString;
use super::Headers;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::Display,
    fmt::{self, Debug},
    str::from_utf8,
    str::Utf8Error,
};

pub const MAX_HEADERS_SIZE: usize = 8 * 1024;
pub const MAX_BODY_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
    headers: Headers<'buf>,
    body: Option<&'buf [u8]>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.query_string.as_ref()
    }

    pub fn headers(&self) -> &Headers<'buf> {
        &self.headers
    }

    pub fn body(&self) -> Option<&'buf [u8]> {
        self.body
    }

    pub fn body_text(&self) -> Option<Result<&'buf str, Utf8Error>> {
        self.body.map(from_utf8)
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let header_end = match find_header_end(buf) {
            Some(end) => end,
            None if buf.len() >= MAX_HEADERS_SIZE => return Err(ParseError::InvalidHeader),
            None => return Err(ParseError::Incomplete),
        };

        let header_text = from_utf8(&buf[..header_end])?;
        let (request_line, headers_section) = header_text
            .split_once("\r\n")
            .ok_or(ParseError::InvalidRequest)?;

        let (method, request_line) =
            get_next_word(request_line).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request_line) =
            get_next_word(request_line).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request_line) =
            get_next_word(request_line).ok_or(ParseError::InvalidRequest)?;

        if !request_line.is_empty() {
            return Err(ParseError::InvalidRequest);
        }
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let headers = Headers::try_from(headers_section)?;
        let content_length = headers.content_length()?.unwrap_or(0);
        if content_length > MAX_BODY_SIZE {
            return Err(ParseError::BodyTooLarge);
        }

        let needed = header_end + content_length;
        if buf.len() < needed {
            return Err(ParseError::Incomplete);
        }

        let body = if content_length == 0 {
            None
        } else {
            Some(&buf[header_end..needed])
        };

        Ok(Self {
            path,
            method,
            query_string,
            headers,
            body,
        })
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    if request.is_empty() {
        None
    } else {
        Some((request, ""))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    Incomplete,
    InvalidHeader,
    BodyTooLarge,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            Self::Incomplete => "Incomplete",
            Self::InvalidHeader => "Invalid Header",
            Self::BodyTooLarge => "Body Too Large",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Method;

    fn parse(raw: &str) -> Result<Request<'_>, ParseError> {
        Request::try_from(raw.as_bytes())
    }

    #[test]
    fn parses_get_with_headers_and_no_body() {
        let request = parse("GET /hello?q=1 HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();

        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.path(), "/hello");
        assert_eq!(request.query_string().unwrap().get("q"), Some("1"));
        assert_eq!(request.headers().get("host"), Some("localhost"));
        assert_eq!(request.body(), None);
    }

    #[test]
    fn parses_post_with_content_length_and_body() {
        let request = parse("POST /items HTTP/1.1\r\nContent-Length: 5\r\n\r\nhello").unwrap();

        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.path(), "/items");
        assert_eq!(request.body(), Some(&b"hello"[..]));
        assert_eq!(request.body_text().unwrap().unwrap(), "hello");
    }

    #[test]
    fn incomplete_when_headers_truncated() {
        assert_eq!(
            parse("GET / HTTP/1.1\r\nHost: localhost\r\n").unwrap_err(),
            ParseError::Incomplete
        );
    }

    #[test]
    fn incomplete_when_body_shorter_than_content_length() {
        assert_eq!(
            parse("POST / HTTP/1.1\r\nContent-Length: 10\r\n\r\nshort").unwrap_err(),
            ParseError::Incomplete
        );
    }

    #[test]
    fn rejects_oversize_headers() {
        let mut raw = b"GET / HTTP/1.1\r\nHost: ".to_vec();
        raw.extend(std::iter::repeat(b'a').take(MAX_HEADERS_SIZE));
        assert_eq!(
            Request::try_from(raw.as_slice()).unwrap_err(),
            ParseError::InvalidHeader
        );
    }

    #[test]
    fn rejects_oversize_body() {
        let header = format!(
            "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
            MAX_BODY_SIZE + 1
        );
        assert_eq!(
            Request::try_from(header.as_bytes()).unwrap_err(),
            ParseError::BodyTooLarge
        );
    }

    #[test]
    fn allows_binary_body_without_utf8() {
        let mut raw = b"POST / HTTP/1.1\r\nContent-Length: 3\r\n\r\n".to_vec();
        raw.extend_from_slice(&[0xff, 0xfe, 0xfd]);

        let request = Request::try_from(raw.as_slice()).unwrap();
        assert_eq!(request.body(), Some(&[0xff, 0xfe, 0xfd][..]));
        assert!(request.body_text().unwrap().is_err());
    }
}
