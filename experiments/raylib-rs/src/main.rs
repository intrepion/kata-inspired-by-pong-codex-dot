use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use raylib::prelude::*;

/// Returns aspect-preserving court scale and letterbox offsets.
fn viewport(width: f32, height: f32) -> (f32, f32, f32) {
    let scale = (width / W).min(height / H).max(0.001);
    (scale, (width - W * scale) / 2.0, (height - H * scale) / 2.0)
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
    let (mut window, thread) = raylib::init()
        .size(W as i32, H as i32)
        .title("Pong / raylib-rs")
        .resizable()
        .build();
    window.set_target_fps(120);
    let mut game = Game::new(pong_scenario::SEED);
    let mut accumulator = 0.0;
    let mut demo = pong_scenario::Demo::from_args();
    let mut scenario = pong_scenario::Scenario::default();
    let mut benchmark = benchmarking.then(|| {
        pong_benchmark::Frames::new(
            window.get_screen_width() as u32,
            window.get_screen_height() as u32,
        )
    });
    while !window.window_should_close() {
        let focused = window.is_window_focused();
        if !focused && demo.is_none() && !benchmarking {
            game.pause();
            accumulator = 0.0;
        }
        if focused && demo.is_none() && !benchmarking {
            if window.is_key_pressed(KeyboardKey::KEY_ENTER)
                || window.is_key_pressed(KeyboardKey::KEY_R)
            {
                game.start();
            }
            if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
                if game.state == State::Ready {
                    game.start();
                } else {
                    game.toggle_pause();
                }
            }
            let up =
                window.is_key_down(KeyboardKey::KEY_W) || window.is_key_down(KeyboardKey::KEY_UP);
            let down =
                window.is_key_down(KeyboardKey::KEY_S) || window.is_key_down(KeyboardKey::KEY_DOWN);
            accumulator += window.get_frame_time().min(0.1);
            while accumulator >= 1.0 / 120.0 {
                game.step(1.0 / 120.0, down as u8 as f32 - up as u8 as f32);
                accumulator -= 1.0 / 120.0;
            }
        }
        if benchmarking {
            accumulator += window.get_frame_time().min(0.1);
            while accumulator >= pong_scenario::STEP {
                if scenario.finished() {
                    scenario = pong_scenario::Scenario::default();
                    game = Game::new(pong_scenario::SEED);
                }
                scenario.advance(&mut game);
                accumulator -= pong_scenario::STEP;
            }
        }
        if let Some(demo) = &mut demo {
            demo.advance(&mut game, window.get_frame_time());
        }
        let (s, ox, oy) = viewport(
            window.get_screen_width() as f32,
            window.get_screen_height() as f32,
        );
        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_rectangle_rec(
            Rectangle::new(ox, oy, W * s, H * s),
            Color::new(6, 10, 23, 255),
        );
        for y in (0..500).step_by(25) {
            d.draw_rectangle_rec(
                Rectangle::new(
                    ox + (W / 2.0 - 1.0) * s,
                    oy + y as f32 * s,
                    2.0 * s,
                    12.0 * s,
                ),
                Color::new(45, 64, 89, 255),
            );
        }
        for (x, y) in [(38.0, game.player), (W - 38.0, game.cpu)] {
            d.draw_rectangle_rec(
                Rectangle::new(
                    ox + (x - PADDLE_W / 2.0) * s,
                    oy + (y - PADDLE_H / 2.0) * s,
                    PADDLE_W * s,
                    PADDLE_H * s,
                ),
                Color::new(89, 242, 217, 255),
            );
        }
        d.draw_circle_v(
            Vector2::new(ox + game.ball.x * s, oy + game.ball.y * s),
            BALL * s,
            Color::WHITE,
        );
        d.draw_text(
            &format!("{}      {}", game.scores[0], game.scores[1]),
            (ox + W * s / 2.0 - 60.0 * s) as i32,
            (oy + 25.0 * s) as i32,
            (32.0 * s).max(10.0) as i32,
            Color::WHITE,
        );
        d.draw_text(
            &format!(
                "{:?} | W/S arrows | Enter/R restart | Space pause | Esc exit",
                game.state
            ),
            (ox + 15.0 * s) as i32,
            (oy + H * s - 26.0 * s) as i32,
            (14.0 * s).max(8.0) as i32,
            Color::GRAY,
        );
        if let Some(demo) = &demo {
            d.draw_text(
                &format!("SCRIPTED / tick {}", demo.tick()),
                12,
                12,
                18,
                Color::YELLOW,
            );
        }
        if let Some(demo) = &mut demo {
            if let Some(path) = demo.capture_path() {
                // Flush raylib's draw batch before reading the current backbuffer.
                // Reading after EndDrawing swaps buffers can return the previous frame.
                unsafe {
                    raylib::ffi::rlDrawRenderBatchActive();
                }
                d.load_image_from_screen(&thread)
                    .export_image(path.to_str().expect("UTF-8 capture path"));
                assert!(
                    std::fs::metadata(&path).is_ok_and(|m| m.len() > 0),
                    "capture export failed"
                );
            }
            if demo.finished() {
                break;
            }
        }
        // Drop performs EndDrawing, including raylib's existing target-FPS wait.
        drop(d);
        if let Some(frames) = benchmark.as_mut() {
            if frames.frame(
                window.get_screen_width() as u32,
                window.get_screen_height() as u32,
            ) {
                println!("PONG_BENCHMARK {}", frames.report());
                break;
            }
        }
    }
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
    fn letterboxing_preserves_aspect() {
        assert_eq!(viewport(1800.0, 1000.0), (2.0, 0.0, 0.0));
        assert_eq!(viewport(1000.0, 500.0), (1.0, 50.0, 0.0));
        assert_eq!(viewport(900.0, 700.0), (1.0, 0.0, 100.0));
    }
}
