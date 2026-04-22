use crate::db::Database;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;

pub struct Client {
    addr: SocketAddr,
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    tx: broadcast::Sender<String>,
    db: Arc<Database>,
}

impl Client {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        tx: broadcast::Sender<String>,
        db: Arc<Database>,
    ) -> Self {
        let (tcp_reader, writer) = stream.into_split();
        Client {
            addr,
            reader: BufReader::new(tcp_reader),
            writer,
            tx,
            db,
        }
    }

    pub async fn run(mut self) {
        println!("[{}] Client connected", self.addr);

        let username = match self.authenticate().await {
            Some(u) => u,
            None => return, // disconnected during login
        };

        if let Err(e) = self.fetch_history().await {
            eprintln!("[{}] Failed to fetch history: {e}", self.addr);
            return;
        };

        self.chat_loop(username).await;
    }

    // ============ private helpers ============
    async fn authenticate(&mut self) -> Option<String> {
        self.send("Enter username: ").await.ok()?;

        let mut line = String::new();
        match self.reader.read_line(&mut line).await {
            Ok(0) | Err(_) => return None,
            Ok(_) => {}
        }

        let username = line.trim().to_string();
        if username.is_empty() {
            self.send("Username cannot be empty.\n").await.ok()?;
            return None;
        }

        if let Err(e) = self.db.create_user_if_not_exists(&username).await {
            eprintln!("[{}] Failed to register user: {e}", self.addr);
        }

        println!("[{}] Logged in as '{username}'", self.addr);
        Some(username)
    }

    async fn fetch_history(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let history = self.db.get_recent_messages(20).await?;

        if history.is_empty() {
            self.send("No message history yet.\n").await?;
        } else {
            self.send("--- Message History ---\n").await?;
            for (user, content) in history {
                self.send(&format!("[{user}]: {content}\n")).await?;
            }
            self.send("--- Live Chat ---\n").await?;
        }

        Ok(())
    }

    async fn send(&mut self, text: &str) -> Result<(), std::io::Error> {
        self.writer.write_all(text.as_bytes()).await
    }

    async fn chat_loop(&mut self, username: String) {
        let mut rx = self.tx.subscribe();
        let mut line = String::new();

        loop {
            tokio::select! {
                // sent message
                result = self.reader.read_line(&mut line) => {
                    match result {
                        Err(e) => {
                            eprintln!("[{}] Error reading line: {}", username, e);
                            break;
                        },
                        Ok(0) => {
                            // read 0 bytes: client disconnect
                            println!("[{}] disconnected", username);
                            break;
                        }
                        Ok(_) => {
                            let content = line.trim().to_string();
                            line.clear();

                            if content.is_empty() { continue; }

                            if let Err(e) = self.db.save_message(&username, &content).await {
                                eprintln!("[{}] Failed to save message: {}", username, e);
                            }

                            let message = format!("[{username}]: {content}");
                            if let Err(e) = self.tx.send(message) {
                                eprintln!("[{}] Broadcast failed: {}", username, e);
                                break;
                            }
                        }
                    }
                }

                // received message
                result = rx.recv() => {
                    match result {
                        Err(e) => {
                            eprintln!("[{}] Received error: {}", username, e);
                            break;
                        }
                        Ok(msg) => {
                            if let Err(e) = self.send(&format!("{msg}\n")).await {
                                eprintln!("[{}] Write error: {}", username, e);
                                break;
                            }
                        }
                    }

                }
            }
        }
    }
}
