mod http;
mod server;
//use http::{Method, Request};

fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run()
}
