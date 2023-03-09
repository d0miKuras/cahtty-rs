mod request_handlers;
use std::net::TcpListener;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::spawn;

use crate::request_handlers::{handle_sent_messages, send_to_client};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
    type Senders = Vec<Sender<String>>;
    let senders: Senders = Vec::new();
    let senders_mutex: Mutex<Senders> = Mutex::new(senders); // since this will be shared between threads
    let senders_mutex_ptr: Arc<Mutex<Senders>> = Arc::new(senders_mutex);
    let senders_mutex_ptr_copy = senders_mutex_ptr.clone();

    spawn(|| {
        request_handlers::receive_messages(receiver, senders_mutex_ptr);
    });

    for inc in listener.incoming() {
        // this'll handle client connection and sending messages
        if let Err(e) = inc {
            eprintln!("Client connection failed: {}", e);
            continue;
        }
        let stream = inc.unwrap();
        let client_ip = stream.peer_addr().unwrap();
        println!("New client connected: {}", client_ip);

        let stream_copy = stream.try_clone().expect("Cannot clone TCP stream"); // this is passed cuz we're passing the stream to 2 threads
        let sender_copy = sender.clone(); // one per client
        spawn(|| {
            // this thread handles sent messages from the new client
            handle_sent_messages(stream_copy, sender_copy);
        });

        let (client_sender, client_receiver): (Sender<String>, Receiver<String>) = channel(); // sender/receiver pair per client
        spawn(|| {
            send_to_client(stream, client_receiver);
        });
        let mut guard = senders_mutex_ptr_copy.lock().unwrap();
        let senders = &mut *guard;
        senders.push(client_sender);
    }
}
