//! Opt-in application-level input replay. Never injects operating-system input.
use pong_core::Game;

pub const STEP: f32 = 1.0 / 120.0;
pub const END_TICK: u32 = 1440;
pub const SEED: u64 = 42;

#[derive(Default)]
pub struct Scenario {
    tick: u32,
}

impl Scenario {
    pub fn tick(&self) -> u32 {
        self.tick
    }
    pub fn finished(&self) -> bool {
        self.tick >= END_TICK
    }

    /// Applies events before this fixed simulation step. The clock advances even
    /// while paused; callers must initialize Game with SEED for reproducibility.
    pub fn advance(&mut self, game: &mut Game) {
        if self.finished() {
            return;
        }
        match self.tick {
            0 | 1200 => game.start(),
            360 => game.pause(),
            480 => game.toggle_pause(),
            _ => {}
        }
        let input = match self.tick {
            0..=119 => -1.0,
            120..=239 => 1.0,
            _ => 0.0,
        };
        game.step(STEP, input);
        self.tick += 1;
    }

    /// JSONL telemetry is state evidence, not evidence of rendered correctness.
    pub fn report(&self, game: &Game) -> String {
        format!("{{\"scenario\":\"smoke-v1\",\"seed\":{SEED},\"tick\":{},\"state\":\"{:?}\",\"player\":{},\"cpu\":{},\"ball\":[{},{},{},{}],\"scores\":[{},{}],\"finished\":{}}}",
            self.tick, game.state, game.player, game.cpu, game.ball.x, game.ball.y,
            game.ball.vx, game.ball.vy, game.scores[0], game.scores[1], self.finished())
    }
}
