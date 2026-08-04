mod api_handler;
mod http;
mod server;
mod website_handler;

use api_handler::ApiHandler;
use server::Server;
use std::thread;
use website_handler::WebsiteHandler;

fn main() {
    thread::spawn(|| {
        let server = Server::new("127.0.0.1:8081".to_string());
        server.run(ApiHandler::new("data/items.txt"));
    });

    let server = Server::new("127.0.0.1:80".to_string());
    server.run(WebsiteHandler);
}
