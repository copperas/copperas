use lib::config::Config;

pub struct EventManager {}

impl EventManager {
  pub fn new(config: &Config) -> EventManager {
      EventManager {}
  }

  pub fn manage(&self, event: glutin::DeviceEvent) {}
}
