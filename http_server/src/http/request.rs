use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    // means String can be absent
    method: Method,  // super goes one level up
}
