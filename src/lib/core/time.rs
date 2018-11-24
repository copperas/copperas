pub struct Time {
    delta: f64
}

impl Time {
    pub fn get_delta(&self) -> f64{
        self.delta
    }

    pub fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    pub fn get_time() -> std::time::Instant {
        std::time::Instant::now()
    }
}
