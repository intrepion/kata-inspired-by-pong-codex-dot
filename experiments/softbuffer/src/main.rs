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
    app: App,
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    last: Option<Instant>,
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
        self.window = Some(w);
        self.last = Some(Instant::now());
    }
    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Focused(false) => self.app.blur(),
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW | KeyCode::ArrowUp) => self.app.up = pressed,
                    PhysicalKey::Code(KeyCode::KeyS | KeyCode::ArrowDown) => {
                        self.app.down = pressed
                    }
                    PhysicalKey::Code(KeyCode::Space) if pressed && !event.repeat => {
                        self.app.action()
                    }
                    PhysicalKey::Code(KeyCode::KeyR) if pressed && !event.repeat => {
                        self.app.game.start()
                    }
                    PhysicalKey::Code(KeyCode::Escape) if pressed => el.exit(),
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                self.app.tick(
                    self.last
                        .replace(now)
                        .map_or(0., |t| (now - t).as_secs_f32()),
                );
                let w = self.window.as_ref().unwrap();
                let size = w.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    let s = self.surface.as_mut().unwrap();
                    s.resize(width, height).unwrap();
                    let mut b = s.buffer_mut().unwrap();
                    pong_low_level_common::raster(&self.app, &mut b, size.width, size.height);
                    b.present().unwrap();
                }
                w.set_title(&self.app.title("softbuffer / CPU"));
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
    EventLoop::new()
        .unwrap()
        .run_app(&mut Frontend::default())
        .unwrap();
}
