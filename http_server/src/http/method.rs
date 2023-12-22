pub enum Method {
    // in memory they are just incremental integers by default. Can associate different types for each variant.
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
