mod client;
mod server;

use server::Server;

const PORT_ADDRESS: &str = "0.0.0.0:9000";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new(PORT_ADDRESS);
    server.run().await
}
