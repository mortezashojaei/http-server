use super::api_handler::ApiHandler;
use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    api: ApiHandler,
}

impl WebsiteHandler {
    pub fn new(api: ApiHandler) -> Self {
        WebsiteHandler { api }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        if request.path().starts_with("/api") {
            return self.api.handle_request(request);
        }

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
