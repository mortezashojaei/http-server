// HTTP core lives in `http` and `server`.
// Everything under `example` is demo usage of that core, not part of the protocol.
mod example;
mod http;
mod server;

use example::api::ApiHandler;
use example::website::WebsiteHandler;
use server::Server;
use std::thread;

fn main() {
    // Sibling example apps: each gets its own Server + Handler on a dedicated port.
    // API on :8081 — search/pagination over example/data/items.txt
    thread::spawn(|| {
        let server = Server::new("127.0.0.1:8081".to_string());
        server.run(ApiHandler::new("example/data/items.txt"));
    });

    // Website on :80 — http://localhost/ works without typing a port (needs privileges).
    let server = Server::new("127.0.0.1:80".to_string());
    server.run(WebsiteHandler);
}
