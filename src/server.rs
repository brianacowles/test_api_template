use super::response::Response;
use super::request::{Method, Request};
use super::router::Router;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::io::prelude::*;
use std::net::TcpListener;

/// the address the server will start listening on
const ADDRESS: &str = "localhost:5000";

/// represents a multi-threaded web server
pub struct Server {
    /// a TCP socket server that listens on `ADDRESS`
    listener: TcpListener,
    /// a thread pool for the server to use for request procesing
    pool: ThreadPool,
}

impl Server {
    /// creates a new server with a thread pool of `pool_size` threads
    pub fn new(pool_size: usize) -> Server {
        // try to create listener, exit early if unsuccessful
        let listener = match TcpListener::bind(ADDRESS) {
            Ok(l) => l,
            Err(e) => {
                println!("Error creating server: \"{}\"", e);
                std::process::exit(1);
            }
        };

        // create a new threadpool
        let pool = ThreadPoolBuilder::new()
            .num_threads(pool_size)
            .build()
            .unwrap();

        Server { listener, pool }
    }

    /// causes the server to start listening to reqeuests which are processed by
    /// threads in the server's thread pool
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.expect("Error unwrapping stream");

            self.pool.install(move || {
                // read the stream into a buffer and convert to a string
                let mut buffer = [0; 2048];
                stream.read(&mut buffer).expect("Error reading stream");

                // return if buffer was empty
                if buffer == [0; 2048] {
                    return;
                }

                // convert buffer to string and create a new request object
                let request_str = String::from_utf8_lossy(&buffer).to_string();
                let request = Request::new(request_str);

                // get the response based on the method used
                let response = match request.method {
                    {% if get %}
                    Method::Get => Router::get(request),
                    {% endif %}
                    {% if post %}
                    Method::Post => Router::post(request),
                    {% endif %}
                    {% if delete %}
                    Method::Delete => Router::delete(request),
                    {% endif %}
                    {% if put %}
                    Method::Put => Router::put(request),
                    {% endif %}
                    _ => Response::invalid_method()
                };

                // write the response to the stream
                stream
                    .write(response.to_string().as_bytes())
                    .expect("Error writing to stream");
                stream.flush().expect("Error flushing stream");
            })
        }
    }
}
