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

    pub fn prepare(&mut self) {
        unsafe { self.window.make_current() }.unwrap();
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers().unwrap();
    }
}
