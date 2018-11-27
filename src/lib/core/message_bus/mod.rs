use std::sync::mpsc::sync_channel;
use std::sync::mpsc::{ SyncSender, Receiver};

pub struct MessageBus {
    sender:   SyncSender<Message>,
    receiver: Receiver<Message>
}

impl MessageBus {
    pub fn new(bound: usize) -> Self {
        let (sender, receiver) = sync_channel(bound);
        Self { sender: sender, receiver: receiver }
    }

    pub fn new_sender(&self) -> SyncSender<Message> {
        self.sender.clone()
    }

    pub fn receiver(&self) -> &Receiver<Message> {
        &self.receiver
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

    pub fn data(&self) -> Box<MessageData> {
        self.data
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
