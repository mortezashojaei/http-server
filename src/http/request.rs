use super::method::{Method, MethodError};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::Display,
    fmt::{self, Debug},
    str::from_utf8,
    str::Utf8Error,
};

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // In case we didnt have utf8error From implemention inside ParseError we need to use this one:
        // from_utf8(value).or(Err(ParseError::InvalidEncoding))?;
        let request = from_utf8(value)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if (protocol != "HTTP/1.1") {
            return Err(ParseError::InvalidRequest);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find("?") {
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
    return None;
}

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

impl Debug for ParseError {
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
