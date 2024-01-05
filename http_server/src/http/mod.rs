// something like __init__.py on python
pub use request::Request;
pub use request::ParseError;
pub use method::Method;  // like exposing through __all__ on python

pub mod request;
pub mod method;