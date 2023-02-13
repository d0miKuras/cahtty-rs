use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
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
        let guard - senders_mutex_pointer.lock().unwrap();
        let senders = &*guard;
        let msg = message_result.unwrap();
    }
}
