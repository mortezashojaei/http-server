use std::str::FromStr;

pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;
    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
