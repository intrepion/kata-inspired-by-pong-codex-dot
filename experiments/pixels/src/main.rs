use pixels::{Pixels, SurfaceTexture};
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use std::{sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
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
fn handle_key(
    game: &mut Game,
    movement: &mut MovementInput,
    key: KeyCode,
    pressed: bool,
    repeat: bool,
) -> bool {
    if !pressed || (!repeat && game.state == State::Playing) {
        movement.key(key, pressed);
    }
    let previous = game.state;
    match key {
        KeyCode::Enter | KeyCode::NumpadEnter | KeyCode::KeyR if pressed && !repeat => {
            movement.clear();
            game.start();
        }
        KeyCode::Space if pressed && !repeat => {
            if game.state == State::Ready {
                game.start();
            } else {
                let was_playing = game.state == State::Playing;
                game.toggle_pause();
                if was_playing && game.state == State::Paused {
                    movement.clear();
                }
            }
        }
        KeyCode::Escape if pressed => return true,
        _ => {}
    }
    if previous == State::Over && game.state == State::Playing {
        movement.clear();
    }
    false
}

fn handle_focused_key(
    game: &mut Game,
    movement: &mut MovementInput,
    focused: bool,
    key: KeyCode,
    pressed: bool,
    repeat: bool,
) -> bool {
    focused && handle_key(game, movement, key, pressed, repeat)
}

struct App {
    focused: bool,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    game: Game,
    demo: Option<pong_scenario::Demo>,
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    scenario: pong_scenario::Scenario,
    movement: MovementInput,
    last: Instant,
    acc: f32,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, e: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let w = Arc::new(
            e.create_window(
                Window::default_attributes()
                    .with_title("Pong pixels")
                    .with_inner_size(winit::dpi::LogicalSize::new(W, H)),
            )
            .unwrap(),
        );
        let s = w.inner_size();
        self.pixels = Some(
            Pixels::new(
                W as u32,
                H as u32,
                SurfaceTexture::new(s.width, s.height, w.clone()),
            )
            .unwrap(),
        );
        if self.benchmarking {
            self.benchmark = Some(pong_benchmark::Frames::new(s.width, s.height));
        }
        self.focused = w.has_focus();
        self.window = Some(w);
        self.last = Instant::now();
    }
    fn window_event(&mut self, e: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => e.exit(),
            WindowEvent::Focused(focused) => {
                self.focused = focused;
                if !focused && self.demo.is_none() && !self.benchmarking {
                    self.movement.clear();
                    self.game.pause();
                    self.acc = 0.0;
                }
            }
            WindowEvent::Resized(s) => {
                if s.width > 0 && s.height > 0 {
                    if let Some(p) = self.pixels.as_mut() {
                        if let Err(err) = p.resize_surface(s.width, s.height) {
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
                    if handle_focused_key(
                        &mut self.game,
                        &mut self.movement,
                        self.focused,
                        k,
                        pressed,
                        event.repeat,
                    ) {
                        e.exit();
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
                if let Some(p) = self.pixels.as_mut() {
                    draw(p.frame_mut(), &self.game);
                    if let Some(path) = self.demo.as_mut().and_then(|d| d.capture_path()) {
                        // Exact application RGBA source buffer, not GPU-scaled output.
                        let rgb: Vec<u32> = p
                            .frame()
                            .as_chunks::<4>()
                            .0
                            .iter()
                            .map(|c| {
                                (u32::from(c[0]) << 16) | (u32::from(c[1]) << 8) | u32::from(c[2])
                            })
                            .collect();
                        pong_scenario::Demo::save_rgb(&path, &rgb, W as u32, H as u32);
                    }
                    if let Err(err) = p.render() {
                        eprintln!("Render: {err}");
                        e.exit();
                        return;
                    }
                }
                if let (Some(frames), Some(w)) = (&mut self.benchmark, &self.window) {
                    let size = w.inner_size();
                    if frames.frame(size.width, size.height) {
                        println!("PONG_BENCHMARK {}", frames.report());
                        e.exit();
                        return;
                    }
                }
                if self.demo.as_ref().is_some_and(|d| d.finished()) {
                    e.exit();
                }
                if let Some(w) = &self.window {
                    w.set_title(&format!(
                        "Pong pixels {} | {} : {} | {:?} | Arrows/W/S • Enter restart • Space pause",
                        if self.demo.is_some() { "SCRIPTED" } else { "" },
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
fn state_message(game: &Game) -> Option<&'static str> {
    match game.state {
        State::Ready => Some("READY"),
        State::Paused => Some("PAUSED"),
        State::Playing => None,
        State::Over if game.scores[0] > game.scores[1] => Some("YOU WIN"),
        State::Over if game.scores[1] > game.scores[0] => Some("CPU WINS"),
        State::Over => Some("OVER"),
    }
}
fn status_glyph(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [14, 17, 17, 31, 17, 17, 17],
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'D' => [30, 17, 17, 17, 17, 17, 30],
        b'E' => [31, 16, 16, 30, 16, 16, 31],
        b'I' => [14, 4, 4, 4, 4, 4, 14],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'R' => [30, 17, 17, 30, 20, 18, 17],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'V' => [17, 17, 17, 17, 17, 10, 4],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        _ => [0; 7],
    }
}
fn draw(frame: &mut [u8], g: &Game) {
    for (i, p) in frame.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let x = (i % W as usize) as f32;
        let y = (i / W as usize) as f32;
        let paddle = (x - 38.0).abs() < PADDLE_W / 2.0 && (y - g.player).abs() < PADDLE_H / 2.0
            || (x - (W - 38.0)).abs() < PADDLE_W / 2.0 && (y - g.cpu).abs() < PADDLE_H / 2.0;
        let ball = (x - g.ball.x).powi(2) + (y - g.ball.y).powi(2) < BALL * BALL;
        let net = (x - W / 2.0).abs() < 1.0 && (y as u32 % 25) < 12;
        p.copy_from_slice(&if ball {
            [240, 250, 255, 255]
        } else if paddle {
            [89, 242, 217, 255]
        } else if net {
            [45, 64, 89, 255]
        } else {
            [6, 10, 23, 255]
        });
    }
    if let Some(message) = state_message(g) {
        const CELL: usize = 4;
        let left = (W as usize - message.len() * 6 * CELL) / 2;
        // Clear the scene behind the non-playing message so a ball or net
        // crossing a glyph cannot make its text ambiguous.
        for y in 152..196 {
            for x in (left - 8)..(left + message.len() * 6 * CELL + 8) {
                let offset = (y * W as usize + x) * 4;
                frame[offset..offset + 4].copy_from_slice(&[6, 10, 23, 255]);
            }
        }
        for (index, ch) in message.bytes().enumerate() {
            for (row, bits) in status_glyph(ch).into_iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) == 0 {
                        continue;
                    }
                    for y in (160 + row * CELL)..(160 + (row + 1) * CELL) {
                        for x in (left + index * 6 * CELL + col * CELL)
                            ..(left + index * 6 * CELL + (col + 1) * CELL)
                        {
                            let offset = (y * W as usize + x) * 4;
                            frame[offset..offset + 4].copy_from_slice(&[255, 180, 80, 255]);
                        }
                    }
                }
            }
        }
    }
    // Draw scores in the court so they remain visible without a window title.
    const DIGITS: [u8; 10] = [0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f];
    const SEGMENTS: [(usize, usize, usize, usize); 7] = [
        (0, 0, 22, 4),
        (18, 0, 4, 20),
        (18, 18, 4, 20),
        (0, 34, 22, 4),
        (0, 18, 4, 20),
        (0, 0, 4, 20),
        (0, 17, 22, 4),
    ];
    for (origin_x, score) in [(350usize, g.scores[0]), (528, g.scores[1])] {
        let bits = DIGITS[score.min(9) as usize];
        for (index, &(dx, dy, width, height)) in SEGMENTS.iter().enumerate() {
            if bits & (1 << index) == 0 {
                continue;
            }
            for y in (25 + dy)..(25 + dy + height) {
                for x in (origin_x + dx)..(origin_x + dx + width) {
                    let offset = (y * W as usize + x) * 4;
                    frame[offset..offset + 4].copy_from_slice(&[220, 230, 245, 255]);
                }
            }
        }
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
    let e = EventLoop::new().unwrap();
    e.set_control_flow(ControlFlow::Poll);
    let mut a = App {
        focused: false,
        window: None,
        pixels: None,
        game: Game::new(pong_scenario::SEED),
        benchmarking,
        benchmark: None,
        scenario: pong_scenario::Scenario::default(),
        demo: pong_scenario::Demo::from_args(),
        movement: MovementInput::default(),
        last: Instant::now(),
        acc: 0.0,
    };
    e.run_app(&mut a).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn court_shows_state_and_distinguishes_winner() {
        let mut game = Game::new(42);
        let render = |game: &Game| {
            let mut frame = vec![0; W as usize * H as usize * 4];
            draw(&mut frame, game);
            frame
                .chunks_exact(4)
                .filter(|pixel| *pixel == [255, 180, 80, 255])
                .count()
        };
        assert_eq!(state_message(&game), Some("READY"));
        assert!(render(&game) > 100, "Ready indicator");
        game.start();
        assert_eq!(state_message(&game), None);
        assert_eq!(render(&game), 0, "no label in play");
        game.pause();
        assert_eq!(state_message(&game), Some("PAUSED"));
        assert!(render(&game) > 100, "Paused indicator");
        game.state = State::Over;
        game.scores = [7, 2];
        assert_eq!(state_message(&game), Some("YOU WIN"));
        let player = render(&game);
        game.scores = [2, 7];
        assert_eq!(state_message(&game), Some("CPU WINS"));
        let cpu = render(&game);
        assert!(player > 100 && cpu > 100);
        assert_ne!(player, cpu);
        game.scores = [7, 7];
        assert_eq!(state_message(&game), Some("OVER"));
        assert!(render(&game) > 100, "defensive tie fallback");
    }
    #[test]
    fn message_backdrop_clears_ball_only_outside_play() {
        let mut game = Game::new(42);
        game.ball.x = 390.;
        game.ball.y = 155.;
        let mut frame = vec![0; W as usize * H as usize * 4];
        draw(&mut frame, &game);
        let offset = (155 * W as usize + 390) * 4;
        assert_eq!(&frame[offset..offset + 4], &[6, 10, 23, 255]);
        game.state = State::Playing;
        draw(&mut frame, &game);
        assert_eq!(&frame[offset..offset + 4], &[240, 250, 255, 255]);
    }
    #[test]
    fn court_displays_current_scores_without_window_title() {
        let mut game = Game::new(42);
        game.scores = [0, 1];
        let mut frame = vec![0; W as usize * H as usize * 4];
        draw(&mut frame, &game);
        let pixel =
            |x: usize, y: usize| &frame[(y * W as usize + x) * 4..(y * W as usize + x) * 4 + 4];
        assert_eq!(pixel(355, 25), &[220, 230, 245, 255]);
        assert_eq!(pixel(547, 30), &[220, 230, 245, 255]);
        assert_eq!(pixel(533, 25), &[6, 10, 23, 255]);
    }
    #[test]
    fn benchmark_is_opt_in() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "benchmark and replay/capture modes are distinct")]
    fn rejects_capture_mixing() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
    #[test]
    fn framebuffer_has_opaque_background_paddles_and_ball() {
        let mut frame = vec![0; W as usize * H as usize * 4];
        let game = Game::new(42);
        draw(&mut frame, &game);
        let pixel =
            |x: usize, y: usize| &frame[(y * W as usize + x) * 4..(y * W as usize + x) * 4 + 4];
        assert_eq!(pixel(0, 0), &[6, 10, 23, 255]);
        assert_eq!(pixel(38, game.player as usize), &[89, 242, 217, 255]);
        assert_eq!(
            pixel(game.ball.x as usize, game.ball.y as usize),
            &[240, 250, 255, 255]
        );
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut game = Game::new(42);
        game.start();
        game.scores = [3, 2];
        game.pause();
        let mut movement = MovementInput::default();
        handle_focused_key(&mut game, &mut movement, false, KeyCode::Space, true, false);
        assert_eq!(game.state, State::Paused);
        handle_focused_key(&mut game, &mut movement, false, KeyCode::KeyR, true, false);
        assert_eq!(game.state, State::Paused);
        assert_eq!(game.scores, [3, 2]);
        handle_focused_key(&mut game, &mut movement, true, KeyCode::Space, true, false);
        assert_eq!(game.state, State::Playing);
    }
    #[test]
    fn keypad_enter_starts_and_ignores_repeat_release_and_unfocused_input() {
        let mut game = Game::new(42);
        let mut movement = MovementInput::default();
        handle_focused_key(&mut game, &mut movement, false, KeyCode::NumpadEnter, true, false);
        assert_eq!(game.state, State::Ready);
        handle_key(&mut game, &mut movement, KeyCode::NumpadEnter, false, false);
        handle_key(&mut game, &mut movement, KeyCode::NumpadEnter, true, true);
        assert_eq!(game.state, State::Ready);
        handle_key(&mut game, &mut movement, KeyCode::NumpadEnter, true, false);
        assert_eq!(game.state, State::Playing);
        game.scores = [3, 2];
        handle_key(&mut game, &mut movement, KeyCode::NumpadEnter, true, true);
        assert_eq!(game.scores, [3, 2]);
        handle_key(&mut game, &mut movement, KeyCode::NumpadEnter, true, false);
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn active_restart_clears_held_movement_until_fresh_press() {
        for restart in [KeyCode::Enter, KeyCode::NumpadEnter, KeyCode::KeyR] {
            let mut game = Game::new(42);
            game.start();
            let mut movement = MovementInput::default();
            handle_key(&mut game, &mut movement, KeyCode::KeyW, true, false);
            handle_key(&mut game, &mut movement, restart, true, false);
            assert_eq!(movement.axis(), 0.);
            handle_key(&mut game, &mut movement, KeyCode::KeyW, true, true);
            assert_eq!(movement.axis(), 0.);
            handle_key(&mut game, &mut movement, KeyCode::KeyW, false, false);
            handle_key(&mut game, &mut movement, KeyCode::KeyW, true, false);
            assert_eq!(movement.axis(), -1.);
        }
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        use pong_core::{Ball, BALL, H, W};
        for key in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ] {
            for restart in [KeyCode::Enter, KeyCode::NumpadEnter, KeyCode::KeyR] {
                let mut game = Game::new(42);
                game.start();
                let mut input = MovementInput::default();
                handle_key(&mut game, &mut input, key, true, false);
                game.scores = [6, 2];
                game.serve = 0.;
                game.ball = Ball {
                    x: W + BALL - 1.,
                    y: H / 2.,
                    vx: 350.,
                    vy: 0.,
                };
                game.step(0.02, 0.);
                assert_eq!(game.state, State::Over);
                handle_key(&mut game, &mut input, restart, true, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(input.axis(), 0., "{key:?} {restart:?}");
                handle_key(&mut game, &mut input, key, true, true);
                assert_eq!(input.axis(), 0.);
                handle_key(&mut game, &mut input, key, false, false);
                handle_key(&mut game, &mut input, key, true, false);
                assert_ne!(input.axis(), 0.);
            }
        }
    }

    #[test]
    fn nonplaying_presses_do_not_rearm_after_start() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [
                KeyCode::KeyW,
                KeyCode::ArrowUp,
                KeyCode::KeyS,
                KeyCode::ArrowDown,
            ] {
                let mut game = Game::new(42);
                game.state = state;
                let mut movement = MovementInput::default();
                handle_key(&mut game, &mut movement, key, true, false);
                handle_key(&mut game, &mut movement, KeyCode::Enter, true, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(movement.axis(), 0., "{state:?} {key:?}");
                handle_key(&mut game, &mut movement, key, true, true);
                assert_eq!(movement.axis(), 0.);
                handle_key(&mut game, &mut movement, key, false, false);
                handle_key(&mut game, &mut movement, key, true, false);
                assert_ne!(movement.axis(), 0.);
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
            let mut game = Game::new(42);
            let mut movement = MovementInput::default();
            handle_key(&mut game, &mut movement, KeyCode::Enter, true, false);
            handle_key(&mut game, &mut movement, key, true, false);
            assert_ne!(movement.axis(), 0.);
            handle_key(&mut game, &mut movement, KeyCode::Space, true, false);
            assert_eq!(game.state, State::Paused);
            assert_eq!(movement.axis(), 0.);
            handle_key(&mut game, &mut movement, KeyCode::Space, true, false);
            assert_eq!(game.state, State::Playing);
            handle_key(&mut game, &mut movement, key, true, true);
            assert_eq!(movement.axis(), 0., "repeat must not restore cleared input");
            handle_key(&mut game, &mut movement, key, false, false);
            handle_key(&mut game, &mut movement, key, true, false);
            assert_ne!(movement.axis(), 0., "fresh press must move");
        }
    }
    #[test]
    fn action_keys_keep_start_pause_restart_and_quit_behavior() {
        let mut game = Game::new(42);
        let mut movement = MovementInput::default();
        assert!(!handle_key(
            &mut game,
            &mut movement,
            KeyCode::Space,
            false,
            false
        ));
        assert_eq!(game.state, State::Ready);
        handle_key(&mut game, &mut movement, KeyCode::Space, true, false);
        assert_eq!(game.state, State::Playing);
        handle_key(&mut game, &mut movement, KeyCode::Space, true, true);
        assert_eq!(game.state, State::Playing);
        handle_key(&mut game, &mut movement, KeyCode::Space, true, false);
        assert_eq!(game.state, State::Paused);
        handle_key(&mut game, &mut movement, KeyCode::Space, true, false);
        assert_eq!(game.state, State::Playing);
        game.scores = [2, 4];
        handle_key(&mut game, &mut movement, KeyCode::KeyR, true, false);
        assert_eq!(game.scores, [0, 0]);
        assert!(handle_key(
            &mut game,
            &mut movement,
            KeyCode::Escape,
            true,
            false
        ));
    }
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
