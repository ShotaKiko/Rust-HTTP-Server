use http::Method;
use http::Request;
use server::Server; //from mod use Server struct

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
