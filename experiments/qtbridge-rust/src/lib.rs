use pong_core::{Game, State};
/// Qt-independent controller; Qt renders snapshots and never duplicates physics.
pub struct Controller {
    pub game: Game,
    accumulator: f32,
}
impl Default for Controller {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            accumulator: 0.0,
        }
    }
}
impl Controller {
    pub fn tick(&mut self, elapsed: f32, axis: f32) {
        if !elapsed.is_finite() {
            return;
        }
        self.accumulator += elapsed.clamp(0.0, 0.1);
        while self.accumulator >= 1.0 / 120.0 {
            self.game.step(1.0 / 120.0, axis.clamp(-1.0, 1.0));
            self.accumulator -= 1.0 / 120.0;
        }
    }
    pub fn start(&mut self) {
        self.game.start();
        self.accumulator = 0.0;
    }
    pub fn toggle(&mut self) {
        if matches!(self.game.state, State::Ready | State::Over) {
            self.start();
        } else {
            self.game.toggle_pause();
        }
    }
    pub fn pause(&mut self) {
        self.game.pause();
        self.accumulator = 0.0;
    }
    pub fn snapshot(&self) -> Vec<f64> {
        vec![
            self.game.player as f64,
            self.game.cpu as f64,
            self.game.ball.x as f64,
            self.game.ball.y as f64,
            self.game.scores[0] as f64,
            self.game.scores[1] as f64,
            match self.game.state {
                State::Ready => 0.0,
                State::Playing => 1.0,
                State::Paused => 2.0,
                State::Over => 3.0,
            },
        ]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_step_matches_core() {
        let mut c = Controller::default();
        c.start();
        let mut g = c.game.clone();
        c.tick(1.0 / 120.0, 1.0);
        g.step(1.0 / 120.0, 1.0);
        assert_eq!(c.game.player, g.player);
        assert_eq!(c.game.ball.x, g.ball.x);
    }
    #[test]
    fn pause_and_restart() {
        let mut c = Controller::default();
        c.start();
        c.pause();
        let y = c.game.player;
        c.tick(0.1, 1.0);
        assert_eq!(y, c.game.player);
        c.game.scores = [3, 2];
        c.start();
        assert_eq!(c.game.scores, [0, 0]);
    }
    #[test]
    fn invalid_time_is_ignored_and_stalls_bounded() {
        let mut c = Controller::default();
        c.start();
        let y = c.game.player;
        c.tick(f32::NAN, 1.0);
        assert_eq!(y, c.game.player);
        let mut d = Controller::default();
        d.start();
        c.tick(10.0, 1.0);
        d.tick(0.1, 1.0);
        assert_eq!(c.snapshot(), d.snapshot());
    }
}
