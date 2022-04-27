use super::request::Request;
use super::response::Response;
use database;

/// Object responsible for processing user-submitted requests as "true" requests
/// to the database.
pub struct Router;

impl Router {
    {% if get %}
    /// Handles GET requests and returns a response
    pub fn get(request: Request) -> Response {
        // first, verify that request.uri matches a valid route
        // then, perform the desired behavior based on the route
        // easily return a response using built-in methods:
        //      e.g. Response::no_found()
        unimplemented!();
    }
    {% endif %}

    {% if post %}
    /// Handles POST requests and submits the specified recipe to the database.
    pub fn post(request: Request) -> Response {
        unimplemented!();
    }
    {% endif %}

    {% if delete %}
    /// Handles DELETE requests and removes a specific recipe from the database.
    pub fn delete(request: Request) -> Response {
        unimplemented!();
    }
    {% endif %}

    {% if put %}
    /// Handles DELETE requests and removes a specific recipe from the database.
    pub fn put(request: Request) -> Response {
        unimplemented!();
    }
    {% endif %}
}
