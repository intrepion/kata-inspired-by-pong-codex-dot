use piston_window::graphics::{clear, ellipse, rectangle, Transformed};

use piston_window::*;
use pong_core::*;
mod clock;
use clock::UpdateClock;
use std::time::Instant;
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
fn window_title(game: &Game) -> String {
    let status: String = match game.state {
        State::Over if game.scores[0] > game.scores[1] => "YOU WIN - Enter to restart".into(),
        State::Over if game.scores[1] > game.scores[0] => "CPU WINS - Enter to restart".into(),
        State::Over => "MATCH OVER - Enter to restart".into(),
        state => format!("{state:?}"),
    };
    format!("Pong Piston | {status} | YOU {} : {} CPU | Enter: play | W/S/arrows | Space: pause | R: restart", game.scores[0], game.scores[1])
}
fn winner_label(game: &Game) -> Option<&'static str> {
    if game.state != State::Over {
        return None;
    }
    if game.scores[0] > game.scores[1] {
        Some("YOU WIN")
    } else if game.scores[1] > game.scores[0] {
        Some("CPU WINS")
    } else {
        None
    }
}
fn result_glyph(ch: u8) -> [u8; 7] {
    match ch {
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'I' => [14, 4, 4, 4, 4, 4, 14],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        _ => [0; 7],
    }
}
fn winner_cells(game: &Game) -> Vec<[f64; 4]> {
    let Some(message) = winner_label(game) else {
        return Vec::new();
    };
    let left = (W as f64 - message.len() as f64 * 36.) / 2.;
    let mut cells = Vec::new();
    for (i, ch) in message.bytes().enumerate() {
        for (y, row) in result_glyph(ch).iter().enumerate() {
            for col in 0..5 {
                if row & (1 << (4 - col)) != 0 {
                    cells.push([
                        left + i as f64 * 36. + col as f64 * 6.,
                        225. + y as f64 * 6.,
                        6.,
                        6.,
                    ]);
                }
            }
        }
    }
    cells
}
fn state_indicator_cells(state: State) -> Vec<[f64; 4]> {
    match state {
        State::Ready => [8., 16., 24., 32., 24., 16., 8.]
            .into_iter()
            .enumerate()
            .map(|(i, width)| [430., 222. + i as f64 * 8., width, 8.])
            .collect(),
        State::Paused => vec![[420., 225., 12., 50.], [468., 225., 12., 50.]],
        State::Playing | State::Over => Vec::new(),
    }
}
fn indicator_backing(game: &Game) -> Option<[f64; 4]> {
    match game.state {
        State::Ready | State::Paused => Some([412., 214., 76., 66.]),
        State::Over => winner_label(game).map(|message| {
            let width = message.len() as f64 * 36.;
            [(W as f64 - width) / 2. - 8., 217., width + 16., 58.]
        }),
        State::Playing => None,
    }
}

