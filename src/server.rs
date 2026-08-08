use crate::http::{ParseError, Request, Response, StatusCode, MAX_BODY_SIZE, MAX_HEADERS_SIZE};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, Some("Bad Request".to_string()))
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap_or_else(|e| {
            eprintln!("Failed to bind to {}: {}", self.addr, e);
            std::process::exit(1);
        });
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    if let Err(e) = Self::handle_connection(&mut stream, &mut handler) {
                        println!("Error Happened: {}", e);
                    }
                }
                Err(e) => {
                    println!("Error Happened: {}", e);
                }
            }
        }
    }

    fn handle_connection(
        stream: &mut (impl Read + Write),
        handler: &mut impl Handler,
    ) -> std::io::Result<()> {
        let max_message = MAX_HEADERS_SIZE + MAX_BODY_SIZE;
        let mut buffer = Vec::with_capacity(1024);
        let mut chunk = [0u8; 1024];

        loop {
            let bytes_read = stream.read(&mut chunk)?;
            if bytes_read == 0 {
                if buffer.is_empty() {
                    return Ok(());
                }

                let response = match Request::try_from(buffer.as_slice()) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };
                return response.send(stream);
            }

            if buffer.len() + bytes_read > max_message {
                let response = handler.handle_bad_request(&ParseError::InvalidRequest);
                return response.send(stream);
            }

            buffer.extend_from_slice(&chunk[..bytes_read]);

            match Request::try_from(buffer.as_slice()) {
                Ok(request) => {
                    print!(
                        "Received a request: {}",
                        String::from_utf8_lossy(&buffer)
                    );
                    let response = handler.handle_request(&request);
                    return response.send(stream);
                }
                Err(ParseError::Incomplete) => continue,
                Err(e) => {
                    let response = handler.handle_bad_request(&e);
                    return response.send(stream);
                }
            }
        }
    }
}
