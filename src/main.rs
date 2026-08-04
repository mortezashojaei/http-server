mod api_handler;
mod http;
mod server;
mod website_handler;

use api_handler::ApiHandler;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let api = ApiHandler::new("data/items.txt");
    server.run(WebsiteHandler::new(api));
}
