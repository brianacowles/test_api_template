/// Type that represents the method of an HTTP request: "Get" to retrieve
/// recipes, "Post" to submit recipes, and "Delete" to remove recipes.
#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Delete,
}

/// Representation of an HTTP request.
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub body: String,
}

impl Request {
    /// Creates a new Request object from an HTTP request-formatted String.
    pub fn new(s: String) -> Request {
        let mut split = s.split(" ");
        let method = match split.next() {
            Some("GET") => Method::Get,
            Some("POST") => Method::Post,
            Some("DELETE") => Method::Delete,
            _ => panic!("Invalid request method"),
        };
        let uri = String::from(split.next().unwrap());
        // r is the headers (not handled) followed by the request body
        let r = split.collect::<Vec<&str>>().join(" ");
        let body_string = r.split("\r\n\r\n").collect::<Vec<&str>>()[1];
        let body = String::from(body_string.trim_end_matches('\0'));
        Request {
            method: method,
            uri: uri,
            body: body,
        }
    }
}
