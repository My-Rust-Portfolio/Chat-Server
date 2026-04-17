use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::broadcast;

pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<String>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr, tx: broadcast::Sender<String>) -> Self {
        Client { stream, addr, tx }
    }

    pub async fn run(self) {
        println!("[{}] Client connected", self.addr);

        let (tcp_reader, mut writer) = self.stream.into_split();
        let mut reader = BufReader::new(tcp_reader);
        let mut rx = self.tx.subscribe();
        let mut line = String::new();

        loop {
            tokio::select! {
                // sent message
                result = reader.read_line(&mut line) => {
                    match result {
                        Err(e) => {
                            eprintln!("[{}] ERROR: Read error: {}", self.addr, e);
                            break;
                        },
                        Ok(0) => {
                            // read 0 bytes: client disconnect
                            println!("[{}] Client disconnected", self.addr);
                            break;
                        }
                        Ok(_) => {
                            let message = format!("[{}]: {}", self.addr, line.trim());
                            println!("{}", message);
                            if let Err(e) = self.tx.send(message) {
                                eprintln!("ERROR: Broadcast failed {}", e);
                                break;
                            }

                            // to reuse
                            line.clear();
                        }
                    }
                }

                // received message
                result = rx.recv() => {
                    match result {
                        Err(e) => {
                            eprintln!("[{}] ERROR: Received error: {}", self.addr, e);
                            break;
                        }
                        Ok(msg) => {
                            if let Err(e) = writer.write_all(format!("{msg}\n").as_bytes()).await {
                                eprintln!("[{}] ERROR: Write error: {}", self.addr, e);
                                break;
                            }
                        }
                    }

                }
            }
        }
    }
}
