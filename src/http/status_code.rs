#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    pub fn reason_phrase(self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StatusCode;

    #[test]
    fn reason_phrases_cover_all_variants() {
        assert_eq!(StatusCode::Ok.reason_phrase(), "OK");
        assert_eq!(StatusCode::BadRequest.reason_phrase(), "Bad Request");
        assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
        assert_eq!(
            StatusCode::InternalServerError.reason_phrase(),
            "Internal Server Error"
        );
    }
}
