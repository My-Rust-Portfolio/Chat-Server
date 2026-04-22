use crate::client::Client;
use crate::db::Database;
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

const BROADCAST_CAPACITY: usize = 32;
pub struct Server {
    address: String,
    db: Arc<Database>,
}

impl Server {
    pub async fn init(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set in .env")?;
        let database = Database::connect(&database_url).await?;
        println!("Connect to PostgreSQL");

        Ok(Server {
            address: address.to_string(),
            db: Arc::new(database),
        })
    }

    pub async fn run(&self) -> io::Result<()> {
        let listener = self.bind().await?;
        self.handle_connections(listener).await
    }

    // ============ private helpers ============
    async fn bind(&self) -> io::Result<TcpListener> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("[{}] Server listening", self.address);
        Ok(listener)
    }

    async fn handle_connections(&self, listener: TcpListener) -> io::Result<()> {
        let (tx, _rx) = broadcast::channel::<String>(BROADCAST_CAPACITY);
        loop {
            match listener.accept().await {
                Err(e) => eprintln!("[{}] Connection failed with error {}", &self.address, e),
                Ok((stream, addr)) => {
                    let client = Client::new(stream, addr, tx.clone(), Arc::clone(&self.db));
                    tokio::spawn(async move {
                        client.run().await;
                    });
                }
            }
        }
    }
}
