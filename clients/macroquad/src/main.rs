use macroquad::prelude::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
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
    let scripted = std::env::args().any(|arg| arg == "--scripted-test");
    let mut scenario = pong_scenario::Scenario::default();
    let mut g = Game::new(if scripted {
        pong_scenario::SEED
    } else {
        miniquad::date::now() as u64
    });
    if scripted {
        println!("{}", scenario.report(&g));
    }
    let mut accumulator = 0.;
    let subscriber = macroquad::input::utils::register_input_subscriber();
    let mut focus = FocusEvents::default();
    loop {
        macroquad::input::utils::repeat_all_miniquad_input(&mut focus, subscriber);
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
        let input = (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) as i32
            - (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) as i32;
        accumulator += get_frame_time().min(0.1);
        while accumulator >= 1. / 120. {
            if scripted {
                scenario.advance(&mut g);
                if scenario.tick() % 120 == 0 {
                    println!("{}", scenario.report(&g));
                }
            } else {
                g.step(1. / 120., input as f32);
            }
            accumulator -= 1. / 120.;
            if scripted && scenario.finished() {
                break;
            }
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
        if scripted {
            draw_text(
                format!(
                    "SCRIPTED INPUT / tick {} / {}",
                    scenario.tick(),
                    pong_scenario::END_TICK
                ),
                440.,
                30.,
                20.,
                YELLOW,
            );
        }
        next_frame().await;
        if scripted && scenario.finished() {
            break;
        }
    }
}
