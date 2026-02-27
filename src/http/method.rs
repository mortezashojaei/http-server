use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug)]
pub struct MethodError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_methods() {
        for (raw, expected) in [
            ("GET", Method::GET),
            ("DELETE", Method::DELETE),
            ("POST", Method::POST),
            ("PUT", Method::PUT),
        ] {
            assert_eq!(raw.parse::<Method>().unwrap(), expected);
        }
    }

    #[test]
    fn rejects_unknown_method() {
        assert!("FOO".parse::<Method>().is_err());
    }
}
