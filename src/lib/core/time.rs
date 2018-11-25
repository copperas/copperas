use std::time::{ Duration, Instant };

pub struct ClockManager {
    delta:       Duration,
    cycle_start: Instant
}

impl ClockManager {
    pub fn new(u_delta: u64) -> ClockManager {
        ClockManager {
            delta:       Duration::from_micros(u_delta),
            cycle_start: Instant::now()
        }
    }

    pub fn start_cycle(&mut self) {
        self.cycle_start = Instant::now();
    }

    pub fn cycle_ended(&self) -> bool {
        Instant::now().duration_since(self.cycle_start) >= self.delta
    }
}
