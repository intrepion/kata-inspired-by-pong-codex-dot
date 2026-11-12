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
