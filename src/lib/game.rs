use lib::window::Window;
use glutin::{Event, EventsLoop, WindowEvent};

pub struct Game {
    events_loop: EventsLoop,
    window:      Window
}

impl Game {
    pub fn new() -> Game {
        const WIDTH:  f64    = 750.0;
        const HEIGHT: f64    = 468.75;
        let   title:  String = String::from("C O P P E R A S");

        println!("Create game loop");
        let events_loop = glutin::EventsLoop::new();

        println!("Create window");
        let window      = Window::new(WIDTH, HEIGHT, &title, &events_loop);

        Game { events_loop: events_loop, window: window }
    }

    pub fn start(&self) {
        // self.run()
    }

    pub fn stop(&self) {

    }

    fn run(&mut self) {
        let mut running = true;
        while running {
            self.events_loop.poll_events(|event| {
                match event {
                    Event::WindowEvent{ event, .. } => match event {
                        WindowEvent::CloseRequested  => running = false,
                        WindowEvent::Resized(l_size) => self.window.resize(l_size),
                        _ => ()
                    },
                    _ => ()
                }
            })
        }
    }

    fn render(&self) {}
}
