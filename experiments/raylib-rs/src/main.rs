use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use raylib::prelude::*;

/// Reserve a scaled footer below the aspect-preserving court.
const FOOTER_H: f32 = 36.;
fn viewport(width: f32, height: f32) -> (f32, f32, f32) {
    let scale = (width / W).min(height / (H + FOOTER_H)).max(0.001);
    (
        scale,
        (width - W * scale) / 2.0,
        (height - (H + FOOTER_H) * scale) / 2.0,
    )
}

fn status_position(ox: f32, oy: f32, scale: f32) -> (f32, f32) {
    (ox + 15. * scale, oy + (H + 8.) * scale)
}
fn escape_requests_exit(focused: bool, pressed: bool) -> bool { focused && pressed }
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
}
fn winner_message(game: &Game) -> Option<&'static str> {
    if game.state != State::Over {
        return None;
    }
    Some(if game.scores[0] > game.scores[1] {
        "YOU WIN - ENTER OR R TO RESTART"
    } else if game.scores[1] > game.scores[0] {
        "CPU WINS - ENTER OR R TO RESTART"
    } else {
        "MATCH OVER - ENTER OR R TO RESTART"
    })
}
fn court_message(game: &Game) -> Option<&'static str> {
    match game.state {
        State::Ready => Some("READY - SPACE OR ENTER TO START"),
        State::Paused => Some("PAUSED - SPACE TO RESUME"),
        State::Over => winner_message(game),
        State::Playing => None,
    }
}

