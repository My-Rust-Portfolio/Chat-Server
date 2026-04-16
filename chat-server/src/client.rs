use crate::types::ClientList;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
    // Server owns the full list
    clients: ClientList,
}

impl Client {
    pub fn new(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> Self {
        let addr = match stream.peer_addr() {
            Ok(a) => a,
            Err(e) => panic!("Could not read peer address: {}", e),
        };

        Client {
            stream,
            addr,
            clients,
        }
    }

    pub fn run(self) {
        println!("[{}] Client connected", self.addr);

        let reader_stream = match self.stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[{}] Failed to clone reader stream: {}", self.addr, e);
                return;
            }
        };

        self.register();

        let reader = BufReader::new(reader_stream);
        for line in reader.lines() {
            match line {
                Ok(txt) => {
                    println!("[{}] {txt}", self.addr);
                    self.broadcast(&format!("[{}]: {txt}\n", self.addr));
                }
                Err(_) => {
                    println!("Client Disconnected");
                    break;
                }
            }
        }
    }

    fn register(&self) {
        let stream_copy = match self.stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "[{}] Failed to clone stream while registering: {}",
                    self.addr, e
                );
                return;
            }
        };

        match self.clients.lock() {
            Ok(mut list) => list.push(stream_copy),
            Err(e) => eprintln!(
                "[{}] Failed to lock client list while registering: {}",
                self.addr, e
            ),
        }
    }

    fn broadcast(&self, message: &str) {
        match self.clients.lock() {
            Err(e) => eprintln!(
                "[{}] Failed to lock client list while broadcasting: {}",
                self.addr, e
            ),
            Ok(mut list) => {
                list.retain_mut(|client| client.write_all(message.as_bytes()).is_ok());
            }
        }
    }
}
