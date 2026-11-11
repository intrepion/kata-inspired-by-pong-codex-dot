//! Shared presentation/controller only. All physics live in pong-core.
use pong_core::{Game, State};
#[derive(Clone)]
pub struct Session {
    pub game: Game,
    up: bool,
    down: bool,
    accumulator: f64,
}
impl Default for Session {
    fn default() -> Self {
        Self {
            game: Game::new(42),
            up: false,
            down: false,
            accumulator: 0.0,
        }
    }
}
impl Session {
    pub fn key(&mut self, key: &str, pressed: bool, repeat: bool) {
        match key {
            "ArrowUp" | "w" | "W" => self.up = pressed,
            "ArrowDown" | "s" | "S" => self.down = pressed,
            "Enter" if pressed && !repeat => self.start(),
            " " | "Escape" if pressed && !repeat => self.game.toggle_pause(),
            _ => {}
        }
    }
    pub fn start(&mut self) {
        self.game.start();
        self.accumulator = 0.0;
    }
    pub fn pause(&mut self) {
        self.game.pause();
        self.up = false;
        self.down = false;
        self.accumulator = 0.0;
    }
    pub fn tick(&mut self, dt: f64) {
        if !dt.is_finite() || dt < 0.0 {
            return;
        }
        self.accumulator += dt.min(0.1);
        while self.accumulator >= 1.0 / 120.0 {
            self.game
                .step(1.0 / 120.0, self.down as u8 as f32 - self.up as u8 as f32);
            self.accumulator -= 1.0 / 120.0;
        }
    }
    pub fn svg(&self) -> String {
        let g = &self.game;
        let status = match g.state {
            State::Ready => "Press Enter or Start",
            State::Playing => "First to seven",
            State::Paused => "Paused • Space to resume",
            State::Over => {
                if g.scores[0] >= 7 {
                    "You win! Enter to play again"
                } else {
                    "CPU wins • Enter to play again"
                }
            }
        };
        format!(
            r##"<svg viewBox="0 0 900 500" role="img" aria-label="Pong court" xmlns="http://www.w3.org/2000/svg"><rect width="900" height="500" rx="10" fill="#101925"/><path d="M450 0V500" stroke="#52677d" stroke-width="3" stroke-dasharray="12 14"/><text x="365" y="64" text-anchor="middle" fill="white" font-size="42">{}</text><text x="535" y="64" text-anchor="middle" fill="white" font-size="42">{}</text><rect x="32" y="{}" width="12" height="90" rx="3" fill="#64e1cf"/><rect x="856" y="{}" width="12" height="90" rx="3" fill="#ffb86b"/><circle cx="{}" cy="{}" r="8" fill="white"/><text x="450" y="465" text-anchor="middle" fill="#e0e9f3" font-size="20">{status}</text></svg>"##,
            g.scores[0],
            g.scores[1],
            g.player - 45.0,
            g.cpu - 45.0,
            g.ball.x,
            g.ball.y
        )
    }
}
pub fn handled_key(k: &str) -> bool {
    matches!(
        k,
        "ArrowUp" | "ArrowDown" | "w" | "W" | "s" | "S" | "Enter" | " " | "Escape"
    )
}
pub const CSS:&str="body{margin:0;background:#080f19;color:#eef6ff;font:16px system-ui}main{max-width:1000px;margin:2rem auto;padding:0 20px;outline:none}h1{font-size:28px}svg{width:100%;display:block}button,a{color:inherit}button{background:#21354a;border:1px solid #7790a8;padding:12px 20px;margin:12px 8px 12px 0;border-radius:8px;cursor:pointer;touch-action:none}button:focus-visible{outline:3px solid #64e1cf}p{color:#c2d3e4}";
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeats_do_not_restart() {
        let mut s = Session::default();
        s.key("Enter", true, false);
        s.game.scores[0] = 3;
        s.key("Enter", true, true);
        assert_eq!(s.game.scores[0], 3);
    }
    #[test]
    fn blur_releases_input() {
        let mut s = Session::default();
        s.start();
        s.key("ArrowDown", true, false);
        s.pause();
        s.game.toggle_pause();
        s.tick(0.1);
        assert_eq!(s.game.player, 250.0);
    }
    #[test]
    fn fixed_steps_match() {
        let mut a = Session::default();
        a.start();
        let mut b = a.clone();
        for _ in 0..60 {
            a.tick(1.0 / 60.0);
        }
        for _ in 0..120 {
            b.tick(1.0 / 120.0);
        }
        assert!((a.game.ball.x - b.game.ball.x).abs() < 0.001);
    }
}
