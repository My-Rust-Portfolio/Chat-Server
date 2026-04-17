use crate::client::Client;
use std::io;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

const BROADCAST_CAPACITY: usize = 32;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Server listening on {}", self.address);

        let (tx, _rx) = broadcast::channel::<String>(BROADCAST_CAPACITY);

        loop {
            match listener.accept().await {
                Err(e) => eprintln!(
                    "[{}] ERROR: Connection failed with error {}",
                    &self.address, e
                ),
                Ok((stream, addr)) => {
                    let client = Client::new(stream, addr, tx.clone());
                    tokio::spawn(async move {
                        client.run().await;
                    });
                }
            }
        }
    }
}
