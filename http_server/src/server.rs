// everything inside of a module is private by default
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
    }
}
