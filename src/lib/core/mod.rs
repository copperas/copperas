pub mod config;
pub mod event_manager;
pub mod menu;
pub mod message_bus;
pub mod time;

use std::sync::mpsc::{ Receiver, Sender, TryRecvError };

use lib::graphics;

use self::config::Config;
use self::event_manager::{ EventManager, CloseWindow };
use self::message_bus::{
    MessageBus, address::Address, message::Message, message_data::MessageData
};
use self::time::ClockManager;

pub struct Core {
    receiver: Receiver<Message>,
    sender:   Sender<Message>,
    running:  bool
}

impl Core {
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn run(config_path: &str) {
        let mut config           = Config::new(config_path);
        let mut message_bus      = MessageBus::new(0); // Set propper length later
        let mut core             = Core::new(message_bus.register_service(Address::Core));
        let mut clock_manager    = ClockManager::new(config.time().delta());
        let mut event_manager    = EventManager::new(
            config.controlls(), message_bus.register_service(Address::EventManager)
        );
        let mut window           = graphics::window::new(config.window(), &event_manager);

        while core.running() {
            event_manager.manage_events();
            message_bus.pump_message();
        }
    }

    fn new(rns: (Sender<Message>, Receiver<Message>)) -> Core {
        let(sender, receiver) = rns;
        Core { receiver: receiver, sender: sender, running: false }
    }

    fn close(received: Result<Message, TryRecvError>) -> bool {
        let success = |message| { println!("{:?}", message); true };
        match received {
            Ok(message) => success(message),
            Err(_)  => false
        }
    }

    pub fn process_messages(&self) {
        match self.try_recv() {
            Ok(message) => self.read_message(message),
            Err(error)  => println!("Tried to recieve message, got error: {:?}", error)
        }
    }

    fn try_recv(&self) -> Result<Message, TryRecvError> {
        self.receiver.try_recv()
    }

    fn read_message(&self, message: Message) {
        let boxy = message.data().extract();
    }

    fn running(&self) -> bool {
        self.running
    }

    fn start(&mut self) {
        self.running = true;
    }
}

enum CoreActions {
    Exit
}
