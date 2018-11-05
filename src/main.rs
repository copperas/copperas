extern crate winit;
pub mod lib;

use winit::{EventsLoop, WindowBuilder, WindowEvent, Event, dpi, ControlFlow};

fn main() {
    const WIDTH: f64 = 750.0;
    const HEIGHT: f64 = 468.75;
    let title: String = String::from("C O P P E R A S");

    println!("Create game loop");
    let mut game_loop = EventsLoop::new();

    println!("Create window");
    let _window = WindowBuilder::new().with_dimensions(
        dpi::LogicalSize::new(WIDTH, HEIGHT)
    ).with_title(title).build(&game_loop).unwrap();
    game_loop.run_forever(|event| {
        println!("Listening to events");
        println!("{:?}", event);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => ControlFlow::Break,
            _ => ControlFlow::Continue
        }
    });

        println!("Load menu");


}
