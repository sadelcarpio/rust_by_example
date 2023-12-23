// everything inside of a module is private by default
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // associated function
    pub fn new(addr: String) -> Self {  // Self: alias for the name of the struct
        Self { addr }
    }
    // method
    pub fn run(self) {  // run takes ownership if not using reference (&)
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();  // Result is a recoverable error Result<T, E>;
        loop {
            // best practice: use match for handling result
            match listener.accept() {
                Ok((stream, _)) => {

                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
            // other approach: using if statement
            let res = listener.accept();  // example of recoverable error
            if res.is_err() {
                continue;
            }
            let (stream, addr) = res.unwrap();  // here we are sure there is no error
        }
    }
}
