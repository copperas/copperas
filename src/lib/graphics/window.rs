use lib::core::config::Window as WindowConfig;
use lib::core::event_manager::EventManager;

pub fn new(config: &WindowConfig, em: &EventManager) -> winit::Window {
    let el = em.events_loop();
    let wb = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(config.width(), config.height()))
        .with_title(config.title());
    wb.build(el).unwrap()
}
