use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

pub fn receive_messages(
    receiver: Receiver<String>,
    senders_mutex_pointer: Arc<Mutex<Vec<Sender<String>>>>,
) {
}
