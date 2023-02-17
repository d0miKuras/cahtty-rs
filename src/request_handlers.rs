use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

pub fn receive_messages(
    receiver: Receiver<String>,
    senders_mutex_pointer: Arc<Mutex<Vec<Sender<String>>>>,
) {
    loop {
        let message_result = receiver.recv();
        if let Err(e) = message_result {
            eprintln!("Error waiting for message result: {e}");
            continue;
        }
        let guard = senders_mutex_pointer.lock().unwrap();
        let senders = &*guard;
        let msg = message_result.unwrap();
        for sender in senders {
            sender.send(msg.to_string()).expect("Error sending message");
        }
    }
}

pub fn handle_sent_messages(stream: TcpStream, sender: Sender<String>) {
    let mut buffer = BufReader::new(stream);
    let mut message = String::new();

    let request = buffer.read_line(&mut message);
    if let Err(e) = request {
        eprintln!("Failed to receive a message: {e}");
        return;
    }
}
