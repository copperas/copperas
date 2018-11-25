pub mod config;
pub mod event_manager;
pub mod menu;
pub mod time;

use self::config::Config;
use self::event_manager::EventManager;
use self::time::ClockManager;

use lib::graphics;

pub fn run(config_path: &str) {
    let mut config           = Config::new(config_path);
    let mut clock_manager    = ClockManager::new(config.time().delta());
    let mut event_manager    = EventManager::new(config.controlls());
    let mut window           = graphics::window::new(config.window());
    loop {

    }
}
