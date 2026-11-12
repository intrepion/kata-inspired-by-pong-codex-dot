mod renderer;
use pong_core::{Game, State, H, W};
use renderer::Renderer;
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
#[derive(Default)]
struct MovementInput {
    held: [bool; 4],
}
impl MovementInput {
    fn key(&mut self, key: KeyCode, pressed: bool) {
        let binding = match key {
            KeyCode::ArrowUp => 0,
            KeyCode::KeyW => 1,
            KeyCode::ArrowDown => 2,
            KeyCode::KeyS => 3,
            _ => return,
        };
        self.held[binding] = pressed;
    }
    fn axis(&self) -> f32 {
        (self.held[2] || self.held[3]) as u8 as f32 - (self.held[0] || self.held[1]) as u8 as f32
    }
    fn clear(&mut self) {
        self.held = [false; 4];
    }
}
struct App {
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    scenario: pong_scenario::Scenario,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    game: Game,
    demo: Option<pong_scenario::Demo>,
    movement: MovementInput,
    last: Instant,
    acc: f32,
    failed: bool,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, e: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let w = Arc::new(
            e.create_window(
                Window::default_attributes()
                    .with_title("Pong raw wgpu")
                    .with_inner_size(winit::dpi::LogicalSize::new(W, H)),
            )
            .unwrap(),
        );
        self.renderer = Some(pollster::block_on(Renderer::new(w.clone())));
        self.window = Some(w);
        self.last = Instant::now();
    }
    fn window_event(&mut self, e: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => e.exit(),
            WindowEvent::Focused(false) if self.demo.is_none() && !self.benchmarking => {
                self.movement.clear();
                self.game.pause();
                self.acc = 0.0;
            }
            WindowEvent::Resized(s) => {
                if s.width > 0 && s.height > 0 {
                    if let Some(p) = self.renderer.as_mut() {
                        if let Err(err) = p.resize(s.width, s.height) {
                            eprintln!("Resize: {err}");
                            e.exit();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. }
                if self.demo.is_none() && !self.benchmarking =>
            {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(k) = event.physical_key {
                    self.movement.key(k, pressed);
                    match k {
                        KeyCode::Enter | KeyCode::KeyR if pressed && !event.repeat => {
                            self.game.start()
                        }
                        KeyCode::Space if pressed && !event.repeat => {
                            if self.game.state == State::Ready {
                                self.game.start()
                            } else {
                                self.game.toggle_pause()
                            }
                        }
                        KeyCode::Escape if pressed => e.exit(),
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                if let Some(demo) = &mut self.demo {
                    demo.advance(&mut self.game, (now - self.last).as_secs_f32());
                } else {
                    self.acc += (now - self.last).as_secs_f32().min(0.1);
                }
                self.last = now;
                while self.acc >= 1.0 / 120.0 {
                    if self.benchmarking {
                        if self.scenario.finished() {
                            self.scenario = pong_scenario::Scenario::default();
                            self.game = Game::new(pong_scenario::SEED);
                        }
                        self.scenario.advance(&mut self.game);
                    } else {
                        self.game.step(1.0 / 120.0, self.movement.axis());
                    }
                    self.acc -= 1.0 / 120.0;
                }
                if let Some(p) = self.renderer.as_mut() {
                    let capture = self.demo.as_mut().and_then(|d| d.capture_path());
                    match p.render(&self.game, capture.as_deref()) {
                        Ok(true) if self.benchmarking => {
                            let (width, height) = p.dimensions();
                            let frames = self
                                .benchmark
                                .get_or_insert_with(|| pong_benchmark::Frames::new(width, height));
                            if frames.frame(width, height) {
                                println!("PONG_BENCHMARK {}", frames.report());
                                e.exit();
                                return;
                            }
                        }
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Render: {err}");
                            self.failed = true;
                            e.exit();
                            return;
                        }
                    }
                }
                if self.demo.as_ref().is_some_and(|d| d.finished()) {
                    e.exit();
                }
                if let Some(w) = &self.window {
                    w.set_title(&format!(
                        "Pong raw wgpu | {} : {} | {:?} | Arrows/W/S • Enter restart • Space pause",
                        self.game.scores[0], self.game.scores[1], self.game.state
                    ));
                }
            }
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}
fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let e = EventLoop::new().unwrap();
    e.set_control_flow(ControlFlow::Poll);
    let mut a = App {
        benchmarking,
        benchmark: None,
        scenario: pong_scenario::Scenario::default(),
        window: None,
        renderer: None,
        game: Game::new(pong_scenario::SEED),
        demo: pong_scenario::Demo::from_args(),
        movement: MovementInput::default(),
        last: Instant::now(),
        acc: 0.0,
        failed: false,
    };
    e.run_app(&mut a).unwrap();
    if a.failed {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::benchmark_args;
    #[test]
    fn opt_in_only() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "distinct")]
    fn reject_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn aliases_release_independently_in_both_orders() {
        for (a, b, axis) in [
            (KeyCode::ArrowUp, KeyCode::KeyW, -1.),
            (KeyCode::ArrowDown, KeyCode::KeyS, 1.),
        ] {
            for (first, second) in [(a, b), (b, a)] {
                let mut input = MovementInput::default();
                input.key(first, true);
                input.key(second, true);
                input.key(first, false);
                assert_eq!(input.axis(), axis);
                input.key(second, false);
                assert_eq!(input.axis(), 0.);
            }
        }
    }
    #[test]
    fn repeat_opposite_and_focus_clear() {
        let mut input = MovementInput::default();
        input.key(KeyCode::KeyW, true);
        input.key(KeyCode::KeyW, true);
        assert_eq!(input.axis(), -1.);
        input.key(KeyCode::ArrowDown, true);
        assert_eq!(input.axis(), 0.);
        input.clear();
        assert_eq!(input.axis(), 0.);
        input.key(KeyCode::KeyW, false);
        assert_eq!(input.axis(), 0.);
    }
}
