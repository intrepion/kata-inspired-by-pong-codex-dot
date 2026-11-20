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
fn reset_live_clock(acc: &mut f32, last: &mut Instant, now: Instant) { *acc = 0.; *last = now; }
fn handle_live_key(game: &mut Game, movement: &mut MovementInput, gate: &FocusGate, key: KeyCode, repeat: bool, acc: &mut f32, last: &mut Instant, now: Instant) -> bool {
    let before = game.state;
    let quit = focused_key_down(game, movement, gate, key, repeat);
    let restart = gate.allows_input() && !repeat && matches!(key, KeyCode::Enter | KeyCode::KpEnter | KeyCode::R);
    if before != game.state || restart { reset_live_clock(acc, last, now); }
    quit
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
fn score_vertices(v: &mut Vec<Vertex>, scores: [u32; 2]) {
    for side in 0..2 {
        let direction = side as f32 * 2. - 1.;
        let x = W / 2. + direction * 30.;
        if scores[side] == 0 {
            for (dx, dy, w, h) in [
                (-3., 0., 2., 16.),
                (3., 0., 2., 16.),
                (0., -7., 4., 2.),
                (0., 7., 4., 2.),
            ] {
                rect(v, x + dx, 24. + dy, w, h, [1., 1., 1.]);
            }
        } else {
            for i in 0..scores[side] {
                rect(
                    v,
                    W / 2. + direction * (30. + i as f32 * 16.),
                    24.,
                    8.,
                    16.,
                    [1., 1., 1.],
                );
            }
        }
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
// Preserve the normal design and raise the small-window glyph floor without clipping.
fn hud_cell(scale: f32, normal: f32, cap: f32) -> f32 {
    if scale.is_finite() && scale > 0. {
        (1.5 / scale).clamp(normal, cap)
    } else {
        normal
    }
}
fn text_width(text: &str, cell: f32) -> f32 {
    (text.len() as f32 * 6. - 1.) * cell
}
fn status_text(v: &mut Vec<Vertex>, text: &str, y: f32, scale: f32) {
    let x = W / 2. - text_width(text, scale) / 2.;
    text_at(v, text, x, y, scale);
}
fn text_at(v: &mut Vec<Vertex>, text: &str, x: f32, y: f32, scale: f32) {
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
fn score_label_vertices(v: &mut Vec<Vertex>, scale: f32) {
    let cell = hud_cell(scale, 2., 6.);
    for (label, center) in [("YOU", 371.), ("CPU", 527.)] {
        text_at(v, label, center - text_width(label, cell) / 2., 42., cell);
    }
}
const PANEL: [f32; 3] = [0.025, 0.04, 0.09];
fn state_panel(v: &mut Vec<Vertex>, width: f32, top: f32, bottom: f32) {
    rect(v, W / 2., (top + bottom) / 2., width, bottom - top, PANEL);
}
fn game_over_vertices(game: &Game, v: &mut Vec<Vertex>, scale: f32) {
    if game.state == State::Over {
        let winner = if game.scores[0] > game.scores[1] { "YOU WIN" } else { "CPU WINS" };
        let hint = "ENTER TO RESTART";
        let winner_cell = hud_cell(scale, 4., 8.);
        let hint_cell = hud_cell(scale, 2., 6.);
        let winner_top = H / 2. - 6. - 7. * winner_cell;
        let hint_top = H / 2. + 8.;
        let width = 220_f32.max(text_width(winner, winner_cell).max(text_width(hint, hint_cell)) + 16.);
        state_panel(v, width, (H / 2. - 48.).min(winner_top - 8.),
            (H / 2. + 36.).max(hint_top + 7. * hint_cell + 8.));
        status_text(v, winner, winner_top, winner_cell);
        status_text(v, hint, hint_top, hint_cell);
    }
}
fn state_vertices(game: &Game, v: &mut Vec<Vertex>, scale: f32) {
    let cell = hud_cell(scale, 2., 6.);
    match game.state {
        State::Ready | State::Paused => {
            let hint = if game.state == State::Ready { "ENTER TO START" } else { "SPACE TO RESUME" };
            let top = H / 2. - 40.;
            state_panel(v, 240_f32.max(text_width(hint, cell) + 16.), H / 2. - 106.,
                (H / 2. - 24.).max(top + 7. * cell + 5.));
            if game.state == State::Ready {
                for (i, width) in [8., 16., 24., 16., 8.].into_iter().enumerate() {
                    rect(v, W / 2. - 12. + width / 2., H / 2. - 86. + i as f32 * 8.,
                        width, 8., [1., 1., 1.]);
                }
            } else {
                rect(v, W / 2. - 14., H / 2. - 70., 8., 40., [1., 1., 1.]);
                rect(v, W / 2. + 14., H / 2. - 70., 8., 40., [1., 1., 1.]);
            }
            status_text(v, hint, top, cell);
        }
        State::Over => game_over_vertices(game, v, scale),
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
        // Draw a zero for empty groups and retain pips for positive scores.
        score_vertices(&mut v, self.game.scores);
        let (w, h) = window::screen_size();
        let s = (w / W).min(h / H);
        score_label_vertices(&mut v, s);
        state_vertices(&self.game, &mut v, s);
        self.ctx
            .buffer_update(self.bindings.vertex_buffers[0], BufferSource::slice(&v));
        self.ctx
            .begin_default_pass(PassAction::clear_color(0.025, 0.04, 0.09, 1.));
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
        if handle_live_key(
            &mut self.game,
            &mut self.movement,
            &self.focus_gate,
            k,
            repeat,
            &mut self.acc,
            &mut self.last,
            Instant::now(),
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
        reset_live_clock(&mut self.acc, &mut self.last, Instant::now());
    }
    fn window_restored_event(&mut self) {
        self.focus_gate.on_restore();
        if self.demo.is_none() && !self.benchmarking {
            reset_live_clock(&mut self.acc, &mut self.last, Instant::now());
        }
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
    fn zero_scores_have_outlines_and_positive_scores_keep_pips() {
        let mut vertices = Vec::new();
        score_vertices(&mut vertices, [0, 0]);
        assert_eq!(vertices.len(), 8 * 6);
        vertices.clear();
        score_vertices(&mut vertices, [0, 1]);
        assert_eq!(vertices.len(), 5 * 6);
        let pip_left = (W / 2. + 30. - 4.) / W * 2. - 1.;
        assert!((vertices[24].pos[0] - pip_left).abs() < 1e-6);
        vertices.clear();
        score_vertices(&mut vertices, [2, 3]);
        assert_eq!(vertices.len(), 5 * 6);
    }
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
        game_over_vertices(&game, &mut vertices, 1.);
        assert!(vertices.is_empty());
        game.state = State::Over;
        game.scores = [7, 2];
        game_over_vertices(&game, &mut vertices, 1.);
        assert_eq!(vertices[0].color, [0.025, 0.04, 0.09]);
        assert_eq!(vertices[5].color, [0.025, 0.04, 0.09]);
        assert_eq!(vertices[6].color, [1., 1., 1.]);
        assert!((vertices[0].pos[0] - ((W / 2. - 110.) / W * 2. - 1.)).abs() < 1e-6);
        let player_count = vertices.len();
        assert!(player_count > 0 && player_count < 4096);
        vertices.clear();
        game.scores = [2, 7];
        game_over_vertices(&game, &mut vertices, 1.);
        assert_ne!(vertices.len(), player_count);
        vertices.clear();
        game.start();
        game_over_vertices(&game, &mut vertices, 1.);
        assert!(vertices.is_empty());
    }
    #[test]
    fn viewport_hud_cells_and_state_backings_stay_readable_and_on_court() {
        for scale in [0.25, 1., 2.] {
            let small = hud_cell(scale, 2., 6.);
            assert!(small * scale >= 1.5);
            assert_eq!(small, if scale < 1. { 6. } else { 2. });
            let mut game = Game::new(42);
            for state in [State::Ready, State::Paused, State::Over] {
                game.state = state;
                for scores in [[7, 2], [2, 7]] {
                    game.scores = scores;
                    let mut vertices = Vec::new();
                    state_vertices(&game, &mut vertices, scale);
                    let [left, top] = vertices[0].pos;
                    let [right, bottom] = vertices[2].pos;
                    assert!([-1., 1.].windows(2).all(|r| left >= r[0] && bottom >= r[0] && right <= r[1] && top <= r[1]));
                    for Vertex { pos: [x, y], .. } in &vertices[6..] {
                        assert!(*x >= left - 1e-6 && *x <= right + 1e-6);
                        assert!(*y >= bottom - 1e-6 && *y <= top + 1e-6);
                    }
                }
            }
        }
    }
    #[test]
    fn score_labels_and_full_winner_court_fit_vertex_buffer() {
        let mut vertices = Vec::new();
        score_label_vertices(&mut vertices, 1.);
        assert!(!vertices.is_empty());
        assert!(vertices.iter().any(|v| v.pos[0] < 0.));
        assert!(vertices.iter().any(|v| v.pos[0] > 0.));
        // Court contains 20 net dashes, two paddles, ball, and at most 14 score pips.
        let scene_vertices = (20 + 2 + 1 + 14) * 6;
        let mut game = Game::new(42);
        game.state = State::Over;
        for scores in [[7, 2], [2, 7]] {
            game.scores = scores;
            let mut state = Vec::new();
            state_vertices(&game, &mut state, 1.);
            assert!(scene_vertices + vertices.len() + state.len() < 4096);
        }
    }
    #[test]
    fn ready_and_paused_indicators_are_distinct_and_absent_during_play() {
        let mut game = Game::new(42);
        let mut vertices = Vec::new();
        state_vertices(&game, &mut vertices, 1.);
        let ready = vertices.len();
        assert!(ready > 0);
        vertices.clear();
        game.start();
        state_vertices(&game, &mut vertices, 1.);
        assert!(vertices.is_empty());
        game.pause();
        state_vertices(&game, &mut vertices, 1.);
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
    fn ignored_actions_movement_and_escape_preserve_clock() {
        use std::time::Duration;
        let mut game = Game::new(42);
        let mut movement = MovementInput::default();
        let mut gate = FocusGate::default();
        let base = Instant::now();
        let mut last = base;
        let mut acc = 0.004;
        for key in [KeyCode::Enter, KeyCode::KpEnter, KeyCode::R, KeyCode::Space] {
            handle_live_key(&mut game, &mut movement, &gate, key, true, &mut acc, &mut last, base + Duration::from_secs(1));
            assert_eq!((acc, last), (0.004, base), "repeated {key:?} reset clock");
        }
        gate.on_loss(true);
        handle_live_key(&mut game, &mut movement, &gate, KeyCode::R, false, &mut acc, &mut last, base + Duration::from_secs(2));
        assert_eq!((acc, last), (0.004, base));
        gate.on_restore();
        game.state = State::Over;
        handle_live_key(&mut game, &mut movement, &gate, KeyCode::Space, false, &mut acc, &mut last, base + Duration::from_secs(3));
        assert_eq!(game.state, State::Over);
        assert_eq!((acc, last), (0.004, base));
        handle_live_key(&mut game, &mut movement, &gate, KeyCode::W, false, &mut acc, &mut last, base + Duration::from_secs(4));
        assert_eq!((acc, last), (0.004, base));
        assert!(handle_live_key(&mut game, &mut movement, &gate, KeyCode::Escape, false, &mut acc, &mut last, base + Duration::from_secs(5)));
        assert_eq!((acc, last), (0.004, base));
    }
    #[test]
    fn action_transitions_discard_pretransition_time() {
        use std::time::Duration;
        let mut game=Game::new(42);let mut movement=MovementInput::default();let mut gate=FocusGate::default();
        let base=Instant::now();let mut last=base;let mut acc=0.007;let start=base+Duration::from_secs(1);
        handle_live_key(&mut game,&mut movement,&gate,KeyCode::Enter,false,&mut acc,&mut last,start);
        assert_eq!(game.state,State::Playing);assert_eq!(acc,0.);assert_eq!(last,start);
        acc=0.007;let pause=start+Duration::from_millis(10);
        handle_live_key(&mut game,&mut movement,&gate,KeyCode::Space,false,&mut acc,&mut last,pause);
        assert_eq!(game.state,State::Paused);assert_eq!(acc,0.);assert_eq!(last,pause);
        acc=0.006;let resume=pause+Duration::from_secs(2);
        handle_live_key(&mut game,&mut movement,&gate,KeyCode::Space,false,&mut acc,&mut last,resume);
        assert_eq!(game.state,State::Playing);assert_eq!(acc,0.);assert_eq!(last,resume);
        acc=0.006;let restart=resume+Duration::from_millis(10);
        handle_live_key(&mut game,&mut movement,&gate,KeyCode::KpEnter,false,&mut acc,&mut last,restart);
        assert_eq!(acc,0.);assert_eq!(last,restart);
        acc=0.004;gate.on_loss(true);
        handle_live_key(&mut game,&mut movement,&gate,KeyCode::R,false,&mut acc,&mut last,restart+Duration::from_millis(1));
        assert_eq!(acc,0.004);assert_eq!(last,restart,"unfocused actions must not reset the clock");
        let regained=restart+Duration::from_secs(5);
        reset_live_clock(&mut acc,&mut last,regained);
        assert_eq!(acc,0.);assert_eq!(last,regained);
    }
    #[test]
    fn input_controls_pause_full_match_and_keypad_restart() {
        let mut game = Game::new(42);
        let mut movement = MovementInput::default();
        assert!(!handle_key_down(&mut game, &mut movement, KeyCode::Enter, false));
        assert_eq!(game.state, State::Playing);
        for _ in 0..300 { game.step(1. / 120., movement.axis()); }
        assert!(!handle_key_down(&mut game, &mut movement, KeyCode::Space, false));
        assert_eq!(game.state, State::Paused);
        let before = (game.player, game.cpu, game.ball.x, game.ball.y, game.scores);
        for _ in 0..120 { game.step(1. / 120., movement.axis()); }
        assert_eq!((game.player, game.cpu, game.ball.x, game.ball.y, game.scores), before, "paused game must remain still");
        handle_key_down(&mut game, &mut movement, KeyCode::Space, false);
        assert_eq!(game.state, State::Playing);
        let mut steps = 0;
        while game.state != State::Over && steps < 60_000 { game.step(1. / 120., movement.axis()); steps += 1; }
        assert_eq!(game.state, State::Over, "unattended match must finish");
        let scores = game.scores;
        assert!(scores.iter().any(|score| *score == 7));
        handle_key_down(&mut game, &mut movement, KeyCode::KpEnter, false);
        assert_eq!(game.state, State::Playing);
        assert_eq!(game.scores, [0, 0]);
        println!("PASS Miniquad model match {scores:?} in {steps} fixed steps; keypad reset");
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