// Returns whether the event resets the fixed-step clock.
fn key_press(
    game: &mut Game,
    held: &mut [bool; 4],
    pressed: &mut std::collections::HashSet<Key>,
    key: Key,
) -> bool {
    if !pressed.insert(key) {
        return false;
    }
    for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
        if key == k && game.state == State::Playing {
            held[i] = true;
        }
    }
    let previous = game.state;
    match key {
        Key::Return => {
            if game.state == State::Paused {
                game.toggle_pause();
            } else if game.state != State::Playing {
                game.start();
            }
        }
        Key::R => {
            *held = [false; 4];
            game.start();
        }
        Key::Space => game.toggle_pause(),
        Key::Escape => game.pause(),
        _ => {}
    }
    if (previous == State::Playing && game.state == State::Paused)
        || (previous == State::Over && game.state == State::Playing)
    {
        *held = [false; 4];
    }
    matches!(key, Key::Return | Key::R | Key::Space | Key::Escape)
}
fn focused_key_press(
    game: &mut Game,
    held: &mut [bool; 4],
    pressed: &mut std::collections::HashSet<Key>,
    focused: bool,
    key: Key,
) -> bool {
    focused && key_press(game, held, pressed, key)
}
fn key_release(held: &mut [bool; 4], pressed: &mut std::collections::HashSet<Key>, key: Key) {
    pressed.remove(&key);
    for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
        if key == k {
            held[i] = false;
        }
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;
    #[test]
    fn indicator_backing_contains_each_visible_cell() {
        let mut game = Game::new(42);
        for (state, scores) in [(State::Ready, [0, 0]), (State::Paused, [0, 0]), (State::Over, [7, 2]), (State::Over, [2, 7])] {
            game.state = state; game.scores = scores;
            let panel = indicator_backing(&game).unwrap();
            let cells: Vec<_> = state_indicator_cells(state).into_iter().chain(winner_cells(&game)).collect();
            assert!(!cells.is_empty());
            for [x, y, w, h] in cells {
                assert!(x >= panel[0] && y >= panel[1] && x+w <= panel[0]+panel[2] && y+h <= panel[1]+panel[3]);
            }
        }
        game.state = State::Playing;
        assert_eq!(indicator_backing(&game), None);
    }
    #[test]
    fn ready_play_symbol_and_pause_bars_match_state() {
        let mut game = Game::new(42);
        let ready = state_indicator_cells(game.state);
        assert_eq!(ready.len(), 7);
        assert!(ready.contains(&[430., 246., 32., 8.]));
        game.start();
        assert!(state_indicator_cells(game.state).is_empty());
        game.pause();
        assert_eq!(
            state_indicator_cells(game.state),
            vec![[420., 225., 12., 50.], [468., 225., 12., 50.]]
        );
        game.state = State::Over;
        assert!(state_indicator_cells(game.state).is_empty());
    }
    #[test]
    fn unfocused_key_actions_do_not_resume_or_restart() {
        let mut g = Game::new(42);
        g.start();
        g.scores = [3, 2];
        g.pause();
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        focused_key_press(&mut g, &mut held, &mut pressed, false, Key::Return);
        assert_eq!(g.state, State::Paused);
        focused_key_press(&mut g, &mut held, &mut pressed, false, Key::R);
        assert_eq!(g.state, State::Paused);
        assert_eq!(g.scores, [3, 2]);
        focused_key_press(&mut g, &mut held, &mut pressed, true, Key::Return);
        assert_eq!(g.state, State::Playing);
    }
    #[test]
    fn title_names_winner_after_scoring_and_clears_on_restart() {
        let mut game = Game::new(42);
        assert!(window_title(&game).contains("Ready"));
        for (scores, x, vx, message) in [
            ([6, 3], W + BALL - 1., 350., "YOU WIN - Enter to restart"),
            ([2, 6], -BALL + 1., -350., "CPU WINS - Enter to restart"),
        ] {
            game.start();
            game.scores = scores;
            game.serve = 0.;
            game.ball.x = x;
            game.ball.y = H / 2.;
            game.ball.vx = vx;
            game.ball.vy = 0.;
            game.step(0.02, 0.);
            assert_eq!(game.state, State::Over);
            assert!(window_title(&game).contains(message));
            assert_eq!(
                winner_label(&game),
                Some(if scores[0] == 6 {
                    "YOU WIN"
                } else {
                    "CPU WINS"
                })
            );
            assert!(!winner_cells(&game).is_empty());
            game.start();
            assert!(winner_cells(&game).is_empty());
            assert!(!window_title(&game).contains(message));
            assert!(window_title(&game).contains("Playing"));
        }
    }
    #[test]
    fn active_restart_clears_held_movement() {
        let mut g = Game::new(42);
        g.start();
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        key_press(&mut g, &mut held, &mut pressed, Key::R);
        assert_eq!(held, [false; 4]);
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        assert_eq!(held, [false; 4]);
        key_release(&mut held, &mut pressed, Key::W);
        key_press(&mut g, &mut held, &mut pressed, Key::W);
        assert!(held[0]);
    }
    #[test]
    fn held_movement_does_not_carry_through_match_end_restart() {
        for key in [Key::W, Key::Up, Key::S, Key::Down] {
            for restart in [Key::Return, Key::R] {
                let mut game = Game::new(42);
                game.start();
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, key);
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
                key_press(&mut game, &mut held, &mut pressed, restart);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{key:?} {restart:?}");
                key_press(&mut game, &mut held, &mut pressed, key);
                assert_eq!(held, [false; 4]);
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
            }
        }
    }

    #[test]
    fn nonplaying_presses_do_not_rearm_after_start() {
        for state in [State::Ready, State::Paused, State::Over] {
            for key in [Key::W, Key::Up, Key::S, Key::Down] {
                let mut game = Game::new(42);
                game.state = state;
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                assert_eq!(game.state, State::Playing);
                assert_eq!(held, [false; 4], "{state:?} {key:?}");
                key_press(&mut game, &mut held, &mut pressed, key);
                assert_eq!(held, [false; 4]);
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
            }
        }
    }

    #[test]
    fn actions_and_aliases_preserve_independent_key_behavior() {
        let mut game = Game::new(42);
        let mut held = [false; 4];
        let mut pressed = std::collections::HashSet::new();
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::Return));
        assert_eq!(game.state, State::Playing);
        key_release(&mut held, &mut pressed, Key::Return);
        key_press(&mut game, &mut held, &mut pressed, Key::W);
        key_press(&mut game, &mut held, &mut pressed, Key::Up);
        key_release(&mut held, &mut pressed, Key::W);
        assert!(held[1]);
        key_press(&mut game, &mut held, &mut pressed, Key::Space);
        assert_eq!(game.state, State::Paused);
        assert!(!key_press(&mut game, &mut held, &mut pressed, Key::Space));
        assert_eq!(game.state, State::Paused);
        key_release(&mut held, &mut pressed, Key::Space);
        key_press(&mut game, &mut held, &mut pressed, Key::Space);
        assert_eq!(game.state, State::Playing);
        key_press(&mut game, &mut held, &mut pressed, Key::Up);
        assert!(!held[1]);
        key_press(&mut game, &mut held, &mut pressed, Key::W);
        assert!(held[0]);
        game.scores = [2, 4];
        assert!(key_press(&mut game, &mut held, &mut pressed, Key::R));
        assert_eq!(game.scores, [0, 0]);
    }
    #[test]
    fn explicit_pause_clears_movement_until_release_and_fresh_press() {
        for pause in [Key::Space, Key::Escape] {
            for key in [Key::W, Key::Up, Key::S, Key::Down] {
                let mut game = Game::new(42);
                let mut held = [false; 4];
                let mut pressed = std::collections::HashSet::new();
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                key_release(&mut held, &mut pressed, Key::Return);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v));
                key_press(&mut game, &mut held, &mut pressed, pause);
                assert_eq!(game.state, State::Paused);
                assert!(!held.iter().any(|v| *v));
                key_press(&mut game, &mut held, &mut pressed, Key::Return);
                assert_eq!(game.state, State::Playing);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(!held.iter().any(|v| *v), "repeat restored movement");
                key_release(&mut held, &mut pressed, key);
                key_press(&mut game, &mut held, &mut pressed, key);
                assert!(held.iter().any(|v| *v), "fresh press should move");
            }
        }
    }
}
fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let mut window: PistonWindow = WindowSettings::new(
        "Pong Piston | Enter: start | W/S/arrows: move | Space: pause | R: restart",
        [1000, 680],
    )
    .exit_on_esc(false)
    .resizable(true)
    .build()
    .expect("create Piston window");
    window.set_ups(120);
    let mut game = Game::new(pong_scenario::SEED);
    let mut scenario = pong_scenario::Scenario::default();
    let mut benchmark: Option<pong_benchmark::Frames> = None;
    let mut demo = pong_scenario::Demo::from_args();
    let mut held = [false; 4];
    let mut pressed = std::collections::HashSet::new();
    let mut clock = UpdateClock::new(Instant::now());
    let mut last_title = String::new();
    let mut focused = true;
    while let Some(e) = window.next() {
        if demo.is_none() && !benchmarking {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if focused_key_press(&mut game, &mut held, &mut pressed, focused, key) {
                    clock.reset(Instant::now());
                }
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                key_release(&mut held, &mut pressed, key);
            }
            if let Some(now_focused) = e.focus_args() {
                focused = now_focused;
                if !focused {
                    game.pause();
                    held = [false; 4];
                    pressed.clear();
                    clock.reset(Instant::now());
                }
            }
        }
        if e.update_args().is_some() {
            let elapsed = clock.elapsed(Instant::now());
            if let Some(demo) = &mut demo {
                demo.advance(&mut game, elapsed as f32);
            } else {
                for _ in 0..clock.steps(elapsed) {
                    if benchmarking {
                        if scenario.finished() {
                            scenario = pong_scenario::Scenario::default();
                            game = Game::new(pong_scenario::SEED);
                        }
                        scenario.advance(&mut game);
                    } else {
                        game.step(
                            1. / 120.,
                            (held[2] || held[3]) as u8 as f32 - (held[0] || held[1]) as u8 as f32,
                        );
                    }
                }
            }
        }
        if e.render_args().is_some() {
            let title = window_title(&game);
            if title != last_title {
                window.set_title(title.clone());
                last_title = title;
            }
        }
        window.draw_2d(&e, |c, g, _| draw_scene(c, g, &game));
        if let Some(args) = e.render_args() {
            if benchmarking {
                let frames = benchmark.get_or_insert_with(|| {
                    pong_benchmark::Frames::new(args.draw_size[0], args.draw_size[1])
                });
                if frames.frame(args.draw_size[0], args.draw_size[1]) {
                    println!("PONG_BENCHMARK {}", frames.report());
                    break;
                }
            }
            if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
                capture(&mut window, args, &game, &path).expect("app-owned render target capture");
            }
            if demo.as_ref().is_some_and(|d| d.finished()) {
                break;
            }
        }
    }
}

