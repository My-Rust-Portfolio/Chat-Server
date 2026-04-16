use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const PORT_ADDRESS: &str = "0.0.0.0:9000";

type ClientList = Arc<Mutex<Vec<TcpStream>>>;

fn handle_client(stream: TcpStream, clients: ClientList) {
    if let Ok(addr) = stream.peer_addr() {
        println!("Client connected: {addr}");

        {
            let mut list = clients.lock().unwrap();
            list.push(stream.try_clone().expect("Failed to clone stream"));
        }

        // BufReader for reading line by line instead of raw bytes
        let reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
        for line in reader.lines() {
            match line {
                Err(_) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(txt) => {
                    println!("[{addr}] received: {txt}");
                    let response = format!("Message received: {txt}\n");

                    let mut list = clients.lock().unwrap();
                    list.retain_mut(|client| client.write_all(response.as_bytes()).is_ok());
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(PORT_ADDRESS)?;
    println!("Listening to {PORT_ADDRESS}");

    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    // listener.incomin() blocks the thread until someone connects
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Connection to {PORT_ADDRESS} failed with error {e}"),
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
        }
    }

    Ok(())
}
