mod http;
mod server;

use http::{Request, Response, StatusCode};
use server::{Handler, Server};

struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.path() {
            "/" => Response::new(
                StatusCode::Ok,
                Some("<h1>Welcome to Glyph's tiny server</h1>".into()),
            ),
            "/health" => Response::new(StatusCode::Ok, Some("OK".into())),
            _ => Response::new(StatusCode::NotFound, Some("Not Found".into())),
        }
    }

    fn handle_bad_request(&self, err: &http::ParseError) -> Response {
        Response::new(
            StatusCode::BadRequest,
            Some(format!("Bad request: {}", err)),
        )
    }
}

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string(), WebsiteHandler);
    if let Err(err) = server.run() {
        eprintln!("Server error: {}", err);
    }
}
