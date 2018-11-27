use std::sync::mpsc::channel;
use std::sync::mpsc::{ Sender, Receiver, TryRecvError};

pub struct MessageBus {
    sender:   Sender<Message>,
    receiver: Receiver<Message>
}

impl MessageBus {
    pub fn new(_bound: usize) -> Self {
        let (sender, receiver) = channel();
        Self { sender: sender, receiver: receiver }
    }

    pub fn new_sender(&self) -> Sender<Message> {
        self.sender.clone()
    }

    pub fn try_recv(&self) -> Result<Message, TryRecvError> {
        println!("Receiving message!");
        self.receiver.try_recv()
    }
}

#[derive(Debug)]
pub struct Message {
    title: String,
    data:  Box<MessageData>
}

impl Message {
    pub fn new(title: String, data: Box<MessageData>) -> Self {
        Self { title: title, data: data }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn data(&self) -> &Box<MessageData> {
        &self.data
    }
}

pub trait MessageData {}

impl<T> MessageData for T  where T: Sized {}

impl std::fmt::Debug for MessageData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Some data here (NOTE: This is a stub!)") // TODO: Implement a better formatter
    }
}

impl std::cmp::PartialEq for MessageData {
    fn eq(&self, other: &(dyn MessageData + 'static)) -> bool {
        self == other
    }
}
