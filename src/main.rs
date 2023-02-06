use std::net::TcpListener;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::spawn;

fn main() {
    let listener = TcpListener::bind("0:0:0:0:9091").unwrap();
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    let senders: Vec<Sender<String>> = Vec::new();
}
