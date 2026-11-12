use miniquad::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use std::time::Instant;
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}
struct App {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    game: Game,
    demo: Option<pong_scenario::Demo>,
    up: bool,
    down: bool,
    last: Instant,
    acc: f32,
}
impl App {
    fn new() -> Self {
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
        Self {
            ctx,
            pipeline,
            bindings: Bindings {
                vertex_buffers: vec![vertex],
                index_buffer: index,
                images: vec![],
            },
            game: Game::new(42),
            demo: pong_scenario::Demo::from_args(),
            up: false,
            down: false,
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
            self.game
                .step(1. / 120., self.down as u8 as f32 - self.up as u8 as f32);
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
        if self.demo.as_ref().is_some_and(|d| d.finished()) {
            window::request_quit();
        }
    }
    fn key_down_event(&mut self, k: KeyCode, _: KeyMods, repeat: bool) {
        if self.demo.is_some() {
            return;
        }
        match k {
            KeyCode::Up | KeyCode::W => self.up = true,
            KeyCode::Down | KeyCode::S => self.down = true,
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
        match k {
            KeyCode::Up | KeyCode::W => self.up = false,
            KeyCode::Down | KeyCode::S => self.down = false,
            _ => {}
        }
    }
    fn window_minimized_event(&mut self) {
        if self.demo.is_some() {
            return;
        }
        self.game.pause();
        self.up = false;
        self.down = false;
        self.acc = 0.;
    }
}
fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Pong Miniquad | Enter start/restart • W/S arrows • Space pause".into(),
            window_width: W as i32,
            window_height: H as i32,
            ..Default::default()
        },
        || Box::new(App::new()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rectangle_is_two_triangles_in_top_down_court_coordinates() {
        let mut vertices = Vec::new();
        rect(&mut vertices, W / 2.0, H / 2.0, W, H, [1.0; 3]);
        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0].pos, [-1.0, 1.0]);
        assert_eq!(vertices[2].pos, [1.0, -1.0]);
    }
}
