pub mod config;
pub mod event_manager;
pub mod menu;
pub mod message_bus;
pub mod time;

use std::sync::mpsc::TryRecvError;

use lib::graphics;

use self::config::Config;
use self::event_manager::{ EventManager, CloseWindow };
use self::message_bus::{ MessageBus, Message };
use self::time::ClockManager;

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run(config_path: &str) {
    let mut config           = Config::new(config_path);
    let message_bus          = MessageBus::new(0); // Set propper length later
    let mut clock_manager    = ClockManager::new(config.time().delta());
    let mut event_manager    = EventManager::new(config.controlls(), message_bus.new_sender());
    let mut window           = graphics::window::new(config.window(), &event_manager);
    let mut running          = true;
    while running {
        event_manager.manage_events();
        
        if close(message_bus.try_recv()) { running = false }
    }
}

fn close(received: Result<Message, TryRecvError>) -> bool {
    let success = |message| { println!("{:?}", message); true };
    match received {
        Ok(message) => success(message),
        Err(_)  => false
    }
}
