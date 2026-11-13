use macroquad::prelude::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
#[derive(Default)]
struct MovementGate {
    suppressed: [bool; 4],
    was_playing: bool,
}
impl MovementGate {
    fn finish_frame(&mut self, state: State) {
        self.was_playing &= state == State::Playing;
    }
}
fn movement_axis(game: &Game, gate: &mut MovementGate, held: [bool; 4], lost_focus: bool) -> f32 {
    let playing = game.state == State::Playing;
    let suppress = lost_focus || !playing || !gate.was_playing;
    gate.was_playing = playing;
    for (blocked, down) in gate.suppressed.iter_mut().zip(held) {
        *blocked = down && (suppress || *blocked);
    }
    let active = std::array::from_fn::<_, 4, _>(|i| held[i] && !gate.suppressed[i]);
    (active[2] || active[3]) as u8 as f32 - (active[0] || active[1]) as u8 as f32
}
#[derive(Default)]
struct FocusEvents {
    lost: bool,
}
impl miniquad::EventHandler for FocusEvents {
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn window_minimized_event(&mut self) {
        self.lost = true;
    }
}
fn conf() -> Conf {
    Conf {
        window_title: "Pong comparison - Macroquad".into(),
        window_width: 1000,
        window_height: 680,
        high_dpi: true,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let benchmarking = std::env::args().any(|a| a == "--benchmark");
    assert!(
        !benchmarking || !std::env::args().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    let mut benchmark = benchmarking
        .then(|| pong_benchmark::Frames::new(screen_width() as u32, screen_height() as u32));
    let mut bench_scenario = pong_scenario::Scenario::default();
    let mut demo = pong_scenario::Demo::from_args();
    let scripted = demo.is_some() || benchmarking;
    let mut g = Game::new(if scripted {
        pong_scenario::SEED
    } else {
        miniquad::date::now() as u64
    });
    let mut accumulator = 0.;
    let subscriber = macroquad::input::utils::register_input_subscriber();
    let mut focus = FocusEvents::default();
    let mut movement = MovementGate::default();
    loop {
        macroquad::input::utils::repeat_all_miniquad_input(&mut focus, subscriber);
        let lost_focus = focus.lost;
        if !scripted {
            if focus.lost {
                g.pause();
                accumulator = 0.;
                focus.lost = false;
            }
            if is_key_pressed(KeyCode::Enter) {
                if g.state == State::Paused {
                    g.toggle_pause()
                } else if g.state != State::Playing {
                    g.start()
                }
            }
            if is_key_pressed(KeyCode::R) {
                g.start()
            }
            if is_key_pressed(KeyCode::Space) {
                g.toggle_pause()
            }
            if is_key_pressed(KeyCode::Escape) {
                g.pause()
            }
        }
        let input = movement_axis(
            &g,
            &mut movement,
            [
                is_key_down(KeyCode::Up),
                is_key_down(KeyCode::W),
                is_key_down(KeyCode::Down),
                is_key_down(KeyCode::S),
            ],
            lost_focus,
        );
        if benchmarking {
            accumulator += get_frame_time().min(0.1);
            while accumulator >= pong_scenario::STEP {
                if bench_scenario.finished() {
                    bench_scenario = pong_scenario::Scenario::default();
                    g = Game::new(pong_scenario::SEED);
                }
                bench_scenario.advance(&mut g);
                accumulator -= pong_scenario::STEP;
            }
        } else if let Some(demo) = &mut demo {
            demo.advance(&mut g, get_frame_time());
        } else {
            accumulator += get_frame_time().min(0.1);
            while accumulator >= 1. / 120. {
                g.step(1. / 120., input);
                accumulator -= 1. / 120.;
            }
            movement.finish_frame(g.state);
        }
        clear_background(Color::from_hex(0x0c211e));
        let scale = (screen_width() / W)
            .min((screen_height() - 140.) / H)
            .max(0.1);
        let ox = (screen_width() - W * scale) / 2.;
        let oy = 95.;
        draw_text("RALLYLIGHT / MACROQUAD", 20., 30., 24., WHITE);
        draw_text(
            format!("YOU {}    CPU {}", g.scores[0], g.scores[1]),
            20.,
            65.,
            28.,
            Color::from_hex(0xd9ffae),
        );
        draw_rectangle(ox, oy, W * scale, H * scale, Color::from_hex(0x153f37));
        for y in (0..500).step_by(28) {
            draw_rectangle(
                ox + 450. * scale,
                oy + y as f32 * scale,
                2. * scale,
                12. * scale,
                GRAY,
            )
        }
        for (x, y) in [(32., g.player), (W - 44., g.cpu)] {
            draw_rectangle(
                ox + x * scale,
                oy + (y - PADDLE_H / 2.) * scale,
                PADDLE_W * scale,
                PADDLE_H * scale,
                Color::from_hex(0xd9ffae),
            );
        }
        draw_circle(
            ox + g.ball.x * scale,
            oy + g.ball.y * scale,
            BALL * scale,
            WHITE,
        );
        if g.state != State::Playing {
            draw_text(
                format!("{:?} - Enter to play", g.state),
                ox + 220. * scale,
                oy + 250. * scale,
                28. * scale,
                WHITE,
            );
        }
        draw_text(
            "W/S or arrows | Enter start | Space pause | R restart | Esc pause",
            20.,
            screen_height() - 20.,
            19.,
            WHITE,
        );
        if let Some(demo) = &demo {
            draw_text(
                format!(
                    "SCRIPTED INPUT / tick {} / {}",
                    demo.tick(),
                    pong_scenario::END_TICK
                ),
                440.,
                30.,
                20.,
                YELLOW,
            );
        }
        if let Some(path) = demo.as_mut().and_then(|d| d.capture_path()) {
            get_screen_data().export_png(path.to_str().expect("UTF-8 capture path"));
        }
        next_frame().await;
        if let Some(bench) = &mut benchmark {
            if bench.frame(screen_width() as u32, screen_height() as u32) {
                println!("PONG_BENCHMARK {}", bench.report());
                break;
            }
        }
        if demo.as_ref().is_some_and(|d| d.finished()) {
            break;
        }
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    #[test]
    fn winning_score_keeps_held_movement_suppressed_on_restart() {
        for i in 0..4 {
            for (scores, x, vx) in [([6, 2], W + BALL - 1., 350.), ([2, 6], -BALL + 1., -350.)] {
                let mut game = Game::new(42);
                game.start();
                let mut gate = MovementGate::default();
                movement_axis(&game, &mut gate, [false; 4], false);
                let mut held = [false; 4];
                held[i] = true;
                assert_ne!(movement_axis(&game, &mut gate, held, false), 0.);
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
                assert_eq!(movement_axis(&game, &mut gate, held, false), 0.);
                held[i] = false;
                movement_axis(&game, &mut gate, held, false);
                held[i] = true;
                assert_ne!(movement_axis(&game, &mut gate, held, false), 0.);
            }
        }
    }
    #[test]
    fn game_over_between_samples_blocks_held_on_immediate_restart() {
        let mut game = Game::new(42);
        game.start();
        let mut gate = MovementGate::default();
        movement_axis(&game, &mut gate, [false; 4], false);
        game.state = State::Over;
        gate.finish_frame(game.state);
        game.start();
        assert_eq!(
            movement_axis(&game, &mut gate, [true, false, false, false], false),
            0.
        );
    }
    #[test]
    fn same_frame_start_and_focus_loss_suppress_held_keys_independently() {
        let mut game = Game::new(42);
        let mut gate = MovementGate::default();
        game.start();
        assert_eq!(
            movement_axis(&game, &mut gate, [true, false, false, false], false),
            0.
        );
        movement_axis(&game, &mut gate, [false; 4], false);
        assert_eq!(
            movement_axis(&game, &mut gate, [true, false, false, false], false),
            -1.
        );
        assert_eq!(
            movement_axis(&game, &mut gate, [true, false, false, false], true),
            0.
        );
        assert_eq!(
            movement_axis(&game, &mut gate, [true, true, false, false], false),
            -1.
        );
        assert_eq!(
            movement_axis(&game, &mut gate, [true, false, false, false], false),
            0.
        );
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
                    movement_axis(&game, &mut gate, held, false),
                    0.,
                    "{state:?}"
                );
                game.state = State::Playing;
                assert_eq!(movement_axis(&game, &mut gate, held, false), 0.);
                held[i] = false;
                movement_axis(&game, &mut gate, held, false);
                held[i] = true;
                assert_ne!(movement_axis(&game, &mut gate, held, false), 0.);
            }
        }
    }
}
