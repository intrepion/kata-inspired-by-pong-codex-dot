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
fn restart_game(game: &mut Game, gate: &mut MovementGate) {
    game.start();
    gate.was_playing = false;
}
fn status_message(game: &Game) -> Option<&'static str> {
    match game.state {
        State::Ready => Some("Ready - Enter to play"),
        State::Playing => None,
        State::Paused => Some("Paused - Enter to play"),
        State::Over if game.scores[0] > game.scores[1] => Some("YOU WIN - Enter to restart"),
        State::Over if game.scores[1] > game.scores[0] => Some("CPU WINS - Enter to restart"),
        State::Over => Some("Match over - Enter to restart"),
    }
}
fn apply_frame_actions(game: &mut Game, movement: &mut MovementGate, lost_focus: bool, actions: [bool; 5]) {
    if lost_focus { return; }
    if actions[0] || actions[4] {
        if game.state == State::Paused { game.toggle_pause(); }
        else if game.state != State::Playing { restart_game(game, movement); }
    }
    if actions[1] { restart_game(game, movement); }
    if actions[2] { game.toggle_pause(); }
    if actions[3] { game.pause(); }
}
fn apply_timed_actions(game: &mut Game, movement: &mut MovementGate, lost_focus: bool, actions: [bool; 5], accumulator: &mut f32, elapsed: f32) -> f32 {
    let before = game.state;
    apply_frame_actions(game, movement, lost_focus, actions);
    if lost_focus || before != game.state || actions[1] {
        *accumulator = 0.;
        0.
    } else {
        elapsed.min(0.1)
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
        let mut live_elapsed = 0.;
        if !scripted {
            if focus.lost {
                g.pause();
                accumulator = 0.;
                focus.lost = false;
            }
            live_elapsed = apply_timed_actions(&mut g, &mut movement, lost_focus, [
                is_key_pressed(KeyCode::Enter), is_key_pressed(KeyCode::R),
                is_key_pressed(KeyCode::Space), is_key_pressed(KeyCode::Escape),
                is_key_pressed(KeyCode::KpEnter),
            ], &mut accumulator, get_frame_time());
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
            accumulator += live_elapsed;
            while accumulator >= 1. / 120. {
                g.step(1. / 120., input);
                accumulator -= 1. / 120.;
            }
            movement.finish_frame(g.state);
        }
        clear_background(Color::from_hex(0x0c211e));
        let ui_scale = (screen_height() / 240.).clamp(0., 1.);
        let scale = (screen_width() / W)
            .min((screen_height() - 140. * ui_scale) / H)
            .max(0.);
        let ox = (screen_width() - W * scale) / 2.;
        let oy = 95. * ui_scale;
        draw_text("RALLYLIGHT / MACROQUAD", 20. * ui_scale, 30. * ui_scale, 24. * ui_scale, WHITE);
        draw_text(
            format!("YOU {}    CPU {}", g.scores[0], g.scores[1]),
            20. * ui_scale,
            65. * ui_scale,
            28. * ui_scale,
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
        if let Some(message) = status_message(&g) {
            let width = measure_text(message, None, 28, 1.0).width;
            let x = ox + (W - width) * scale / 2.;
            let y = oy + 330. * scale;
            draw_rectangle(
                x - 12. * scale,
                y - 32. * scale,
                (width + 24.) * scale,
                40. * scale,
                Color::from_hex(0x153f37),
            );
            draw_text(message, x, y, 28. * scale, WHITE);
        }
        let controls = "W/S or arrows | Enter start | Space pause | R restart | Esc pause";
        let controls_width = measure_text(controls, None, 19, 1.0).width;
        let controls_scale = ((screen_width() - 40. * ui_scale).max(0.) / controls_width).min(ui_scale);
        draw_text(controls, 20. * ui_scale, screen_height() - 20. * ui_scale, 19. * controls_scale, WHITE);
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
    fn actions_discard_pretransition_time_and_partial_step() {
        let mut game = Game::new(42); let mut gate = MovementGate::default(); let mut acc = 0.007;
        for (actions, expected) in [
            ([true,false,false,false,false], State::Playing),
            ([false,false,true,false,false], State::Paused),
            ([false,false,false,false,true], State::Playing),
            ([false,true,false,false,false], State::Playing),
        ] {
            let elapsed = apply_timed_actions(&mut game, &mut gate, false, actions, &mut acc, 0.09);
            assert_eq!(game.state, expected); assert_eq!(acc, 0., "old partial time survived");
            assert_eq!(elapsed, 0., "pre-transition time survived"); acc = 0.007;
        }
        let elapsed = apply_timed_actions(&mut game, &mut gate, false, [false; 5], &mut acc, 0.02);
        assert_eq!(acc, 0.007); assert_eq!(elapsed, 0.02);
        let elapsed = apply_timed_actions(&mut game, &mut gate, false, [true,false,false,false,false], &mut acc, 0.02);
        assert_eq!(acc, 0.007); assert_eq!(elapsed, 0.02, "Enter while playing does not restart");
        let elapsed = apply_timed_actions(&mut game, &mut gate, true, [false,true,false,false,false], &mut acc, 0.09);
        assert_eq!(acc, 0.); assert_eq!(elapsed, 0., "focus loss must discard elapsed time");
    }
    #[test]
    fn both_enter_keys_start_resume_and_restart_once() {
        for (main,keypad) in [(true,false),(false,true),(true,true)] {
            let mut game=Game::new(42);let mut gate=MovementGate::default();let actions=[main,false,false,false,keypad];
            apply_frame_actions(&mut game,&mut gate,false,actions);assert_eq!(game.state,State::Playing);
            game.pause();apply_frame_actions(&mut game,&mut gate,false,actions);assert_eq!(game.state,State::Playing);
            game.state=State::Over;game.scores=[7,2];apply_frame_actions(&mut game,&mut gate,false,actions);assert_eq!(game.state,State::Playing);assert_eq!(game.scores,[0,0]);
        }
    }
    #[test]
    fn keypad_resume_keeps_held_movement_blocked_until_release() {
        for i in 0..4 {
            let mut game=Game::new(42);let mut gate=MovementGate::default();
            apply_frame_actions(&mut game,&mut gate,false,[false,false,false,false,true]);
            let mut held=[false;4];movement_axis(&game,&mut gate,held,false);held[i]=true;assert_ne!(movement_axis(&game,&mut gate,held,false),0.);
            game.pause();assert_eq!(movement_axis(&game,&mut gate,held,false),0.);
            apply_frame_actions(&mut game,&mut gate,false,[false,false,false,false,true]);assert_eq!(movement_axis(&game,&mut gate,held,false),0.);
            held[i]=false;movement_axis(&game,&mut gate,held,false);held[i]=true;assert_ne!(movement_axis(&game,&mut gate,held,false),0.);
        }
    }
    #[test]
    fn focus_loss_frame_cannot_restart_or_resume_from_queued_actions() {
        for i in [0,1,2,4] {
            let mut game=Game::new(42);game.start();game.pause();let mut gate=MovementGate::default();
            let mut actions=[false;5];actions[i]=true;
            apply_frame_actions(&mut game,&mut gate,true,actions);
            assert_eq!(game.state,State::Paused,"action {i} resumed on focus-loss frame");
            apply_frame_actions(&mut game,&mut gate,false,actions);
            assert_eq!(game.state,State::Playing,"fresh action {i} should work on a later frame");
        }
    }
    #[test]
    fn status_names_winner_and_clears_after_restart() {
        let mut game = Game::new(42);
        assert_eq!(status_message(&game), Some("Ready - Enter to play"));
        game.start();
        assert_eq!(status_message(&game), None);
        game.pause();
        assert_eq!(status_message(&game), Some("Paused - Enter to play"));
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
            assert_eq!(status_message(&game), Some(message));
            game.start();
            assert_eq!(status_message(&game), None);
        }
    }
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

#[cfg(test)]
mod active_restart_tests {
    use super::*;
    #[test]
    fn held_key_must_not_survive_active_restart() {
        for i in 0..4 {
            let mut game = Game::new(42);
            let mut gate = MovementGate::default();
            game.start();
            movement_axis(&game, &mut gate, [false; 4], false);
            let mut held = [false; 4];
            held[i] = true;
            assert_ne!(movement_axis(&game, &mut gate, held, false), 0.);
            restart_game(&mut game, &mut gate);
            assert_eq!(movement_axis(&game, &mut gate, held, false), 0.);
            held[i] = false;
            movement_axis(&game, &mut gate, held, false);
            held[i] = true;
            assert_ne!(movement_axis(&game, &mut gate, held, false), 0.);
        }
    }
}
