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

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run(config_path: &str) {
    let mut running       = true;
    let mut config        = Config::new(config_path);
    let mut clock_manager = ClockManager::new(config.time().delta());
    let mut event_manager = EventManager::new(config.controlls());
    let mut window        = graphics::window::new(config.window(), &event_manager);
    while running {
        event_manager.manage_events(running);
    }
}
