mod client;
mod server;
mod types;

use server::Server;

const PORT_ADDRESS: &str = "0.0.0.0:9000";

fn main() -> std::io::Result<()> {
    let server = Server::new(PORT_ADDRESS);
    server.run()
}
