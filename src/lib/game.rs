use std::time::Instant;

use lib::core::config::Config;
use lib::core::event_manager::EventManager;




static VS_SRC_PATH: &'static str = "./src/lib/graphics/shaders/triangle.vert";
static FS_SRC_PATH: &'static str = "./src/lib/graphics/shaders/triangle.frag";

pub struct Core<'a> {
    config:        &'a Config,
    events_loop:   EventsLoop,
    event_manager: EventManager
}

impl<'a> Core<'a> {
    pub fn new(config: &mut Config) -> Core {
        let title: &str   = config.get_window().get_title();
        let height: f64   = config.get_window().get_height();
        let width:  f64   = config.get_window().get_width();
        println!("Create game loop");
        let events_loop   = glutin::EventsLoop::new();
        let event_manager = EventManager::new(&config);

        Core {
            config: config, events_loop: events_loop, event_manager: event_manager
        }
    }

    pub fn start(&mut self) {
        self.run();
    }

    fn run(&mut self) {
    }
}
