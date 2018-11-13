use glutin::{Event, EventsLoop, WindowEvent};

use lib::config::Config;
use lib::event_manager::EventManager;
use lib::graphics;
use lib::window::Window;


static VS_SRC_PATH: &'static str = "./src/lib/graphics/shaders/triangle.vert";
static FS_SRC_PATH: &'static str = "./src/lib/graphics/shaders/triangle.frag";

pub struct Game<'a> {
    config:      &'a Config,
    events_loop: EventsLoop,
    window:      Window
}

impl<'a> Game<'a> {
    pub fn new(config: &mut Config) -> Game {
        let title: &str = config.get_window().get_title();
        let height: f64 = config.get_window().get_height();
        let width:  f64 = config.get_window().get_width();
        println!("Create game loop");
        let events_loop = glutin::EventsLoop::new();

        println!("Create window");
        let window      = Window::new(width, height, title, &events_loop);

        Game { config: config, events_loop: events_loop, window: window }
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
        // Load shaders
        let vs_src = graphics::load_shader(VS_SRC_PATH);
        let fs_src = graphics::load_shader(FS_SRC_PATH);

        // Create shaders
        let vs      = graphics::compile_shader(&vs_src, gl::VERTEX_SHADER);
        let fs      = graphics::compile_shader(&fs_src, gl::FRAGMENT_SHADER);
        let program = graphics::link_program(vs, fs);

        let vao = 0;
        let vbo = 0;

        graphics::draw_triangle(program, vao, vbo);

        let mut running = true;
        while running {
            events_loop.poll_events(|event| {
                println!("{:?}", event);
                match event {
                    Event::WindowEvent{ event, .. } => match event {
                        WindowEvent::CloseRequested  => running = false,
                        WindowEvent::Resized(l_size) => window.resize(l_size),
                        _ => ()
                    },
                    // Event::DeviceEvent{ event, .. } => event_manager::manage(event),
                    _ => ()
                }
            });
            window.clear();
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

            window.swap_buffers();
        }
        graphics::clean_up(program, fs, vs, vbo, vao);
    }
}
