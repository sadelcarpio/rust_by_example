use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

use super::method::{Method, MethodError};

#[derive(Debug)]
pub struct Request<'buf> {  // generic over a lifetime
    path: &'buf str,  // lifetime specifier is needed so path does not point to empty buffer
    query_string: Option<QueryString<'buf>>,
    // means String can be absent
    method: Method,  // super goes one level up
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {  // make sure we are using the same lifetime
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {  // verify buf has same lifetime
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;  // much more simplified
        let request = str::from_utf8(buf)?;  // as long as Utf8Error is converted to ParseError via From trait
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }
        // variable shadowing, ok_or maps Some, None to Ok, Err, to be similar to .or()
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;  // simplified
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        // only matches on Some, since None is not necessary
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));  // means +1 byte, not +1 character
            path = &path[..i]
        }
        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.char_indices() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
