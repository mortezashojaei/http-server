use super::api_handler::ApiHandler;
use super::http::{Request, Response};
use super::server::Handler;
use super::website_handler::WebsiteHandler;

pub struct AppHandler {
    website: WebsiteHandler,
    api: ApiHandler,
}

impl AppHandler {
    pub fn new(api: ApiHandler) -> Self {
        AppHandler {
            website: WebsiteHandler,
            api,
        }
    }
}

impl Handler for AppHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        if request.path().starts_with("/api") {
            return self.api.handle_request(request);
        }

        self.website.handle_request(request)
    }
}
