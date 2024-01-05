#![allow(dead_code)]

use server::Server;

mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());  // double colon for associated functions
    server.run();  // dot for methods
}
