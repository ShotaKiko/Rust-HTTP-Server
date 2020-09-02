use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    //constructor
    pub fn new(address: String) -> Self {
        Self { address: address }
    }

    pub fn run(self) {
        println!("Server up on port {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
    }
}
