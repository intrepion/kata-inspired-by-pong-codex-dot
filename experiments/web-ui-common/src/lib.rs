//! Shared presentation/controller only. All physics live in pong-core.
/// Borrowed exports from the normal scene renderer, available only after replay completion.
/// SVG source is not a screenshot or proof of webview presentation.
pub struct DemoExports<'a> {
    pub reports: &'a [String],
    pub captures: &'a [(u32, String)],
}

use pong_core::{Game, State};
use std::collections::HashSet;
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Clone)]
pub struct Session {
    pub game: Game,
    held: [bool; 4], // Arrow and W/S keyboard bindings have independent identities.
    pointers: [HashSet<i32>; 2],
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
            held: [false; 4],
            pointers: Default::default(),
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
            "ArrowUp" if !pressed || (!repeat && self.game.state == State::Playing) => {
                self.held[0] = pressed
            }
            "w" | "W" if !pressed || (!repeat && self.game.state == State::Playing) => {
                self.held[1] = pressed
            }
            "ArrowDown" if !pressed || (!repeat && self.game.state == State::Playing) => {
                self.held[2] = pressed
            }
            "s" | "S" if !pressed || (!repeat && self.game.state == State::Playing) => {
                self.held[3] = pressed
            }
            "Enter" if pressed && !repeat => self.start(),
            " " | "Escape" if pressed && !repeat => self.toggle_pause(),
            _ => {}
        }
    }
    /// On-screen movement controls are separate from keyboard bindings.
    pub fn movement_button(&mut self, direction: Direction, pointer_id: i32, pressed: bool) {
        if self.demo.is_none() {
            let pointers = &mut self.pointers[match direction {
                Direction::Up => 0,
                Direction::Down => 1,
            }];
            if pressed {
                if self.game.state == State::Playing {
                    pointers.insert(pointer_id);
                }
            } else {
                pointers.remove(&pointer_id);
            }
        }
    }
    pub fn start(&mut self) {
        if self.demo.is_some() {
            return;
        }
        if self.game.state == State::Over {
            self.held = [false; 4];
            self.pointers.iter_mut().for_each(HashSet::clear);
        }
        self.game.start();
        self.accumulator = 0.0;
    }
    /// Ignore focus transitions within the game controls; pause when focus leaves them.
    pub fn focus_left(&mut self, next_inside: bool) {
        if !next_inside {
            self.pause();
        }
    }
    pub fn pause(&mut self) {
        if self.demo.is_some() {
            return;
        }
        self.game.pause();
        self.held = [false; 4];
        self.pointers.iter_mut().for_each(HashSet::clear);
        self.accumulator = 0.0;
    }
    pub fn toggle_pause(&mut self) {
        if self.demo.is_some() {
            return;
        }
        if self.game.state == State::Playing {
            self.pause();
        } else {
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
            self.game.step(
                1.0 / 120.0,
                (self.held[2] || self.held[3] || !self.pointers[1].is_empty()) as u8 as f32
                    - (self.held[0] || self.held[1] || !self.pointers[0].is_empty()) as u8 as f32,
            );
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
    fn held_inputs_do_not_carry_through_match_end_restart() {
        use pong_core::{Ball, BALL, H, W};
        for pointer in [false, true] {
            let mut s = Session::default();
            s.start();
            if pointer {
                s.movement_button(Direction::Up, 7, true);
            } else {
                s.key("w", true, false);
            }
            s.game.scores = [6, 2];
            s.game.serve = 0.;
            s.game.ball = Ball {
                x: W + BALL - 1.,
                y: H / 2.,
                vx: 350.,
                vy: 0.,
            };
            s.tick(0.02);
            assert_eq!(s.game.state, State::Over);
            s.start();
            if !pointer {
                s.key("w", true, true);
            }
            s.tick(0.02);
            assert_eq!(s.game.player, H / 2., "pointer={pointer}");
            if pointer {
                s.movement_button(Direction::Up, 7, false);
                s.movement_button(Direction::Up, 8, true);
            } else {
                s.key("w", false, false);
                s.key("w", true, false);
            }
            s.tick(0.02);
            assert_ne!(s.game.player, H / 2.);
        }
    }

    #[test]
    fn nonplaying_inputs_wait_for_release_and_fresh_press() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in ["ArrowUp", "w", "ArrowDown", "s"] {
                let mut s = Session::default();
                s.game.state = state;
                s.key(key, true, false);
                if state == State::Paused {
                    s.toggle_pause();
                } else {
                    s.start();
                }
                s.key(key, true, true);
                s.tick(0.05);
                assert_eq!(s.game.player, 250., "{state:?} {key}");
                s.key(key, false, false);
                s.key(key, true, false);
                s.tick(0.05);
                assert_ne!(s.game.player, 250.);
            }
            for direction in [Direction::Up, Direction::Down] {
                let mut s = Session::default();
                s.game.state = state;
                s.movement_button(direction, 7, true);
                if state == State::Paused {
                    s.toggle_pause();
                } else {
                    s.start();
                }
                s.tick(0.05);
                assert_eq!(s.game.player, 250., "pointer {state:?}");
                s.movement_button(direction, 7, false);
                s.movement_button(direction, 8, true);
                s.tick(0.05);
                assert_ne!(s.game.player, 250.);
            }
        }
    }

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
    fn releasing_one_binding_keeps_other_binding_held() {
        let mut s = Session::default();
        s.start();
        s.key("w", true, false);
        s.key("ArrowUp", true, false);
        s.key("w", false, false);
        s.tick(0.05);
        assert!(s.game.player < 250.);
    }
    #[test]
    fn button_release_does_not_cancel_keyboard_and_keyboard_release_does_not_cancel_button() {
        for button_released_first in [true, false] {
            let mut s = Session::default();
            s.start();
            s.key("ArrowDown", true, false);
            s.movement_button(Direction::Down, 1, true);
            if button_released_first {
                s.movement_button(Direction::Down, 1, false);
            } else {
                s.key("ArrowDown", false, false);
            }
            s.tick(0.05);
            assert!(s.game.player > 250.);
            s.key("ArrowDown", false, false);
            s.movement_button(Direction::Down, 1, false);
            let stopped = s.game.player;
            s.tick(0.05);
            assert_eq!(s.game.player, stopped);
        }
    }
    #[test]
    fn opposite_sources_cancel_and_blur_clears_every_source() {
        let mut s = Session::default();
        s.start();
        s.key("w", true, false);
        s.movement_button(Direction::Down, 1, true);
        s.tick(0.05);
        assert_eq!(s.game.player, 250.);
        s.pause();
        s.toggle_pause();
        s.tick(0.05);
        assert_eq!(s.game.player, 250.);
    }
    #[test]
    fn case_changed_release_and_scripted_button_lockout() {
        let mut s = Session::default();
        s.start();
        s.key("s", true, false);
        s.key("S", false, false);
        s.tick(0.05);
        assert_eq!(s.game.player, 250.);
        s.enable_demo();
        s.movement_button(Direction::Up, 1, true);
        assert_eq!(s.held, [false; 4]);
        assert!(s.pointers.iter().all(HashSet::is_empty));
    }
    #[test]
    fn same_button_pointers_release_independently() {
        let mut s = Session::default();
        s.start();
        s.movement_button(Direction::Up, 7, true);
        s.movement_button(Direction::Up, 12, true);
        s.movement_button(Direction::Up, 7, false);
        s.tick(0.05);
        assert!(s.game.player < 250.);
        s.movement_button(Direction::Up, 12, false);
        let stopped = s.game.player;
        s.tick(0.05);
        assert_eq!(s.game.player, stopped);
    }
    #[test]
    fn unrelated_release_cannot_cancel_a_held_pointer() {
        let mut s = Session::default();
        s.start();
        s.movement_button(Direction::Down, 7, true);
        s.movement_button(Direction::Down, 99, false);
        s.tick(0.05);
        assert!(s.game.player > 250.);
    }
    #[test]
    fn opposing_pointers_cancel_and_pause_clears_them() {
        let mut s = Session::default();
        s.start();
        s.movement_button(Direction::Up, 1, true);
        s.movement_button(Direction::Down, 2, true);
        s.tick(0.05);
        assert_eq!(s.game.player, 250.);
        s.movement_button(Direction::Up, 1, false);
        s.tick(0.05);
        assert!(s.game.player > 250.);
        s.pause();
        s.toggle_pause();
        let stopped = s.game.player;
        s.tick(0.05);
        assert_eq!(s.game.player, stopped);
    }
    #[test]
    fn explicit_pause_clears_held_keyboard_and_pointer_inputs() {
        for pause_key in [None, Some(" "), Some("Escape")] {
            for pointer in [false, true] {
                let mut s = Session::default();
                s.start();
                if pointer {
                    s.movement_button(Direction::Down, 17, true);
                } else {
                    s.key("ArrowDown", true, false);
                }
                s.tick(0.05);
                assert!(s.game.player > 250.);
                if let Some(key) = pause_key {
                    s.key(key, true, false);
                } else {
                    s.toggle_pause();
                }
                assert_eq!(s.game.state, State::Paused);
                if let Some(key) = pause_key {
                    s.key(key, true, false);
                } else {
                    s.toggle_pause();
                }
                let stopped = s.game.player;
                s.tick(0.05);
                assert_eq!(
                    s.game.player, stopped,
                    "input persisted through explicit pause"
                );
                if pointer {
                    s.movement_button(Direction::Down, 18, true);
                } else {
                    s.key("ArrowDown", true, false);
                }
                s.tick(0.05);
                assert!(
                    s.game.player > stopped,
                    "fresh input did not move after resume"
                );
            }
        }
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

#[cfg(test)]
mod focus_tests {
    use super::*;
    #[test]
    fn focusing_an_internal_button_does_not_cancel_its_pause_click() {
        let mut session = Session::default();
        session.start();
        session.focus_left(true);
        session.toggle_pause();
        assert_eq!(session.game.state, State::Paused);
        session.toggle_pause();
        assert_eq!(session.game.state, State::Playing);
        session.key("w", true, false);
        session.focus_left(true);
        session.tick(0.05);
        assert!(
            session.game.player < 250.0,
            "internal focus preserves held movement"
        );
        session.focus_left(false);
        assert_eq!(session.game.state, State::Paused);
        session.toggle_pause();
        let stopped = session.game.player;
        session.tick(0.05);
        assert_eq!(
            session.game.player, stopped,
            "external focus loss clears held movement"
        );
    }
}
