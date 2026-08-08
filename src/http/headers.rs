use super::request::ParseError;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: Vec<(&'buf str, &'buf str)>,
}

impl<'buf> Headers<'buf> {
    pub fn get(&self, name: &str) -> Option<&'buf str> {
        self.data
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }

    pub fn content_length(&self) -> Result<Option<usize>, ParseError> {
        match self.get("content-length") {
            None => Ok(None),
            Some(value) => value
                .parse()
                .map(Some)
                .map_err(|_| ParseError::InvalidHeader),
        }
    }
}

impl<'buf> TryFrom<&'buf str> for Headers<'buf> {
    type Error = ParseError;

    fn try_from(headers_section: &'buf str) -> Result<Self, Self::Error> {
        let mut data = Vec::new();

        for line in headers_section.split("\r\n") {
            if line.is_empty() {
                break;
            }

            let (name, value) = line.split_once(':').ok_or(ParseError::InvalidHeader)?;
            let name = name.trim();
            if name.is_empty() {
                return Err(ParseError::InvalidHeader);
            }
            data.push((name, value.trim()));
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_headers_and_looks_up_case_insensitively() {
        let headers =
            Headers::try_from("Host: example.com\r\nContent-Type: text/plain\r\n\r\n").unwrap();

        assert_eq!(headers.get("host"), Some("example.com"));
        assert_eq!(headers.get("CONTENT-TYPE"), Some("text/plain"));
        assert_eq!(headers.get("missing"), None);
    }

    #[test]
    fn content_length_parses_or_rejects() {
        let headers = Headers::try_from("Content-Length: 12\r\n\r\n").unwrap();
        assert_eq!(headers.content_length().unwrap(), Some(12));

        let headers = Headers::try_from("Host: a\r\n\r\n").unwrap();
        assert_eq!(headers.content_length().unwrap(), None);

        let headers = Headers::try_from("Content-Length: nope\r\n\r\n").unwrap();
        assert_eq!(headers.content_length(), Err(ParseError::InvalidHeader));
    }

    #[test]
    fn rejects_header_without_colon() {
        assert_eq!(
            Headers::try_from("NotAHeader\r\n\r\n").unwrap_err(),
            ParseError::InvalidHeader
        );
    }
}
