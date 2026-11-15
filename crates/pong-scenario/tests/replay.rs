use pong_core::{Game, State};
use pong_scenario::{Scenario, END_TICK, SEED};

fn advance_to(s: &mut Scenario, g: &mut Game, tick: u32) {
    while s.tick() < tick {
        s.advance(g);
    }
}

#[test]
fn scheduled_inputs_move_clamp_release_pause_resume_and_restart() {
    let mut s = Scenario::default();
    let mut g = Game::new(SEED);
    assert_eq!(g.state, State::Ready);
    advance_to(&mut s, &mut g, 120);
    assert_eq!(g.state, State::Playing);
    assert_eq!(g.player, 45.0);
    advance_to(&mut s, &mut g, 240);
    assert_eq!(g.player, 455.0);
    advance_to(&mut s, &mut g, 360);
    assert_eq!(g.player, 455.0, "released input must not keep moving");
    let before_pause = (g.ball.x, g.ball.y, g.cpu, g.scores);
    advance_to(&mut s, &mut g, 480);
    assert_eq!(g.state, State::Paused);
    assert_eq!((g.ball.x, g.ball.y, g.cpu, g.scores), before_pause);
    s.advance(&mut g);
    assert_eq!(g.state, State::Playing);
    advance_to(&mut s, &mut g, 1200);
    // Confirm restart clears real state rather than just changing the label.
    g.scores = [3, 4];
    s.advance(&mut g);
    assert_eq!(g.scores, [0, 0]);
    assert_eq!(g.player, 250.0);
    assert_eq!(g.state, State::Playing);
}

#[test]
fn repeat_is_deterministic_and_bounded_even_with_different_render_batches() {
    let (mut a, mut b) = (Scenario::default(), Scenario::default());
    let (mut ga, mut gb) = (Game::new(SEED), Game::new(SEED));
    advance_to(&mut a, &mut ga, END_TICK);
    for _ in 0..(END_TICK / 12) {
        for _ in 0..12 {
            b.advance(&mut gb);
        }
    }
    assert!(a.finished());
    assert_eq!(a.report(&ga), b.report(&gb));
    let final_report = a.report(&ga);
    a.advance(&mut ga);
    assert_eq!(a.report(&ga), final_report);
}

#[test]
fn shared_demo_clock_preserves_capture_checkpoints_under_slow_frames() {
    use pong_scenario::Demo;
    let mut demo = Demo::new(Some("unused-test-directory".into()));
    let mut game = Game::new(SEED);
    demo.advance(&mut game, f32::NAN);
    assert_eq!(demo.tick(), 0);
    let mut captured = Vec::new();
    while !demo.finished() {
        demo.advance(&mut game, 0.1);
        if demo.capture_path().is_some() {
            captured.push(demo.tick());
            assert!(demo.capture_path().is_none(), "capture once per checkpoint");
        }
    }
    assert_eq!(captured, [120, 240, 480, 600, 1320, 1440]);
    let mut baseline = Scenario::default();
    let mut expected = Game::new(SEED);
    advance_to(&mut baseline, &mut expected, END_TICK);
    assert_eq!(game.scores, expected.scores);
    assert_eq!(game.ball.x, expected.ball.x);
    assert_eq!(game.ball.y, expected.ball.y);
}

#[test]
fn cpu_capture_encodes_exact_rgb_values() {
    let path = std::env::temp_dir().join(format!("pong-png-test-{}.png", std::process::id()));
    pong_scenario::Demo::save_rgb(&path, &[0x00ff0000, 0x0000ff00], 2, 1);
    let decoder = png::Decoder::new(std::fs::File::open(&path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut bytes = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut bytes).unwrap();
    assert_eq!((info.width, info.height), (2, 1));
    assert_eq!(&bytes[..info.buffer_size()], &[255, 0, 0, 0, 255, 0]);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn repeated_update_callbacks_hold_pending_frame_until_renderer_consumes_it() {
    let mut demo = pong_scenario::Demo::new(Some("unused-test-directory".into()));
    let mut game = Game::new(SEED);
    while demo.tick() < 120 {
        demo.advance(&mut game, 0.1);
    }
    let ball = (game.ball.x, game.ball.y);
    for _ in 0..20 {
        demo.advance(&mut game, 0.1);
    }
    assert_eq!(demo.tick(), 120);
    assert_eq!((game.ball.x, game.ball.y), ball);
    assert!(demo.capture_path().is_some());
    demo.advance(&mut game, 0.1);
    assert!(demo.tick() > 120);
}
