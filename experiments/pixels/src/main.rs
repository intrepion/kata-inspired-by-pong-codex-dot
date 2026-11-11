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
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    game: Game,
    up: bool,
    down: bool,
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
        self.window = Some(w);
        self.last = Instant::now();
    }
    fn window_event(&mut self, e: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => e.exit(),
            WindowEvent::Focused(false) => {
                self.up = false;
                self.down = false;
                self.game.pause();
                self.acc = 0.0;
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
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(k) = event.physical_key {
                    match k {
                        KeyCode::KeyW | KeyCode::ArrowUp => self.up = pressed,
                        KeyCode::KeyS | KeyCode::ArrowDown => self.down = pressed,
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
                self.acc += (now - self.last).as_secs_f32().min(0.1);
                self.last = now;
                while self.acc >= 1.0 / 120.0 {
                    self.game
                        .step(1.0 / 120.0, self.down as u8 as f32 - self.up as u8 as f32);
                    self.acc -= 1.0 / 120.0;
                }
                if let Some(p) = self.pixels.as_mut() {
                    draw(p.frame_mut(), &self.game);
                    if let Err(err) = p.render() {
                        eprintln!("Render: {err}");
                        e.exit();
                    }
                }
                if let Some(w) = &self.window {
                    w.set_title(&format!(
                        "Pong pixels | {} : {} | {:?} | Arrows/W/S • Enter restart • Space pause",
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
fn draw(frame: &mut [u8], g: &Game) {
    for (i, p) in frame.chunks_exact_mut(4).enumerate() {
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
}
fn main() {
    let e = EventLoop::new().unwrap();
    e.set_control_flow(ControlFlow::Poll);
    let mut a = App {
        window: None,
        pixels: None,
        game: Game::new(42),
        up: false,
        down: false,
        last: Instant::now(),
        acc: 0.0,
    };
    e.run_app(&mut a).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn framebuffer_has_opaque_background_paddles_and_ball() {
        let mut frame = vec![0; W as usize * H as usize * 4];
        let game = Game::new(42);
        draw(&mut frame, &game);
        let pixel = |x: usize, y: usize| &frame[(y * W as usize + x)*4..(y * W as usize + x)*4+4];
        assert_eq!(pixel(0,0), &[6,10,23,255]);
        assert_eq!(pixel(38,game.player as usize), &[89,242,217,255]);
        assert_eq!(pixel(game.ball.x as usize,game.ball.y as usize), &[240,250,255,255]);
    }
}
