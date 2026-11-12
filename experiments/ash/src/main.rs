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
    demo: Option<pong_scenario::Demo>,
    pending_capture: Option<std::path::PathBuf>,
    capture_enabled: bool,
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
        match unsafe { renderer::Renderer::new(&w, self.capture_enabled) } {
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
            WindowEvent::Focused(false) if self.demo.is_none() => self.app.blur(),
            WindowEvent::KeyboardInput { event, .. } if self.demo.is_none() => {
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
                let elapsed = self
                    .last
                    .replace(now)
                    .map_or(0., |t| (now - t).as_secs_f32());
                // Retry an unpresentable checkpoint without advancing its state.
                if self.pending_capture.is_none() {
                    if let Some(demo) = &mut self.demo {
                        demo.advance(&mut self.app.game, elapsed);
                        self.pending_capture = demo.capture_path();
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
        demo: pong_scenario::Demo::from_args(),
        capture_enabled: std::env::args().any(|a| a == "--scripted-captures"),
        ..Default::default()
    };
    EventLoop::new().unwrap().run_app(&mut app).unwrap();
    if app.failed {
        std::process::exit(1);
    }
}
