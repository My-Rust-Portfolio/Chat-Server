use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const PORT_ADDRESS: &str = "0.0.0.0:9000";

fn handle_client(stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        println!("Client connected: {addr}");

        let mut writer = stream.try_clone().expect("Failed to clone stream");
        // BufReader for reading line by line instead of raw bytes
        let reader = BufReader::new(stream);

        for line in reader.lines() {
            match line {
                Err(_) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(txt) => {
                    println!("[{addr}] received: {txt}");
                    let response = format!("Message received: {txt}\n");
                    match writer.write_all(response.as_bytes()) {
                        Err(e) => println!("Error writing bytes: {e}"),
                        Ok(_) => {}
                    }
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(PORT_ADDRESS)?;
    println!("Listening to {PORT_ADDRESS}");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Connection to {PORT_ADDRESS} failed with error {e}"),
            Ok(stream) => handle_client(stream),
        }
    }

    Ok(())
}
