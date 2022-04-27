mod request;
mod response;
mod router;
mod server;

use server::Server;

fn main() {
    let server = Server::new({{threads}});

    server.run();
}
