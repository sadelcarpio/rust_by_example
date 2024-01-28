use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut dyn Write) -> IoResult<()>{  // indicates Write is a trait (dynamic Dispatch)
        // stream is a variable that has the Write trait, therefore implements the write function
        // the compiler figures out the concrete implementation of Write trait
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(stream,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               body,
        )
    }
}
