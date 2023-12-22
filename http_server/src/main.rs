fn main() {
    let string = String::from("🍔🍟🍺🍺");  // cannot be sure 1 character takes 1 byte
    // Slicing works for bytes not for each character
    let string_slice = &string[..4];  // borrows the existing string and slices (does not take ownership)
    let string_borrow: &str = &string;  // converts to sring slice
    let string_literal = "1234";
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    // let server = Server::new("127.0.0.1:8080");  // double colon for associated functions
    // server.run();  // dot for methods
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
