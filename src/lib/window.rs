use glutin::{ContextBuilder, dpi::*, EventsLoop, GlContext, GlWindow, WindowBuilder};

pub struct Window {
    window: GlWindow
}

impl Window {
    pub fn new(width: f64, height: f64, title: &str, gloop: &EventsLoop) -> Window {
        let sizes   = LogicalSize::new(width, height);
        let builder = WindowBuilder::new().with_title(title)
                                          .with_dimensions(sizes);
        let window = GlWindow::new(builder, ContextBuilder::new(), gloop).unwrap();
        Window { window: window }
    }

    pub fn resize(&self, l_size: LogicalSize) {
        let dpi_factor = self.window.get_hidpi_factor();
        self.window.resize(l_size.to_physical(dpi_factor));
    }
}
