use lib::core::message_bus::{ MessageData, address::Address };

#[derive(Debug)]
pub struct Message {
    receiver: Address,
    sender:   Address,
    title:    String,
    pub data:     MessageData,
}

impl Message {
    pub fn new(title: String, data: MessageData, receiver: Address, sender: Address) -> Self {
        Self { receiver: receiver, sender: sender, title: title, data: data }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn receiver(&self) -> Address {
        self.receiver
    }

    pub fn sender(&self) -> Address {
        self.sender
    }
}
