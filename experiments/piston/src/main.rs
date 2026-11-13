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
        if key == k {
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
        Key::R => game.start(),
        Key::Space => game.toggle_pause(),
        Key::Escape => game.pause(),
        _ => {}
    }
    if previous == State::Playing && game.state == State::Paused {
        *held = [false; 4];
    }
    matches!(key, Key::Return | Key::R | Key::Space | Key::Escape)
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
    while let Some(e) = window.next() {
        if demo.is_none() && !benchmarking {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key_press(&mut game, &mut held, &mut pressed, key) {
                    clock.reset(Instant::now());
                }
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                key_release(&mut held, &mut pressed, key);
            }
            if let Some(false) = e.focus_args() {
                game.pause();
                held = [false; 4];
                pressed.clear();
                clock.reset(Instant::now());
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
            let title = format!("Pong Piston | YOU {} : {} CPU | {:?} | Enter: play | W/S/arrows | Space: pause | R: restart",game.scores[0],game.scores[1],game.state);
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
