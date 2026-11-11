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
    app: App,
    renderer: Option<renderer::Renderer>,
    window: Option<Rc<Window>>,
    last: Option<Instant>,
    failed: bool,
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
        match unsafe { renderer::Renderer::new(&w) } {
            Ok(r) => self.renderer = Some(r),
            Err(e) => {
                eprintln!("Cannot initialize Vulkan: {e}");
                self.failed = true;
                el.exit();
                return;
            }
        };
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
                if size.width > 0 && size.height > 0 {
                    match unsafe { self.renderer.as_mut().unwrap().draw(&self.app) } {
                        Ok(true) => {
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
                        Ok(false) => {}
                        Err(e) => {
                            eprintln!("Vulkan draw: {e}");
                            self.failed = true;
                            el.exit();
                        }
                    }
                }
                w.set_title(&self.app.title("Vulkan / ash"));
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
    let mut app = Frontend::default();
    EventLoop::new().unwrap().run_app(&mut app).unwrap();
    if app.failed {
        std::process::exit(1);
    }
}