#[derive(Default)]
struct MovementGate {
    suppressed: [bool; 4],
    was_playing: bool,
}
impl MovementGate {
    fn finish_frame(&mut self, state: State) {
        self.was_playing &= state == State::Playing;
    }
    fn axis(&mut self, held: [bool; 4], entering_pause: bool) -> f32 {
        for (blocked, down) in self.suppressed.iter_mut().zip(held) {
            *blocked = down && (entering_pause || *blocked);
        }
        let active = std::array::from_fn::<_, 4, _>(|i| held[i] && !self.suppressed[i]);
        (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
    }
}
fn restart(game: &mut Game, movement: &mut MovementGate) {
    movement.was_playing = false;
    game.start();
}
fn movement_with_pause(
    game: &mut Game,
    movement: &mut MovementGate,
    held: [bool; 4],
    pause_pressed: bool,
) -> f32 {
    if pause_pressed {
        if game.state == State::Ready {
            game.start();
        } else {
            game.toggle_pause();
        }
    }
    let playing = game.state == State::Playing;
    let suppress = !playing || !movement.was_playing;
    movement.was_playing = playing;
    movement.axis(held, suppress)
}
fn lose_focus(game: &mut Game, movement: &mut MovementGate, accumulator: &mut f32) {
    movement.was_playing = false;
    game.pause();
    *accumulator = 0.;
}
fn main() {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    let (mut window, thread) = raylib::init()
        .size(W as i32, H as i32)
        .title("Pong / raylib-rs")
        .resizable()
        .build();
    window.set_target_fps(120);
    window.set_exit_key(None);
    let mut game = Game::new(pong_scenario::SEED);
    let mut movement = MovementGate::default();
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
        if escape_requests_exit(focused, window.is_key_pressed(KeyboardKey::KEY_ESCAPE)) { break; }
        if !focused && demo.is_none() && !benchmarking {
            lose_focus(&mut game, &mut movement, &mut accumulator);
        }
        if focused && demo.is_none() && !benchmarking {
            if window.is_key_pressed(KeyboardKey::KEY_ENTER)
                || window.is_key_pressed(KeyboardKey::KEY_R)
            {
                restart(&mut game, &mut movement);
            }
            let axis = movement_with_pause(
                &mut game,
                &mut movement,
                [
                    window.is_key_down(KeyboardKey::KEY_UP),
                    window.is_key_down(KeyboardKey::KEY_W),
                    window.is_key_down(KeyboardKey::KEY_DOWN),
                    window.is_key_down(KeyboardKey::KEY_S),
                ],
                window.is_key_pressed(KeyboardKey::KEY_SPACE),
            );
            accumulator += window.get_frame_time().min(0.1);
            while accumulator >= 1.0 / 120.0 {
                game.step(1.0 / 120.0, axis);
                accumulator -= 1.0 / 120.0;
            }
            movement.finish_frame(game.state);
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
        if let Some(message) = court_message(&game) {
            let font_size = (24.0 * s).max(1.0) as i32;
            let width = d.measure_text(message, font_size);
            let x = (ox + W * s / 2.0 - width as f32 / 2.0) as i32;
            let y = (oy + 235.0 * s) as i32;
            let padding = (8.0 * s).max(1.0) as i32;
            d.draw_rectangle(x - padding, y - padding, width + 2 * padding, font_size + 2 * padding, Color::new(6, 10, 23, 255));
            d.draw_text(
                message,
                x,
                y,
                font_size,
                Color::WHITE,
            );
        }
        let (status_x, status_y) = status_position(ox, oy, s);
        d.draw_text(
            &format!(
                "{:?} | W/S arrows | Enter/R restart | Space pause | Esc exit",
                game.state
            ),
            status_x as i32,
            status_y as i32,
            (14.0 * s).max(1.0) as i32,
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
    fn escape_only_exits_while_focused() {
        assert!(!escape_requests_exit(false, true));
        assert!(escape_requests_exit(true, true));
        assert!(!escape_requests_exit(true, false));
    }
    #[test]
    fn court_prompt_tracks_nonplaying_states_and_winner() {
        let mut game = Game::new(42);
        assert_eq!(court_message(&game), Some("READY - SPACE OR ENTER TO START"));
        game.start(); assert_eq!(court_message(&game), None);
        game.pause(); assert_eq!(court_message(&game), Some("PAUSED - SPACE TO RESUME"));
        game.state = State::Over;
        game.scores = [7, 2]; assert_eq!(court_message(&game), Some("YOU WIN - ENTER OR R TO RESTART"));
        game.scores = [2, 7]; assert_eq!(court_message(&game), Some("CPU WINS - ENTER OR R TO RESTART"));
        game.scores = [7, 7]; assert_eq!(court_message(&game), Some("MATCH OVER - ENTER OR R TO RESTART"));
        game.start(); assert_eq!(court_message(&game), None);
    }
    #[test]
    fn focus_loss_then_immediate_resume_suppresses_held_key() {
        let mut game = Game::new(42);
        game.start();
        let mut gate = MovementGate::default();
        movement_with_pause(&mut game, &mut gate, [false; 4], false);
        let held = [true, false, false, false];
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), -1.);
        let mut accumulator = 0.05;
        lose_focus(&mut game, &mut gate, &mut accumulator);
        assert_eq!(game.state, State::Paused);
        assert_eq!(accumulator, 0.);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
        assert_eq!(game.state, State::Playing);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
        movement_with_pause(&mut game, &mut gate, [false; 4], false);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), -1.);
    }
    #[test]
    fn status_is_outside_court_and_inside_window() {
        for (width, height) in [(900., 500.), (1800., 1000.), (900., 700.)] {
            let (scale, ox, oy) = viewport(width, height);
            let (x, y) = status_position(ox, oy, scale);
            assert!(y >= oy + H * scale, "status overlaps court");
            assert!(x >= 0. && x < width);
            assert!(y + 14. * scale <= height);
        }
    }
    #[test]
    fn winner_message_tracks_winning_point_and_restart() {
        let mut game = Game::new(42);
        assert_eq!(winner_message(&game), None);
        for (scores, x, vx, message) in [
            (
                [6, 3],
                W + BALL - 1.,
                350.,
                "YOU WIN - ENTER OR R TO RESTART",
            ),
            (
                [2, 6],
                -BALL + 1.,
                -350.,
                "CPU WINS - ENTER OR R TO RESTART",
            ),
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
            assert_eq!(winner_message(&game), Some(message));
            game.start();
            assert_eq!(winner_message(&game), None);
        }
    }
    #[test]
    fn active_restart_blocks_held_direction_until_release() {
        let mut game = Game::new(42);
        game.start();
        let mut movement = MovementGate::default();
        movement_with_pause(&mut game, &mut movement, [false; 4], false);
        assert_eq!(
            movement_with_pause(&mut game, &mut movement, [true, false, false, false], false),
            -1.
        );
        restart(&mut game, &mut movement);
        assert_eq!(
            movement_with_pause(&mut game, &mut movement, [true, false, false, false], false),
            0.
        );
        movement_with_pause(&mut game, &mut movement, [false; 4], false);
        assert_eq!(
            movement_with_pause(&mut game, &mut movement, [true, false, false, false], false),
            -1.
        );
    }
    #[test]
    fn winning_score_keeps_held_movement_suppressed_on_restart() {
        for i in 0..4 {
            for (scores, x, vx) in [([6, 2], W + BALL - 1., 350.), ([2, 6], -BALL + 1., -350.)] {
                let mut game = Game::new(42);
                game.start();
                let mut gate = MovementGate::default();
                movement_with_pause(&mut game, &mut gate, [false; 4], false);
                let mut held = [false; 4];
                held[i] = true;
                assert_ne!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
                game.scores = scores;
                game.serve = 0.;
                game.ball.x = x;
                game.ball.y = H / 2.;
                game.ball.vx = vx;
                game.ball.vy = 0.;
                game.step(0.02, 0.);
                assert_eq!(game.state, State::Over);
                gate.finish_frame(game.state);
                game.start();
                assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
                held[i] = false;
                movement_with_pause(&mut game, &mut gate, held, false);
                held[i] = true;
                assert_ne!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
            }
        }
    }
    #[test]
    fn game_over_between_samples_blocks_held_on_immediate_restart() {
        let mut game = Game::new(42);
        game.start();
        let mut gate = MovementGate::default();
        movement_with_pause(&mut game, &mut gate, [false; 4], false);
        game.state = State::Over;
        gate.finish_frame(game.state);
        game.start();
        assert_eq!(
            movement_with_pause(&mut game, &mut gate, [true, false, false, false], false),
            0.
        );
    }
    #[test]
    fn movement_held_on_same_frame_as_resume_stays_suppressed() {
        let mut game = Game::new(42);
        game.state = State::Paused;
        let mut gate = MovementGate::default();
        let held = [true, false, false, false];
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
        assert_eq!(game.state, State::Playing);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
    }
    #[test]
    fn nonplaying_held_keys_do_not_move_after_resume() {
        for state in [State::Ready, State::Paused, State::Over] {
            for i in 0..4 {
                let mut game = Game::new(42);
                game.state = state;
                let mut gate = MovementGate::default();
                let mut held = [false; 4];
                held[i] = true;
                assert_eq!(
                    movement_with_pause(&mut game, &mut gate, held, false),
                    0.,
                    "{state:?}"
                );
                if state == State::Over {
                    game.start();
                } else {
                    movement_with_pause(&mut game, &mut gate, held, true);
                }
                assert_eq!(game.state, State::Playing);
                assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
                held[i] = false;
                movement_with_pause(&mut game, &mut gate, held, false);
                held[i] = true;
                assert_ne!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
            }
        }
    }

    #[test]
    fn pause_suppression_is_per_binding_and_opposites_cancel() {
        let mut gate = MovementGate::default();
        assert_eq!(gate.axis([true, true, false, false], false), -1.);
        assert_eq!(gate.axis([true, true, false, false], true), 0.);
        assert_eq!(gate.axis([true, false, false, false], false), 0.);
        assert_eq!(gate.axis([true, true, false, false], false), -1.);
        assert_eq!(gate.axis([true, true, true, false], false), 0.);
        assert_eq!(gate.axis([false, true, true, false], false), 0.);
    }
    #[test]
    fn pause_suppresses_held_keys_until_release_and_fresh_press() {
        for i in 0..4 {
            let mut game = Game::new(42);
            let mut gate = MovementGate::default();
            let mut held = [false; 4];
            held[i] = true;
            assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
            assert_eq!(game.state, State::Playing);
            held[i] = false;
            movement_with_pause(&mut game, &mut gate, held, false);
            held[i] = true;
            assert_ne!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
            assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
            assert_eq!(game.state, State::Paused);
            assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
            assert_eq!(game.state, State::Playing);
            held[i] = false;
            assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
            held[i] = true;
            assert_ne!(movement_with_pause(&mut game, &mut gate, held, false), 0.);
        }
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
    fn letterboxing_preserves_aspect() {
        assert_eq!(viewport(1800.0, 1072.0), (2.0, 0.0, 0.0));
        assert_eq!(viewport(1000.0, 536.0), (1.0, 50.0, 0.0));
        assert_eq!(viewport(900.0, 736.0), (1.0, 0.0, 100.0));
    }
}
