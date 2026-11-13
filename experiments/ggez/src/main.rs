use ggez::{
    event,
    graphics::{self, Color, DrawParam, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use pong_core::*;
use pong_scenario::Demo;
use std::time::{Duration, Instant};
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
fn status_message(game: &Game) -> Option<&'static str> {
    match game.state {
        State::Ready => Some("Ready - Enter to play"),
        State::Playing => None,
        State::Paused => Some("Paused - Enter to play"),
        State::Over if game.scores[0] > game.scores[1] => Some("YOU WIN - Enter to restart"),
        State::Over if game.scores[1] > game.scores[0] => Some("CPU WINS - Enter to restart"),
        State::Over => Some("Match over - Enter to restart"),
    }
}
fn handle_key_down(game: &mut Game, held: &mut [bool; 4], key: KeyCode, repeated: bool) {
    for (i, k) in [
        KeyCode::KeyW,
        KeyCode::ArrowUp,
        KeyCode::KeyS,
        KeyCode::ArrowDown,
    ]
    .into_iter()
    .enumerate()
    {
        if key == k && !repeated && game.state == State::Playing {
            held[i] = true;
        }
    }
    let previous = game.state;
    if !repeated {
        if key == KeyCode::KeyR {
            *held = [false; 4];
            game.start();
        } else if key == KeyCode::Enter {
            if game.state == State::Paused {
                game.toggle_pause();
            } else if game.state != State::Playing {
                game.start();
            }
        } else if key == KeyCode::Space {
            game.toggle_pause();
        } else if key == KeyCode::Escape {
            game.pause();
        }
    }
    if (previous == State::Playing && game.state == State::Paused)
        || (previous == State::Over && game.state == State::Playing)
    {
        *held = [false; 4];
    }
}
fn handle_focused_key_down(
    game: &mut Game,
    held: &mut [bool; 4],
    focused: bool,
    key: KeyCode,
    repeated: bool,
) {
    if focused {
        handle_key_down(game, held, key, repeated);
    }
}
struct App {
    focused: bool,
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    scenario: pong_scenario::Scenario,
    game: Game,
    demo: Option<Demo>,
    capture: graphics::ScreenImage,
    pending: Option<(std::path::PathBuf, graphics::Image)>,
    held: [bool; 4],
    accumulator: f32,
    last_update: Instant,
}
impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Previous draw has now passed through ggez end_frame and been submitted.
        if let Some((path, image)) = self.pending.take() {
            let bytes = image.to_pixels(ctx)?;
            let rgb: Vec<u32> = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|p| (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]))
                .collect();
            Demo::save_rgb(&path, &rgb, image.width(), image.height());
            if self.demo.as_ref().is_some_and(Demo::finished) {
                ctx.request_quit();
                return Ok(());
            }
        }
        // ggez 0.10 runs update/draw from about_to_wait with ControlFlow::Poll.
        // Vsync alone does not guarantee a CPU frame cap on every compositor.
        // Sleep only the remainder of a 120 Hz frame, allowing the OS/AX time to run.
        let frame = Duration::from_secs_f64(1. / 120.);
        if let Some(remaining) = frame.checked_sub(self.last_update.elapsed()) {
            std::thread::sleep(remaining);
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f32().min(0.1);
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.game, elapsed);
            self.last_update = now;
            return Ok(());
        }
        self.accumulator += elapsed;
        self.last_update = now;
        while self.accumulator >= 1. / 120. {
            if self.benchmarking {
                if self.scenario.finished() {
                    self.scenario = pong_scenario::Scenario::default();
                    self.game = Game::new(pong_scenario::SEED);
                }
                self.scenario.advance(&mut self.game);
            } else {
                self.game.step(
                    1. / 120.,
                    (self.held[2] || self.held[3]) as u8 as f32
                        - (self.held[0] || self.held[1]) as u8 as f32,
                );
            }
            self.accumulator -= 1. / 120.;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = ctx.gfx.drawable_size();
        let s = (w / W).min(h / (H + 140.)).max(0.001);
        let ox = (w - W * s) / 2.;
        let oy = (h - (H + 140.) * s) / 2. + 95. * s;
        let path = self.demo.as_mut().and_then(Demo::capture_path);
        let image = path.as_ref().map(|_| self.capture.image(ctx));
        let mut c = if let Some(image) = &image {
            graphics::Canvas::from_image(ctx, image.clone(), Color::from_rgb(12, 33, 30))
        } else {
            graphics::Canvas::from_frame(ctx, Color::from_rgb(12, 33, 30))
        };
        let mut rect = |x: f32, y: f32, w: f32, h: f32, col: Color| {
            c.draw(
                &graphics::Quad,
                DrawParam::default()
                    .dest([ox + x * s, oy + y * s])
                    .scale([w * s, h * s])
                    .color(col),
            )
        };
        rect(0., 0., W, H, Color::from_rgb(21, 63, 55));
        for y in (0..500).step_by(28) {
            rect(449., y as f32, 2., 12., Color::from_rgb(70, 110, 90));
        }
        for (x, y) in [(32., self.game.player), (W - 44., self.game.cpu)] {
            rect(
                x,
                y - PADDLE_H / 2.,
                PADDLE_W,
                PADDLE_H,
                Color::from_rgb(217, 255, 174),
            );
        }
        rect(
            self.game.ball.x - BALL,
            self.game.ball.y - BALL,
            BALL * 2.,
            BALL * 2.,
            Color::WHITE,
        );
        c.draw(
            &Text::new(format!(
                "PONG / ggez     YOU {} : {} CPU",
                self.game.scores[0], self.game.scores[1]
            )),
            DrawParam::default().dest([ox, oy - 65. * s]).scale([s, s]),
        );
        c.draw(
            &Text::new("W/S or arrows: move | Enter: play/resume | Space: pause | R: restart"),
            DrawParam::default()
                .dest([ox, oy + (H + 15.) * s])
                .scale([s, s]),
        );
        if let Some(message) = status_message(&self.game) {
            c.draw(
                &Text::new(message),
                DrawParam::default()
                    .dest([ox + 300. * s, oy + 220. * s])
                    .scale([s, s]),
            );
        }
        c.finish(ctx)?;
        if let Some(frames) = &mut self.benchmark {
            if frames.frame(w as u32, h as u32) {
                let report = frames.report();
                self.benchmark = None;
                println!("PONG_BENCHMARK {}", report);
                ctx.request_quit();
            }
        }
        if let (Some(path), Some(image)) = (path, image) {
            // Present the very same texture that was exported, not a synthetic screenshot.
            let mut screen = graphics::Canvas::from_frame(ctx, Color::BLACK);
            screen.draw(&image, DrawParam::default());
            screen.finish(ctx)?;
            self.pending = Some((path, image));
        }
        if self.pending.is_none() && self.demo.as_ref().is_some_and(Demo::finished) {
            ctx.request_quit();
        }
        Ok(())
    }
    fn key_down_event(&mut self, _: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if self.demo.is_some() || self.benchmarking {
            return Ok(());
        }
        if let ggez::winit::keyboard::PhysicalKey::Code(key) = input.event.physical_key {
            handle_focused_key_down(&mut self.game, &mut self.held, self.focused, key, repeated);
        }
        Ok(())
    }
    fn key_up_event(&mut self, _: &mut Context, input: KeyInput) -> GameResult {
        for (i, k) in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ]
        .into_iter()
        .enumerate()
        {
            if input.event.physical_key == k {
                self.held[i] = false;
            }
        }
        Ok(())
    }
    fn focus_event(&mut self, _: &mut Context, gained: bool) -> GameResult {
        self.focused = gained;
        if self.demo.is_some() || self.benchmarking {
            return Ok(());
        }
        if !gained {
            self.game.pause();
            self.held = [false; 4];
            self.accumulator = 0.;
        }
        Ok(())
    }
}
#[cfg(test)]
mod control_tests {
    use super::*;
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut game = Game::new(42);
        game.start();
        game.scores = [3, 2];
        game.pause();
        let mut held = [false; 4];
        handle_focused_key_down(&mut game, &mut held, false, KeyCode::Space, false);
        assert_eq!(game.state, State::Paused);
        handle_focused_key_down(&mut game, &mut held, false, KeyCode::KeyR, false);
        assert_eq!(game.state, State::Paused);
        assert_eq!(game.scores, [3, 2]);
        handle_focused_key_down(&mut game, &mut held, true, KeyCode::Space, false);
        assert_eq!(game.state, State::Playing);
    }
    #[test]
    fn status_names_winner_and_clears_after_restart() {
        let mut game = Game::new(42);
        assert_eq!(status_message(&game), Some("Ready - Enter to play"));
        game.start();
        assert_eq!(status_message(&game), None);
        game.pause();
        assert_eq!(status_message(&game), Some("Paused - Enter to play"));
        for (scores, x, vx, message) in [
            ([6, 3], W + BALL - 1., 350., "YOU WIN - Enter to restart"),
            ([2, 6], -BALL + 1., -350., "CPU WINS - Enter to restart"),
        ] {
            game.start();
            game.scores = scores;
            game.serve = 0.;
            game.ball.x = x;
            game.ball.y = H / 2.;
            game.ball.vx = vx;
            game.ball.vy = 0.;
            game.step(0.02, 0.);
            assert_eq!(game.state, State::Over);
            assert_eq!(status_message(&game), Some(message));
            game.start();
            assert_eq!(status_message(&game), None);
        }
    }
    #[test]
    fn active_restart_clears_held_movement() {
        let mut g = Game::new(42);
        g.start();
        let mut held = [false; 4];
        handle_key_down(&mut g, &mut held, KeyCode::KeyW, false);
        handle_key_down(&mut g, &mut held, KeyCode::KeyR, false);
        assert_eq!(held, [false; 4]);
        handle_key_down(&mut g, &mut held, KeyCode::KeyW, true);
        assert_eq!(held, [false; 4]);
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        for key in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ] {
            for restart in [KeyCode::Enter, KeyCode::KeyR] {
                let mut game = Game::new(42);
                game.start();
                let mut held = [false; 4];
                handle_key_down(&mut game, &mut held, key, false);
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
                handle_key_down(&mut game, &mut held, restart, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{key:?} {restart:?}");
                handle_key_down(&mut game, &mut held, key, true);
                assert_eq!(held, [false; 4]);
                handle_key_down(&mut game, &mut held, key, false);
                assert!(held.iter().any(|v| *v));
            }
        }
    }

    #[test]
    fn action_keys_keep_start_resume_restart_and_repeat_behavior() {
        let mut game = Game::new(42);
        let mut held = [false; 4];
        handle_key_down(&mut game, &mut held, KeyCode::Enter, true);
        assert_eq!(game.state, State::Ready);
        handle_key_down(&mut game, &mut held, KeyCode::Enter, false);
        assert_eq!(game.state, State::Playing);
        handle_key_down(&mut game, &mut held, KeyCode::Space, true);
        assert_eq!(game.state, State::Playing);
        handle_key_down(&mut game, &mut held, KeyCode::Space, false);
        assert_eq!(game.state, State::Paused);
        handle_key_down(&mut game, &mut held, KeyCode::Enter, false);
        assert_eq!(game.state, State::Playing);
        game.scores = [2, 4];
        handle_key_down(&mut game, &mut held, KeyCode::KeyR, false);
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn presses_while_paused_do_not_rearm_movement_on_resume() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [
                KeyCode::KeyW,
                KeyCode::ArrowUp,
                KeyCode::KeyS,
                KeyCode::ArrowDown,
            ] {
                let mut game = Game::new(42);
                game.state = state;
                let mut held = [false; 4];
                handle_key_down(&mut game, &mut held, key, false);
                handle_key_down(&mut game, &mut held, KeyCode::Enter, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{state:?} {key:?}");
                handle_key_down(&mut game, &mut held, key, true);
                assert_eq!(held, [false; 4], "repeat must not rearm {key:?}");
                handle_key_down(&mut game, &mut held, key, false);
                assert!(held.iter().any(|value| *value), "fresh press arms {key:?}");
            }
        }
    }
    #[test]
    fn explicit_pause_clears_movement_until_fresh_press() {
        for pause in [KeyCode::Space, KeyCode::Escape] {
            for key in [
                KeyCode::KeyW,
                KeyCode::ArrowUp,
                KeyCode::KeyS,
                KeyCode::ArrowDown,
            ] {
                let mut game = Game::new(42);
                let mut held = [false; 4];
                handle_key_down(&mut game, &mut held, KeyCode::Enter, false);
                handle_key_down(&mut game, &mut held, key, false);
                assert!(held.iter().any(|v| *v));
                handle_key_down(&mut game, &mut held, pause, false);
                assert_eq!(game.state, State::Paused);
                assert!(!held.iter().any(|v| *v));
                handle_key_down(&mut game, &mut held, KeyCode::Enter, false);
                assert_eq!(game.state, State::Playing);
                handle_key_down(&mut game, &mut held, key, true);
                assert!(!held.iter().any(|v| *v), "repeat restored movement");
                handle_key_down(&mut game, &mut held, key, false);
                assert!(held.iter().any(|v| *v), "fresh press should move");
            }
        }
    }
}
fn main() -> GameResult {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let (ctx, events) = ggez::ContextBuilder::new("pong-ggez", "kata")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong ggez"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1000., 680.)
                .resizable(true),
        )
        .build()?;
    let focused = ctx.gfx.window().has_focus();
    let capture =
        graphics::ScreenImage::new_raw(&ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);
    let (width, height) = ctx.gfx.drawable_size();
    event::run(
        ctx,
        events,
        App {
            focused,
            benchmarking,
            benchmark: benchmarking
                .then(|| pong_benchmark::Frames::new(width as u32, height as u32)),
            scenario: pong_scenario::Scenario::default(),
            game: Game::new(pong_scenario::SEED),
            demo: Demo::from_args(),
            capture,
            pending: None,
            held: [false; 4],
            accumulator: 0.,
            last_update: Instant::now(),
        },
    )
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
