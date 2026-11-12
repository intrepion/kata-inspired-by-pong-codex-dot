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
struct App {
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
        if self.game.state != State::Playing {
            c.draw(
                &Text::new(format!("{:?} - Enter to play", self.game.state)),
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
        let key = input.event.physical_key;
        for (i, k) in [
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ]
        .into_iter()
        .enumerate()
        {
            if key == k {
                self.held[i] = true;
            }
        }
        if !repeated {
            if key == KeyCode::KeyR {
                self.game.start();
            } else if key == KeyCode::Enter {
                if self.game.state == State::Paused {
                    self.game.toggle_pause();
                } else if self.game.state != State::Playing {
                    self.game.start();
                }
            } else if key == KeyCode::Space {
                self.game.toggle_pause();
            } else if key == KeyCode::Escape {
                self.game.pause();
            }
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
    let capture =
        graphics::ScreenImage::new_raw(&ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);
    let (width, height) = ctx.gfx.drawable_size();
    event::run(
        ctx,
        events,
        App {
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
