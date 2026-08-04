use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
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
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(0) => {}
                        Ok(n) => {
                            print!(
                                "Received a request: {}",
                                String::from_utf8_lossy(&buffer[..n])
                            );
                            let response = match Request::try_from(&buffer[..n]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Error Happened: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error Happened: {}", e);
                }
            }
        }
    }
}