fn draw_scene<G: piston_window::graphics::Graphics>(
    c: piston_window::graphics::Context,
    g: &mut G,
    game: &Game,
) {
    clear([0.047, 0.129, 0.118, 1.], g);
    let size = c.get_view_size();
    let s = (size[0] / W as f64).min(size[1] / H as f64);
    let t = c
        .transform
        .trans((size[0] - W as f64 * s) / 2., (size[1] - H as f64 * s) / 2.)
        .scale(s, s);
    rectangle(
        [0.082, 0.247, 0.216, 1.],
        [0., 0., W as f64, H as f64],
        t,
        g,
    );
    for y in (0..500).step_by(28) {
        rectangle([0.27, 0.43, 0.35, 1.], [449., y as f64, 2., 12.], t, g);
    }
    for (x, y) in [(32., game.player), (W - 44., game.cpu)] {
        rectangle(
            [0.85, 1., 0.68, 1.],
            [
                x as f64,
                (y - PADDLE_H / 2.) as f64,
                PADDLE_W as f64,
                PADDLE_H as f64,
            ],
            t,
            g,
        );
    }
    ellipse(
        [1.; 4],
        [
            (game.ball.x - BALL) as f64,
            (game.ball.y - BALL) as f64,
            (BALL * 2.) as f64,
            (BALL * 2.) as f64,
        ],
        t,
        g,
    );
    // Score bars avoid platform-dependent font files; full numeric score/status in title.
    for (side, n) in game.scores.iter().enumerate() {
        for i in 0..*n {
            rectangle(
                [0.85, 1., 0.68, 1.],
                [330. + side as f64 * 170. + i as f64 * 12., 20., 8., 18.],
                t,
                g,
            );
        }
    }
    // Cover the ball and net before drawing the state, preserving legibility.
    if let Some(panel) = indicator_backing(game) {
        rectangle([0.047, 0.129, 0.118, 1.], panel, t, g);
    }
    for cell in state_indicator_cells(game.state)
        .into_iter()
        .chain(winner_cells(game))
    {
        rectangle([0.85, 1., 0.68, 1.], cell, t, g);
    }
}

