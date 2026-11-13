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
// Keep the application action boundary testable without constructing a GPU window.
// A true return requests quit; it does not reinterpret unmapped engine keys.
fn control_key(game: &mut Game, key: KeyCode, repeat: bool) -> bool {
    match key {
        KeyCode::Enter | KeyCode::KpEnter | KeyCode::R if !repeat => game.start(),
        KeyCode::Space if !repeat => {
            if game.state == State::Ready {
                game.start();
            } else {
                game.toggle_pause();
            }
        }
        KeyCode::Escape => return true,
        _ => {}
    }
    false
}

fn handle_key_down(
    game: &mut Game,
    movement: &mut MovementInput,
    key: KeyCode,
    repeat: bool,
) -> bool {
    if !repeat && game.state == State::Playing {
        movement.key(key, true);
    }
    let previous = game.state;
    if !repeat && matches!(key, KeyCode::Enter | KeyCode::KpEnter | KeyCode::R) {
        movement.clear();
    }
    let quit = control_key(game, key, repeat);
    if (previous == State::Playing && game.state == State::Paused)
        || (previous == State::Over && game.state == State::Playing)
    {
        movement.clear();
    }
    quit
}

#[derive(Default)]
struct FocusGate {
    lost: bool,
}
impl FocusGate {
    fn on_loss(&mut self, reliable: bool) {
        if reliable {
            self.lost = true;
        }
    }
    fn on_restore(&mut self) {
        self.lost = false;
    }
    fn allows_input(&self) -> bool {
        !self.lost
    }
}
fn focused_key_down(
    game: &mut Game,
    movement: &mut MovementInput,
    gate: &FocusGate,
    key: KeyCode,
    repeat: bool,
) -> bool {
    gate.allows_input() && handle_key_down(game, movement, key, repeat)
}
struct App {
    focus_gate: FocusGate,
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
            BufferSource::empty::<Vertex>(4096),
        );
        let indices: Vec<u16> = (0..4096).collect();
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
            focus_gate: FocusGate::default(),
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
// Minimal 5×7 glyphs keep the status readable without a font dependency.
fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'C' => [15, 16, 16, 16, 16, 16, 15],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 25, 21, 19, 19, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        ' ' => [0; 7],
        _ => unreachable!("unsupported status glyph"),
    }
}
fn status_text(v: &mut Vec<Vertex>, text: &str, y: f32, scale: f32) {
    let x = W / 2. - (text.len() as f32 * 6. - 1.) * scale / 2.;
    for (i, c) in text.chars().enumerate() {
        for (row, bits) in glyph(c).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    rect(
                        v,
                        x + (i * 6 + col) as f32 * scale + scale / 2.,
                        y + row as f32 * scale + scale / 2.,
                        scale,
                        scale,
                        [1., 1., 1.],
                    );
                }
            }
        }
    }
}
fn game_over_vertices(game: &Game, v: &mut Vec<Vertex>) {
    if game.state == State::Over {
        let winner = if game.scores[0] > game.scores[1] {
            "YOU WIN"
        } else {
            "CPU WINS"
        };
        status_text(v, winner, H / 2. - 34., 4.);
        status_text(v, "ENTER TO RESTART", H / 2. + 8., 2.);
    }
}
fn state_vertices(game: &Game, v: &mut Vec<Vertex>) {
    match game.state {
        State::Ready => {
            rect(v, W / 2., H / 2. - 65., 240., 82., [0.025, 0.04, 0.09]);
            // A play triangle made from horizontal bars, above the centered ball.
            for (i, width) in [8., 16., 24., 16., 8.].into_iter().enumerate() {
                rect(
                    v,
                    W / 2. - 12. + width / 2.,
                    H / 2. - 86. + i as f32 * 8.,
                    width,
                    8.,
                    [1., 1., 1.],
                );
            }
            status_text(v, "ENTER TO START", H / 2. - 40., 2.);
        }
        State::Paused => {
            rect(v, W / 2., H / 2. - 65., 240., 82., [0.025, 0.04, 0.09]);
            rect(v, W / 2. - 14., H / 2. - 70., 8., 40., [1., 1., 1.]);
            rect(v, W / 2. + 14., H / 2. - 70., 8., 40., [1., 1., 1.]);
            status_text(v, "SPACE TO RESUME", H / 2. - 40., 2.);
        }
        State::Over => game_over_vertices(game, v),
        State::Playing => {}
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
        state_vertices(&self.game, &mut v);
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
        if focused_key_down(
            &mut self.game,
            &mut self.movement,
            &self.focus_gate,
            k,
            repeat,
        ) {
            window::request_quit();
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
        self.focus_gate.on_loss(cfg!(target_os = "linux"));
        if self.demo.is_some() || self.benchmarking {
            return;
        }
        self.game.pause();
        self.movement.clear();
        self.acc = 0.;
    }
    fn window_restored_event(&mut self) {
        self.focus_gate.on_restore();
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
    fn game_over_message_distinguishes_winner_and_clears_on_restart() {
        let mut game = Game::new(42);
        let mut vertices = Vec::new();
        game_over_vertices(&game, &mut vertices);
        assert!(vertices.is_empty());
        game.state = State::Over;
        game.scores = [7, 2];
        game_over_vertices(&game, &mut vertices);
        let player_count = vertices.len();
        assert!(player_count > 0 && player_count < 4096);
        vertices.clear();
        game.scores = [2, 7];
        game_over_vertices(&game, &mut vertices);
        assert_ne!(vertices.len(), player_count);
        vertices.clear();
        game.start();
        game_over_vertices(&game, &mut vertices);
        assert!(vertices.is_empty());
    }
    #[test]
    fn ready_and_paused_indicators_are_distinct_and_absent_during_play() {
        let mut game = Game::new(42);
        let mut vertices = Vec::new();
        state_vertices(&game, &mut vertices);
        let ready = vertices.len();
        assert!(ready > 0);
        vertices.clear();
        game.start();
        state_vertices(&game, &mut vertices);
        assert!(vertices.is_empty());
        game.pause();
        state_vertices(&game, &mut vertices);
        assert!(vertices.len() > 18);
        assert_ne!(ready, vertices.len());
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

#[cfg(test)]
mod control_tests {
    use super::*;
    #[test]
    fn focus_gate_blocks_reliable_loss_and_preserves_ambiguous_move() {
        let mut gate = FocusGate::default();
        let mut game = Game::new(42);
        game.start();
        game.scores = [3, 2];
        game.pause();
        let mut movement = MovementInput::default();
        gate.on_loss(true);
        assert!(!gate.allows_input());
        focused_key_down(&mut game, &mut movement, &gate, KeyCode::Space, false);
        focused_key_down(&mut game, &mut movement, &gate, KeyCode::R, false);
        assert_eq!(game.state, State::Paused);
        assert_eq!(game.scores, [3, 2]);
        gate.on_restore();
        assert!(gate.allows_input());
        focused_key_down(&mut game, &mut movement, &gate, KeyCode::Space, false);
        assert_eq!(game.state, State::Playing);
        gate.on_loss(false);
        assert!(
            gate.allows_input(),
            "ambiguous macOS window move must not disable input"
        );
    }
    #[test]
    fn active_restart_clears_held_movement() {
        for restart in [KeyCode::Enter, KeyCode::KpEnter, KeyCode::R] {
            let mut g = Game::new(42);
            g.start();
            let mut held = MovementInput::default();
            handle_key_down(&mut g, &mut held, KeyCode::W, false);
            handle_key_down(&mut g, &mut held, restart, false);
            assert_eq!(held.axis(), 0.);
            handle_key_down(&mut g, &mut held, KeyCode::W, true);
            assert_eq!(held.axis(), 0.);
        }
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        use pong_core::{Ball, BALL, H, W};
        for key in [KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down] {
            for restart in [KeyCode::Enter, KeyCode::KpEnter, KeyCode::R] {
                let mut game = Game::new(42);
                game.start();
                let mut input = MovementInput::default();
                handle_key_down(&mut game, &mut input, key, false);
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
                handle_key_down(&mut game, &mut input, restart, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(input.axis(), 0., "{key:?} {restart:?}");
                handle_key_down(&mut game, &mut input, key, true);
                assert_eq!(input.axis(), 0.);
                input.key(key, false);
                handle_key_down(&mut game, &mut input, key, false);
                assert_ne!(input.axis(), 0.);
            }
        }
    }

    #[test]
    fn nonplaying_presses_do_not_rearm_after_start() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down] {
                let mut game = Game::new(42);
                game.state = state;
                let mut movement = MovementInput::default();
                handle_key_down(&mut game, &mut movement, key, false);
                handle_key_down(&mut game, &mut movement, KeyCode::Enter, false);
                assert_eq!(game.state, State::Playing);
                assert_eq!(movement.axis(), 0., "{state:?} {key:?}");
                handle_key_down(&mut game, &mut movement, key, true);
                assert_eq!(movement.axis(), 0.);
                movement.key(key, false);
                handle_key_down(&mut game, &mut movement, key, false);
                assert_ne!(movement.axis(), 0.);
            }
        }
    }

    #[test]
    fn keypad_enter_starts_and_restarts_without_repeat() {
        let mut game = Game::new(42);
        control_key(&mut game, KeyCode::KpEnter, false);
        assert_eq!(game.state, State::Playing);
        game.scores = [2, 3];
        control_key(&mut game, KeyCode::KpEnter, true);
        assert_eq!(game.scores, [2, 3]);
        control_key(&mut game, KeyCode::KpEnter, false);
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn mapped_start_pause_resume_and_restart_work() {
        for key in [KeyCode::Enter, KeyCode::R, KeyCode::Space] {
            let mut game = Game::new(42);
            assert!(!control_key(&mut game, key, false));
            assert_eq!(game.state, State::Playing);
        }
        let mut game = Game::new(42);
        control_key(&mut game, KeyCode::Enter, false);
        control_key(&mut game, KeyCode::Space, false);
        assert_eq!(game.state, State::Paused);
        control_key(&mut game, KeyCode::Space, false);
        assert_eq!(game.state, State::Playing);
        game.scores = [3, 4];
        control_key(&mut game, KeyCode::R, false);
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn explicit_pause_clears_movement_until_fresh_press() {
        for key in [KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down] {
            let mut game = Game::new(42);
            let mut movement = MovementInput::default();
            handle_key_down(&mut game, &mut movement, KeyCode::Enter, false);
            handle_key_down(&mut game, &mut movement, key, false);
            assert_ne!(movement.axis(), 0.);
            handle_key_down(&mut game, &mut movement, KeyCode::Space, false);
            assert_eq!(game.state, State::Paused);
            assert_eq!(movement.axis(), 0.);
            handle_key_down(&mut game, &mut movement, KeyCode::Space, false);
            assert_eq!(game.state, State::Playing);
            handle_key_down(&mut game, &mut movement, key, true);
            assert_eq!(movement.axis(), 0., "repeat must not restore cleared input");
            handle_key_down(&mut game, &mut movement, key, false);
            assert_ne!(movement.axis(), 0., "fresh press should move");
        }
    }
    #[test]
    fn repeat_and_unknown_do_not_start_or_pause() {
        let mut game = Game::new(42);
        for key in [KeyCode::Enter, KeyCode::R, KeyCode::Space, KeyCode::Unknown] {
            control_key(&mut game, key, true);
            assert_eq!(game.state, State::Ready);
        }
        assert!(!control_key(&mut game, KeyCode::Unknown, false));
        assert_eq!(game.state, State::Ready);
        control_key(&mut game, KeyCode::Enter, false);
        control_key(&mut game, KeyCode::Space, true);
        assert_eq!(game.state, State::Playing);
        assert!(control_key(&mut game, KeyCode::Escape, false));
    }
}
