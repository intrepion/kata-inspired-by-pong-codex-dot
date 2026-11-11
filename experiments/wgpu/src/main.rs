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
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
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
            WindowEvent::Focused(false) => {
                self.up = false;
                self.down = false;
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
                if let Some(p) = self.renderer.as_mut() {
                    if let Err(err) = p.render(&self.game) {
                        eprintln!("Render: {err}");
                        e.exit();
                    }
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
    let e = EventLoop::new().unwrap();
    e.set_control_flow(ControlFlow::Poll);
    let mut a = App {
        window: None,
        renderer: None,
        game: Game::new(42),
        up: false,
        down: false,
        last: Instant::now(),
        acc: 0.0,
    };
    e.run_app(&mut a).unwrap();
}
