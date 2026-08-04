use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("Welcome".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello".to_string())),
                _ => Response::new(StatusCode::NotFound, Some("Not Found".to_string())),
            },
            _ => Response::new(StatusCode::NotFound, Some("Not Found".to_string())),
        }
    }
}
