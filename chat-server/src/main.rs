mod client;
mod db;
mod server;

use server::Server;

const PORT_ADDRESS: &str = "0.0.0.0:9000";

#[tokio::main]
async fn main() {
    let server = match Server::init(PORT_ADDRESS).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR Failed to start server: {}", e);
            return;
        }
    };

    if let Err(e) = server.run().await {
        eprintln!("ERROR Server error {}", e);
    }
}
