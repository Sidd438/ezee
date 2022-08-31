use server::Server;
mod server;
mod http;
use http::Request;
use http::METHOD;

fn main() {
    let server:Server = Server::run("127.0.0.1:8000".to_string());
    server.start();
}