fn capture(
    window: &mut PistonWindow,
    args: RenderArgs,
    game: &Game,
    path: &std::path::Path,
) -> Result<(), String> {
    let capture = Some(path);
    let texture = window.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Pong app capture target"),
        size: wgpu::Extent3d {
            width: window.surface_config.width,
            height: window.surface_config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: window.surface_config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let (_, commands) = window
        .g2d
        .draw(&window.surface_config, &view, args.viewport(), |c, g| {
            draw_scene(c, g, game)
        });
    window.queue.submit([commands]);
    let mut encoder = window.device.create_command_encoder(&Default::default());
    let readback = if capture.is_some() {
        match window.surface_config.format {
            wgpu::TextureFormat::Rgba8Unorm
            | wgpu::TextureFormat::Rgba8UnormSrgb
            | wgpu::TextureFormat::Bgra8Unorm
            | wgpu::TextureFormat::Bgra8UnormSrgb => {}
            other => return Err(format!("Capture pixel format unsupported: {other:?}")),
        }
        let stride = (window.surface_config.width * 4).div_ceil(256) * 256;
        let buffer = window.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("app capture readback"),
            size: u64::from(stride) * u64::from(window.surface_config.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: Some(window.surface_config.height),
                },
            },
            wgpu::Extent3d {
                width: window.surface_config.width,
                height: window.surface_config.height,
                depth_or_array_layers: 1,
            },
        );
        Some((buffer, stride))
    } else {
        None
    };
    window.queue.submit([encoder.finish()]);
    if let (Some(path), Some((buffer, stride))) = (capture, readback) {
        let (tx, rx) = std::sync::mpsc::channel();
        buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            let _ = tx.send(result);
        });
        window
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .map_err(|e| e.to_string())?;
        rx.recv_timeout(std::time::Duration::from_secs(1))
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        let bytes = buffer.get_mapped_range(..);
        let bgra = matches!(
            window.surface_config.format,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
        );
        let mut rgb = Vec::with_capacity(
            window.surface_config.width as usize * window.surface_config.height as usize,
        );
        for row in bytes
            .chunks(stride as usize)
            .take(window.surface_config.height as usize)
        {
            for c in row[..window.surface_config.width as usize * 4]
                .as_chunks::<4>()
                .0
            {
                let (r, b) = if bgra { (c[2], c[0]) } else { (c[0], c[2]) };
                rgb.push((u32::from(r) << 16) | (u32::from(c[1]) << 8) | u32::from(b));
            }
        }
        pong_scenario::Demo::save_rgb(
            path,
            &rgb,
            window.surface_config.width,
            window.surface_config.height,
        );
        drop(bytes);
        buffer.unmap();
    }

    Ok(())
}

#[cfg(test)]
mod benchmark_tests {
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
}
