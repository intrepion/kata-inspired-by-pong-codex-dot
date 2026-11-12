use std::time::Instant;

/// Piston's event loop can discard scheduled updates under rendering load.
/// Measure real elapsed time instead of counting UpdateArgs::dt values.
/// Bound catch-up after stalls, and clear elapsed/debt on user state transitions.
pub struct UpdateClock {
    last: Instant,
    accumulator: f64,
}
impl UpdateClock {
    pub fn new(now: Instant) -> Self {
        Self {
            last: now,
            accumulator: 0.0,
        }
    }
    pub fn elapsed(&mut self, now: Instant) -> f64 {
        let elapsed = now.saturating_duration_since(self.last).as_secs_f64();
        self.last = now;
        elapsed.min(0.1)
    }
    pub fn reset(&mut self, now: Instant) {
        self.last = now;
        self.accumulator = 0.0;
    }
    pub fn steps(&mut self, elapsed: f64) -> usize {
        self.accumulator += elapsed.clamp(0.0, 0.1);
        let count = ((self.accumulator + 1e-12) * 120.0).floor() as usize;
        self.accumulator = (self.accumulator - count as f64 / 120.0).max(0.0);
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn slow_render_callbacks_do_not_drop_simulation_time() {
        let start = Instant::now();
        let mut clock = UpdateClock::new(start);
        let mut demo = pong_scenario::Demo::new(None);
        let mut game = pong_core::Game::new(pong_scenario::SEED);
        // Piston may emit one fixed 1/120 update after each 50ms render.
        for frame in 1..=240 {
            demo.advance(
                &mut game,
                clock.elapsed(start + Duration::from_millis(frame * 50)) as f32,
            );
        }
        assert!(
            demo.tick() >= 1439,
            "12 wall seconds advanced only {} ticks",
            demo.tick()
        );
    }

    #[test]
    fn slow_updates_keep_every_capture_checkpoint() {
        let start = Instant::now();
        let mut clock = UpdateClock::new(start);
        let mut demo = pong_scenario::Demo::new(Some("unused-test-output".into()));
        let mut game = pong_core::Game::new(pong_scenario::SEED);
        let mut captures = Vec::new();
        for frame in 1..=300 {
            demo.advance(
                &mut game,
                clock.elapsed(start + Duration::from_millis(frame * 50)) as f32,
            );
            if let Some(path) = demo.capture_path() {
                captures.push(path);
            }
            if demo.finished() {
                break;
            }
        }
        assert!(demo.finished());
        assert_eq!(captures.len(), pong_scenario::CAPTURE_TICKS.len());
        for (path, tick) in captures.iter().zip(pong_scenario::CAPTURE_TICKS.iter()) {
            assert_eq!(
                path.file_name().unwrap().to_str().unwrap(),
                format!("tick-{tick:04}.png")
            );
        }
    }
    #[test]
    fn slow_callbacks_match_normal_gameplay_fixed_steps() {
        let start = Instant::now();
        let mut clock = UpdateClock::new(start);
        let mut slow = pong_core::Game::new(42);
        let mut reference = pong_core::Game::new(42);
        slow.start();
        reference.start();
        for frame in 1..=20 {
            let dt = clock.elapsed(start + Duration::from_millis(frame * 50));
            for _ in 0..clock.steps(dt) {
                slow.step(1.0 / 120.0, 0.0);
            }
        }
        for _ in 0..120 {
            reference.step(1.0 / 120.0, 0.0);
        }
        assert_eq!(slow.ball.x, reference.ball.x);
        assert_eq!(slow.ball.y, reference.ball.y);
    }
    #[test]
    fn long_stall_has_bounded_catchup_and_no_debt() {
        let start = Instant::now();
        let mut clock = UpdateClock::new(start);
        let dt = clock.elapsed(start + Duration::from_secs(60));
        assert_eq!(clock.steps(dt), 12);
        let dt = clock.elapsed(start + Duration::from_secs(60));
        assert_eq!(clock.steps(dt), 0);
    }
    #[test]
    fn pause_and_resume_discard_elapsed_and_partial_step() {
        let start = Instant::now();
        let mut clock = UpdateClock::new(start);
        let mut game = pong_core::Game::new(42);
        game.start();
        assert_eq!(clock.steps(0.004), 0);
        game.pause();
        clock.reset(start);
        let x = game.ball.x;
        let dt = clock.elapsed(start + Duration::from_secs(30));
        for _ in 0..clock.steps(dt) {
            game.step(1.0 / 120.0, 1.0);
        }
        assert_eq!(game.ball.x, x);
        game.toggle_pause();
        clock.reset(start + Duration::from_secs(60));
        let dt = clock.elapsed(start + Duration::from_secs(60));
        assert_eq!(clock.steps(dt), 0);
        let dt = clock.elapsed(start + Duration::from_secs(60) + Duration::from_millis(4));
        assert_eq!(clock.steps(dt), 0);
        let dt = clock.elapsed(start + Duration::from_secs(60) + Duration::from_millis(9));
        assert_eq!(clock.steps(dt), 1);
    }
}
