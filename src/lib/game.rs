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

    pub fn start(&mut self) {
        self.run();
    }

    pub fn stop(&self) {

    }


    fn get_events_loop_and_window(&mut self) -> (&mut EventsLoop, &mut Window) {
        (&mut self.events_loop, &mut self.window)
    }

    fn run(&mut self) {
        let (events_loop, window) = self.get_events_loop_and_window();
        Game::render(events_loop, window)
    }

    fn render(events_loop: &mut EventsLoop, window: &mut Window) {
        window.prepare();
        let mut running = true;
        while running {
            events_loop.poll_events(|event| {
                match event {
                    Event::WindowEvent{ event, .. } => match event {
                        WindowEvent::CloseRequested  => running = false,
                        WindowEvent::Resized(l_size) => window.resize(l_size),
                        _ => ()
                    },
                    _ => ()
                }
            });
            window.clear();
        }
        window.swap_buffers();
    }
}
