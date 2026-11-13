//! Graphical CP437 tile renderer. Not a terminal application.
use bracket_lib::prelude::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use std::collections::HashSet;
const COURT_RGB: (u8, u8, u8) = (9, 14, 25);
const DIVIDER_RGB: (u8, u8, u8) = (120, 142, 166);
fn court_color() -> RGB {
    RGB::from_u8(COURT_RGB.0, COURT_RGB.1, COURT_RGB.2)
}
const COLS: i32 = 90;
const ROWS: i32 = 50;
const TOP: i32 = 10; // Five 16px header rows above the 8px playfield tiles.
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
struct Pong {
    benchmarking: bool,
    benchmark: Option<pong_benchmark::Frames>,
    benchmark_done: bool,
    scenario: pong_scenario::Scenario,
    game: Game,
    keys: HashSet<VirtualKeyCode>,
    suppressed: HashSet<VirtualKeyCode>,
    focused: bool,
    accumulator: f32,
    demo: Option<pong_scenario::Demo>,
    pending_capture: Option<std::path::PathBuf>,
}
impl Pong {
    fn new() -> Self {
        Self {
            benchmarking: false,
            benchmark: None,
            benchmark_done: false,
            scenario: pong_scenario::Scenario::default(),
            game: Game::new(pong_scenario::SEED),
            keys: HashSet::new(),
            suppressed: HashSet::new(),
            focused: true,
            accumulator: 0.,
            demo: None,
            pending_capture: None,
        }
    }
    fn event(&mut self, event: BEvent) -> bool {
        match event {
            BEvent::CloseRequested => return true,
            BEvent::Focused { focused } => {
                self.focused = focused;
                self.keys.clear();
                self.suppressed.clear();
                self.game.pause();
                self.accumulator = 0.;
            }
            BEvent::Resized { .. } => {
                self.game.pause();
                self.keys.clear();
                self.suppressed.clear();
                self.accumulator = 0.;
            }
            BEvent::KeyboardInput { key, pressed, .. } => {
                if !pressed {
                    self.keys.remove(&key);
                    self.suppressed.remove(&key);
                    return false;
                }
                if !self.focused || !self.keys.insert(key) {
                    return false;
                }
                if self.game.state != State::Playing
                    && matches!(
                        key,
                        VirtualKeyCode::W
                            | VirtualKeyCode::Up
                            | VirtualKeyCode::S
                            | VirtualKeyCode::Down
                    )
                {
                    self.suppressed.insert(key);
                }
                match key {
                    VirtualKeyCode::Escape | VirtualKeyCode::Q => return true,
                    VirtualKeyCode::Space => {
                        if matches!(self.game.state, State::Ready | State::Over) {
                            self.game.start()
                        } else {
                            let was_playing = self.game.state == State::Playing;
                            self.game.toggle_pause();
                            if was_playing && self.game.state == State::Paused {
                                for key in [
                                    VirtualKeyCode::W,
                                    VirtualKeyCode::Up,
                                    VirtualKeyCode::S,
                                    VirtualKeyCode::Down,
                                ] {
                                    if self.keys.contains(&key) {
                                        self.suppressed.insert(key);
                                    }
                                }
                            }
                        }
                        self.accumulator = 0.;
                    }
                    VirtualKeyCode::R => {
                        self.game.start();
                        self.accumulator = 0.;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }
    fn advance(&mut self, seconds: f32) {
        if !self.focused || self.game.state != State::Playing {
            self.accumulator = 0.;
            return;
        }
        if !seconds.is_finite() {
            return;
        }
        self.accumulator += seconds.clamp(0., 0.1);
        let active = |key| self.keys.contains(&key) && !self.suppressed.contains(&key);
        let up = active(VirtualKeyCode::W) || active(VirtualKeyCode::Up);
        let down = active(VirtualKeyCode::S) || active(VirtualKeyCode::Down);
        while self.accumulator >= 1. / 120. {
            self.game
                .step(1. / 120., down as u8 as f32 - up as u8 as f32);
            self.accumulator -= 1. / 120.;
        }
    }
}
// Quantize only presentation; physics remains continuous in shared 900×500 world.
fn cells(x: f32, y: f32, w: f32, h: f32) -> (i32, i32, i32, i32) {
    let x0 = (x / W * COLS as f32).floor() as i32;
    let y0 = (y / H * ROWS as f32).floor() as i32;
    let x1 = ((x + w) / W * COLS as f32).ceil() as i32;
    let y1 = ((y + h) / H * ROWS as f32).ceil() as i32;
    (
        x0.clamp(0, COLS),
        y0.clamp(0, ROWS),
        x1.clamp(0, COLS),
        y1.clamp(0, ROWS),
    )
}
fn rect(ctx: &mut BTerm, x: f32, y: f32, w: f32, h: f32, color: RGB) {
    let (x0, y0, x1, y1) = cells(x, y, w, h);
    for yy in y0..y1 {
        for xx in x0..x1 {
            ctx.set(xx, yy + TOP, color, color, to_cp437(' '));
        }
    }
}
fn header_lines(game: &Game) -> [String; 4] {
    [
        "PONG / BRACKET-LIB - GRAPHICAL RETRO EXPERIMENT".into(),
        format!(
            "YOU {}   CPU {}   {:?}   First to 7",
            game.scores[0], game.scores[1], game.state
        ),
        "W/S or arrows: move | Space: start/pause | R: restart | Q/Esc: quit".into(),
        "Focus loss or resize pauses. Space resumes.".into(),
    ]
}
impl GameState for Pong {
    fn tick(&mut self, ctx: &mut BTerm) {
        let events: Vec<_> = {
            let mut input = INPUT.lock();
            let mut events = Vec::new();
            while let Some(e) = input.pop() {
                events.push(e)
            }
            events
        };
        if let Some(path) = self.pending_capture.take() {
            let bytes = std::fs::read(path).expect("framework screenshot completed");
            assert!(
                bytes.ends_with(&[0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130]),
                "incomplete PNG"
            );
        }
        if self.demo.as_ref().is_some_and(|d| d.finished()) {
            ctx.quitting = true;
            return;
        }
        for e in events {
            if self.benchmarking {
                if matches!(e, BEvent::CloseRequested) {
                    ctx.quitting = true;
                }
                continue;
            }
            if self.demo.is_some() {
                continue;
            }
            if self.event(e) {
                ctx.quitting = true;
            }
        }
        if let Some(demo) = &mut self.demo {
            demo.advance(&mut self.game, ctx.frame_time_ms / 1000.);
        } else if self.benchmarking {
            self.accumulator += (ctx.frame_time_ms / 1000.).clamp(0., 0.1);
            while self.accumulator >= pong_scenario::STEP {
                if self.scenario.finished() {
                    self.scenario = pong_scenario::Scenario::default();
                    self.game = Game::new(pong_scenario::SEED);
                }
                self.scenario.advance(&mut self.game);
                self.accumulator -= pong_scenario::STEP;
            }
        } else {
            self.advance(ctx.frame_time_ms / 1000.);
        }
        ctx.set_active_console(0);
        ctx.cls_bg(court_color());
        // Separate transparent VGA console: 16px text without stretching the court.
        ctx.set_active_console(1);
        ctx.cls();
        for (row, line) in header_lines(&self.game).iter().enumerate() {
            let color = if row == 0 {
                RGB::from_u8(84, 224, 226)
            } else {
                RGB::from_u8(235, 241, 250)
            };
            ctx.print_color(2, row as i32, color, BLACK, line);
        }
        ctx.set_active_console(0);
        for y in (0..ROWS).step_by(3) {
            ctx.set(
                COLS / 2,
                y + TOP,
                RGB::from_u8(DIVIDER_RGB.0, DIVIDER_RGB.1, DIVIDER_RGB.2),
                court_color(),
                to_cp437('|'),
            );
        }
        for (x, y) in [(38., self.game.player), (W - 38., self.game.cpu)] {
            rect(
                ctx,
                x - PADDLE_W / 2.,
                y - PADDLE_H / 2.,
                PADDLE_W,
                PADDLE_H,
                RGB::from_u8(84, 224, 226),
            );
        }
        rect(
            ctx,
            self.game.ball.x - BALL,
            self.game.ball.y - BALL,
            2. * BALL,
            2. * BALL,
            RGB::from_u8(250, 226, 160),
        );
        if self.benchmarking && !self.benchmark_done {
            let frames = self.benchmark.get_or_insert_with(|| {
                pong_benchmark::Frames::new(ctx.width_pixels, ctx.height_pixels)
            });
            if frames.frame(ctx.width_pixels, ctx.height_pixels) {
                self.benchmark_done = true;
                println!("PONG_BENCHMARK {}", frames.report());
                ctx.quitting = true;
            }
        }
        if let Some(path) = self.demo.as_mut().and_then(|d| d.capture_path()) {
            ctx.screenshot(path.to_string_lossy());
            self.pending_capture = Some(path);
        }
    }
}
// The pinned winit/glutin Wayland title renderer has a freetype 0.26 UB
// precondition abort on current Omarchy. Prefer XWayland only when available,
// never alter the desktop session or override an explicit caller selection.
#[cfg(any(target_os = "linux", test))]
fn prefer_x11(explicit_backend: bool, wayland: bool, x11: bool) -> bool {
    !explicit_backend && wayland && x11
}
#[cfg(target_os = "linux")]
fn configure_backend() {
    let nonempty = |name| std::env::var_os(name).is_some_and(|v| !v.is_empty());
    if prefer_x11(
        std::env::var_os("WINIT_UNIX_BACKEND").is_some(),
        nonempty("WAYLAND_DISPLAY"),
        nonempty("DISPLAY"),
    ) {
        // First operation in main, before bracket-lib starts any worker threads.
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        eprintln!("Pong bracket-lib: using process-local X11/XWayland fallback for the pinned Wayland font-renderer bug. Explicit WINIT_UNIX_BACKEND is respected.");
    }
}
fn main() -> BError {
    #[cfg(target_os = "linux")]
    configure_backend();
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let ctx = BTermBuilder::simple(COLS, ROWS + TOP)?
        .with_font("vga8x16.png", 8, 16)
        .with_simple_console_no_bg(COLS, (ROWS + TOP) / 2, "vga8x16.png")
        .with_title("Pong bracket-lib")
        .with_fps_cap(60.)
        .with_automatic_console_resize(false)
        .build()?;
    INPUT.lock().activate_event_queue();
    let mut pong = Pong::new();
    pong.demo = pong_scenario::Demo::from_args();
    pong.benchmarking = benchmarking;
    main_loop(ctx, pong)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn benchmark_opt_in() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "distinct")]
    fn rejects_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
    #[test]
    fn backend_fallback_is_scoped_and_respects_choice() {
        assert!(prefer_x11(false, true, true));
        assert!(!prefer_x11(true, true, true));
        assert!(!prefer_x11(false, true, false));
        assert!(!prefer_x11(false, false, true));
    }
    #[test]
    fn header_fits_separate_double_height_console() {
        for state in [State::Ready, State::Playing, State::Paused, State::Over] {
            let mut game = Game::new(42);
            game.state = state;
            game.scores = [7, 7];
            let lines = header_lines(&game);
            assert!(lines
                .iter()
                .all(|line| line.is_ascii() && line.len() + 2 <= COLS as usize));
            assert!(lines.len() * 2 < TOP as usize);
        }
        assert_eq!((ROWS + TOP) % 2, 0);
    }
    fn key(k: VirtualKeyCode, pressed: bool) -> BEvent {
        BEvent::KeyboardInput {
            key: k,
            scan_code: 0,
            pressed,
        }
    }
    #[test]
    fn nonplaying_presses_stay_suppressed_until_release() {
        for state in [State::Ready, State::Paused, State::Over] {
            for movement in [
                VirtualKeyCode::W,
                VirtualKeyCode::Up,
                VirtualKeyCode::S,
                VirtualKeyCode::Down,
            ] {
                let mut p = Pong::new();
                p.game.state = state;
                p.event(key(movement, true));
                p.event(key(VirtualKeyCode::Space, true));
                assert_eq!(p.game.state, State::Playing);
                let before = p.game.player;
                p.advance(0.05);
                assert_eq!(p.game.player, before, "{state:?} {movement:?}");
                p.event(key(movement, false));
                p.event(key(movement, true));
                p.advance(0.05);
                assert_ne!(p.game.player, before);
            }
        }
    }
    #[test]
    fn explicit_pause_blocks_held_movement_until_release() {
        for keycode in [
            VirtualKeyCode::W,
            VirtualKeyCode::Up,
            VirtualKeyCode::S,
            VirtualKeyCode::Down,
        ] {
            let mut p = Pong::new();
            p.event(key(VirtualKeyCode::Space, true));
            p.event(key(VirtualKeyCode::Space, false));
            p.event(key(keycode, true));
            p.advance(0.05);
            assert_ne!(p.game.player, 250.);
            p.event(key(VirtualKeyCode::Space, true));
            assert_eq!(p.game.state, State::Paused);
            p.event(key(VirtualKeyCode::Space, true));
            assert_eq!(p.game.state, State::Paused);
            p.event(key(VirtualKeyCode::Space, false));
            p.event(key(VirtualKeyCode::Space, true));
            assert_eq!(p.game.state, State::Playing);
            p.event(key(keycode, true));
            let stopped = p.game.player;
            p.advance(0.05);
            assert_eq!(p.game.player, stopped, "repeat restored movement");
            p.event(key(keycode, false));
            p.event(key(keycode, true));
            p.advance(0.05);
            assert_ne!(p.game.player, stopped, "fresh press did not move");
        }
    }
    #[test]
    fn pause_suppression_is_per_binding_and_focus_clears_it() {
        let mut p = Pong::new();
        p.game.start();
        p.event(key(VirtualKeyCode::W, true));
        p.event(key(VirtualKeyCode::Up, true));
        p.event(key(VirtualKeyCode::Space, true));
        p.event(key(VirtualKeyCode::Space, false));
        p.event(key(VirtualKeyCode::Space, true));
        p.event(key(VirtualKeyCode::W, false));
        p.event(key(VirtualKeyCode::W, true));
        let before = p.game.player;
        p.advance(0.05);
        assert!(p.game.player < before);
        p.event(key(VirtualKeyCode::W, false));
        let stopped = p.game.player;
        p.advance(0.05);
        assert_eq!(p.game.player, stopped);
        p.event(BEvent::Focused { focused: false });
        assert!(p.suppressed.is_empty());
    }
    #[test]
    fn held_release_and_pause() {
        let mut p = Pong::new();
        p.event(key(VirtualKeyCode::Space, true));
        p.event(key(VirtualKeyCode::Space, false));
        p.event(key(VirtualKeyCode::W, true));
        p.advance(0.05);
        assert!(p.game.player < 250.);
        p.event(key(VirtualKeyCode::W, false));
        let y = p.game.player;
        p.advance(0.05);
        assert_eq!(y, p.game.player);
        p.event(key(VirtualKeyCode::Space, true));
        p.event(key(VirtualKeyCode::Space, true));
        assert_eq!(p.game.state, State::Paused);
    }
    #[test]
    fn focus_clears_input() {
        let mut p = Pong::new();
        p.game.start();
        p.event(key(VirtualKeyCode::W, true));
        p.event(BEvent::Focused { focused: false });
        assert!(p.keys.is_empty());
        assert_eq!(p.game.state, State::Paused);
        p.advance(0.1);
        assert_eq!(p.game.player, 250.);
    }
    #[test]
    fn world_to_tiles_clips() {
        assert_eq!(cells(0., 0., W, H), (0, 0, COLS, ROWS));
        assert_eq!(cells(-20., -20., 10., 10.), (0, 0, 0, 0));
        let (x0, y0, x1, y1) = cells(446., 246., 8., 8.);
        assert!(x1 > x0 && y1 > y0);
    }
    #[test]
    fn catchup_bounded() {
        let mut p = Pong::new();
        p.game.start();
        p.event(key(VirtualKeyCode::S, true));
        p.advance(100.);
        assert!(p.game.player <= 297.1);
    }
    #[test]
    fn divider_contrast_exceeds_non_text_minimum() {
        fn luminance(rgb: (u8, u8, u8)) -> f64 {
            let linear = |c: u8| {
                let v = f64::from(c) / 255.0;
                if v <= 0.04045 {
                    v / 12.92
                } else {
                    ((v + 0.055) / 1.055).powf(2.4)
                }
            };
            0.2126 * linear(rgb.0) + 0.7152 * linear(rgb.1) + 0.0722 * linear(rgb.2)
        }
        assert!((luminance(DIVIDER_RGB) + 0.05) / (luminance(COURT_RGB) + 0.05) >= 3.0);
    }
}
