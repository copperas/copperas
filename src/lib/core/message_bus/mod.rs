pub mod message;
pub mod message_data;
pub mod address;

use std::sync::mpsc::channel;
use std::sync::mpsc::{ Sender, Receiver, TryRecvError};
use std::collections::HashMap;
// use std::vec::Vec;

use self::{ message::Message, message_data::MessageData, address::Address };

pub struct MessageBus {
    sender:   Sender<Message>,
    receiver: Receiver<Message>,
    // services: Vec<(Address, Sender<Message>, Receiver<Message>)>
    services: HashMap<Address, Sender<Message>>
}

impl MessageBus {
    pub fn new(_bound: usize) -> Self {
        let (sender, receiver) = channel();
        Self { sender: sender, receiver: receiver, services: HashMap::new() }
    }

    pub fn register_service(&mut self, service: Address) -> (Sender<Message>, Receiver<Message>) {
        let (sender, receiver) = channel();
        self.services.insert(service, sender);
        (self.sender.clone(), receiver)
    }

    pub fn try_recv(&self) -> Result<Message, TryRecvError> {
        println!("Receiving message!");
        self.receiver.try_recv()
    }
}
