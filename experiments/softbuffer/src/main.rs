use pong_low_level_common::App;
use std::{num::NonZeroU32, rc::Rc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
#[derive(Default)]
struct Frontend {
    focused: bool,
    app: App,
    movement_held: [bool; 4],
    demo: Option<pong_scenario::Demo>,
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    bench_scenario: pong_scenario::Scenario,
    bench_accumulator: f32,
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    last: Option<Instant>,
}
impl Frontend {
    fn movement_key(&mut self, key: KeyCode, pressed: bool) {
        let index = match key {
            KeyCode::KeyW => 0,
            KeyCode::ArrowUp => 1,
            KeyCode::KeyS => 2,
            KeyCode::ArrowDown => 3,
            _ => return,
        };
        self.movement_held[index] = pressed;
        self.app.up = self.movement_held[0] || self.movement_held[1];
        self.app.down = self.movement_held[2] || self.movement_held[3];
    }
    fn clear_movement(&mut self) {
        self.movement_held = [false; 4];
        self.app.blur();
    }
    fn handle_focused_key(
        &mut self,
        focused: bool,
        key: KeyCode,
        pressed: bool,
        repeat: bool,
    ) -> bool {
        focused && self.handle_key(key, pressed, repeat)
    }
    fn handle_key(&mut self, key: KeyCode, pressed: bool, repeat: bool) -> bool {
        if self.app.game.state == pong_low_level_common::State::Over
            && pressed
            && !repeat
            && matches!(key, KeyCode::Space | KeyCode::KeyR)
        {
            self.clear_movement();
        }
        match key {
            key @ (KeyCode::KeyW | KeyCode::ArrowUp | KeyCode::KeyS | KeyCode::ArrowDown) => {
                if !pressed
                    || (!repeat && self.app.game.state == pong_low_level_common::State::Playing)
                {
                    self.movement_key(key, pressed);
                }
            }
            KeyCode::Space if pressed && !repeat => {
                let was_playing = self.app.game.state == pong_low_level_common::State::Playing;
                self.app.action();
                if was_playing && self.app.game.state == pong_low_level_common::State::Paused {
                    self.clear_movement();
                }
            }
            KeyCode::KeyR if pressed && !repeat => {
                self.clear_movement();
                self.app.game.start();
            }
            KeyCode::Escape if pressed => return true,
            _ => {}
        }
        false
    }
}
impl ApplicationHandler for Frontend {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let w = Rc::new(
            el.create_window(
                Window::default_attributes()
                    .with_title("Pong softbuffer")
                    .with_inner_size(winit::dpi::LogicalSize::new(900., 500.)),
            )
            .unwrap(),
        );
        let ctx = softbuffer::Context::new(w.clone()).unwrap();
        self.surface = Some(softbuffer::Surface::new(&ctx, w.clone()).unwrap());
        if self.benchmarking {
            let size = w.inner_size();
            self.benchmark = Some(pong_benchmark::Frames::new(size.width, size.height));
            self.app.game = pong_low_level_common::Game::new(pong_scenario::SEED);
        }
        self.focused = w.has_focus();
        self.window = Some(w);
        self.last = Some(Instant::now());
    }
    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Focused(focused) => {
                self.focused = focused;
                if !focused && self.demo.is_none() && !self.benchmarking {
                    self.clear_movement();
                }
            }
            WindowEvent::KeyboardInput { event, .. }
                if self.demo.is_none() && !self.benchmarking =>
            {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(key) = event.physical_key {
                    if self.handle_focused_key(self.focused, key, pressed, event.repeat) {
                        el.exit();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let elapsed = self
                    .last
                    .replace(now)
                    .map_or(0., |t| (now - t).as_secs_f32());
                if self.benchmarking {
                    self.bench_accumulator += elapsed.clamp(0., 0.1);
                    while self.bench_accumulator >= pong_scenario::STEP {
                        if self.bench_scenario.finished() {
                            self.bench_scenario = pong_scenario::Scenario::default();
                            self.app.game = pong_low_level_common::Game::new(pong_scenario::SEED);
                        }
                        self.bench_scenario.advance(&mut self.app.game);
                        self.bench_accumulator -= pong_scenario::STEP;
                    }
                } else if let Some(demo) = &mut self.demo {
                    demo.advance(&mut self.app.game, elapsed);
                } else {
                    self.app.tick(elapsed);
                }
                let w = self.window.as_ref().unwrap();
                let size = w.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    let s = self.surface.as_mut().unwrap();
                    s.resize(width, height).unwrap();
                    let mut b = s.buffer_mut().unwrap();
                    pong_low_level_common::raster(&self.app, &mut b, size.width, size.height);
                    if let Some(path) = self.demo.as_mut().and_then(|d| d.capture_path()) {
                        pong_scenario::Demo::save_rgb(&path, &b, size.width, size.height);
                    }
                    b.present().unwrap();
                    // Callback boundary after CPU raster + present submission, not display time.
                    if let Some(bench) = &mut self.benchmark {
                        if bench.frame(size.width, size.height) {
                            println!("PONG_BENCHMARK {}", bench.report());
                            el.exit();
                        }
                    }
                }
                w.set_title(&self.app.title(if self.benchmarking {
                    "softbuffer / BENCHMARK"
                } else if self.demo.is_some() {
                    "softbuffer / SCRIPTED"
                } else {
                    "softbuffer / CPU"
                }));
                if self.demo.as_ref().is_some_and(|d| d.finished()) {
                    el.exit();
                }
            }
            _ => {}
        }
    }
    fn new_events(&mut self, _: &ActiveEventLoop, cause: winit::event::StartCause) {
        if matches!(
            cause,
            winit::event::StartCause::ResumeTimeReached { .. } | winit::event::StartCause::Init
        ) {
            if let Some(w) = &self.window {
                w.request_redraw();
            }
        }
    }
    fn about_to_wait(&mut self, el: &ActiveEventLoop) {
        el.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + std::time::Duration::from_millis(8),
        ));
    }
}
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    EventLoop::new()
        .unwrap()
        .run_app(&mut Frontend {
            demo: pong_scenario::Demo::from_args(),
            benchmarking,
            ..Frontend::default()
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn benchmark_is_opt_in() {
        assert!(!benchmark_args(&[]));
        assert!(!benchmark_args(&["--scripted-test".into()]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "benchmark and replay/capture modes are distinct")]
    fn rejects_capture_mixing() {
        benchmark_args(&["--benchmark".into(), "--scripted-captures".into()]);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut f = Frontend::default();
        f.app.game.start();
        f.app.game.scores = [3, 2];
        f.clear_movement();
        f.handle_focused_key(false, KeyCode::Space, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Paused);
        f.handle_focused_key(false, KeyCode::KeyR, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Paused);
        assert_eq!(f.app.game.scores, [3, 2]);
        f.handle_focused_key(true, KeyCode::Space, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
    }
    #[test]
    fn active_restart_suppresses_movement_until_fresh_press() {
        let mut f = Frontend::default();
        f.app.game.start();
        f.handle_key(KeyCode::KeyW, true, false);
        assert!(f.app.up);
        f.handle_key(KeyCode::KeyR, true, false);
        assert!(!f.app.up);
        f.handle_key(KeyCode::KeyW, true, true);
        assert!(!f.app.up);
        f.handle_key(KeyCode::KeyW, false, false);
        f.handle_key(KeyCode::KeyW, true, false);
        assert!(f.app.up);
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        use pong_low_level_common::{State, BALL, H, W};
        for key in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ] {
            for restart in [KeyCode::Space, KeyCode::KeyR] {
                let mut f = Frontend::default();
                f.app.game.start();
                f.handle_key(key, true, false);
                f.app.game.scores = [6, 2];
                f.app.game.serve = 0.;
                f.app.game.ball.x = W + BALL - 1.;
                f.app.game.ball.y = H / 2.;
                f.app.game.ball.vx = 350.;
                f.app.game.ball.vy = 0.;
                f.app.game.step(0.02, 0.);
                assert_eq!(f.app.game.state, State::Over);
                f.handle_key(restart, true, false);
                assert_eq!(f.app.game.state, State::Playing);
                assert!(!f.app.up && !f.app.down, "{key:?} {restart:?}");
                f.handle_key(key, true, true);
                assert!(!f.app.up && !f.app.down);
                f.handle_key(key, false, false);
                f.handle_key(key, true, false);
                assert!(f.app.up || f.app.down);
            }
        }
    }

    #[test]
    fn nonplaying_presses_do_not_rearm_after_start() {
        use pong_low_level_common::State;
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [
                KeyCode::KeyW,
                KeyCode::ArrowUp,
                KeyCode::KeyS,
                KeyCode::ArrowDown,
            ] {
                let mut f = Frontend::default();
                f.app.game.state = state;
                f.handle_key(key, true, false);
                f.handle_key(KeyCode::Space, true, false);
                assert_eq!(f.app.game.state, State::Playing);
                assert!(!f.app.up && !f.app.down, "{state:?} {key:?}");
                f.handle_key(key, true, true);
                assert!(!f.app.up && !f.app.down);
                f.handle_key(key, false, false);
                f.handle_key(key, true, false);
                assert!(f.app.up || f.app.down);
            }
        }
    }

    #[test]
    fn explicit_pause_clears_movement_until_fresh_press() {
        for key in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ] {
            let mut f = Frontend::default();
            f.handle_key(KeyCode::Space, true, false);
            f.handle_key(key, true, false);
            assert!(f.app.up || f.app.down);
            f.handle_key(KeyCode::Space, true, false);
            assert_eq!(f.app.game.state, pong_low_level_common::State::Paused);
            assert!(!f.app.up && !f.app.down);
            f.handle_key(KeyCode::Space, true, false);
            assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
            f.handle_key(key, true, true);
            assert!(!f.app.up && !f.app.down, "repeat restored held movement");
            f.handle_key(key, false, false);
            f.handle_key(key, true, false);
            assert!(f.app.up || f.app.down, "fresh press should move");
        }
    }
    #[test]
    fn action_keys_preserve_start_pause_restart_and_quit() {
        let mut f = Frontend::default();
        assert!(!f.handle_key(KeyCode::Space, false, false));
        assert_eq!(f.app.game.state, pong_low_level_common::State::Ready);
        f.handle_key(KeyCode::Space, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
        f.handle_key(KeyCode::Space, true, true);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
        f.handle_key(KeyCode::Space, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Paused);
        f.handle_key(KeyCode::Space, true, false);
        assert_eq!(f.app.game.state, pong_low_level_common::State::Playing);
        f.app.game.scores = [2, 4];
        f.handle_key(KeyCode::KeyR, true, false);
        assert_eq!(f.app.game.scores, [0, 0]);
        assert!(f.handle_key(KeyCode::Escape, true, false));
    }
    #[test]
    fn aliases_remain_active_until_each_binding_releases() {
        for (a, b, up) in [
            (KeyCode::KeyW, KeyCode::ArrowUp, true),
            (KeyCode::KeyS, KeyCode::ArrowDown, false),
        ] {
            for (first, second) in [(a, b), (b, a)] {
                let mut f = Frontend::default();
                f.movement_key(first, true);
                f.movement_key(second, true);
                f.movement_key(first, false);
                assert!(
                    if up { f.app.up } else { f.app.down },
                    "other alias is still held"
                );
                f.movement_key(second, false);
                assert!(!f.app.up && !f.app.down);
            }
        }
    }
    #[test]
    fn repeats_opposites_and_focus_clear_are_safe() {
        let mut f = Frontend::default();
        f.movement_key(KeyCode::KeyW, true);
        f.movement_key(KeyCode::KeyW, true);
        f.movement_key(KeyCode::KeyW, false);
        assert!(!f.app.up);
        f.movement_key(KeyCode::ArrowUp, true);
        f.movement_key(KeyCode::KeyS, true);
        assert!(f.app.up && f.app.down);
        f.clear_movement();
        assert!(!f.app.up && !f.app.down);
        f.movement_key(KeyCode::KeyW, true);
        f.movement_key(KeyCode::KeyW, false);
        assert!(
            !f.app.up && !f.app.down,
            "blur must also clear alias storage"
        );
    }
}
