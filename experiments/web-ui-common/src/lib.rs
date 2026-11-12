//! Shared presentation/controller only. All physics live in pong-core.
/// Borrowed exports from the normal scene renderer, available only after replay completion.
/// SVG source is not a screenshot or proof of webview presentation.
pub struct DemoExports<'a> {
    pub reports: &'a [String],
    pub captures: &'a [(u32, String)],
}

use pong_core::{Game, State};
#[derive(Clone)]
pub struct Session {
    pub game: Game,
    up: bool,
    down: bool,
    accumulator: f64,
    demo: Option<Replay>,
}
#[derive(Clone, Default)]
struct Replay {
    scenario: pong_scenario::Scenario,
    reports: Vec<String>,
    captures: Vec<(u32, String)>,
}
fn query_requests_demo() -> bool {
    #[cfg(target_arch = "wasm32")]
    return web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map(|q| {
            q.trim_start_matches('?')
                .split('&')
                .any(|v| v.split('=').next() == Some("scripted-test"))
        })
        .unwrap_or(false);
    #[cfg(not(target_arch = "wasm32"))]
    false
}
impl Default for Session {
    fn default() -> Self {
        let mut session = Self {
            game: Game::new(42),
            up: false,
            down: false,
            accumulator: 0.0,
            demo: None,
        };
        if query_requests_demo() {
            session.enable_demo();
        }
        session
    }
}
impl Session {
    pub fn key(&mut self, key: &str, pressed: bool, repeat: bool) {
        if self.demo.is_some() {
            return;
        }
        match key {
            "ArrowUp" | "w" | "W" => self.up = pressed,
            "ArrowDown" | "s" | "S" => self.down = pressed,
            "Enter" if pressed && !repeat => self.start(),
            " " | "Escape" if pressed && !repeat => self.game.toggle_pause(),
            _ => {}
        }
    }
    pub fn start(&mut self) {
        if self.demo.is_some() {
            return;
        }
        self.game.start();
        self.accumulator = 0.0;
    }
    pub fn pause(&mut self) {
        if self.demo.is_some() {
            return;
        }
        self.game.pause();
        self.up = false;
        self.down = false;
        self.accumulator = 0.0;
    }
    pub fn toggle_pause(&mut self) {
        if self.demo.is_none() {
            self.game.toggle_pause();
        }
    }
    pub fn completed_demo_exports(&self) -> Option<DemoExports<'_>> {
        self.demo
            .as_ref()
            .filter(|d| d.scenario.finished())
            .map(|d| DemoExports {
                reports: &d.reports,
                captures: &d.captures,
            })
    }
    pub fn enable_demo(&mut self) {
        self.game = Game::new(pong_scenario::SEED);
        self.accumulator = 0.0;
        let mut replay = Replay::default();
        replay.reports.push(replay.scenario.report(&self.game));
        self.demo = Some(replay);
    }
    pub fn tick(&mut self, dt: f64) {
        if !dt.is_finite() || dt < 0.0 {
            return;
        }
        if self.demo.as_ref().is_some_and(|d| d.scenario.finished()) {
            return;
        }
        self.accumulator += dt.min(0.1);
        if self.demo.is_some() {
            while self.accumulator >= 1.0 / 120.0 {
                let demo = self.demo.as_mut().unwrap();
                demo.scenario.advance(&mut self.game);
                self.accumulator -= 1.0 / 120.0;
                let tick = demo.scenario.tick();
                if tick % 120 == 0 {
                    demo.reports.push(demo.scenario.report(&self.game));
                }
                if pong_scenario::CAPTURE_TICKS.contains(&tick) {
                    let svg = self.court_svg();
                    self.demo.as_mut().unwrap().captures.push((tick, svg));
                    break;
                }
                if demo.scenario.finished() {
                    break;
                }
            }
            return;
        }
        while self.accumulator >= 1.0 / 120.0 {
            self.game
                .step(1.0 / 120.0, self.down as u8 as f32 - self.up as u8 as f32);
            self.accumulator -= 1.0 / 120.0;
        }
    }
    pub fn svg(&self) -> String {
        let mut output = self.court_svg();
        if let Some(demo) = &self.demo {
            output.push_str(&format!("<p>Scripted replay: tick {} / {}. Inputs disabled. SVG exports are app-generated scene markup, not browser screenshots.</p>", demo.scenario.tick(), pong_scenario::END_TICK));
            if demo.scenario.finished() {
                output.push_str(&download(
                    "telemetry.jsonl",
                    "application/x-ndjson",
                    &demo.reports.join("\n"),
                ));
                for (tick, svg) in &demo.captures {
                    output.push_str(&download(
                        &format!("tick-{tick:04}.svg"),
                        "image/svg+xml",
                        svg,
                    ));
                }
            }
        }
        output
    }
    fn court_svg(&self) -> String {
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
    fn shared_demo_is_bounded_and_exports_six_scenes() {
        let mut s = Session::default();
        assert!(s.completed_demo_exports().is_none());
        s.enable_demo();
        assert!(s.completed_demo_exports().is_none());
        let mut expected = Game::new(pong_scenario::SEED);
        let mut reference = pong_scenario::Scenario::default();
        while !reference.finished() {
            reference.advance(&mut expected);
        }
        for _ in 0..1000 {
            s.tick(0.1);
            s.start();
            s.pause();
            s.toggle_pause();
            s.key("ArrowUp", true, false);
        }
        let demo = s.demo.as_ref().unwrap();
        assert_eq!(demo.scenario.tick(), pong_scenario::END_TICK);
        assert_eq!(demo.reports.len(), 13);
        assert_eq!(demo.reports.last().unwrap(), &reference.report(&expected));
        assert_eq!(
            demo.captures.iter().map(|c| c.0).collect::<Vec<_>>(),
            pong_scenario::CAPTURE_TICKS
        );
        assert!(demo
            .captures
            .iter()
            .all(|(_, svg)| svg.starts_with("<svg") && !svg.contains("Download")));
        assert_eq!(s.svg().matches("download=").count(), 7);
        let exported = s.completed_demo_exports().unwrap();
        assert_eq!(exported.reports, demo.reports);
        assert_eq!(exported.captures, demo.captures);
    }
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

fn download(name: &str, mime: &str, text: &str) -> String {
    let encoded: String = text
        .as_bytes()
        .iter()
        .map(|b| format!("%{b:02X}"))
        .collect();
    format!("<p><a download=\"{name}\" href=\"data:{mime};charset=utf-8,{encoded}\">Download {name}</a></p>")
}
