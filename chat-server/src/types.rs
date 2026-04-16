use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub type ClientList = Arc<Mutex<Vec<TcpStream>>>;
