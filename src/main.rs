mod example;
mod http;
mod server;

use example::api::ApiHandler;
use example::website::WebsiteHandler;
use server::Server;
use std::thread;

fn main() {
    // Example apps built on top of the HTTP server.
    thread::spawn(|| {
        let server = Server::new("127.0.0.1:8081".to_string());
        server.run(ApiHandler::new("example/data/items.txt"));
    });

    let server = Server::new("127.0.0.1:80".to_string());
    server.run(WebsiteHandler);
}
