use piston_window::graphics::{clear, ellipse, rectangle, Transformed};

use piston_window::*;
use pong_core::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Pong Piston | Enter: start | W/S/arrows: move | Space: pause | R: restart",
        [1000, 680],
    )
    .exit_on_esc(false)
    .resizable(true)
    .build()
    .expect("create Piston window");
    window.set_ups(120);
    let mut game = Game::new(42);
    let mut held = [false; 4];
    let mut pressed = std::collections::HashSet::new();
    let mut acc = 0.;
    let mut last_title = String::new();
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
                if key == k {
                    held[i] = true;
                }
            }
            if pressed.insert(key) {
                match key {
                    Key::Return => {
                        if game.state == State::Paused {
                            game.toggle_pause();
                        } else if game.state != State::Playing {
                            game.start();
                        }
                    }
                    Key::R => game.start(),
                    Key::Space => game.toggle_pause(),
                    Key::Escape => game.pause(),
                    _ => {}
                }
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            pressed.remove(&key);
            for (i, k) in [Key::W, Key::Up, Key::S, Key::Down].into_iter().enumerate() {
                if key == k {
                    held[i] = false;
                }
            }
        }
        if let Some(false) = e.focus_args() {
            game.pause();
            held = [false; 4];
            pressed.clear();
            acc = 0.;
        }
        if let Some(u) = e.update_args() {
            acc += u.dt.min(0.1);
            while acc >= 1. / 120. {
                game.step(
                    1. / 120.,
                    (held[2] || held[3]) as u8 as f32 - (held[0] || held[1]) as u8 as f32,
                );
                acc -= 1. / 120.;
            }
        }
        if e.render_args().is_some() {
            let title = format!("Pong Piston | YOU {} : {} CPU | {:?} | Enter: play | W/S/arrows | Space: pause | R: restart",game.scores[0],game.scores[1],game.state);
            if title != last_title {
                window.set_title(title.clone());
                last_title = title;
            }
        }
        window.draw_2d(&e, |c, g, _| {
            clear([0.047, 0.129, 0.118, 1.], g);
            let size = c.get_view_size();
            let s = (size[0] / W as f64).min(size[1] / H as f64);
            let t = c
                .transform
                .trans((size[0] - W as f64 * s) / 2., (size[1] - H as f64 * s) / 2.)
                .scale(s, s);
            rectangle(
                [0.082, 0.247, 0.216, 1.],
                [0., 0., W as f64, H as f64],
                t,
                g,
            );
            for y in (0..500).step_by(28) {
                rectangle([0.27, 0.43, 0.35, 1.], [449., y as f64, 2., 12.], t, g);
            }
            for (x, y) in [(32., game.player), (W - 44., game.cpu)] {
                rectangle(
                    [0.85, 1., 0.68, 1.],
                    [
                        x as f64,
                        (y - PADDLE_H / 2.) as f64,
                        PADDLE_W as f64,
                        PADDLE_H as f64,
                    ],
                    t,
                    g,
                );
            }
            ellipse(
                [1.; 4],
                [
                    (game.ball.x - BALL) as f64,
                    (game.ball.y - BALL) as f64,
                    (BALL * 2.) as f64,
                    (BALL * 2.) as f64,
                ],
                t,
                g,
            );
            // Score bars avoid platform-dependent font files; full numeric score/status in title.
            for (side, n) in game.scores.iter().enumerate() {
                for i in 0..*n {
                    rectangle(
                        [0.85, 1., 0.68, 1.],
                        [330. + side as f64 * 170. + i as f64 * 12., 20., 8., 18.],
                        t,
                        g,
                    );
                }
            }
        });
    }
}
