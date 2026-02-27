use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, _err: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some("Bad request".into()))
    }
}

pub struct Server<H: Handler> {
    addr: String,
    handler: H,
}

impl<H: Handler> Server<H> {
    pub fn new(addr: String, handler: H) -> Self {
        Self { addr, handler }
    }

    pub fn run(self) -> std::io::Result<()> {
        println!("Running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr)?;

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            let response = match Request::try_from(&buffer[..bytes_read]) {
                                Ok(request) => self.handler.handle_request(&request),
                                Err(err) => {
                                    eprintln!("Failed to parse request: {}", err);
                                    self.handler.handle_bad_request(&err)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error reading request: {}", e),
                    }
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }
}
