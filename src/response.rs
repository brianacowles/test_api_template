use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

/// represents the possible header types present in the response
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Header {
    ContentType,
    ContentLength,
}

impl fmt::Display for Header {
    /// formats header type to its corresponding string i.e. ContentType -> "Content-Type"
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Header::ContentType => f.write_str("Content-Type"),
            Header::ContentLength => f.write_str("Content-Length"),
        }
    }
}

/// the http version used by the server, in this case it is 1.1
const HTTP_VERSION: &str = "HTTP/1.1";

/// converts the HTTP response code to its corresponding status phrase as defined by
/// [section 10 of RFC 2616](https://datatracker.ietf.org/doc/html/rfc2616#section-10)
fn to_reason_phrase(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => {
            println!("Error unrecognized status code");
            std::process::exit(1);
        }
    }
}

/// represents an HTTP response
#[derive(Debug)]
pub struct Response {
    /// the response code (i.e. 200)
    status_code: u16,
    /// the headers in the response (i.e. Content-Type) a hashmap is used to avoid duplication
    headers: HashMap<Header, String>,
    /// the body of the response (this may be written in JSON, HTML, XML, etc.)
    body: String,
}

impl Response {
    /// creates a new response
    fn from_status_code(code: u16) -> Response {
        Response {
            status_code: code,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn ok() -> Response {
        let mut response = Response::from_status_code(200);
        response.set_header(Header::ContentType, String::from({% content_type %}));
        response
    }

    pub fn created() -> Response {
        Response::from_status_code(201)
    }

    pub fn no_content() -> Response {
        Response::from_status_code(204)
    }

    pub fn bad_request() -> Response {
        Response::from_status_code(400)
    }

    pub fn not_found() -> Response {
        Response::from_status_code(404)
    }

    /// setter for the response header
    pub fn set_header(&mut self, header: Header, value: String) {
        self.headers.insert(header, value);
    }

    /// setter for the response body
    pub fn set_body(&mut self, body: String) {
        self.body = body;
        self.set_header(Header::ContentLength, self.body.len().to_string())
    }

    /// converts the response to a properly formatted HTTP response as specified by
    /// [section 6 of RFC 2616](https://datatracker.ietf.org/doc/html/rfc2616#section-6)
    pub fn to_string(&self) -> String {
        // create the new headers string
        let mut headers_str = String::new();
        for header in &self.headers {
            let formatted_header = &format!("{}: {}\n", header.0, header.1)[..];
            headers_str.push_str(formatted_header);
        }

        // return the formatted response
        format!(
            "{} {} {}\r\n{}\r\n{}",
            HTTP_VERSION,
            self.status_code,
            to_reason_phrase(self.status_code),
            headers_str,
            self.body
        )
    }
}
