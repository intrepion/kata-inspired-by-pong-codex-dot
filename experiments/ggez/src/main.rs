use ggez::{
    event,
    graphics::{self, Color, DrawParam, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use pong_core::*;
use std::time::{Duration, Instant};
struct App {
    game: Game,
    held: [bool; 4],
    accumulator: f32,
    last_update: Instant,
}
impl event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // ggez 0.10 runs update/draw from about_to_wait with ControlFlow::Poll.
        // Vsync alone does not guarantee a CPU frame cap on every compositor.
        // Sleep only the remainder of a 120 Hz frame, allowing the OS/AX time to run.
        let frame = Duration::from_secs_f64(1. / 120.);
        if let Some(remaining) = frame.checked_sub(self.last_update.elapsed()) {
            std::thread::sleep(remaining);
        }
        let now = Instant::now();
        self.accumulator += now.duration_since(self.last_update).as_secs_f32().min(0.1);
        self.last_update = now;
        while self.accumulator >= 1. / 120. {
            self.game.step(
                1. / 120.,
                (self.held[2] || self.held[3]) as u8 as f32
                    - (self.held[0] || self.held[1]) as u8 as f32,
            );
            self.accumulator -= 1. / 120.;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (w, h) = ctx.gfx.drawable_size();
        let s = (w / W).min(h / (H + 140.)).max(0.001);
        let ox = (w - W * s) / 2.;
        let oy = (h - (H + 140.) * s) / 2. + 95. * s;
        let mut c = graphics::Canvas::from_frame(ctx, Color::from_rgb(12, 33, 30));
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
        c.finish(ctx)
    }
    fn key_down_event(&mut self, _: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
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
        if !gained {
            self.game.pause();
            self.held = [false; 4];
            self.accumulator = 0.;
        }
        Ok(())
    }
}
fn main() -> GameResult {
    let (ctx, events) = ggez::ContextBuilder::new("pong-ggez", "kata")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong ggez"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1000., 680.)
                .resizable(true),
        )
        .build()?;
    event::run(
        ctx,
        events,
        App {
            game: Game::new(42),
            held: [false; 4],
            accumulator: 0.,
            last_update: Instant::now(),
        },
    )
}
