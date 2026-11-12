pub use pong_core::*;
use std::time::Instant;
pub struct Model {
    pub game: Game,
    pub up: bool,
    pub down: bool,
    last: Instant,
    accumulator: f32,
    held: std::collections::HashSet<String>,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            up: false,
            down: false,
            last: Instant::now(),
            accumulator: 0.,
            held: Default::default(),
        }
    }
}
impl Model {
    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f32();
        self.last = now;
        self.advance(dt);
    }
    pub fn advance(&mut self, dt: f32) {
        self.accumulator += dt.clamp(0., 0.1);
        while self.accumulator >= 1. / 120. {
            self.game
                .step(1. / 120., self.down as i32 as f32 - self.up as i32 as f32);
            self.accumulator -= 1. / 120.;
        }
    }
    pub fn blur(&mut self) {
        self.up = false;
        self.down = false;
        self.held.clear();
        self.game.pause();
        self.accumulator = 0.;
        self.last = Instant::now();
    }
    pub fn key(&mut self, key: &str, pressed: bool) {
        // Toolkit text can change case if Shift changes while a key is held.
        // Normalize logical identity before repeat suppression and release.
        let key = match key {
            "W" => "w",
            "S" => "s",
            "R" => "r",
            "Up" => "up",
            "Down" => "down",
            "Enter" | "Return" => "enter",
            " " => "space",
            "Escape" => "escape",
            other => other,
        };
        if pressed {
            if !self.held.insert(key.to_owned()) {
                return;
            }
        } else {
            self.held.remove(key);
        }
        // Each direction may have several physical bindings held at once.
        // Releasing one binding must not release the others.
        self.up = ["w", "up"].iter().any(|key| self.held.contains(*key));
        self.down = ["s", "down"].iter().any(|key| self.held.contains(*key));
        match key {
            "enter" if pressed => {
                if self.game.state == State::Paused {
                    self.game.toggle_pause()
                } else if self.game.state != State::Playing {
                    self.game.start()
                }
            }
            "space" if pressed => self.game.toggle_pause(),
            "r" if pressed => self.game.start(),
            "escape" if pressed => self.blur(),
            _ => {}
        }
    }
    pub fn status(&self) -> String {
        format!(
            "YOU {} : {} CPU   |   {:?}",
            self.game.scores[0], self.game.scores[1], self.game.state
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn focus_loss_clears_input_and_pauses() {
        let mut m = Model::default();
        m.key("Enter", true);
        m.key("w", true);
        m.blur();
        assert!(!m.up);
        assert_eq!(m.game.state, State::Paused);
    }
    #[test]
    fn released_keys_stop_paddle() {
        let mut m = Model::default();
        m.game.start();
        m.key("w", true);
        m.advance(0.1);
        let y = m.game.player;
        m.key("w", false);
        m.advance(0.1);
        assert_eq!(y, m.game.player);
    }
}

#[cfg(test)]
mod repeat_tests {
    use super::*;
    #[test]
    fn held_pause_does_not_toggle_repeatedly() {
        let mut m = Model::default();
        m.game.start();
        m.key("space", true);
        m.key("space", true);
        assert_eq!(m.game.state, State::Paused);
        m.key("space", false);
        m.key("space", true);
        assert_eq!(m.game.state, State::Playing);
    }
    #[test]
    fn catchup_is_bounded() {
        let mut a = Model::default();
        let mut b = Model::default();
        a.game.start();
        b.game.start();
        a.key("w", true);
        b.key("w", true);
        a.advance(100.);
        b.advance(0.1);
        assert_eq!(a.game.player, b.game.player);
        assert_eq!(a.game.ball.x, b.game.ball.x);
    }
}

#[cfg(test)]
mod alias_tests {
    use super::*;
    #[test]
    fn releasing_one_up_binding_keeps_other_held_binding_active() {
        let mut model = Model::default();
        model.key("Enter", true);
        model.key("w", true);
        model.key("Up", true);
        model.key("w", false);
        assert!(model.up);
        let before = model.game.player;
        model.advance(0.05);
        assert!(model.game.player < before);
        model.key("Up", false);
        assert!(!model.up);
    }
    #[test]
    fn releasing_one_down_binding_keeps_other_held_binding_active() {
        let mut model = Model::default();
        model.key("S", true);
        model.key("down", true);
        model.key("down", false);
        assert!(model.down);
        model.key("S", false);
        assert!(!model.down);
    }
}

#[cfg(test)]
mod key_identity_tests {
    use super::*;
    #[test]
    fn modifier_case_change_on_release_does_not_stick_movement() {
        for (press, release) in [("w", "W"), ("W", "w"), ("s", "S"), ("S", "s")] {
            let mut model = Model::default();
            model.game.start();
            model.key(press, true);
            model.advance(0.02);
            model.key(release, false);
            let stopped = model.game.player;
            model.advance(0.02);
            assert_eq!(model.game.player, stopped, "{press} -> {release}");
            assert!(!model.up && !model.down);
        }
    }
    #[test]
    fn case_changed_repeat_is_not_a_second_restart() {
        let mut model = Model::default();
        model.key("r", true);
        model.game.scores = [3, 2];
        model.key("R", true);
        assert_eq!(model.game.scores, [3, 2]);
        model.key("R", false);
        model.key("r", true);
        assert_eq!(model.game.scores, [0, 0]);
    }
}
