use std::net::TcpListener;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::spawn;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    let senders: Vec<Sender<String>> = Vec::new();
    let senders_mutex: Mutex<Vec<Sender<String>>> = Mutex::new(senders); // since this will be shared between threads
}
