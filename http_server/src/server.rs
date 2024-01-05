// everything inside of a module is private by default
use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;

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
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];  // array of 1024 elements of zeroes
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {  // the request borrows the buffer
                                Ok(request) => {
                                },
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }  // buffer[..] creates a byte slice that contains the entire array (it's not [u8; 1024] only [u8])
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
