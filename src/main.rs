fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run()
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }
    fn run(self) {
        println!("Runing on {}", self.addr);
    }
}
