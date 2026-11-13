use pong_core::{BALL, Ball, H, PADDLE_H, State, W, WIN};
use pong_qtbridge::Controller;
const STEP: f32 = 1.0 / 120.0;
fn running() -> Controller {
    let mut c = Controller::default();
    c.start();
    c.game.serve = 0.0;
    c
}
#[test]
fn qml_snapshot_contract() {
    let c = Controller::default();
    assert_eq!(c.snapshot(), vec![250., 250., 450., 250., 0., 0., 0.]);
}
#[test]
fn paddle_respects_both_edges() {
    let mut c = running();
    for _ in 0..200 {
        c.tick(STEP, -1.0);
    }
    assert_eq!(c.game.player, PADDLE_H / 2.);
    for _ in 0..400 {
        c.tick(STEP, 1.0);
    }
    assert_eq!(c.game.player, H - PADDLE_H / 2.);
}
#[test]
fn paused_snapshot_is_frozen_then_resumes() {
    let mut c = running();
    c.tick(STEP, 1.0);
    c.pause();
    let before = c.snapshot();
    for _ in 0..30 {
        c.tick(STEP, -1.0);
    }
    assert_eq!(before, c.snapshot());
    c.toggle();
    c.tick(STEP, -1.0);
    assert_eq!(c.game.state, State::Playing);
    assert_ne!(c.game.player, before[0] as f32);
}
#[test]
fn controller_drives_paddle_and_wall_bounces() {
    let mut c = running();
    c.game.ball = Ball {
        x: 54.,
        y: 250.,
        vx: -350.,
        vy: 0.,
    };
    c.tick(STEP, 0.);
    assert!(c.game.ball.vx > 0.);
    c.game.ball = Ball {
        x: 450.,
        y: BALL + 0.5,
        vx: 100.,
        vy: -350.,
    };
    c.tick(STEP, 0.);
    assert!(c.game.ball.vy > 0.);
}
#[test]
fn goal_game_over_freeze_and_restart() {
    let mut c = running();
    for n in 1..=WIN {
        c.game.serve = 0.;
        c.game.ball = Ball {
            x: W + BALL + 1.,
            y: 30.,
            vx: 350.,
            vy: 0.,
        };
        c.tick(STEP, 0.);
        assert_eq!(c.snapshot()[4], n as f64);
    }
    assert_eq!(c.snapshot()[6], 3.);
    let end = c.snapshot();
    c.tick(0.1, 1.);
    assert_eq!(end, c.snapshot());
    c.toggle();
    assert_eq!(c.game.state, State::Playing);
    assert_eq!(c.game.scores, [0, 0]);
}
#[test]
fn irregular_frames_match_same_fixed_steps() {
    let mut a = running();
    let mut b = running();
    for _ in 0..30 {
        a.tick(STEP * 2., 1.);
        b.tick(STEP, 1.);
        b.tick(STEP, 1.);
    }
    assert_eq!(a.snapshot(), b.snapshot());
}
