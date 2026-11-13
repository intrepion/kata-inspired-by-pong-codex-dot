use pong_low_level_common::App;
mod renderer;
use std::{rc::Rc, time::Instant};
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
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    benchmark_done: bool,
    scenario: pong_scenario::Scenario,
    accumulator: f32,
    demo: Option<pong_scenario::Demo>,
    pending_capture: Option<std::path::PathBuf>,
    capture_enabled: bool,
    renderer: Option<renderer::Renderer>,
    window: Option<Rc<Window>>,
    last: Option<Instant>,
    failed: bool,
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
                    .with_title("Pong Vulkan ash")
                    .with_inner_size(winit::dpi::LogicalSize::new(900., 500.)),
            )
            .unwrap(),
        );
        match unsafe { renderer::Renderer::new(&w, self.capture_enabled) } {
            Ok(r) => self.renderer = Some(r),
            Err(e) => {
                eprintln!("Cannot initialize Vulkan: {e}");
                self.failed = true;
                el.exit();
                return;
            }
        };
        self.focused = w.has_focus();
        self.window = Some(w);
        self.last = Some(Instant::now());
    }
    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(size) => {
                if let Some(r) = self.renderer.as_mut() {
                    if let Err(e) = unsafe { r.resize(size.width, size.height) } {
                        eprintln!("Vulkan resize: {e}");
                        self.failed = true;
                        el.exit();
                    }
                }
            }
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
                // Retry an unpresentable checkpoint without advancing its state.
                if self.pending_capture.is_none() {
                    if let Some(demo) = &mut self.demo {
                        demo.advance(&mut self.app.game, elapsed);
                        self.pending_capture = demo.capture_path();
                    } else if self.benchmarking {
                        self.accumulator += elapsed.min(0.1);
                        while self.accumulator >= pong_scenario::STEP {
                            if self.scenario.finished() {
                                self.scenario = pong_scenario::Scenario::default();
                                self.app.game =
                                    pong_low_level_common::Game::new(pong_scenario::SEED);
                            }
                            self.scenario.advance(&mut self.app.game);
                            self.accumulator -= pong_scenario::STEP;
                        }
                    } else {
                        self.app.tick(elapsed);
                    }
                }
                let w = self.window.as_ref().unwrap();
                let size = w.inner_size();
                if size.width > 0 && size.height > 0 {
                    match unsafe {
                        self.renderer
                            .as_mut()
                            .unwrap()
                            .draw(&self.app, &mut self.pending_capture)
                    } {
                        Ok(outcome) => {
                            if outcome.presented && self.benchmarking && !self.benchmark_done {
                                let frames = self.benchmark.get_or_insert_with(|| {
                                    pong_benchmark::Frames::new(size.width, size.height)
                                });
                                if frames.frame(size.width, size.height) {
                                    self.benchmark_done = true;
                                    println!("PONG_BENCHMARK {}", frames.report());
                                    el.exit();
                                }
                            }
                            if outcome.resize {
                                if let Err(e) = unsafe {
                                    self.renderer
                                        .as_mut()
                                        .unwrap()
                                        .resize(size.width, size.height)
                                } {
                                    eprintln!("Vulkan resize: {e}");
                                    self.failed = true;
                                    el.exit();
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Vulkan draw: {e}");
                            self.failed = true;
                            el.exit();
                        }
                    }
                }
                w.set_title(&self.app.title(if self.demo.is_some() {
                    "Vulkan / ash / SCRIPTED"
                } else {
                    "Vulkan / ash"
                }));
                if self.demo.as_ref().is_some_and(|d| d.finished())
                    && self.pending_capture.is_none()
                {
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
fn main() {
    let mut app = Frontend {
        benchmarking: benchmark_args(&std::env::args().collect::<Vec<_>>()),
        demo: pong_scenario::Demo::from_args(),
        capture_enabled: std::env::args().any(|a| a == "--scripted-captures"),
        ..Default::default()
    };
    EventLoop::new().unwrap().run_app(&mut app).unwrap();
    if app.failed {
        std::process::exit(1);
    }
}

fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are exclusive"
    );
    enabled
}
#[cfg(test)]
mod benchmark_tests {
    use super::*;
    #[test]
    fn opt_in_only() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic]
    fn reject_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
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
