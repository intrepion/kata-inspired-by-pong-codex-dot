use pong_low_level_common::{raster, App, State, W, H, BALL};

fn pixel(app: &App, x: usize, y: usize) -> u32 {
    let mut pixels = vec![0; 900 * 500];
    raster(app, &mut pixels, 900, 500);
    pixels[y * 900 + x]
}

#[test]
fn status_backing_covers_net_only_outside_play() {
    let mut app = App::default();
    assert_eq!(pixel(&app, 449, 230), 0x0c1423, "ready");
    app.action();
    assert_ne!(pixel(&app, 449, 230), 0x0c1423, "playing net remains visible");
    app.action();
    assert_eq!(pixel(&app, 449, 230), 0x0c1423, "paused");
}

#[test]
fn winner_backing_covers_the_net_after_a_winning_point() {
    for (scores, x, vx) in [([6, 2], W + BALL - 1., 350.), ([2, 6], -BALL + 1., -350.)] {
        let mut app = App::default();
        app.game.start();
        app.game.scores = scores;
        app.game.serve = 0.;
        app.game.ball.x = x;
        app.game.ball.y = H / 2.;
        app.game.ball.vx = vx;
        app.game.ball.vy = 0.;
        app.game.step(0.02, 0.);
        assert_eq!(app.game.state, State::Over);
        assert_eq!(pixel(&app, 449, 230), 0x0c1423);
    }
}

#[test]
fn ready_and_pause_show_in_court_instructions_only_outside_play() {
    fn orange_pixels(app: &App) -> usize {
        let mut pixels = vec![0; 900 * 500];
        raster(app, &mut pixels, 900, 500);
        (290..311).flat_map(|y| (280..620).map(move |x| y * 900 + x))
            .filter(|&i| pixels[i] == 0xffb450).count()
    }
    let mut app = App::default();
    assert!(orange_pixels(&app) > 0, "Ready needs start instructions");
    app.action();
    assert_eq!(orange_pixels(&app), 0, "Playing has no instruction");
    app.action();
    assert!(orange_pixels(&app) > 0, "Paused needs resume instructions");
}
