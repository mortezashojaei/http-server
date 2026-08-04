use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }
    pub fn run(self) {
        println!("Runing on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(_request) => {
                                    let response = Response::new(
                                        StatusCode::Ok,
                                        Some("OK".to_string()),
                                    );
                                    if let Err(e) = response.send(&mut stream) {
                                        println!("Failed to send response: {}", e);
                                    }
                                }
                                Err(e) => {
                                    println!("{}", e);
                                    let response = Response::new(
                                        StatusCode::BadRequest,
                                        Some("Bad Request".to_string()),
                                    );
                                    if let Err(e) = response.send(&mut stream) {
                                        println!("Failed to send response: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error Happend: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error Happend: {}", e);
                }
            }
        }
    }
}
