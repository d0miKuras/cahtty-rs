use std::net::TcpListener;
use std::thread::spawn;
use std::sync::{Mutex, Arc};

fn main() {
   let listener = TcpListener::bind("0:0:0:0:9090").unwrap(); 
}
