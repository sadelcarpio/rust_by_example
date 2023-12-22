fn main() {
    let server = Server::new("127.0.0.1:8080");  // double colon for associated functions
    server.run();  // dot for methods
}

struct Server {
    addr: String,
}

impl Server {
    // associated function
    fn new(addr: String) -> Self {  // Self: alias for the name of the struct
        Self { addr }
    }
    // method
    fn run(self) {  // run takes ownership if not using reference (&)

    }
}
