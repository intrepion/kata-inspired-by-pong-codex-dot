use miniquad::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use std::time::Instant;
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}
#[derive(Default)]
struct MovementInput {
    held: [bool; 4],
}
impl MovementInput {
    fn key(&mut self, key: KeyCode, pressed: bool) {
        let binding = match key {
            KeyCode::Up => 0,
            KeyCode::W => 1,
            KeyCode::Down => 2,
            KeyCode::S => 3,
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
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    game: Game,
    demo: Option<pong_scenario::Demo>,
    benchmarking: bool,
    trace_input: bool,
    benchmark: Option<pong_benchmark::Frames>,
    scenario: pong_scenario::Scenario,
    movement: MovementInput,
    last: Instant,
    acc: f32,
}
impl App {
    fn new(benchmarking: bool) -> Self {
        let mut ctx = window::new_rendering_backend();
        let shader=ctx.new_shader(ShaderSource::Glsl{vertex:"#version 100\nattribute vec2 position; attribute vec3 color; varying lowp vec3 c; void main(){gl_Position=vec4(position,0.,1.);c=color;}",fragment:"#version 100\nprecision lowp float; varying vec3 c; void main(){gl_FragColor=vec4(c,1.);}"},ShaderMeta{images:vec![],uniforms:UniformBlockLayout{uniforms:vec![]}}).unwrap();
        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("position", VertexFormat::Float2),
                VertexAttribute::new("color", VertexFormat::Float3),
            ],
            shader,
            PipelineParams::default(),
        );
        let vertex = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<Vertex>(1024),
        );
        let indices: Vec<u16> = (0..1024).collect();
        let index = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );
        let (width, height) = window::screen_size();
        if benchmarking {
            println!(
                "PONG_BENCHMARK_BACKEND {{\"backend\":\"{:?}\"}}",
                ctx.info().backend
            );
        }
        Self {
            trace_input: std::env::args().any(|arg| arg == "--trace-input"),
            benchmarking,
            benchmark: benchmarking
                .then(|| pong_benchmark::Frames::new(width as u32, height as u32)),
            scenario: pong_scenario::Scenario::default(),
            ctx,
            pipeline,
            bindings: Bindings {
                vertex_buffers: vec![vertex],
                index_buffer: index,
                images: vec![],
            },
            game: Game::new(pong_scenario::SEED),
            demo: pong_scenario::Demo::from_args(),
            movement: MovementInput::default(),
            last: Instant::now(),
            acc: 0.0,
        }
    }
}
fn rect(v: &mut Vec<Vertex>, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {
    let points = [
        (x - w / 2., y - h / 2.),
        (x + w / 2., y - h / 2.),
        (x + w / 2., y + h / 2.),
        (x - w / 2., y - h / 2.),
        (x + w / 2., y + h / 2.),
        (x - w / 2., y + h / 2.),
    ];
    for (x, y) in points {
        v.push(Vertex {
            pos: [x / W * 2. - 1., 1. - y / H * 2.],
            color,
        });
    }
}
impl EventHandler for App {
    fn update(&mut self) {
        let now = Instant::now();
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.game, (now - self.last).as_secs_f32());
        } else {
            self.acc += (now - self.last).as_secs_f32().min(0.1);
        }
        self.last = now;
        while self.acc >= 1. / 120. {
            if self.benchmarking {
                if self.scenario.finished() {
                    self.scenario = pong_scenario::Scenario::default();
                    self.game = Game::new(pong_scenario::SEED);
                }
                self.scenario.advance(&mut self.game);
            } else {
                self.game.step(1. / 120., self.movement.axis());
            }
            self.acc -= 1. / 120.;
        }
    }
    fn draw(&mut self) {
        let mut v = Vec::new();
        for y in (0..500).step_by(25) {
            rect(&mut v, W / 2., y as f32, 2., 12., [0.18, 0.25, 0.35]);
        }
        for (x, y) in [(38., self.game.player), (W - 38., self.game.cpu)] {
            rect(&mut v, x, y, PADDLE_W, PADDLE_H, [0.35, 0.95, 0.85]);
        }
        rect(
            &mut v,
            self.game.ball.x,
            self.game.ball.y,
            BALL * 2.,
            BALL * 2.,
            [1., 1., 1.],
        );
        // Score pips keep this renderer font-free.
        for side in 0..2 {
            for i in 0..self.game.scores[side] {
                rect(
                    &mut v,
                    W / 2. + (side as f32 * 2. - 1.) * (30. + i as f32 * 16.),
                    24.,
                    8.,
                    16.,
                    [1., 1., 1.],
                );
            }
        }
        self.ctx
            .buffer_update(self.bindings.vertex_buffers[0], BufferSource::slice(&v));
        self.ctx
            .begin_default_pass(PassAction::clear_color(0.025, 0.04, 0.09, 1.));
        let (w, h) = window::screen_size();
        let s = (w / W).min(h / H);
        self.ctx.apply_viewport(
            ((w - W * s) / 2.) as i32,
            ((h - H * s) / 2.) as i32,
            (W * s) as i32,
            (H * s) as i32,
        );
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, v.len() as i32, 1);
        if let Some(path) = self.demo.as_mut().and_then(|d| d.capture_path()) {
            assert_eq!(
                self.ctx.info().backend,
                Backend::OpenGl,
                "Miniquad capture currently supports only its OpenGL framebuffer"
            );
            let (width, height) = (w as u32, h as u32);
            let mut rgba = vec![0u8; width as usize * height as usize * 4];
            // Supported OpenGL readback from THIS app's active default render pass.
            // No OS/window-manager capture. Pack alignment is four for RGBA bytes.
            unsafe {
                gl::glReadPixels(
                    0,
                    0,
                    width as i32,
                    height as i32,
                    gl::GL_RGBA,
                    gl::GL_UNSIGNED_BYTE,
                    rgba.as_mut_ptr().cast(),
                );
                assert_eq!(
                    gl::glGetError(),
                    gl::GL_NO_ERROR,
                    "application framebuffer readback failed"
                );
            }
            let mut rgb = Vec::with_capacity(width as usize * height as usize);
            for row in rgba.chunks_exact(width as usize * 4).rev() {
                for c in row.as_chunks::<4>().0 {
                    rgb.push((u32::from(c[0]) << 16) | (u32::from(c[1]) << 8) | u32::from(c[2]));
                }
            }
            pong_scenario::Demo::save_rgb(&path, &rgb, width, height);
        }
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
        let finished = self
            .benchmark
            .as_mut()
            .is_some_and(|frames| frames.frame(w as u32, h as u32));
        if finished {
            let frames = self.benchmark.take().expect("completed sampler");
            println!("PONG_BENCHMARK {}", frames.report());
            window::order_quit();
        }
        if self.demo.as_ref().is_some_and(|d| d.finished()) {
            window::order_quit();
        }
    }
    fn key_down_event(&mut self, k: KeyCode, _: KeyMods, repeat: bool) {
        if self.trace_input {
            eprintln!(
                "PONG_INPUT key_down key={k:?} repeat={repeat} state={:?}",
                self.game.state
            );
        }
        if self.demo.is_some() || self.benchmarking {
            return;
        }
        self.movement.key(k, true);
        match k {
            KeyCode::Enter | KeyCode::R if !repeat => self.game.start(),
            KeyCode::Space if !repeat => {
                if self.game.state == State::Ready {
                    self.game.start()
                } else {
                    self.game.toggle_pause()
                }
            }
            KeyCode::Escape => window::request_quit(),
            _ => {}
        }
    }
    fn key_up_event(&mut self, k: KeyCode, _: KeyMods) {
        if self.trace_input {
            eprintln!("PONG_INPUT key_up key={k:?} state={:?}", self.game.state);
        }
        self.movement.key(k, false);
    }
    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if self.trace_input {
            eprintln!("PONG_INPUT mouse_down button={button:?}");
        }
    }
    fn window_minimized_event(&mut self) {
        if self.demo.is_some() || self.benchmarking {
            return;
        }
        self.game.pause();
        self.movement.clear();
        self.acc = 0.;
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
    miniquad::start(
        conf::Conf {
            window_title: "Pong Miniquad | Enter start/restart • W/S arrows • Space pause".into(),
            window_width: W as i32,
            window_height: H as i32,
            ..Default::default()
        },
        move || Box::new(App::new(benchmarking)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn rectangle_is_two_triangles_in_top_down_court_coordinates() {
        let mut vertices = Vec::new();
        rect(&mut vertices, W / 2.0, H / 2.0, W, H, [1.0; 3]);
        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0].pos, [-1.0, 1.0]);
        assert_eq!(vertices[2].pos, [1.0, -1.0]);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn aliases_release_independently_in_both_orders() {
        for (a, b, axis) in [
            (KeyCode::Up, KeyCode::W, -1.),
            (KeyCode::Down, KeyCode::S, 1.),
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
        input.key(KeyCode::W, true);
        input.key(KeyCode::W, true);
        assert_eq!(input.axis(), -1.);
        input.key(KeyCode::Down, true);
        assert_eq!(input.axis(), 0.);
        input.clear();
        assert_eq!(input.axis(), 0.);
        input.key(KeyCode::W, false);
        assert_eq!(input.axis(), 0.);
    }
}
