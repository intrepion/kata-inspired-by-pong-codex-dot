use super::*;

#[test]
fn cpu_target_arrival_is_evaluated_at_paddle_face_contact() {
    for (target, ball_y, expect_return) in [(251., 304., true), (252., 304.2, false)] {
        let mut game = Game::new(42);
        game.start();
        game.serve = 0.;
        game.cpu = 250.;
        game.cpu_target = target;
        game.cpu_reaction = 1.;
        game.ball = Ball {
            x: 846.,
            y: ball_y,
            vx: 350.,
            vy: 0.,
        };
        // The first target is reached before the ball crosses x=848; the
        // second is not. End-of-step interpolation misclassifies both shots.
        game.step(1. / 120., 0.);
        assert_eq!(game.ball.vx < 0., expect_return, "target={target}");
    }
}
