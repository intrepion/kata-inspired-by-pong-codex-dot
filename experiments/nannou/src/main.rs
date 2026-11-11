use nannou::prelude::*;
use pong_core::{Game, State, BALL, H, PADDLE_H, PADDLE_W, W};
struct Model {
    game: Game,
    accumulator: f32,
}
fn main() {
    nannou::app(model).update(update).run();
}
fn model(app: &App) -> Model {
    // Bound CPU work even when the platform falls back to immediate presentation.
    app.set_update_rate(120.0);
    app.new_window()
        .size(W as u32, H as u32)
        .title("Pong nannou")
        .view(view)
        .unfocused(unfocused)
        .build();
    Model {
        game: Game::new(42),
        accumulator: 0.0,
    }
}
fn unfocused(_: &App, m: &mut Model) {
    m.game.pause();
    m.accumulator = 0.0;
}
fn update(app: &App, m: &mut Model) {
    let k = app.keys();
    if k.just_pressed(KeyCode::Enter) || k.just_pressed(KeyCode::KeyR) {
        m.game.start();
    }
    if k.just_pressed(KeyCode::Space) {
        if m.game.state == State::Ready {
            m.game.start();
        } else {
            m.game.toggle_pause();
        }
    }
    if k.just_pressed(KeyCode::Escape) {
        app.quit();
    }
    let axis = (k.pressed(KeyCode::ArrowDown) || k.pressed(KeyCode::KeyS)) as u8 as f32
        - (k.pressed(KeyCode::ArrowUp) || k.pressed(KeyCode::KeyW)) as u8 as f32;
    m.accumulator += app.time_delta().min(0.1);
    while m.accumulator >= 1.0 / 120.0 {
        m.game.step(1.0 / 120.0, axis);
        m.accumulator -= 1.0 / 120.0;
    }
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
}
