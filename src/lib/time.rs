pub struct Time {
    delta: f64
}

impl Time {
    pub fn getDelta(&self) -> f64{
        self.delta
    }

    pub fn setDelta(&mut self, delta: f64) {
        self.delta = delta;
    }

    pub fn getTime() -> std::time::Instant {
        std::time::Instant::now()
    }
}
