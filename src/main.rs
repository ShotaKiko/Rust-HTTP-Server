fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    //constructor
    fn new(address: String) -> Self {
        Self { address: address }
    }

    fn run(self) {
        println!("Server up on port {}", self.address)
    }
}
