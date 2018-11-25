use lib::core::config::Window as WindowConfig;

pub fn new(config: &WindowConfig, el: &winit::EventsLoop) -> winit::Window {
    let wb = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(config.width(), config.height()))
        .with_title(config.title());
    wb.build(el).unwrap()
}
