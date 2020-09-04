use std::io::Read;
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

        loop {
            match listener.accept() {
                //address not needed
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            //converts to text
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Failed to establish connection : {}", err),
            }
        }
    }
}
