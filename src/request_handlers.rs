use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

const END_OF_LINE: u8 = 10;

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

// this should be run on all client connections.
pub fn handle_sent_messages(stream: TcpStream, sender: Sender<String>) {
    let mut buffer = BufReader::new(stream);
    let mut message = String::new();

    let request = buffer.read_line(&mut message);
    loop {
        if request.is_err() {
            eprintln!("Failed to receive a message");
            continue;
        }

        let message_bytes = message.as_bytes();
        if message_bytes.get(0) == Some(&END_OF_LINE) {
            break;
        }
        let sent = sender.send(message.to_string());
        if sent.is_err() {
            eprintln!("Failed to send message");
            break;
        }

        message.clear(); // reuse the allocated string
    }
}
