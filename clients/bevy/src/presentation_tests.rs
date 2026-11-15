use super::*;

#[test]
fn present_reports_each_winner_and_clears_status_on_restart() {
    for (scores, x, vx, expected) in [
        ([6, 2], W + BALL - 1., 350., "You won! - Enter to play again"),
        ([2, 6], -BALL + 1., -350., "Computer wins - Enter to play again"),
    ] {
        let mut game = Game::new(42);
        game.start();
        game.scores = scores;
        game.serve = 0.;
        game.ball.x = x;
        game.ball.y = H / 2.;
        game.ball.vx = vx;
        game.ball.vy = 0.;
        game.step(0.02, 0.);
        assert_eq!(game.state, State::Over);

        let mut app = App::new();
        app.insert_resource(Match(game)).add_systems(Update, present);
        let status = app.world_mut().spawn((Text2d::new(""), Status)).id();
        let score = app.world_mut().spawn((Text2d::new(""), Score)).id();
        app.update();
        assert_eq!(app.world().get::<Text2d>(status).unwrap().0, expected);
        let scores = app.world().resource::<Match>().0.scores;
        assert_eq!(app.world().get::<Text2d>(score).unwrap().0, format!("YOU  {} : {}  CPU", scores[0], scores[1]));

        app.world_mut().resource_mut::<Match>().0.start();
        app.update();
        assert_ne!(app.world().get::<Text2d>(status).unwrap().0, expected);
        assert_eq!(app.world().get::<Text2d>(score).unwrap().0, "YOU  0 : 0  CPU");
    }
}
