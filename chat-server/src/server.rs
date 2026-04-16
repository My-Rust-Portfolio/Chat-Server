use crate::client::Client;
use crate::types::ClientList;
use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    address: String,
    // Owned by server, client only gets Arc
    clients: ClientList,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Server listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let client = Client::new(stream, Arc::clone(&self.clients));
                    thread::spawn(move || client.run());
                }
                Err(e) => eprintln!("Connection to {} failed with error {}", &self.address, e),
            }
        }

        Ok(())
    }
}
