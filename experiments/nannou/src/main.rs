use nannou::prelude::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
use pong_scenario::Demo;
fn benchmark_args(args: &[String]) -> bool {
    let enabled = args.iter().any(|a| a == "--benchmark");
    assert!(
        !enabled || !args.iter().any(|a| a.starts_with("--scripted")),
        "benchmark and replay/capture modes are distinct"
    );
    enabled
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
struct Model {
    benchmarking: bool,
    benchmark_done: std::sync::atomic::AtomicBool,
    benchmark: std::sync::Mutex<Option<pong_benchmark::Frames>>,
    scenario: pong_scenario::Scenario,
    game: Game,
    movement: MovementGate,
    demo: Option<Demo>,
    pending: Option<(std::path::PathBuf, std::time::Instant)>,
    accumulator: f32,
}
fn main() {
    nannou::app(model).update(update).run();
}
fn model(app: &App) -> Model {
    let benchmarking = benchmark_args(&std::env::args().collect::<Vec<_>>());
    // Bound CPU work even when the platform falls back to immediate presentation.
    app.set_update_rate(120.0);
    app.new_window()
        .size(W as u32, H as u32)
        .title("Pong nannou")
        .view(view)
        .unfocused(unfocused)
        .build();
    Model {
        benchmarking,
        benchmark_done: std::sync::atomic::AtomicBool::new(false),
        benchmark: std::sync::Mutex::new(None),
        scenario: pong_scenario::Scenario::default(),
        game: Game::new(pong_scenario::SEED),
        movement: MovementGate::default(),
        demo: Demo::from_args(),
        pending: None,
        accumulator: 0.0,
    }
}
fn unfocused(_: &App, m: &mut Model) {
    if m.demo.is_some() || m.benchmarking {
        return;
    }
    m.game.pause();
    m.accumulator = 0.0;
}
fn update(app: &App, m: &mut Model) {
    if let Some((path, since)) = &m.pending {
        let complete = std::fs::read(path)
            .is_ok_and(|b| b.ends_with(&[0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130]));
        assert!(
            since.elapsed().as_secs() < 30,
            "screenshot readback timed out: {}",
            path.display()
        );
        if !complete {
            return;
        }
        m.pending = None;
    }
    if let Some(demo) = &mut m.demo {
        if demo.finished() {
            app.quit();
            return;
        }
        demo.advance(&mut m.game, app.time_delta());
        if let Some(path) = demo.capture_path() {
            app.main_window().save_screenshot(path.clone());
            m.pending = Some((path, std::time::Instant::now()));
        }
        return;
    }
    if m.benchmarking {
        m.accumulator += app.time_delta().min(0.1);
        while m.accumulator >= pong_scenario::STEP {
            if m.scenario.finished() {
                m.scenario = pong_scenario::Scenario::default();
                m.game = Game::new(pong_scenario::SEED);
            }
            m.scenario.advance(&mut m.game);
            m.accumulator -= pong_scenario::STEP;
        }
        return;
    }
    let k = app.keys();
    if k.just_pressed(KeyCode::Enter) || k.just_pressed(KeyCode::KeyR) {
        m.game.start();
    }
    if k.just_pressed(KeyCode::Escape) {
        app.quit();
    }
    let axis = movement_with_pause(
        &mut m.game,
        &mut m.movement,
        [
            k.pressed(KeyCode::ArrowUp),
            k.pressed(KeyCode::KeyW),
            k.pressed(KeyCode::ArrowDown),
            k.pressed(KeyCode::KeyS),
        ],
        k.just_pressed(KeyCode::Space),
    );
    m.accumulator += app.time_delta().min(0.1);
    while m.accumulator >= 1.0 / 120.0 {
        m.game.step(1.0 / 120.0, axis);
        m.accumulator -= 1.0 / 120.0;
    }
    m.movement.finish_frame(m.game.state);
    app.main_window().set_title(format!(
        "Pong nannou | {} : {} | {:?} | W/S arrows • Enter restart • Space pause",
        m.game.scores[0], m.game.scores[1], m.game.state
    ));
}
fn view(app: &App, m: &Model) {
    let d = app.draw();
    d.background().color(Srgba::new(0.025, 0.04, 0.09, 1.0));
    let size = app.window_rect();
    let d = d.scale((size.w() / W).min(size.h() / H));
    for y in (0..500).step_by(25) {
        d.rect()
            .x_y(0.0, H / 2.0 - y as f32)
            .w_h(2.0, 12.0)
            .color(Srgba::new(0.18, 0.25, 0.35, 1.0));
    }
    for (x, y) in [(38.0, m.game.player), (W - 38.0, m.game.cpu)] {
        d.rect()
            .x_y(x - W / 2.0, H / 2.0 - y)
            .w_h(PADDLE_W, PADDLE_H)
            .color(Srgba::new(0.35, 0.95, 0.85, 1.0));
    }
    d.ellipse()
        .x_y(m.game.ball.x - W / 2.0, H / 2.0 - m.game.ball.y)
        .radius(BALL)
        .color(WHITE);
    d.text(&format!("{}    {}", m.game.scores[0], m.game.scores[1]))
        .x_y(0.0, 190.0)
        .font_size(32)
        .color(WHITE);
    if m.game.state == State::Paused {
        d.text("PAUSED  •  SPACE TO RESUME")
            .x_y(0.0, -190.0)
            .font_size(24)
            .color(WHITE);
    }
    if m.benchmarking && !m.benchmark_done.load(std::sync::atomic::Ordering::Relaxed) {
        let size = app.main_window().size_pixels();
        let mut sampler = m.benchmark.lock().expect("benchmark sampler");
        let frames = sampler.get_or_insert_with(|| pong_benchmark::Frames::new(size.x, size.y));
        if frames.frame(size.x, size.y) {
            m.benchmark_done
                .store(true, std::sync::atomic::Ordering::Relaxed);
            println!("PONG_BENCHMARK {}", frames.report());
            app.quit();
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
    fn space_starts_pauses_and_resumes_without_restoring_hold() {
        let mut game = Game::new(42);
        let mut gate = MovementGate::default();
        let held = [true, false, false, false];
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
        movement_with_pause(&mut game, &mut gate, [false; 4], false);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), -1.);
        assert_eq!(game.state, State::Playing);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
        assert_eq!(game.state, State::Paused);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, true), 0.);
        assert_eq!(game.state, State::Playing);
        movement_with_pause(&mut game, &mut gate, [false; 4], false);
        assert_eq!(movement_with_pause(&mut game, &mut gate, held, false), -1.);
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
        for index in 0..4 {
            let mut gate = MovementGate::default();
            let mut held = [false; 4];
            held[index] = true;
            assert_ne!(gate.axis(held, false), 0.);
            assert_eq!(gate.axis(held, true), 0.);
            assert_eq!(gate.axis(held, false), 0.);
            held[index] = false;
            assert_eq!(gate.axis(held, false), 0.);
            held[index] = true;
            assert_ne!(gate.axis(held, false), 0.);
        }
    }
}
#[cfg(test)]
mod benchmark_tests {
    use super::benchmark_args;
    #[test]
    fn opt_in_only() {
        assert!(!benchmark_args(&[]));
        assert!(benchmark_args(&["--benchmark".into()]));
    }
    #[test]
    #[should_panic(expected = "distinct")]
    fn reject_combined_modes() {
        benchmark_args(&["--benchmark".into(), "--scripted-test".into()]);
    }
}
