use super::method::{Method, MethodError};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::Display,
    fmt::{self, Debug},
    str::from_utf8,
    str::Utf8Error,
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn query_string(&self) -> Option<&str> {
        self.query_string
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = from_utf8(value)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            method,
            query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
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

    fn build_request(method: &str, path: &str) -> Vec<u8> {
        format!(
            "{} {} HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test\r\n\r\n",
            method, path
        )
        .into_bytes()
    }

    #[test]
    fn parses_basic_request() {
        let raw = build_request("GET", "/hello");
        let request = Request::try_from(raw.as_slice()).unwrap();
        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.path(), "/hello");
        assert!(request.query_string().is_none());
    }

    #[test]
    fn parses_query_string() {
        let raw = build_request("GET", "/items?id=42");
        let request = Request::try_from(raw.as_slice()).unwrap();
        assert_eq!(request.path(), "/items");
        assert_eq!(request.query_string(), Some("id=42"));
    }

    #[test]
    fn rejects_wrong_protocol() {
        let raw = format!("GET / HTTP/2.0\r\n\r\n").into_bytes();
        let err = Request::try_from(raw.as_slice()).unwrap_err();
        assert_eq!(err, ParseError::InvalidProtocol);
    }

    #[test]
    fn rejects_invalid_utf8() {
        let raw = vec![0xff, 0xfe, 0xfd];
        let err = Request::try_from(raw.as_slice()).unwrap_err();
        assert_eq!(err, ParseError::InvalidEncoding);
    }
}
