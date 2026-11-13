use pong_qtbridge::Controller;
use qtbridge::qobject;
use std::time::Instant;
pub struct Backend {
    controller: Controller,
    last: Instant,
}
impl Default for Backend {
    fn default() -> Self {
        Self {
            controller: Controller::default(),
            last: Instant::now(),
        }
    }
}
#[qobject(Singleton)]
impl Backend {
    #[qslot]
    fn frame(&mut self, axis: f64) -> Vec<f64> {
        let now = Instant::now();
        self.controller
            .tick(now.duration_since(self.last).as_secs_f32(), axis as f32);
        self.last = now;
        self.controller.snapshot()
    }
    #[qslot]
    fn start(&mut self) {
        self.controller.start();
        self.last = Instant::now();
    }
    #[qslot]
    fn toggle(&mut self) {
        self.controller.toggle();
        self.last = Instant::now();
    }
    #[qslot]
    fn pause(&mut self) {
        self.controller.pause();
        self.last = Instant::now();
    }
}
