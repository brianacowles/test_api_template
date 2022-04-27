mod request;
mod response;
mod router;
mod server;

use server::Server;

fn main() {
    // create new server with four threads
    // note: the computers used to test this software have i5s, increase as needed
    let server = Server::new({
        {
            threads
        }
    });

    server.run();
}
