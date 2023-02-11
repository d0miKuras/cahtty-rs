mod request_handlers;
use std::net::TcpListener;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::spawn;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    type Senders = Vec<Sender<String>>;
    let senders: Senders = Vec::new();
    let senders_mutex: Mutex<Senders> = Mutex::new(senders); // since this will be shared between threads
    let senders_mutex_ptr: Arc<Mutex<Senders>> = Arc::new(senders_mutex);
}
